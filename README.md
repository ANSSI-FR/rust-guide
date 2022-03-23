# Guide to develop secure applications with Rust

## Objectives

The objective of this document is to provide hints and recommendations for
secure applications development using the Rust programming language.

It is not intended to be a course on how to write Rust programs, there are
already plenty of good learning resources for this purpose (see the *External
references* section below). The purpose is rather to guide the programmer and to
inform them about certain pitfalls, especially in case they are involved in the
development of applications with strong security requirements.  These
recommendations form a complement to the good level of trust the Rust language
already provides. That said, recalls are sometimes necessary for clarity, and
the experienced Rust programmer may rely solely on highlighted inserts
(*Rule*, *Recommendation*, *Warning*, etc.).

It is currently an ongoing version and all contributions are welcome.

## Reading the guide online

[https://anssi-fr.github.io/rust-guide](https://anssi-fr.github.io/rust-guide)

## Building the guide

Install `mdbook` and required preprocessor:

```
cargo install mdbook mdbook-checklist
```

Then build and open the book:

```
$ mdbook serve -o
```

## Call for Contributions

At this time, this guide is intended to be a living document. It still lacks
important points and details, and future versions of the language and compiler
may render some recommendations obsolete. We are eager to discuss and to receive
contributions from anyone who is aware of common or uncommon pitfalls to avoid,
or good coding practices and tools that can help building more robust software
with the Rust language.

Thus, feel free to create pull requests to suggest recommendations or
modifications, or to submit an issue to start discussions. Specifically, please
opt for a *pull request* for small changes like:

 - complementing a paragraph,
 - adding a small example in the form of code snippet,
 - updating some information 
 - fixing typos and English mistakes,
 - etc.

and for an *issue* in case of more substantive changes:

 - suggesting a new recommendation,
 - discussing controversial points,
 - rewording a consistent part of the text,
 - etc.

## Licence

This document is published under the [Open Licence 2.0](LICENCE.md).

## External references

- [The Rust programming language](https://www.rust-lang.org)
- [The Rust book](https://doc.rust-lang.org/stable/book)
- [The Edition Guide](https://doc.rust-lang.org/edition-guide/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines)

One can also find an up-to-date list of various book resources about Rust and
associated tools in the [Rust documentation main
page](https://doc.rust-lang.org).
