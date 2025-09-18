# Généralités sur l'utilisation de `unsafe`

## Comportements ajoutés par Rust *unsafe*

Les capacités du langages peuvent être étendues en utilisant du code `unsafe`. La liste complète de ces capacités est donnée dans le [manuel de référence de Rust][r-unsafety]. On notera les capacités suivantes :

* Déréférencer un *raw pointer*
* Lire ou écrire une variable statique mutable et/ou externe
* Accéder aux champs d'une `union`
* Implémenter un trait `unsafe`
* Déclarer un bloc `extern`

Si ces capacités sont nécessaires à la programmation système, elles font perdre au langage ses [propriétés de sûreté](../guarantees.md).

[r-unsafety]: <https://doc.rust-lang.org/reference/unsafety.html>

## Un mot-clé, deux usages

Le mot-clé `unsafe` a deux usages : le marquage dans une API et le déverrouillage dans une implémentation.

### Marquage `unsafe`

Le marquage `unsafe` est une **délégation de responsabilité** sur la sûreté mémoire du programme du programme en développement.

L'usage de ce mot-clé dans une API *met en garde* l'utilisateur de l'API contre les potentiels effets néfastes de l'usage de l'API.

* Dans une signature de fonction ([r-unsafe.fn]), `unsafe` signifie que le comportement de la fonction peut conduire à des *UB* si le contrat d'usage de la fonction (dans sa documentation) n'est pas respecté.
* Dans la déclaration d'un trait ([r-unsafe.trait]), `unsafe` signifie qu'une implémentation erronée de ce trait peut conduire à des *UB* si le contrat d'implémentation du trait (de préférence documenté) n'est pas respecté.

[r-unsafe.fn]: <https://doc.rust-lang.org/reference/unsafe-keyword.html#r-unsafe.fn>
[r-unsafe.trait]: <https://doc.rust-lang.org/reference/unsafe-keyword.html#r-unsafe.trait>

### Déverrouillage `unsafe`

Le déverrouillage `unsafe` est une **prise de responsabilité** sur la sûreté mémoire du programme en développement.

L'usage d'un bloc `unsafe` dans le corps d'une fonction ou dans la définition d'une constante est imposé par le compilateur ([r-unsafe.block]) pour empêcher l'usage *par inadvertance* d'opérations `unsafe`. Parmi ces opérations, on trouve :

* l'utilisation de fonctions marquées unsafe
* la modification de variable mutables statiques
* l'utilisation de fonctions externes

De manière similaire, l'implémentation d'un trait marqué `unsafe` nécessite `unsafe` ([r-unsafe.impl]) pour indiquer la prise en compte *explicite* par le développeur des contrats de sûreté du trait. Il permet donc de *déverrouiller* l'implémentation de traits `unsafe`.

Enfin, depuis l'édition 2024 de Rust, il est nécessaire également de déverrouiller à l'aide du mot-clé `unsafe` :

* les blocs `extern` ([r-unsafe.extern]) contenant les déclarations externes pour le [FFI](./ffi.md) ;
* certains attributs (par exemple, `no_mangle`, cf. [r-attributes.safety]).

[r-unsafe.impl]: <https://doc.rust-lang.org/reference/unsafe-keyword.html#r-unsafe.impl>
[r-unsafe.block]: <https://doc.rust-lang.org/reference/unsafe-keyword.html#r-unsafe.block>
[r-unsafe.extern]: <https://doc.rust-lang.org/reference/unsafe-keyword.html#r-unsafe.extern>
[r-attributes.safety]: <https://doc.rust-lang.org/reference/attributes.html#r-attributes.safety>

## Limitations et précautions d'usage

Paraphrasant le [Rustonomicon](https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html), le principe fondamental de Rust pourrait se résumer à :

> un code sans `unsafe` ne peut pas mal se comporter

L'utilisation conjointe du système de types et du système d'*ownership* assure l'absence
de comportement indéfini (*UB*) et apporter un haut niveau de sûreté quant à la gestion de la mémoire dans les
programmes écrits en Rust. Le langage permet alors d'éviter les débordements
mémoire, la construction de pointeurs nuls ou invalides, et les problèmes
d'accès concurrents à la mémoire.

Cette promesse faite au développeur de code sans `unsafe` est perdue lors de l'utilisation de code *unsafe*.
C'est donc au développeur de s'assurer qu'aucun *UB* ne peut se produire dans son code.
Aussi, il est important de limiter l'usage de `unsafe` au strict nécessaire :

> **Règle {{#check LANG-UNSAFE | Non-utilisation des blocs *unsafe*}}**
>
> Pour un développement sécurisé, les blocs `unsafe` doivent être évités.
> Ci-dessous, nous listons les seuls cas pour lesquels des blocs `unsafe`
> peuvent être utilisés, à la condition que leur usage soit justifié :
>
> * L'interfaçage entre Rust et d'autres langages (FFI) permet la déclaration de
>   fonctions dont l'implantation est faite en C, en utilisant le préfixe
>   `extern "C"`. Pour utiliser une telle fonction, le mot-clé `unsafe` est
>   requis. Un *wrapper* "sûr" doit être défini pour que le code C soit
>   finalement appelé de façon souple et sûre.
>
> * Pour la programmation des systèmes embarqués, on accède souvent aux
>   registres et à d'autres ressources au travers d'adresses mémoire fixées. Dans
>   ce cas, des blocs `unsafe` sont nécessaires afin de pouvoir initialiser et
>   déréférencer des pointeurs en Rust pour ces adresses. Afin de minimiser le
>   nombre de déclarations `unsafe` pour permettre au développeur de facilement
>   identifier les accès critiques, une abstraction adaptée (structure de
>   données ou module) doit être mise en place.
>
> * Une fonction peut être marquée globalement comme non sûre (en préfixant sa
>   déclaration par le mot-clé `unsafe`) lorsqu'elle exhibe inévitablement des
>   comportements non sûrs en fonction de ses arguments. Par exemple, cela
>   arrive lorsqu'une fonction doit déréférencer un pointeur passé en argument.
>
> À l'exception de l'un ou plusieurs de ces cas `#![forbid(unsafe_code)]` doit
> apparaître dans à la racine de la *crate* (typiquement `main.rs` ou `lib.rs`)
> afin de générer des erreurs de compilation dans le cas ou le mot-clé `unsafe`
> est utilisé dans le projet.

En cas d'usage d'`unsafe`, il est donc de la responsabilité du développeur du programme:

* de s'assurer qu'aucun *UB* n'est possible en cas de déverrouillage ;
* de s'assurer que les conditions d'usage (invariants) soient exhaustives et correctes en cas de marquage.

Au delà du code `unsafe` lui-même, il est important d'encapsuler correctement les opérations `unsafe` dans un composant (*crate* ou module) de manière à rétablir les garanties usuelles de sûreté mémoire de Rust:

> **Règle {{#check LANG-UNSAFE-ENCP | Encapsulation des fonctionnalités *unsafe*}}**
>
> Dans un développement sécurisé d'un composant logiciel Rust (*crate* ou module),
> tout code `unsafe` doit être encapsulé de manière :
>
> * soit à exposer un comportement *safe* à l'utilisateur dans lequel aucune interaction *safe* ne peut aboutir à un *UB* (comportement indéfini) ;
> * soit à exposer des fonctionnalités marquées `unsafe` et dont les conditions d'usages (préconditions, séquencements, etc.) sont commentées exhaustivement et correctement (c'est-à-dire qu'elles impliquent la sûreté mémoire).

Ainsi, une fonction utilisant des opérations `unsafe` peut-être sûre (et donc sans marque `unsafe`) si les opérations `unsafe` ne présentent pas d'*UB* étant donnés les invariants du composant (typiquement l'invariant de type pour une méthode). Inversement, une fonction sans bloc `unsafe` doit être marquée `unsafe` si elle casse ces invariants. Le choix et la connaissance de ces invariants sont donc cruciaux pour le développement sécurisé.

### Exemple 1 : préservation d'un invariant de type

La protection des invariants d'une bibliothèque est primordiale pour se prémunir de
bugs en général, d'*UB* en particulier.

L'exemple qui suit est extrait du [Rustonomicon](https://doc.rust-lang.org/nomicon/working-with-unsafe.html).

Si l'on souhait réimplémenter le type `Vec`, on pourrait utiliser le code suivant :

```rust
{{#include ../../../examples/src/generalities.rs:naive_vec}}
```

La sûreté de ce code repose sur plusieurs invariants, dont l'un stipule que
la plage d'octets allant de `self.ptr` à `self.ptr + self.cap * size_of<T>()` est allouée.

Or, il est possible de casser cet invariant avec du code *safe*. Par exemple, considérons la méthode suivante :

```rust
{{#include ../../../examples/src/generalities.rs:make_room}}
```

Si elle peut être tout à fait légitime pour du code *interne* à l'API,
cette méthode ne doit pas être exposée par l'API ou alors doit être annotée
par `unsafe` car elle peut conduire à des *UB* (même si elle ne comporte pas de blocs *unsafe*s).

### Exemple 2 : relation de confiance *safe*/*unsafe*

La relation de confiance entre le code *safe* et le code `unsafe` est délicate.
Dans un composant logiciel, le code `unsafe` doit être écrit de manière à ce qu'aucun usage *safe* du composant ne puisse conduire à des *UB*.

Le cas suivant illustre ce principe.
On souhaite ici proposer une API générique permettant de localiser un objet dans une zone mémoire.
On demande donc à l'utilisateur de l'API de fournir une implémentation à ce trait :

```rust,ignore
{{#include ../../../examples/src/generalities.rs:Locatable}}
```

L'implémentation d'un tel trait peut être réalisée en utilisant du code **sans** `unsafe`.

Par exemple, on peut implémenter ce trait pour le type `bool` comme suit :

```rust,ignore align
{{#include ../../../examples/src/generalities.rs:Locatable_bool_OK}}
```

<div class="warning">

Cette API est mauvaise pour deux raisons :

* si l'implémentation de `Locatable` ne donne pas l'index d'un objet de type `T`, alors la fonction `read_unaligned` peut produire un *UB*.
* si l'implémentation de `Locatable` renvoie un index en dehors du tableau ou un index tel que l'objet de type `T` ne soit pas entièrement contenu dans le tableau, alors un dépassement de tableau se produit.

</div>

Par exemple, cette implémentation de `Locatable` est fautive mais n'est pas `unsafe` :

```rust,ignore align
{{#include ../../../examples/src/generalities.rs:Locatable_bool_KO}}
```

L'exécution du programme suivant produit un *UB* :

```rust,ignore align
{{#include ../../../examples/src/generalities.rs:Locatable_UB}}
```

Voici le retour obtenu avec l'outil de détection de comportement indéfini `miri` :

```default
$ cargo +nightly miri r --bin overflow
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/cargo-miri runner target/miri/x86_64-unknown-linux-gnu/debug/overflow`
error: Undefined Behavior: in-bounds pointer arithmetic failed: attempting to offset pointer by 101 bytes, but got alloc249 which is only 3 bytes from the end of the allocation
  --> src/overflow.rs:16:29
   |
16 |         let ptr: *const T = buf.as_ptr().add(start).cast();
   |                             ^^^^^^^^^^^^^^^^^^^^^^^ Undefined Behavior occurred here
   |
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
help: alloc249 was allocated here:
  --> src/overflow.rs:22:9
   |
22 |     let buf = [4, 1, 99];
   |         ^^^
   = note: BACKTRACE (of the first span):
   = note: inside `find::<bool>` at src/overflow.rs:16:29: 16:52
note: inside `main`
  --> src/overflow.rs:23:38
   |
23 |     let located_bool: Option<bool> = find(&buf); // UB here!
   |                                      ^^^^^^^^^^

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to 1 previous error
```

Dans cet exemple, il est rappelé que la responsabilité de l'exécution *safe*
(sans *UB*) d'un code Rust incombe à la personne utilisant des blocs *unsafe*.

S'il n'est pas possible de se protéger contre les fonctions/traits *safe*s lors de l'écriture d'une fonction contenant un bloc *unsafe*, deux solutions sont possibles :

* marquer la fonction comme *unsafe* : ainsi la responsabilité de sa bonne exécution revient à la personne utilisant cette fonction, notamment en l'obligeant à vérifier dans la documentation de la fonction que les arguments fournis répondent bien à la   spécification de la fonction
* marquer le trait dont dépend la fonction comme *unsafe* : ainsi, de même, la responsabilité de la bonne exécution du programme revient à l'implémenteur du trait en s'assurant que l'implémentation répond bien aux spécifications du trait (présente dans sa documentation).

<!--
Dans ce cas particulier, la solution la plus adaptée est sans doute de marquer le trait `Locatable` comme `unsafe` :

```rust
{{#include ../../../examples/unsafe2/src/fix.rs::14}}
```

Le supertrait `Copy` permet de s'assurer que le type peut être copié par une simple lecture mémoire.
Une alternative serait de l'ajouter aux conditions de sûreté du trait.
-->
