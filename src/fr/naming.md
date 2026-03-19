---
references:
  - type: article
    title: General naming conventions
    date: 2014
    url: https://rust-lang.github.io/rfcs/0430-finalizing-naming-conventions.html
    id: RFC-430
  - type: article
    title: Rust API Guidelines
    url: https://rust-lang.github.io/api-guidelines/
    display-name: Rust API Guidelines
    id: rust-guidelines
---

# Nommage

La convention de nommage employée par la bibliothèque standard est *de facto* le
standard pour le nommage des éléments des programmes écrits en Rust. Un effort a
été fait pour formaliser ces conventions de nommage, d'abord dans la [RFC 430 @RFC-430],
puis dans le document des [bonnes pratiques Rust @rust-guidelines].

La règle de base [`(C-CASE)`] consiste à utiliser :

- `UpperCamelCase` pour les types, traits, variants d'énumérations et paramètres génériques de type ;
- `snake_case` pour les fonctions, méthodes, macros, variables et modules ;
- `SCREAMING_SNAKE_CASE` pour les variables statiques, les constantes et les paramètres génériques constants ;
- `'lowercase` pour les durées de vie (*lifetimes*).

Les [bonnes pratiques Rust @rust-guidelines] recommandent également des conventions de nommage
plus précises pour certaines constructions particulières :

- [`(C-CONV)`] pour les méthodes de conversion (`as_`, `to_`, `into_`) ;
- [`(C-GETTER)`] pour les accesseurs ;
- [`(C-ITER)`] pour les méthodes produisant des itérateurs ;
- [`(C-ITER-TY)`] pour les types itérateur ;
- [`(C-FEATURE)`] pour les noms de *features* (fonctionnalités activables par configuration) ;
- [`(C-WORD-ORDER)`] pour la cohérence sur l'ordre des mots.

<div class="note">

Les règles de base sont vérifiées par le compilateur (jeu d'avertissements `nonstandard_style`).

En complément du compilateur, l'outil [`clippy`](devenv.md#clippy) permet de faciliter l'adoption des conventions de nommage usuelles à travers la catégorie `style`.
Par exemple, la vérification [`wrong_self_convention`](https://rust-lang.github.io/rust-clippy/master/index.html#wrong_self_convention) contrôle la cohérence entre les noms des méthodes de conversion et le type du receveur (`self`, `&self`, `&mut self`), suivant [`(C-CONV)`].

<!--
clippy::enum_variant_names
clippy::self_named_constructors
-->

</div>

<div class="reco" id="LANG-NAMING" type="Règle" title="Respect des conventions de nommage">

Le développement d'une application sécurisée DOIT suivre les conventions de
nommage décrites dans les [bonnes pratiques Rust @rust-guidelines].

</div>

[`(C-CASE)`]: https://rust-lang.github.io/api-guidelines/naming.html#casing-conforms-to-rfc-430-c-case
[`(C-CONV)`]: https://rust-lang.github.io/api-guidelines/naming.html#ad-hoc-conversions-follow-as_-to_-into_-conventions-c-conv
[`(C-GETTER)`]: https://rust-lang.github.io/api-guidelines/naming.html#getter-names-follow-rust-convention-c-getter
[`(C-ITER)`]: https://rust-lang.github.io/api-guidelines/naming.html#methods-on-collections-that-produce-iterators-follow-iter-iter_mut-into_iter-c-iter
[`(C-ITER-TY)`]: https://rust-lang.github.io/api-guidelines/naming.html#iterator-type-names-match-the-methods-that-produce-them-c-iter-ty
[`(C-FEATURE)`]: https://rust-lang.github.io/api-guidelines/naming.html#feature-names-are-free-of-placeholder-words-c-feature
[`(C-WORD-ORDER)`]: https://rust-lang.github.io/api-guidelines/naming.html#names-use-a-consistent-word-order-c-word-order