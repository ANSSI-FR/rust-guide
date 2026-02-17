---
references:
  - type: web
    title: Recover
    date: 2015
    url: https://rust-lang.github.io/rfcs/1236-stabilize-catch-panic.html
    id: RFC-1236
---

# Gestion des erreurs

<!--
<mark>TODO</mark>: décrire les bonnes pratiques de gestion d'erreurs.
-->

Le type `Result` est la façon privilégiée en Rust pour décrire le type de retour
des fonctions dont le traitement peut échouer. Un objet `Result` doit être
testé et jamais ignoré.

<div class="reco" id="LANG-ERRWRAP" type="Recommandation" title="Mise en place d'un type `Error` personnalisé pouvant contenir toutes les erreurs possibles">

Une *crate* PEUT implanter son propre type `Error` qui peut contenir toutes
les erreurs possibles. Des précautions supplémentaires DOIVENT être prises :
ce type DOIT être *exception-safe* ([RFC 1236 @RFC-1236]) et implémenter les traits
`Error + Send + Sync + 'static` ainsi que `Display`.

</div>

Des *crates* tierces peuvent être utilisées pour faciliter la gestion d'erreurs.
La plupart ([snafu], [thiserror]) proposent la création de types
d'erreurs personnalisées qui implémentent les traits nécessaires et permettent
l'encapsulation d'autres erreurs.

Une autre approche (notamment proposée dans [anyhow]) consiste à envelopper
automatiquement les erreurs dans un seul type d'erreur universel. Une telle
approche ne devrait pas être utilisée dans des bibliothèques ou des systèmes
complexes parce qu'elle ne permet pas de fournir de contexte sur les erreurs
ainsi initialement enveloppées, contrairement à la première approche.

[snafu]: https://crates.io/crates/snafu
[thiserror]: https://crates.io/crates/thiserror
[anyhow]: https://crates.io/crates/anyhow

## *Panics*

Dans le cas général, la gestion explicite des erreurs (`Result`) doit être préférée à la place de
l'utilisation de la macro `panic`. La cause de l'erreur doit être rendue
disponible, et les erreurs trop génériques doivent être évitées.

<div class="reco" id="LANG-LIMIT-PANIC" type="Règle" title="Usage limité des `panic`">

Une fonction Rust NE PEUT émettre de `panic` QUE lorsque les conditions de son usage ont été violées.

</div>

Les *crates* fournissant des bibliothèques ne doivent pas utiliser de fonctions
ou d'instructions qui peuvent échouer en engendrant un `panic`.

Les motifs de code suivants provoquent explicitement des `panic` :

- une utilisation de `unwrap` ou de `expect` ;
- une utilisation de `assert!`.

De la règle [précédente](#LANG-LIMIT-PANIC) découle la règle suivante.

<div class="reco" id="LANG-LIMIT-EXPECT" type="Règle" title="Limitation des fonctions déclenchant des `panic`s">

Les usages de `unwrap`, `expect` et de `assert!` DOIVENT être restreints aux seuls cas interdits par la spécification de la fonction.

</div>

Les fonctions suivantes sont connues pour émettre des `panic` en cas d'arguments ne respectant pas les conditions d'usage.

- un accès non vérifié à un tableau (voir la recommendation [suivante](#LANG-ARRINDEXING)) ;
- un dépassement d'entier (en mode *debug*, voir le chapitre sur le [traitement des entiers](integer.md#chapter-integer)) ;
- une division par zéro ;
- allocation importante de mémoire ;
- l'utilisation de `format!` pour le formatage d'une chaîne de caractères.

<div class="reco" id="LANG-ARRINDEXING" type="Règle" title="Test des indices d'accès aux tableaux ou utilisation de la méthode `get`">

L'indice d'accès à un tableau DOIT être testé, ou la méthode `get` DOIT être
utilisée pour récupérer une `Option`.

</div>

<div class="warning">

Dans certains domaines critiques pour la sureté, il est obligatoire de passer en mode sans échec dès qu'une erreur susceptible d'entraîner un comportement indéfini se produit.
Dans ces situations, il est judicieux d'interrompre l'exécution puisque cela permet d'arrêter le système avant que des données ne soient corrompues, ou des défaillances liées à la sûreté ne se propagent.

Pour un avion ou d'autres types de véhicule, ce comportement « fail-fast » peut être crucial : l'unité de contrôle principale doit s'arrêter immédiatement en cas de défaillance grave, puis transférer le contrôle à un sous-système redondant ou de secours capable d'arrêter le véhicule en toute sécurité ou de poursuivre son fonctionnement en mode réduit. Le redémarrage sur un système secondaire fiable garantit que le véhicule reste contrôlable, protège les occupants et évite les conséquences dangereuses qui pourraient résulter de la poursuite de l'exécution dans un état imprévisible.

Pour ce cas d'usage, activer l'attribut `panic = 'abort'` dans la section `[profile.release]` du fichier Cargo.toml permet d'arrêter le programme dès l'émission du `panic`.

</div>

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

L'intégration d'autres langages en Rust passe par une *FFI* (*Foreign Function Interface*).
Cette fonctionnalité est décrite plus en détails dans le [chapitre dédié](unsafe/ffi.md#ffi-panic).

<!-- ## Macros -->

<!--
<mark>TODO</mark> : complexité cyclomatique du code macro-expansé, limites de
récursion, ...
-->
