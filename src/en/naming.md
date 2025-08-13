# Naming

Currently, the standard library serves as the de facto standard for naming
conventions in the Rust ecosystem. Efforts have been made to formalize these
conventions, first in [RFC 430], and later in the [Rust API Guidelines].

The basic rules are:

- `UpperCamelCase` for types, traits, and enum variants,
- `snake_case` for functions, methods, macros, variables, and modules,
- `SCREAMING_SNAKE_CASE` for statics, constants, and const generic parameters,
- `'lowercase` for lifetimes.

The [Rust API Guidelines] also prescribe more precise naming conventions for
specific constructs:

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
