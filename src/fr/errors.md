# Gestion des erreurs

<!--
<mark>TODO</mark>: décrire les bonnes pratiques de gestion d'erreurs.
-->

Le type `Result` est la façon privilégiée en Rust pour décrire le type de retour
des fonctions dont le traitement peut échouer. Un objet `Result` doit être
testé et jamais ignoré.

> **Recommandation {{#check LANG-ERRWRAP | Mise en place d'un type `Error` personnalisé, pouvant contenir toutes les erreurs possibles}}**
>
> Une *crate* peut implanter son propre type `Error` qui peut contenir toutes
> les erreurs possibles. Des précautions supplémentaires doivent être prises :
> ce type doit être *exception-safe* (RFC 1236) et implémenter les traits
> `Error + Send + Sync + 'static` ainsi que `Display`.

> **Recommandation {{#check LANG-ERRDO | Utilisation de l'opérateur `?` et non-utilisation de la macro `try!`}}**
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

## *Panics*

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

> **Règle {{#check LANG-NOPANIC | Non-utilisation de fonctions qui peuvent causer des `panic`}}**
>
> Les fonctions et instructions qui peuvent causer des `panic` à l'exécution
> ne doivent pas être utilisées.

> **Règle {{#check LANG-ARRINDEXING | Test des indices d'accès aux tableaux ou utilisation de la méthode `get`}}**
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

## FFI et `panic`s

Lorsque du code Rust est appelé depuis du code écrit dans un autre
langage (par exemple, du code C), le code Rust doit être écrit de sorte à ne
jamais pouvoir paniquer.
Dérouler (*unwinding*) depuis le code Rust vers le code étranger résulte en un
comportement indéfini.

> **Règle {{#check LANG-FFIPANIC | Gestion correcte des `panic!` dans les FFI}}**
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
