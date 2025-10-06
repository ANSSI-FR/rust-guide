# Gestion de la mémoire

<!-- ## À propos de la sûreté mémoire en Rust -->

<!--
<mark>TODO</mark> : expliquer les allocations/désallocations sûres,
l'ownership/borrowing, et identifier les constructions de langage qui peuvent
casser la sûreté mémoire (par exemple, comportements *unsounds* dans des
versions plus anciennes du compilateur).
-->

Dans la très grande majorité des cas, en Rust non-*unsafe*, le compilateur détermine **automatiquement** 
quand il peut libérer la mémoire occupée par une valeur du programme. 
Mais, comme rappelé dans les [généralités du langage](../guarantees.md#garantie-de-rust), ce n'est pas une garantie : un
code non-*unsafe* peut mener à des fuites mémoires. Aussi certaines règles présentées dans 
ce chapitre ne sont pas strictement *unsafe*. Cependant,

> même si certaines des fonctions présentées dans la suite ne sont pas `unsafe`, elle
> elle ne devrait être utilisée qu'en Rust *unsafe*.

## `forget` et fuites de mémoire

Rust fournit
des fonctions spéciales pour réclamer manuellement la mémoire : les fonctions
`forget` et `drop` du module `std::mem` (ou `core::mem`). `drop` déclenche
simplement une récupération prématurée de la mémoire tout en appelant les
destructeurs associés lorsque nécessaire, `forget` quant à elle n'appelle pas
ces destructeurs.

```rust
{{#include ../../../examples/src/memory.rs:drop_example}}
```

Les deux fonctions sont considérées comme **sûres du point de vue mémoire** par
Rust. Toutefois, `forget` rendra toute ressource gérée par la valeur libérée
inaccessible, mais non libérée.

```rust
{{#include ../../../examples/src/memory.rs:forget_example}}
```

En particulier, l'utilisation de `forget` peut causer la rétention en mémoire de
ressources critiques, menant à des interblocages et à la persistance de données
sensibles en mémoire. C'est pourquoi `forget` doit être considérée comme
**non sécurisée**.

<div class="reco" id="MEM-FORGET" type="Règle" title="Non-utilisation de `forget`">

Dans un développement sécurisé en Rust (*unsafe* ou non), la fonction `forget` de `std::mem`
(`core::mem`) ne doit pas être utilisée.

</div>

<!-- -->

<div class="reco" id="MEM-FORGET-LINT" type="Recommandation" title="Utilisation du *lint* clippy pour détecter l'utilisation de `forget`">

Le *lint* `mem_forget` de Clippy peut être utilisé pour automatiquement
détecter toute utilisation de la fonction `forget`. Pour s'assurer de l'absence
d'appel à `forget`, ajouter la directive suivante en début de fichier racine
(en général `src/lib.rs` ou `src/main.rs`) :

```rust,noplaypen,ignore
#![deny(clippy::mem_forget)]
```

</div>

La bibliothèque standard inclut d'autres moyens d'*oublier* une valeur :

- `Box::leak` pour libérer une ressource ;
- `Box::into_raw` pour exploiter une valeur dans un bloc *unsafe*, notamment
  dans une FFI ;
- `ManuallyDrop` (dans `std::mem` ou `core::mem`) pour assurer la libération
  manuelle d'une valeur.

Ces alternatives peuvent mener au même type de problème de sécurité, mais ont
l'avantage de faire apparaître explicitement leur but.

<div class="reco" id="MEM-LEAK" type="Règle" title="Non-utilisation de `Box::leak`">

Dans un développement sécurisé (*unsafe* ou non) en Rust, le code ne doit pas faire fuiter de la
mémoire ou des ressources *via* `Box::leak`.

</div>

`ManuallyDrop` et `Box::into_raw` passent la responsabilité de la libération de
la ressource concernée du compilateur au développeur.

<div class="reco" id="MEM-MANUALLYDROP" type="Règle" title="Libération des valeurs *wrappées* dans `ManuallyDrop`">

Dans un développement sécurisé en Rust, toute valeur *wrappée* dans le type
`ManuallyDrop` doit être *unwrapped* pour permettre sa libération automatique
(`ManuallyDrop::into_inner`) ou bien doit être manuellement libérée (*unsafe*
`ManuallyDrop::drop`).

</div>

<!-- -->

## *Raw pointers*

L'utilisation principale des pointeurs *raw* est de traduire les pointeurs C en Rust.
Comme leur nom l'indique, ces types sont *bruts* et n'ont pas toutes les capacités des
pointeurs *intelligents* (*smart pointer*) de Rust. En particulier, leur libération est
à la charge du programmeur.

<div class="reco" id="MEM-NORAWPOINTER" type="Règle" title="Pas de conversion en pointeur *raw* en Rust non-*usafe*">

Dans un développement sécurisé en Rust non-*unsafe*, les références et les *smart pointers*
ne doivent pas être convertis en *raw pointers*. En particulier, les fonctions `into_raw` ou `into_non_null`
des *smart pointers* `Box`, `Rc`, `Arc` ou `Weak` ne doivent pas être utilisées dans un code Rust non-*unsafe*.

</div>

<div class="reco" id="MEM-INTOFROMRAW" type="Règle" title="Appel systématique à `from_raw` pour les valeurs créées avec `into_raw`">

Dans un développement sécurisé en Rust, tout pointeur créé par un appel à
`into_raw` (ou `into_non_null`) depuis un des types suivants doit
finalement être transformé en valeur avec l'appel à la fonction `from_raw`
correspondant, pour permettre sa libération :

- `std::boxed::Box` (ou `alloc::boxed::Box`) ;
- `std::rc::Rc` (ou `alloc::rc::Rc`) ;
- `std::rc::Weak` (ou `alloc::rc::Weak`) ;
- `std::sync::Arc` (ou `alloc::sync::Arc`) ;
- `std::sync::Weak` (ou `alloc::sync::Weak`) ;
- `std::ffi::CString` ;
- `std::ffi::OsString`.

```rust align
{{#include ../../../examples/src/memory.rs:raw_pointer}}
```

</div>

La réciproque est aussi vrai, c'est à dire que les fonctions `from_raw` ne
devraient pas être utilisées sur des *raw pointers* qui ne sont pas issus de la fonction
`into_raw` associée. En effet, pour les cas comme `Rc`, la documentation officielle 
[limite](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.from_raw) explicitement ces fonctions
à ce cas d'usage, et, dans le cas de `Box`, la conversion de pointeurs C en Box 
[n'est pas sûre](https://doc.rust-lang.org/std/boxed/index.html#memory-layout), 

<div class="reco" id="MEM-INTOFROMRAW" type="Règle" title="Appel de `from_raw` uniquement pour les valeurs issues de `into_raw`">

Dans un développement de sécurité en Rust, les fonctions `from_raw` ne doivent être appelées que sur des
valeurs issues de la fonction `into_raw`

</div>

<!-- -->

<div class="note">

Dans le cas de `Box::into_raw`, le nettoyage automatique est possible, mais
est bien plus compliqué que de *re-boxer* le pointeur brut et doit être
évité :

```rust align
{{#include ../../../examples/src/memory.rs:into_raw}}
```

Puisque les autres types (`Rc` et `Arc`) sont opaques et plus complexes, la
libération manuelle n'est pas possible.

</div>

## Mémoire non initialisée

Par défaut, le langage Rust impose que toutes les valeurs soient initialisées, pour
prévenir l'utilisation de mémoire non initialisée (à l'exception de
l'utilisation de `std::mem::uninitialized` ou de `std::mem::MaybeUninit`).

<div class="reco" id="MEM-UNINIT" type="Règle" title="Pas de mémoire non initialisée">

La fonction `std::mem::uninitialized` (dépréciée depuis la version 1.38) ne doit jamais être utilisée.
Le type `std::mem::MaybeUninit` (stabilisé dans la version 1.36) ne doit être
utilisé qu'en fournissant une justification pour chaque cas d'usage.

</div>

L'utilisation de mémoire non initialisée peut induire deux problèmes de
sécurité distincts :

- la libération de mémoire non initialisée (étant également un problème de
  sûreté mémoire) ;
- la non-libération de mémoire initialisée.

<div class="note">

Le type `std::mem::MaybeUninit` est une amélioration de la fonction
`std::mem::uninitialized`. En effet, il rend la libération des valeurs non
initialisées bien plus difficile. Toutefois, cela ne change pas le second
problème : la non-libération de la mémoire initialisée est bien possible.
C'est problématique en particulier si l'on considère l'utilisation de `Drop`
pour effacer des valeurs sensibles.

</div>

## Cycle dans les références comptées (`Rc` et `Arc`)

La **combinaison** de la mutabilité *[intérieure](https://doc.rust-lang.org/reference/interior-mutability.html)*, des références comptées et des types récursifs n'est pas sûre. En effet, elle peut conduire à fuites mémoire, et donc éventuellement à des attaques par déni de service et en des fuites de secrets.

L'exemple non-`unsafe` suivant montre, la création d'une fuite mémoire en utilisant la mutabilité intérieure et les références comptées.

```rust align
{{#include ../../../examples/src/memory.rs:cyclic}}
```

La fuite peut-être mise en évidence grâce à `valgrind` :

```
$ valgrind --leak-check=full target/release/safe-rust-leak 
==153637== Memcheck, a memory error detector
==153637== Copyright (C) 2002-2022, and GNU GPL'd, by Julian Seward et al.
==153637== Using Valgrind-3.19.0 and LibVEX; rerun with -h for copyright info
==153637== Command: target/release/safe-rust-leak
==153637== 
Hello, world!
==153637== 
==153637== HEAP SUMMARY:
==153637==     in use at exit: 48 bytes in 2 blocks
==153637==   total heap usage: 10 allocs, 8 frees, 3,144 bytes allocated
==153637== 
==153637== 48 (24 direct, 24 indirect) bytes in 1 blocks are definitely lost in loss record 2 of 2
==153637==    at 0x48417B4: malloc (vg_replace_malloc.c:381)
==153637==    by 0x10F8D4: safe_rust_leak::main (in /home/toto/src/safe-rust-leak/target/release/safe-rust-leak)
==153637==    by 0x10F7E2: std::sys::backtrace::__rust_begin_short_backtrace (in /home/toto/src/safe-rust-leak/target/release/safe-rust-leak)
==153637==    by 0x10F7D8: std::rt::lang_start::{{closure}} (in /home/toto/src/safe-rust-leak/target/release/safe-rust-leak)
==153637==    by 0x12A90F: std::rt::lang_start_internal (in /home/toto/src/safe-rust-leak/target/release/safe-rust-leak)
==153637==    by 0x10FA54: main (in /home/toto/src/safe-rust-leak/target/release/safe-rust-leak)
==153637== 
==153637== LEAK SUMMARY:
==153637==    definitely lost: 24 bytes in 1 blocks
==153637==    indirectly lost: 24 bytes in 1 blocks
==153637==      possibly lost: 0 bytes in 0 blocks
==153637==    still reachable: 0 bytes in 0 blocks
==153637==         suppressed: 0 bytes in 0 blocks
==153637== 
==153637== For lists of detected and suppressed errors, rerun with: -s
==153637== ERROR SUMMARY: 1 errors from 1 contexts (suppressed: 0 from 0)
```

<div class="reco" id="MEM-MUT-REC-RC" type="Règle" title="Éviter les références comptées récursives mutables">

Éviter de définir des types à la fois récursifs, mutables *intérieurement*, et dont la récursion se base sur l'utilisation des références comptées `Rc` ou `Arc`.

</div>
