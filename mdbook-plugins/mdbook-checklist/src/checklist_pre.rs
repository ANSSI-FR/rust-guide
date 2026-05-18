use crate::checklist::Checklist;
use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{Tag, TagEnd};
use quick_xml::errors::IllFormedError;
use quick_xml::Reader;

// A preprocessor for collecting the `{{#check <name> | <description>}}` marks
// and generating a 'checklist' chapter.
pub struct ChecklistPre;

const NAME: &str = "checklist-preprocessor";

impl Preprocessor for ChecklistPre {
    fn name(&self) -> &str {
        NAME
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let mut checklist = Checklist::new();
        if let Some(cfg) = ctx.config.get_preprocessor(NAME) {
            checklist.update_config(cfg);
        }

        book.for_each_mut(|section: &mut BookItem| {
            if let BookItem::Chapter(ref mut chapter) = *section {
                collect_and_replace(chapter, &mut checklist);
            }
        });

        let checklist_chapter = checklist.generate_chapter();
        book.sections.push(BookItem::Chapter(checklist_chapter));

        Ok(book)
    }
}

fn collect_and_replace(chapter: &Chapter, checklist: &mut Checklist) {
    use pulldown_cmark::{Event, Parser, TextMergeStream};

    let iterator = TextMergeStream::new(Parser::new(&chapter.content));

    let mut html_value = String::new();

    for event in iterator {
        match event {
            Event::Start(Tag::HtmlBlock) => {
                html_value = String::new();
            }
            Event::Start(_) => {}
            Event::End(TagEnd::HtmlBlock) => {
                let mut reader = Reader::from_str(&html_value);
                reader.config_mut().trim_text(true);
                for Reco { id, typ, title } in get_reco(reader) {
                    checklist.insert(
                        &chapter.name,
                        chapter.path.as_ref().unwrap(),
                        id,
                        format!("{typ} - {title}"),
                    );
                }
            }
            Event::Html(cow_str) => {
                html_value.push_str(&cow_str);
            }
            _ => {}
        }
    }
}

struct Reco {
    typ: String,
    id: String,
    title: String,
}

fn get_reco(mut reader: Reader<&[u8]>) -> Vec<Reco> {
    use quick_xml::events::Event;
    let mut res = Vec::new();
    loop {
        let event = reader.read_event();
        //eprintln!("xml event: {event:?}");
        match event {
            Err(quick_xml::Error::IllFormed(IllFormedError::UnmatchedEndTag(_))) => {
                //html extraction from markdown does not respect closing tag
            }
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            Ok(Event::Eof) => return res,

            Ok(Event::Start(e)) => {
                if e.local_name().as_ref() == b"div" {
                    if let Some(class) = e
                        .html_attributes()
                        .filter_map(|attr| attr.ok())
                        .find(|attr| attr.key.local_name().as_ref() == b"class")
                        .and_then(|attr| attr.unescape_value().ok())
                    {
                        if class == "reco" {
                            let typ = e
                                .html_attributes()
                                .filter_map(|attr| attr.ok())
                                .find(|attr| attr.key.local_name().as_ref() == b"type")
                                .and_then(|attr| attr.unescape_value().ok());
                            let title = e
                                .html_attributes()
                                .filter_map(|attr| attr.ok())
                                .find(|attr| attr.key.local_name().as_ref() == b"title")
                                .and_then(|attr| attr.unescape_value().ok());
                            let id = e
                                .html_attributes()
                                .filter_map(|attr| attr.ok())
                                .find(|attr| attr.key.local_name().as_ref() == b"id")
                                .and_then(|attr| attr.unescape_value().ok());
                            match (id, typ, title) {
                                (None, _, _) => {
                                    eprintln!("Recommendation div tag without \"id\" attribute")
                                }
                                (_, None, _) => {
                                    eprintln!("Recommendation div tag without \"type\" attribute")
                                }
                                (_, _, None) => {
                                    eprintln!("Recommendation div tag without \"title\" attribute")
                                }
                                (Some(id), Some(typ), Some(title)) => res.push(Reco {
                                    typ: typ.to_string(),
                                    id: id.to_string(),
                                    title: title.to_string(),
                                }),
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }
}
