# Nommage

La convention de nommage employée par la bibliothèque standard est *de facto* le
standard pour le nommage des éléments des programmes écrits en Rust. Un effort a
été fait pour formaliser ces conventions de nommage, d'abord dans la [RFC 430],
puis dans le document des *[Rust API Guidelines]*.

La règle de base consiste à utiliser :

- `UpperCamelCase` pour les types, traits et valeurs d'énumérations ;
- `snake_case` pour les fonctions, méthodes, macros, variables et modules ;
- `SCREAMING_SNAKE_CASE` pour les variables statiques et les constantes ;
- `'lowercase` pour les durées de vie (*lifetimes*).

Les [Rust API Guidelines] recommandent également des conventions de nommage
plus précises pour certaines constructions particulières :

- (C-CONV) pour les méthodes de conversion (`as_`, `to_`, `into_`) ;
- (C-GETTER) pour les accesseurs ;
- (C-ITER) pour les méthodes produisant des itérateurs ;
- (C-ITER-TY) pour les types itérateur ;
- (C-FEATURE) pour les noms de *features* ;
- (C-WORD-ORDER) pour la cohérence sur l'ordre des mots.

> **Règle {{#check LANG-NAMING | Respect des conventions de nommage}}**
>
> Le développement d'une application sécurisée doit suivre les conventions de
> nommage décrites dans les [Rust API Guidelines].

[rfc 430]: https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md
[rust api guidelines]: https://rust-lang.github.io/api-guidelines/

