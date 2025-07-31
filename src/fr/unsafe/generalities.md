# Généralités sur l'utilisation de `unsafe`

## Comportements ajoutés par Rust *unsafe*

Les capacités du langages peuvent être étendues en utilisant du code *unsafe*. La liste complète de ces capacités est donnée dans le [manuel de Rust](https://doc.rust-lang.org/reference/unsafety.html). On notera les capacités suivantes.

* Déréférencer un *raw pointer*
* Modifier une variable mutable statique
* Accéder aux champs d'une `union`
* Déclarer un block `extern`

Si ces capacités sont nécessaires à la programmation système, elles font perdre au langage ses [propriétés de sûreté](04_language.md#garanties-du-langage).

## Usages du mot-clé `unsafe`

Le mot-clé `unsafe` a deux usages : dans une API et dans une implémentation.

### `unsafe` dans une API

L'usage de ce mot-clé dans une API *met en garde* l'utilisateur de l'API contre
les potentiels effets néfaste de l'usage de l'API.

* Dans une signature de fonction, `unsafe` signifie que le comportement de la fonction
  peut conduire à des *UB* si le contrat d'usage de la fonction (dans sa documentation)
  n'est pas respecté.
* Dans la définition d'un trait, `unsafe` signifie qu'une implémentation erronée de ce trait
  peut conduire à des UB

### `unsafe` dans une implémentation

L'usage de ce mot-clé dans une implémentation (un bloc de code) est imposé par le compilateur
pour empêcher l'usage *par inadvertance* de fonctions marquées `unsafe`.

## Utilisation de Rust *unsafe*

L'utilisation conjointe du système de types et du système d'*ownership* vise à
apporter un haut niveau de sûreté quant à la gestion de la mémoire dans les
programmes écrits en Rust. Le langage permet alors d'éviter les débordements
mémoire, la construction de pointeurs nuls ou invalides, et les problèmes
d'accès concurrents à la mémoire.
Pour effectuer des actions considérées risquées comme des appels système, des
conversions de types ou la manipulation directe de pointeurs mémoire, le
langage fournit le mot-clé `unsafe`.

> **Règle {{#check LANG-UNSAFE | Non-utilisation des blocs *unsafe*}}**
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

## Précautions générales d'un code unsafe

### Préservation des invariants et encapsulation

La protection des invariants d'une librairie est primordiale pour se prémunir de
bugs en général, d'*UB* en particulier.

L'exemple qui suit est extrait du [Rustonomicon](https://doc.rust-lang.org/nomicon/working-with-unsafe.html).

Si l'on souhait réimplémenter le type `Vec`, on pourrait utiliser le code suivant :

```rust
use std::ptr;

pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

// Note this implementation does not correctly handle zero-sized types.
impl<T> Vec<T> {
    pub fn push(&mut self, elem: T) {
        if self.len == self.cap {
            // not important for this example
            self.reallocate();
        }
        unsafe {
            ptr::write(self.ptr.add(self.len), elem);
            self.len += 1;
        }
    }
}
```

La sûreté de ce code repose sur plusieurs invariants, dont l'un stipule que
la plage d'octets allant de `self.ptr` à `self.ptr + self.cap * size_of<T>()` est allouée.

Or, il est possible de casser cet invariant avec du code *safe*. Par exemple


```rust
fn make_room(&mut self) {
    // grow the capacity
    self.cap += 1;
}
```

Si elle peut être tout à fait légitime pour du code *interne* à l'API,
cette fonction ne doit pas être exposée par l'API ou alors doit être annotée 
par `unsafe` car elle peut conduire à des *UB* (même si elle ne comporte pas de blocs *unsafe*s).

### Relation de confiance *safe*/*unsafe*

<!-- Revoir la reformulation -->

#### Principe

Le paradigme de Rust pourrait se résumer à :

> un code sans `unsafe` ne peut pas mal se comporter

c'est-à-dire qu'il ne peut pas produire d'_UB_.
Cette promesse faite au développeur de code sans `unsafe` est perdue lors de l'utilisation de code *unsafe*.
C'est donc au développeur de s'assurer qu'aucun *UB* ne peut se produire dans son code.

En particulier, un bug dans une fonction *safe* utilisée dans un bloc *unsafe*
**doit être contournée par ce code *unsafe*** de manière à ce qu'il n'engendre pas
d'*UB*.

#### Exemple

On peut illustrer ce principe par le cas suivant.

On souhaite proposer une API permettant de parcourir la mémoire pour y trouver un objet d'un type donné.
On demande donc à l'utilisateur de l'API de fournir une implémentation à ce trait

```rust
trait Locatable {
    /// Find object of type `Self` in the buffer `buf`.
    /// Returns the index of the first byte representing
    /// an object of type `Self`
    fn locate_instance_into(buf: &[u8]) -> Option<usize>;
}
```

L'implémentation d'un tel trait peut être réalisée en utilisant du code **sans** `unsafe`.

Par exemple, on peut implémenter ce trait pour le type `bool` comme suit.

```rust
impl Locatable for bool {
    fn locate_instance_into(buf: &[u8]) -> Option<usize> {
        buf.iter().position(|u| *u == 0 || *u == 1)
    }
}
```

D'autre part, la fonction permettant de reconstruire un type `Locatable` pourrait être la suivante.

```rust
fn locate<T: Locatable + Clone>(start: *const u8, len: usize) -> Option<T> {
    let buf = unsafe { from_raw_parts(start, len) };
    match T::locate_instance_into(buf) {
        Some(begin) => unsafe {
            let start_T: *const T = start.byte_add(begin).cast();
            match start_T.as_ref() {
                None => None, // if start_T is null
                Some(r) => Some(r.clone()),
            }
        },
        None => None,
    }
}
```

<div class="warning">

Cette implémentation est mauvaise pour deux raisons :

* dans le cas où l'implémentation de `Locatable` ne donne pas le bon index de 
  départ de l'objet, alors la fonction `as_ref` peut produire un *UB*.
* dans le cas où l'implémentation de `Locatable` renvoie une valeur en dehors du tableau,
  un dépassement de tableau se produit.

</div>

Par exemple, si l'implémentation de `Locatable` est

```rust
impl Locatable for bool {
    fn locate_instance_into(buf: &[u8]) -> Option<usize> {
        buf.iter().position(|u| *u == 0 || *u == 1).map(|n| n - 2)
    }
}
```

l'exécution du programme suivant produit un *UB*

```rust,should_panic
fn main() {
    let buf = [4, 1, 99];
    let start = buf.as_ptr();
    let located_bool: Option<bool> = locate(start, buf.len()); // UB here!
    println!("{:?}", located_bool)
}
```

Voici le retour obtenu avec `valgrind`

```
$ valgrind ./target/release/rust-unsafe
==123651== Memcheck, a memory error detector
==123651== Copyright (C) 2002-2022, and GNU GPL'd, by Julian Seward et al.
==123651== Using Valgrind-3.19.0 and LibVEX; rerun with -h for copyright info
==123651== Command: ./target/release/rust-unsafe
==123651== 
==123651== valgrind: Unrecognised instruction at address 0x10f860.
==123651==    at 0x10F860: rust_unsafe::main (in /home/toto/src/rust-unsafe/target/release/rust-unsafe)
==123651==    by 0x10F842: std::sys::backtrace::__rust_begin_short_backtrace (in /home/toto/src/rust-unsafe/target/release/rust-unsafe)
==123651==    by 0x10F838: std::rt::lang_start::{{closure}} (in /home/toto/src/rust-unsafe/target/release/rust-unsafe)
==123651==    by 0x129F0F: std::rt::lang_start_internal (in /home/toto/src/rust-unsafe/target/release/rust-unsafe)
==123651==    by 0x10F894: main (in /home/toto/src/rust-unsafe/target/release/rust-unsafe)
==123651== Your program just tried to execute an instruction that Valgrind
==123651== did not recognise.  There are two possible reasons for this.
==123651== 1. Your program has a bug and erroneously jumped to a non-code
==123651==    location.  If you are running Memcheck and you just saw a
==123651==    warning about a bad jump, it's probably your program's fault.
==123651== 2. The instruction is legitimate but Valgrind doesn't handle it,
==123651==    i.e. it's Valgrind's fault.  If you think this is the case or
==123651==    you are not sure, please let us know and we'll try to fix it.
==123651== Either way, Valgrind will now raise a SIGILL signal which will
==123651== probably kill your program.
==123651== 
==123651== Process terminating with default action of signal 4 (SIGILL)
==123651==  Illegal opcode at address 0x10F860
==123651==    at 0x10F860: rust_unsafe::main (in /home/toto/src/rust-unsafe/target/release/rust-unsafe)
==123651==    by 0x10F842: std::sys::backtrace::__rust_begin_short_backtrace (in /home/toto/src/rust-unsafe/target/release/rust-unsafe)
==123651==    by 0x10F838: std::rt::lang_start::{{closure}} (in /home/toto/src/rust-unsafe/target/release/rust-unsafe)
==123651==    by 0x129F0F: std::rt::lang_start_internal (in /home/toto/src/rust-unsafe/target/release/rust-unsafe)
==123651==    by 0x10F894: main (in /home/toto/src/rust-unsafe/target/release/rust-unsafe)
==123651== 
==123651== HEAP SUMMARY:
==123651==     in use at exit: 0 bytes in 0 blocks
==123651==   total heap usage: 7 allocs, 7 frees, 2,072 bytes allocated
==123651== 
==123651== All heap blocks were freed -- no leaks are possible
==123651== 
==123651== For lists of detected and suppressed errors, rerun with: -s
==123651== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```

#### Conclusion

Dans cet exemple, il est rappelé que la responsabilité de l'exécution *safe* 
(sans *UB*) d'un code Rust incombe à la personne utilisant des blocs *unsafe*.

S'il n'est pas possible de se protéger contre les fonctions/traits *safe*s lors de l'écriture d'une fonction contenant un bloc *unsafe*,
deux solutions sont possibles :

* marquer la fonction comme *unsafe* : ainsi la responsabilité de sa bonne exécution
  revient à la personne utilisant cette fonction, notamment en l'obligeant à vérifier
  dans la documentation de la fonction que les arguments fournis répondent bien à la 
  spécification de la fonction
  <!-- à voir car ce n'est pas ce que dit la doc rust -->
* marquer le trait dont dépend la fonction comme *unsafe* : ainsi, de même, la responsabilité
  de la bonne exécution du programme revient à l'implémenteur du trait en s'assurant que
  l'implémentation répond bien aux spécifications du trait (présente dans sa documentation).

> **Règle {{#check LANG-UNSAFE-ENCP | Encapsulation des fonctionnalités *unsafe*}}**
>
> Dans un développement sécurisé d'un composant logiciel Rust (_crate_ ou module),
> tout code _unsafe_ doit être encapsulé de manière :
>
> - soit à exposer un comportement _safe_ à l'utilisateur dans lequel aucune
>   interaction _safe_ ne peut aboutir à un _UB_ (comportement indéfini) ;
> - soit à exposer des fonctionnalités marquées `unsafe` dont les conditions
>   d'usages (préconditions, séquencements, etc.) sont exhaustivement données.

Ainsi, une fonction utilisant des opérations `unsafe` peut-être sûre (et donc
sans marque `unsafe`) si les opérations `unsafe` ne présentent pas d'_UB_ étant
donnés les invariants du composant (typiquement l'invariant de type pour une
méthode). Inversement, une fonction sans bloc `unsafe` doit être marquée
`unsafe` si elle casse ces invariants. Le choix et la connaissance de ces
invariants sont donc cruciaux pour le développement sécurisé.

#### Références

* https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html

