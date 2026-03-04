use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;
use serde::Deserialize;
use serde_yaml_ng::Value;

static RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[([^\]]*) @([a-zA-Z0-9\-_]*)\]").unwrap());

#[derive(Debug, Deserialize)]
struct Entry {
    id: String,
    title: String,
    url: Option<String>,
    #[serde(default)]
    author: Vec<Person>,
}

#[derive(Debug, Deserialize)]
struct Person {
    family: String,
}

pub fn cite_proc<'a>(
    config: &crate::extensions::ExtConfig,
    meta: &Value,
    content: &'a str,
) -> Cow<'a, str> {
    if let Some(bib) = meta.get("references") {
        let bib: Vec<Entry> =
            serde_yaml_ng::from_value(bib.to_owned()).expect("Cannot read CSL-YAML library");
        // let mut new_content = RE.replace_all(content, "[$1](#$2)").into_owned();
        let replacement = |cap: &regex::Captures| {
            let id = &cap[2];
            let name = &cap[1];
            match bib.iter().find(|e| e.id == id).and_then(|e| e.url.as_ref()) {
                Some(url) => format!("[{name}]({url})"),
                None => format!("{name} [{id}](#{id})"),
            }
        };
        let mut new_content = RE.replace_all(content, replacement).into_owned();
        let mut refs = Vec::new();
        for (_, [_name, reference]) in RE.captures_iter(content).map(|c| c.extract()) {
            refs.push(reference);
        }
        if !refs.is_empty() {
            new_content.push_str(&format!("\n\n## {}\n\n", config.title));
            for entry in bib
                .iter()
                .filter(|entry| refs.contains(&(&entry.id as &str)))
            {
                let key = &entry.id;
                let title = &entry.title;
                let title_link = if let Some(url) = &entry.url {
                    format!("[{title}]({url})")
                } else {
                    title.to_string()
                };
                let authors: String = entry
                    .author
                    .iter()
                    .map(|person| format!(", {}", &person.family))
                    .collect();
                new_content.push_str(&format!(
                    "* <a id=\"{key}\"></a> *{title_link}*{authors} ({key})\n"
                ));
            }
        }
        Cow::Owned(new_content)
    } else {
        Cow::Borrowed(content)
    }
    // hayagriva::io::from_yaml_str(s)
}
