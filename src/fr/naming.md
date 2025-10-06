---
references:
  - type: article
    title: General naming conventions
    date: 2014
    url: https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md
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
été fait pour formaliser ces conventions de nommage, d'abord dans la [@RFC-430],
puis dans le document des *[@rust-guidelines]*.

La règle de base consiste à utiliser :

- `UpperCamelCase` pour les types, traits et valeurs d'énumérations ;
- `snake_case` pour les fonctions, méthodes, macros, variables et modules ;
- `SCREAMING_SNAKE_CASE` pour les variables statiques et les constantes ;
- `'lowercase` pour les durées de vie (*lifetimes*).

Les [@rust-guidelines] recommandent également des conventions de nommage
plus précises pour certaines constructions particulières :

- (C-CONV) pour les méthodes de conversion (`as_`, `to_`, `into_`) ;
- (C-GETTER) pour les accesseurs ;
- (C-ITER) pour les méthodes produisant des itérateurs ;
- (C-ITER-TY) pour les types itérateur ;
- (C-FEATURE) pour les noms de *features* ;
- (C-WORD-ORDER) pour la cohérence sur l'ordre des mots.

<div class="reco" id="LANG-NAMING" type="Règle" title="Respect des conventions de nommage">

Le développement d'une application sécurisée doit suivre les conventions de
nommage décrites dans les [@rust-guidelines].

</div>

