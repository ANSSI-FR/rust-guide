use mdbook::{
    BookItem,
    book::Book,
    preprocess::{Preprocessor, PreprocessorContext},
};
use serde::Deserialize;

use crate::{cite, grammarcheck, spellcheck};

pub struct Ext;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ExtConfig {
    pub cite_proc: Option<cite::Config>,
    pub spell_checker: Option<spellcheck::Config>,
    pub grammar_checker: Option<grammarcheck::Config>,
}

impl Preprocessor for Ext {
    fn name(&self) -> &str {
        "extensive-preprocessor"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> anyhow::Result<Book> {
        let lang = ctx.config.book.language.as_ref();
        let renderer = &ctx.renderer;
        let config: ExtConfig = ctx
            .config
            .get_deserialized_opt("preprocessor.extensions")
            .unwrap()
            .expect("Cannot find configuration for preprocessor.extensions");
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                if let Some(path) = &chapter.path {
                    log::info!("preprocess {path:?}")
                }
                let metadata = get_meta(&chapter.content);
                remove_meta(&mut chapter.content);
                chapter.content =
                    cite::cite_proc(&config, renderer, &metadata, &chapter.content).into_owned();
                if let Some(lang) = lang {
                    chapter.content =
                        spellcheck::check(lang, &config, &metadata, &chapter.content).into_owned();
                    chapter.content =
                        grammarcheck::check(lang, &config, &metadata, &chapter.content)
                            .into_owned();
                } else {
                    log::warn!("Missing language indication")
                }
            }
        });
        Ok(book)
    }
}

fn remove_meta(content: &mut String) {
    let mut lines = content.lines();
    if let Some("---") = lines.next() {
        let new_content = lines
            .skip_while(|l| *l != "---")
            .skip(1)
            .collect::<Vec<_>>()
            .join("\n");
        *content = new_content;
    }
}

fn get_meta(content: &str) -> serde_yaml_ng::Value {
    let mut lines = content.lines();
    if let Some("---") = lines.next() {
        let yaml = lines
            .take_while(|l| *l != "---")
            .collect::<Vec<_>>()
            .join("\n");
        match serde_yaml_ng::from_str(&yaml) {
            Ok(v) => v,
            Err(_) => serde_yaml_ng::Value::Null,
        }
    } else {
        serde_yaml_ng::Value::Null
    }
}
