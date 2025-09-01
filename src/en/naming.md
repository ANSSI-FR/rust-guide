# Naming

As of now, the standard library is the de facto standard for naming things in
the Rust world. However, an effort has been made to formalize it, first in
[RFC 430], then in the [Rust API Guidelines].

The basic rule consists in using :

- `UpperCamelCase` for types, traits, enum variants,
- `snake_case` for functions, methods, macros, variables and modules,
- `SCREAMING_SNAKE_CASE` for statics and constants,
- `'lowercase` for lifetimes.

The [Rust API Guidelines] also prescribes more precise naming conventions for
some particular constructions:

- (C-CONV) for conversion methods (`as_`, `to_`, `into_`),
- (C-GETTER) for getters,
- (C-ITER) for iterator-producing methods,
- (C-ITER-TY) for iterator types,
- (C-FEATURE) for feature naming,
- (C-WORD-ORDER) for word order consistency.

> **Rule {{#check LANG-NAMING | Respect naming conventions}}**
>
> Development of a secure application must follow the naming conventions
> outlined in the [Rust API Guidelines].

[rfc 430]: https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md
[rust api guidelines]: https://rust-lang.github.io/api-guidelines/
