use std::iter::{once, repeat_n};

#[derive(Debug, PartialEq, Eq)]
pub enum Element<'a> {
    Line(&'a str),
    Subtext(TextStructure<'a>),
}

impl<'a> Element<'a> {
    fn to_string(&self, indent: &str, inc: &str) -> String {
        match self {
            Element::Line(s) => format!("{indent}{s}\n"),
            Element::Subtext(elements) => {
                let mut res = String::new();
                for e in elements {
                    res.push_str(&e.to_string(&format!("{indent}{inc}"), inc));
                }
                res
            }
        }
    }
}

pub type TextStructure<'a> = Vec<Element<'a>>;

pub fn to_string<'a>(strct: &TextStructure<'a>, indent: &str, inc: &str) -> String {
    let mut res = String::new();
    for e in strct {
        res.push_str(&e.to_string(indent, inc));
    }
    res
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IndentorToken<'a> {
    Indent,
    Line(&'a str),
    Dedent,
}

fn strip_whispace_prefix(s: &str) -> &str {
    let i = s.find(|c| !char::is_whitespace(c)).unwrap_or(s.len());
    &s[i..s.len()]
}

fn tokenize<'a>(text: &'a str) -> impl Iterator<Item = IndentorToken<'a>> {
    let mut indents: Vec<&'a str> = Vec::new();
    indents.push("");
    text.lines().flat_map(move |line| {
        let content = strip_whispace_prefix(line);
        if content.is_empty() {
            return [IndentorToken::Line("")].to_vec();
        }
        let indent = &line[0..(line.len() - content.len())];
        let current_indent = indents[indents.len() - 1];
        if indent == current_indent {
            return [IndentorToken::Line(content)].to_vec();
        }
        if indent.starts_with(current_indent) {
            indents.push(indent);
            return [IndentorToken::Indent, IndentorToken::Line(content)].to_vec();
        }
        let previous_level = indents.len();
        indents.retain(|prefix| indent.starts_with(*prefix));
        let nbr_dedent = previous_level - indents.len();
        let mut nbr_indent = 0;
        if indent != indents[indents.len() - 1] {
            indents.push(indent);
            nbr_indent = 1;
        }
        std::iter::repeat_n(IndentorToken::Dedent, nbr_dedent)
            .chain(repeat_n(IndentorToken::Indent, nbr_indent))
            .chain(once(IndentorToken::Line(content)))
            .collect()
    })
}

fn get_tree<'a>(tokens: &mut impl Iterator<Item = IndentorToken<'a>>) -> TextStructure<'a> {
    let mut res = Vec::new();
    while let Some(token) = tokens.next() {
        match token {
            IndentorToken::Indent => {
                let subtext = get_tree(tokens);
                res.push(Element::Subtext(subtext));
            }
            IndentorToken::Line(line) => res.push(Element::Line(line)),
            IndentorToken::Dedent => break,
        }
    }
    res
}

pub fn parse_indented_text<'a>(text: &'a str) -> TextStructure<'a> {
    get_tree(&mut tokenize(text))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wtrip_prefix() {
        assert_eq!("", strip_whispace_prefix("   "));
        assert_eq!("coucou", strip_whispace_prefix("coucou"));
        assert_eq!("coucou", strip_whispace_prefix(" coucou"));
        assert_eq!("coucou", strip_whispace_prefix("  coucou"));
    }

    #[test]
    fn test_lines_ending_with_newline() {
        let text = "foo\r\nbar\n\nbaz\n";
        let mut lines = text.lines();

        assert_eq!(Some("foo"), lines.next());
        assert_eq!(Some("bar"), lines.next());
        assert_eq!(Some(""), lines.next());
        assert_eq!(Some("baz"), lines.next());

        assert_eq!(None, lines.next());
    }

    #[test]
    fn test_lines_ending_with_many_newline() {
        let text = "foo\r\nbar\n\nbaz\n\n";
        let mut lines = text.lines();

        assert_eq!(Some("foo"), lines.next());
        assert_eq!(Some("bar"), lines.next());
        assert_eq!(Some(""), lines.next());
        assert_eq!(Some("baz"), lines.next());
        assert_eq!(Some(""), lines.next());

        assert_eq!(None, lines.next());
    }

    #[test]
    fn test_lines() {
        let text = "foo\r\nbar\n\nbaz";
        let mut lines = text.lines();

        assert_eq!(Some("foo"), lines.next());
        assert_eq!(Some("bar"), lines.next());
        assert_eq!(Some(""), lines.next());
        assert_eq!(Some("baz"), lines.next());

        assert_eq!(None, lines.next());
    }

    #[test]
    fn parse() {
        let text = r#"coucou
plop
    plap
    plip
        plup
plaf

"#;
        let sem = parse_indented_text(text);
        assert_eq!(Element::Line("coucou"), sem[0]);
        assert_eq!(Element::Line("plop"), sem[1]);
        assert_eq!(
            Element::Subtext(vec![
                Element::Line("plap"),
                Element::Line("plip"),
                Element::Subtext(vec![Element::Line("plup")])
            ]),
            sem[2]
        );
        assert_eq!(Element::Line("plaf"), sem[3]);
        assert_eq!(Element::Line(""), sem[4]);
        assert_eq!(5, sem.len())
    }

    #[test]
    fn border_cases() {
        assert_eq!(Vec::<Element>::new(), parse_indented_text(""));
        assert_eq!(vec![Element::Line("")], parse_indented_text("   "));
        assert_eq!(
            vec![
                Element::Subtext(vec![Element::Line("coucou")]),
                Element::Subtext(vec![Element::Line("plop")])
            ],
            parse_indented_text("   coucou\n plop")
        );
        assert_eq!(
            vec![
                Element::Subtext(vec![Element::Line("coucou")]),
                Element::Line("plop"),
            ],
            parse_indented_text("   coucou\nplop")
        );
    }

    #[test]
    fn pretty_print() {
        use Element::*;
        let sem = vec![
            Line("a"),
            Line("b"),
            Subtext(vec![
                Line("c"),
                Subtext(vec![Line("d")]),
                Line("e"),
                Subtext(vec![Line("f")]),
            ]),
            Line("g"),
        ];
        let expected = r#"a
b
   c
      d
   e
      f
g
"#;
        assert_eq!(expected, to_string(&sem, "", "   "))
    }
}
