# Généralités sur le langage

## Nommage

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

> ### Règle {{#check LANG-NAMING | Respect des conventions de nommage}}
>
> Le développement d'une application sécurisée doit suivre les conventions de
> nommage décrites dans les [Rust API Guidelines].

[rfc 430]: https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md
[rust api guidelines]: https://rust-lang.github.io/api-guidelines/

## Code *unsafe*

L'utilisation conjointe du système de types et du système d'*ownership* vise à
apporter un haut niveau de sûreté quant à la gestion de la mémoire dans les
programmes écrits en Rust. Le langage permet alors d'éviter les débordements
mémoire, la construction de pointeurs nuls ou invalides, et les problèmes
d'accès concurrents à la mémoire.
Pour effectuer des actions considérées risquées comme des appels système, des
conversions de types ou la manipulation directe de pointeurs mémoire, le
langage fournit le mot-clé `unsafe`.

> ### Règle {{#check LANG-UNSAFE | Non-utilisation des blocs *unsafe*}}
>
> Pour un développement sécurisé, les blocs `unsafe` doivent être évités.
> Ci-dessous, nous listons les seuls cas pour lesquels des blocs `unsafe`
> peuvent être utilisés, à la condition que leur usage soit justifié :
>
>  - L'interfaçage entre Rust et d'autres langages (FFI) permet la déclaration
>  de fonctions dont l'implantation est faite en C, en utilisant le préfixe
>  `extern "C"`. Pour utiliser une telle fonction, le mot-clé `unsafe` est
>  requis. Un *wrapper* "sûr" doit être défini pour que le code C soit
>  finalement appelé de façon souple et sûre.
>
>  - Pour la programmation des systèmes embarqués, on accède souvent aux
>  registres et à d'autres ressources au travers d'adresses mémoire fixées
>  Dans ce cas, des blocs `unsafe` sont nécessaires afin de pouvoir initialiser
>  et déréférencer des pointeurs en Rust pour ces adresses. Afin de minimiser le
>  nombre de déclarations `unsafe` pour permettre au développeur de facilement
>  identifier les accès critiques, une abstraction adaptée (structure de
>  données ou module) doit être mise en place.
>
>  - Une fonction peut être marquée globalement comme non sûre (en préfixant sa
>  déclaration par le mot-clé `unsafe`) lorsqu'elle exhibe inévitablement des
>  comportements non sûrs en fonction de ses arguments. Par exemple, cela arrive
>  lorsqu'une fonction doit déréférencer un pointeur passé en argument.
>
> À l'exception de l'un ou plusieurs de ces cas `#![forbid(unsafe_code)]` doit
> apparaître dans à la racine de la *crate* (typiquement `main.rs` ou `lib.rs`)
> afin de générer des erreurs de compilation dans le cas ou le mot-clé `unsafe`
> est utilisé dans le projet.

## Dépassement d'entiers

Bien que des vérifications soient effectuées par Rust en ce qui concerne les
potentiels dépassements d'entiers, des précautions doivent être prises lors de
l'exécution d'opérations arithmétiques sur les entiers.

En particulier, il doit être noté que les profils de compilation *debug* et
*release* produisent des variations de comportements quant à la gestion des
dépassements d'entiers. Dans la configuration *debug*, un dépassement provoque
la terminaison du programme (`panic`), tandis que dans la configuration
*release* la valeur calculée est silencieusement tronquée en fonction de la
valeur maximum qui peut être stockée pour le type considéré.

Ce comportement peut être rendu explicite en utilisant le type générique
`Wrapping`, ou les opérations sur les entiers `overflowing_<op>` et
`wrapping_<op>` (la partie `<op>` étant remplacée par le type de calcul :
`add`, `mul`, `sub`, `shr`, etc.).

```rust
use std::num::Wrapping;
# use std::panic;

# fn main() {
let x: u8 = 242;

# let result = panic::catch_unwind(|| {
println!("{}", x + 50);                      // panique en mode debug, affiche 36 en mode release.
# });
# if result.is_err() { println!("panic"); }
println!("{}", x.overflowing_add(50).0);     // affiche toujours 36.
println!("{}", x.wrapping_add(50));          // affiche toujours 36.
println!("{}", Wrapping(x) + Wrapping(50));  // affiche toujours 36.

// panique toujours :
let (res, c) = x.overflowing_add(50);
# let result = panic::catch_unwind(|| {
if c { panic!("custom error"); }
else { println!("{}", res); }
# });
# if result.is_err() { println!("panic"); }
# }
```

> ### Règle {{#check LANG-ARITH | Utilisation des opérations arithmétiques appropriées au regard des potentiels dépassements}}
>
> Lorsqu'une opération arithmétique peut produire un dépassement d'entier, les
> fonctions spécialisées `overflowing_<op>`, `wrapping_<op>` ou le type
> `Wrapping` doivent être utilisés.

## Gestion des erreurs

<!--
<mark>TODO</mark>: décrire les bonnes pratiques de gestion d'erreurs.
-->

Le type `Result` est la façon privilégiée en Rust pour décrire le type de retour
des fonctions dont le traitement peut échouer. Un objet `Result` doit être
testé et jamais ignoré.

> ### Recommandation {{#check LANG-ERRWRAP | Mise en place d'un type `Error` personnalisé, pouvant contenir toutes les erreurs possibles}}
>
> Une *crate* peut implanter son propre type `Error` qui peut contenir toutes
> les erreurs possibles. Des précautions supplémentaires doivent être prises :
> ce type doit être *exception-safe* (RFC 1236) et implémenter les traits
> `Error + Send + Sync + 'static` ainsi que `Display`.

> ### Recommandation {{#check LANG-ERRDO | Utilisation de l'opérateur `?` et non-utilisation de la macro `try!`}}
>
> L'opérateur `?` doit être utilisé pour améliorer la lisibilité du code.
> La macro `try!` ne doit pas être utilisée.

Des *crates* tierces peuvent être utilisées pour faciliter la gestion d'erreurs.
La plupart ([failure], [snafu], [thiserror]) proposent la création de types
d'erreurs personnalisées qui implémentent les traits nécessaires et permettent
l'encapsulation d'autres erreurs.

Une autre approche (notamment proposée dans [anyhow]) consiste à envelopper
automatiquement les erreurs dans un seul type d'erreur universel. Une telle
approche ne devrait pas être utilisée dans des bibliothèques ou des systèmes
complexes parce qu'elle ne permet pas de fournir de contexte sur les erreurs
ainsi initialement enveloppées, contrairement à la première approche.

[failure]: https://crates.io/crates/failure
[snafu]: https://crates.io/crates/snafu
[thiserror]: https://crates.io/crates/thiserror
[anyhow]: https://crates.io/crates/anyhow

### *Panics*

La gestion explicite des erreurs (`Result`) doit être préférée à la place de
l'utilisation de la macro `panic`. La cause de l'erreur doit être rendue
disponible, et les erreurs trop génériques doivent être évitées.

Les *crates* fournissant des bibliothèques ne doivent pas utiliser de fonctions
ou d'instructions qui peuvent échouer en engendrant un `panic`.

Des motifs courants de code qui provoquent des `panic` sont :

- une utilisation de `unwrap` ou de `expect` ;
- une utilisation de `assert` ;
- un accès non vérifié à un tableau ;
- un dépassement d'entier (en mode *debug*) ;
- une division par zéro ;
- l'utilisation de `format!` pour le formatage d'une chaîne de caractères.

> ### Règle {{#check LANG-NOPANIC | Non-utilisation de fonctions qui peuvent causer des `panic`}}
>
> Les fonctions et instructions qui peuvent causer des `panic` à l'exécution
> ne doivent pas être utilisées.

> ### Règle {{#check LANG-ARRINDEXING | Test des indices d'accès aux tableaux ou utilisation de la méthode `get`}}
>
> L'indice d'accès à un tableau doit être testé, ou la méthode `get` doit être
> utilisée pour récupérer une `Option`.

<!--
<mark>TODO</mark> Vérifier si la crate *[no_panic](https://github.com/dtolnay/no-panic)*
peut détecter tous les cas. Inconvénient : toutes les fonctions doivent être
marquées avec `#[no_panic]`.
-->
<!--
<mark>TODO</mark> Another possibility:
[rustig](https://github.com/Technolution/rustig) (doesn't build here)
-->

### FFI et `panic`s

Lorsque du code Rust est appelé depuis du code écrit dans un autre
langage (par exemple, du code C), le code Rust doit être écrit de sorte à ne
jamais pouvoir paniquer.
Dérouler (*unwinding*) depuis le code Rust vers le code étranger résulte en un
comportement indéfini.

> ### Règle {{#check LANG-FFIPANIC | Gestion correcte des `panic!` dans les FFI}}
>
> Le code Rust appelé depuis une FFI doit soit être assuré de ne pas paniquer,
> soit utiliser `catch_unwind` ou le module `std::panic` pour s'assurer qu'il
> ne va pas abandonner un traitement puis que l'exécution retourne dans le
> langage appelant dans un état instable.

Il est porté à l'attention du développeur que `catch_unwind` ne va traiter que
les cas de `panic`, et va préserver les abandons de processus causés par
d'autres raisons.

<!-- ## Macros -->

<!--
<mark>TODO</mark> : complexité cyclomatique du code macro-expansé, limites de
récursion, ...
-->
