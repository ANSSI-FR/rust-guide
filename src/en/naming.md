---
references:
  - type: article
    title: General naming conventions
    url: https://rust-lang.github.io/rfcs/0430-finalizing-naming-conventions.html
    id: RFC-430
  - type: article
    title: Rust API Guidelines
    url: https://rust-lang.github.io/api-guidelines/
    id: rust-guidelines
---

# Naming

The standard library serves as the de facto standard for naming conventions in Rust.
An effort has been made to formalize the conventions through the [RFC 43 @RFC-430] and later in the [Rust API Guidelines @rust-guidelines].

The basic rule [(C-CASE)] consists in using:

- `UpperCamelCase` for types, traits, enum variants, and generic type parameters,
- `snake_case` for functions, methods, macros, variables, and modules,
- `SCREAMING_SNAKE_CASE` for statics, constants, and generic constant parameters,
- `'lowercase` for lifetimes.

The [Rust API Guidelines @rust-guidelines] also prescribes more precise naming conventions for
particular constructs:

- [(C-CONV)] for conversion methods (`as_`, `to_`, `into_`),
- [(C-GETTER)] for getters,
- [(C-ITER)] for iterator-producing methods,
- [(C-ITER-TY)] for iterator types,
- [(C-FEATURE)] for feature naming (conditionally enabled functionalities),
- [(C-WORD-ORDER)] for word order consistency.

<div class="note">

The basic rule [(C-CASE)] is checked by the compiler (with the `nonstandard_style` lint set).

In addition to the compiler, the [`clippy`](devenv.md#clippy) tool can help in adopting naming conventions with the `style` lint category.
For example, the [`wrong_self_convention`](https://rust-lang.github.io/rust-clippy/master/index.html#wrong_self_convention) lint checks the consistency between conversion method names and their receiver types (`self`, `&self`, `&mut self`) according to [(C-CONV)].

<!--
clippy::enum_variant_names
clippy::self_named_constructors
-->

</div>

<div class="reco" id="LANG-NAMING" type="Rule" title="Respect naming conventions">

Development of a secure application MUST follow the naming conventions
outlined in the [Rust API Guidelines @rust-guidelines].

</div>

[(C-CASE)]: https://rust-lang.github.io/api-guidelines/naming.html#casing-conforms-to-rfc-430-c-case
[(C-CONV)]: https://rust-lang.github.io/api-guidelines/naming.html#ad-hoc-conversions-follow-as_-to_-into_-conventions-c-conv
[(C-GETTER)]: https://rust-lang.github.io/api-guidelines/naming.html#getter-names-follow-rust-convention-c-getter
[(C-ITER)]: https://rust-lang.github.io/api-guidelines/naming.html#methods-on-collections-that-produce-iterators-follow-iter-iter_mut-into_iter-c-iter
[(C-ITER-TY)]: https://rust-lang.github.io/api-guidelines/naming.html#iterator-type-names-match-the-methods-that-produce-them-c-iter-ty
[(C-FEATURE)]: https://rust-lang.github.io/api-guidelines/naming.html#feature-names-are-free-of-placeholder-words-c-feature
[(C-WORD-ORDER)]: https://rust-lang.github.io/api-guidelines/naming.html#names-use-a-consistent-word-order-c-word-order