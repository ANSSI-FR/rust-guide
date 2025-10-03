use mdbook::{
    BookItem,
    book::Book,
    preprocess::{Preprocessor, PreprocessorContext},
};
use serde::Deserialize;

use crate::cite;

pub struct Ext;

#[derive(Debug, Deserialize)]
pub struct ExtConfig {
    pub title: String,
}

impl Preprocessor for Ext {
    fn name(&self) -> &str {
        "extensive-preprocessor"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> anyhow::Result<Book> {
        let config: ExtConfig = ctx
            .config
            .get_deserialized_opt("preprocessor.extensions")
            .unwrap()
            .expect("Cannot find configuration for preprocessor.extensions");
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                let metadata = get_meta(&chapter.content);
                remove_meta(&mut chapter.content);
                chapter.content =
                    cite::cite_proc(&config, &metadata, &chapter.content).into_owned();
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
