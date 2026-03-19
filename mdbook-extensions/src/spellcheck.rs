use std::{
    borrow::Cow,
    collections::HashSet,
    fs,
    io::Write,
    process::{Command, Stdio, exit},
};

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd, TextMergeStream};
use serde::Deserialize;
use serde_yaml_ng::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Config {
    extra_dict: Option<String>,

    /// false as default
    #[serde(default)]
    allow_errors: bool,
}

pub(crate) fn check<'a>(
    lang: &str,
    config: &crate::extensions::ExtConfig,
    meta: &Value,
    content: &'a str,
) -> Cow<'a, str> {
    match &config.spell_checker {
        Some(config) => check_with_conf(lang, config, meta, content),
        None => {
            log::info!("spell-checker preprocessor is disabled");
            Cow::Borrowed(content)
        }
    }
}

fn check_with_conf<'a>(
    lang: &str,
    config: &Config,
    _meta: &Value,
    content: &'a str,
) -> Cow<'a, str> {
    let options = Options::all();
    let parser = TextMergeStream::new(Parser::new_ext(content, options));
    let mut ignore = false;
    let mut words = HashSet::new();
    for event in parser {
        if let Event::Start(Tag::MetadataBlock(_)) = event {
            ignore = true
        }
        if let Event::Start(Tag::CodeBlock(_)) = event {
            ignore = true
        }
        if let Event::End(TagEnd::MetadataBlock(_)) = event {
            ignore = false
        }
        if let Event::End(TagEnd::CodeBlock) = event {
            ignore = false
        }
        if let Event::Text(text) = event {
            if !ignore {
                for word in text.split_whitespace() {
                    log::debug!("word: {word}");
                    words.insert(word.to_owned());
                }
            }
        }
    }

    let words = words.into_iter().collect::<Vec<_>>().join("\n");

    let mut cmd = Command::new("aspell");

    cmd.arg("list").arg("--encoding").arg("utf-8");
    if let Some(extra_dict) = &config.extra_dict {
        if !fs::exists(extra_dict).expect("Cannot check existence of dictionnary") {
            log::error!("Missing dictionary {extra_dict}",);
            exit(1)
        }
        cmd.args(["--personal", &format!("./{extra_dict}")]);
    }
    cmd.arg("-l")
        .arg(lang)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped());
    let mut child = cmd.spawn().expect("Failed to spawn aspell");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(words.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");
    if !output.status.success() {
        log::error!("fail checking spelling");
        exit(1);
    }
    log::debug!("output: {}", String::from_utf8_lossy(&output.stdout));
    let output = String::from_utf8_lossy(&output.stdout);
    for word in output.lines() {
        log::warn!("Unknown word: {word}")
    }
    if !output.is_empty() && !config.allow_errors {
        log::error!("Stopping after founding unknown words");
        exit(1)
    }

    // Write to String buffer.
    Cow::Borrowed(content)
}
