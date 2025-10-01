# mdBook checklist preprocessor

[![Crates.io](https://img.shields.io/crates/v/mdbook-checklist.svg)](https://crates.io/crates/mdbook-checklist)
[![Github CI](https://github.com/ANSSI-FR/mdbook-checklist/workflows/Rust/badge.svg)](https://github.com/ANSSI-FR/mdbook-checklist/actions)
[![Github CI](https://github.com/ANSSI-FR/mdbook-checklist/workflows/Clippy/badge.svg)](https://github.com/ANSSI-FR/mdbook-checklist/actions)

A preprocessor for gathering checks in an mdBook and generating an index.

## Usage

First, you need to install the preprocessor:

```
cargo install mdbook-checklist
```

Next, you need to add the preprocessor to your `book.toml`:

```
[book]
authors = ["Me"]
multilingual = false
src = "src"
title = "The Book"

[preprocessor.checklist]
```

Finally, you can insert marks in your book chapters, according to the following
format: `{{#check <name> | <description>}}`. For example

```
# Chapter 1

{{#check Note-1 | This is an important note}}
```

The mark will be replaced by the name solely (with an anchor to be linked from
the index). Also, for this example, the following index will be generated:

> # Checklist
> 
>  - Chapter 1:
>    - [ ] This is an important note ([Note-1](README.md#Note-1))


## Options

The title `Checklist` of the generated index can be changed:

```
[preprocessor.checklist]
title = "A list of notes"
```

## Licence

This library is published under the [Open Licence 2.0](LICENCE.md).
