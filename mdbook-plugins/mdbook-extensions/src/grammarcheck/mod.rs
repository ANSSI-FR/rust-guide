use std::{borrow::Cow, process::exit};

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use serde::Deserialize;
use serde_yaml_ng::Value;

mod grammalecte;
// mod server;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Config {
    // extra_dict: Option<String>,
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
    match &config.grammar_checker {
        Some(config) => check_with_conf(lang, config, meta, content),
        None => {
            log::info!("grammar-checker preprocessor is disabled");
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
    let parser = Parser::new_ext(content, options);
    let mut ignore = false;
    let mut full_text = String::new();
    for event in parser {
        if let Event::Start(Tag::Paragraph) = event {
            full_text.push('\n');
        }
        if let Event::Start(Tag::Heading { .. }) = event {
            full_text.push('\n');
        }
        if let Event::Start(Tag::List(_)) = event {
            full_text.push('\n');
        }
        if let Event::Start(Tag::Item) = event {
            full_text.push('\n');
        }
        if let Event::Start(Tag::Table(_)) = event {
            full_text.push('\n');
        }
        if let Event::Start(Tag::TableCell) = event {
            full_text.push('\n');
        }
        if let Event::Start(Tag::TableHead) = event {
            full_text.push('\n');
        }
        if let Event::Start(Tag::TableRow) = event {
            full_text.push('\n');
        }
        if let Event::End(TagEnd::Paragraph) = event {
            full_text.push('\n');
        }
        if let Event::End(TagEnd::Heading { .. }) = event {
            full_text.push('\n');
        }
        if let Event::End(TagEnd::List(_)) = event {
            full_text.push('\n');
        }
        if let Event::End(TagEnd::Item) = event {
            full_text.push('\n');
        }
        if let Event::End(TagEnd::Table) = event {
            full_text.push('\n');
        }
        if let Event::End(TagEnd::TableCell) = event {
            full_text.push('\n');
        }
        if let Event::End(TagEnd::TableHead) = event {
            full_text.push('\n');
        }
        if let Event::End(TagEnd::TableRow) = event {
            full_text.push('\n');
        }
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
        if let Event::Code(_) = event {
            full_text.push_str("Bob");
        }
        if let Event::InlineMath(_) = event {
            full_text.push_str("Bob");
        }
        if let Event::SoftBreak = event {
            if !ignore {
                full_text.push(' ');
            }
        }
        if let Event::HardBreak = event {
            if !ignore {
                full_text.push('\n');
            }
        }
        if let Event::Text(text) = event {
            if !ignore {
                full_text.push_str(&text);
            }
        }
    }

    log::debug!("full text: {full_text}");

    if lang == "fr" {
        let check_result = grammalecte::run_grammalecte(&full_text);
        for p in &check_result.paragraphs {
            let para_num = p.num;
            for grammar_error in &p.grammars {
                let sub = find_slice(
                    &full_text,
                    para_num,
                    grammar_error.offset_start,
                    grammar_error.offset_end,
                );
                // log::warn!(
                //     "line {para_num} from {} to {}: error in \"{sub}\": {}",
                //     grammar_error.offset_start,
                //     grammar_error.offset_end,
                //     grammar_error.message
                // );
                log::warn!("error in \"{sub}\": {}", grammar_error.message)
            }
        }

        if check_result
            .paragraphs
            .iter()
            .flat_map(|p| &p.grammars)
            .next()
            .is_some()
            && !config.allow_errors
        {
            log::error!("Stopping after founding grammar error");
            exit(1)
        }
    }

    Cow::Borrowed(content)
}

fn find_slice(text: &str, para_num: usize, start: usize, end: usize) -> &str {
    let line = text.lines().nth(para_num - 1).unwrap();
    let start = line.char_indices().nth(start).unwrap().0;
    if let Some(end) = line.char_indices().nth(end).map(|c| c.0) {
        &line[start..end]
    } else {
        &line[start..]
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let s = "Du point de vue de la sécurité, Bob effectue tous les téléchargements en HTTPS, mais ne valide pas les signatures des fichiers téléchargés. Les protections contre les attaques par déclassement, le pinning de certificats et la validation des signatures sont des travaux actuellement en cours. Pour les cas les plus sensibles, il peut être préférable d’opter pour une méthode d’installation alternative comme celles listées dans la section Install du site officiel du langage Rust.";
        let start = s.char_indices().nth(269).unwrap().0;
        let end = s.char_indices().nth(290).unwrap().0;
        println!("{}", &s[start..end])
    }
}
