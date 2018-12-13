# Guide to develop secure applications with Rust

## Objectives

The object of this document is to provide hints and recommendations for secure
applications development using the Rust programming language.

It is not intended to be a course on how to write Rust programs, there are
already plenty of good learning resources for this purpose (see the *External
references* section below). The purpose is rather to guide the programmer and to
inform him about certain pitfalls, especially in case he is involved in the
development of applications with strong security requirements.  These
recommendations form a complement to the good level of trust the Rust language
already provides. That said, recalls are sometimes necessary for clarity, and
the experienced Rust programmer may rely solely on *Recommendation* or *Warning*
inserts.

It is currently an ongoing version and all contributions are welcome.

## Reading the guide online

[Summary](./src/SUMMARY.md)

## Building the guide

```
$ cargo install mdbook
$ mdbook serve -o
```

## Contributions

Feel free to create pull requests to suggest recommendations or modifications,
or to submit an issue to start discussions.

## Licence

This document is published under the [Open Licence 2.0](LICENCE.md).

## External references

- [The Rust programming language](https://www.rust-lang.org)
- [The Rust book](https://doc.rust-lang.org/stable/book)
- [About Rust editions](https://rust-lang-nursery.github.io/edition-guide)
- [Rust API guidelines](https://rust-lang-nursery.github.io/api-guidelines)

One can also find an up-to-date list of various book resources about Rust and
associated tools in the [Rust documentation main
page](https://doc.rust-lang.org).
