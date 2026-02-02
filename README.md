# Guide to develop secure applications with Rust
![badge_repo](https://img.shields.io/badge/ANSSI--FR-rust--guide-white)
[![category_badge_doctrinal](https://img.shields.io/badge/category-doctrinal-%23e9c7e7)](https://github.com/ANSSI-FR#types-de-projets)
[![openess_badge_A](https://img.shields.io/badge/code.gouv.fr-collaborative-blue)](https://documentation.ouvert.numerique.gouv.fr/les-parcours-de-documentation/ouvrir-un-projet-num%C3%A9rique/#niveau-ouverture)

## French Cybersecurity Agency (ANSSI)

<img src="https://www.sgdsn.gouv.fr/files/styles/ds_image_paragraphe/public/files/Notre_Organisation/logo_anssi.png" alt="ANSSI logo" width="30%">

*This projet is managed by [ANSSI](https://cyber.gouv.fr/). To find out more,
you can go to the
[page](https://cyber.gouv.fr/enjeux-technologiques/open-source/) (in French)
dedicated to the ANSSI open source strategy. You can also click on the badges
above to learn more about their meaning*.

## Objectives

The objective of this document is to provide hints and recommendations for
secure applications development using the Rust programming language.

It is not intended to be a course on how to write Rust programs, there are
already plenty of good learning resources for this purpose (see the *External
references* section below). The purpose is rather to guide the programmer and to
inform them about certain pitfalls, especially in case they are involved in the
development of applications with strong security requirements. These
recommendations form a complement to the good level of trust the Rust language
already provides. That said, reminders are sometimes necessary for clarity, and
the experienced Rust programmer may rely solely on highlighted inserts
(*Rule*, *Recommendation*, *Warning*, etc.).

It is currently an ongoing version and all contributions are welcome.

## Reading the guide online

[https://anssi-fr.github.io/rust-guide](https://anssi-fr.github.io/rust-guide)

## Building the guide

Install `mdbook` and required preprocessor:

```
cargo install mdbook --git https://github.com/hg-anssi/mdBook.git --rev c5a35b9296c6d5e48570e30022bd69403050a9f4 --locked
cargo install --path ./mdbook-checklist --locked
cargo install --path ./mdbook-code-align --locked
cargo install --path ./mdbook-extensions --locked
```

Then build and open the book:

```
$ mdbook serve -o
```

## Call for Contributions

See [CONTRIBUTING.md](CONTRIBUTING.md).

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
