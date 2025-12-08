use crate::semindent::{self, Element};
use anyhow::{anyhow, Ok};
use markdown::{mdast::Node, to_mdast, ParseOptions};
use mdbook::{
    book::Book,
    preprocess::{Preprocessor, PreprocessorContext},
    BookItem,
};

pub(super) struct Align;

fn get_content_mut(item: &mut BookItem) -> Vec<&mut String> {
    match item {
        BookItem::Chapter(chapter) => {
            let content: &mut String = &mut chapter.content;
            chapter
                .sub_items
                .iter_mut()
                .flat_map(|book_item| get_content_mut(book_item))
                .chain([content])
                .collect()
        }
        BookItem::Separator => Vec::new(),
        BookItem::PartTitle(_) => Vec::new(),
    }
}

fn visit_ast<F>(node: &Node, f: &mut F)
where
    F: FnMut(&Node),
{
    f(node);
    if let Some(children) = node.children() {
        for child in children {
            visit_ast(child, f);
        }
    }
}

impl Preprocessor for Align {
    fn name(&self) -> &str {
        "align-preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> anyhow::Result<Book> {
        for content in book.sections.iter_mut().flat_map(get_content_mut) {
            let mut changes = Vec::new();
            let ast = to_mdast(content, &ParseOptions::default())
                .map_err(|md_msg| anyhow!("{}", md_msg))?;
            visit_ast(&ast, &mut |n| {
                if let Node::Code(code) = n {
                    if code
                        .meta
                        .as_ref()
                        .map(|s| s as &str)
                        .unwrap_or_default()
                        .split(' ')
                        .any(|s| s == "align")
                    {
                        if let Some(p) = &code.position {
                            let value = code.value.clone();
                            changes.push((
                                p.clone(),
                                code.lang.clone(),
                                code.meta.clone(),
                                move |prefix| align(prefix, &value),
                            ));
                        }
                    }
                }
            });
            changes.sort_by_key(|(pos, _, _, _)| pos.start.offset);
            let mut new_content = String::new();
            let mut start = 0;
            for (pos, lang, meta, new_code) in changes {
                let last_line = content[0..pos.start.offset].lines().last().unwrap_or("");
                let code_option = match (lang, meta) {
                    (None, None) => "".to_string(),
                    (None, Some(_)) => "".to_string(),
                    (Some(l), None) => l,
                    (Some(l), Some(m)) => format!("{l} {m}"),
                };
                new_content.push_str(&content[start..pos.start.offset]);
                new_content.push_str(&format!("```{}\n", code_option));
                new_content.push_str(&new_code(last_line));
                new_content.push_str(&format!("{last_line}```"));
                start = pos.end.offset;
            }
            new_content.push_str(&content[start..content.len()]);
            *content = new_content;
        }
        Ok(book)
    }
}

fn align(prefix: &str, content: &str) -> String {
    let sem = semindent::parse_indented_text(content);
    let mut unaligned = &sem;
    if let (Some(Element::Subtext(sem)), None) = (sem.first(), sem.get(1)) {
        unaligned = sem;
    }
    semindent::to_string(unaligned, prefix, "   ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use markdown::{to_mdast, ParseOptions};

    fn exemple_book(content: &str) -> Vec<u8> {
        let input = serde_json::json!([
            {
                "root": "/path/to/book",
                "config": {
                    "book": {
                        "authors": ["AUTHOR"],
                        "language": "en",
                        "multilingual": false,
                        "src": "src",
                        "title": "TITLE"
                    },
                    "preprocessor": {
                        "nop": {}
                    }
                },
                "renderer": "html",
                "mdbook_version": "0.4.21"
            },
            {
                "sections": [
                    {
                        "Chapter": {
                            "name": "Chapter 1",
                            "content": content,
                            "number": [1],
                            "sub_items": [],
                            "path": "chapter_1.md",
                            "source_path": "chapter_1.md",
                            "parent_names": []
                        }
                    }
                ],
                "__non_exhaustive": null
            }
        ]);
        serde_json::to_vec(&input).unwrap()
    }

    #[test]
    fn parse_md() {
        let md = r#"coucou

> citation
> 
> ```rust,noplaypen title="Here is an example" fgfg align
> code
> ```

plop
"#;
        let node = to_mdast(md, &ParseOptions::default()).unwrap();
        println!("{:?}", node);
        println!("{}", node.to_string());
    }

    #[test]
    fn last_line() {
        assert_eq!(vec![] as Vec<&str>, "".lines().collect::<Vec<&str>>());
    }

    #[test]
    fn align() {
        let content = r#"# Ceci est le titre

Ceci est un paragraphe

```rust align
    fn main(){println!("Hello, World")}
```

fin
"#;

        let expected = r#"# Ceci est le titre

Ceci est un paragraphe

```rust align
fn main(){println!("Hello, World")}
```

fin
"#;
        let input_json: &[u8] = &exemple_book(content);
        let expected_json: &[u8] = &exemple_book(expected);

        let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
        let (_, expected_book) =
            mdbook::preprocess::CmdPreprocessor::parse_input(expected_json).unwrap();
        let result = Align.run(&ctx, book);
        assert!(result.is_ok());

        let actual_book = result.unwrap();
        assert_eq!(actual_book, expected_book);
    }

    #[test]
    fn align_nested() {
        let content = r#"# Ceci est le titre

Ceci est un paragraphe

> Début de citation
> 
> ```rust align
>     fn main(){println!("Hello, World")}
> ```
> fin de citation

fin
"#;

        let expected = r#"# Ceci est le titre

Ceci est un paragraphe

> Début de citation
> 
> ```rust align
> fn main(){println!("Hello, World")}
> ```
> fin de citation

fin
"#;
        let input_json: &[u8] = &exemple_book(content);
        let expected_json: &[u8] = &exemple_book(expected);

        let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
        let (_, expected_book) =
            mdbook::preprocess::CmdPreprocessor::parse_input(expected_json).unwrap();
        let result = Align.run(&ctx, book);
        assert!(result.is_ok());

        let actual_book = result.unwrap();
        assert_eq!(actual_book, expected_book);
    }

    #[test]
    fn no_align() {
        let content = r#"# Ceci est le titre

Ceci est un paragraphe

```rust
    fn main(){println!("Hello, World")}
```

fin
"#;

        let expected = content;
        let input_json: &[u8] = &exemple_book(content);
        let expected_json: &[u8] = &exemple_book(expected);

        let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
        let (_, expected_book) =
            mdbook::preprocess::CmdPreprocessor::parse_input(expected_json).unwrap();
        let result = Align.run(&ctx, book);
        assert!(result.is_ok());

        let actual_book = result.unwrap();
        assert_eq!(actual_book, expected_book);
    }
}
