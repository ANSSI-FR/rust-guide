# Gestion de la mémoire {#chapter-memory}

<!-- ## À propos de la sûreté mémoire en Rust -->

<!--
<mark>TODO</mark> : expliquer les allocations/désallocations sûres,
l'ownership/borrowing, et identifier les constructions de langage qui peuvent
casser la sûreté mémoire (par exemple, comportements *unsounds* dans des
versions plus anciennes du compilateur).
-->

Dans la très grande majorité des cas, en Rust non-*unsafe*, le compilateur détermine **automatiquement** 
quand il peut libérer la mémoire occupée par une valeur du programme. 
Mais, comme rappelé [plus tôt dans ce guide](../guarantees.md#garantie-de-rust), ce n'est pas une garantie : un
code non-*unsafe* peut mener à des fuites mémoires. Aussi, certaines règles présentées dans ce chapitre ne sont pas strictement *unsafe*.
Cependant,

> même si certaines des fonctions présentées dans la suite ne sont pas `unsafe`,
> elles ne devraient être utilisées qu'en Rust *unsafe*.

## [`mem::forget`] et fuites de mémoire {#forget-and-memory-leaks}

Rust fournit
des fonctions spéciales pour réclamer manuellement la mémoire : les fonctions
[`mem::forget`] et [`mem::drop`] du module `std::mem` (ou `core::mem`). [`mem::drop`] déclenche
simplement une récupération prématurée de la mémoire tout en appelant les
destructeurs associés lorsque nécessaire, [`mem::forget`] quant à elle n'appelle pas
ces destructeurs.

```rust align
{{#include ../../../examples/src/memory.rs:drop_example}}
```

Les deux fonctions sont considérées comme **sûres du point de vue mémoire** par
Rust. Toutefois, [`mem::forget`] rendra toute ressource gérée par la valeur libérée
inaccessible, mais non libérée.

```rust align bad
{{#include ../../../examples/src/memory.rs:forget_example}}
```

En particulier, l'utilisation de [`mem::forget`] peut causer la rétention en mémoire de
ressources critiques, menant à des interblocages et à la persistance de données
sensibles en mémoire. C'est pourquoi [`mem::forget`] doit être considérée comme
**non sécurisée**.

<div class="reco" id="MEM-FORGET" type="Règle" title="Non-utilisation de `mem::forget`">

Dans un développement sécurisé en Rust (*unsafe* ou non), la fonction [`mem::forget`] de `std::mem`
(`core::mem`) NE DOIT PAS être utilisée.

</div>

<!-- -->

<div class="reco" id="MEM-FORGET-LINT" type="Recommandation" title="Utilisation du *lint* clippy pour détecter l'utilisation de `mem::forget`">

Le *lint* `mem_forget` de Clippy DEVRAIT être utilisé pour automatiquement
détecter toute utilisation de la fonction [`mem::forget`]. Pour s'assurer de l'absence
d'appel à [`mem::forget`], ajouter la directive suivante en début de fichier racine
(en général `src/lib.rs` ou `src/main.rs`) :

```rust,noplaypen,ignore
#![deny(clippy::mem_forget)]
```

</div>

La bibliothèque standard inclut d'autres moyens d'*oublier* une valeur :

- [`Box::leak`] pour libérer une ressource ;
- [`Box::into_raw`] pour exploiter une valeur dans un bloc *unsafe*, notamment
  dans une FFI ;
- [`ManuallyDrop`] (dans `std::mem` ou `core::mem`) pour assurer la libération
  manuelle d'une valeur.

Ces alternatives peuvent mener au même type de problème de sécurité, mais ont
l'avantage de faire apparaître explicitement leur but.

<div class="reco" id="MEM-LEAK" type="Règle" title="Non-utilisation de `Box::leak`">

Dans un développement sécurisé (*unsafe* ou non) en Rust, le code NE DOIT PAS faire fuiter de la
mémoire ou des ressources *via* [`Box::leak`].

</div>

[`ManuallyDrop`] et [`Box::into_raw`] passent la responsabilité de la libération de
la ressource concernée du compilateur au développeur.

<div class="reco" id="MEM-MANUALLYDROP" type="Règle" title="Libération des valeurs *wrappées* dans `ManuallyDrop`">

Dans un développement sécurisé en Rust, toute valeur *wrappée* dans le type
[`ManuallyDrop`] DOIT être *unwrapped* pour permettre sa libération automatique
([`ManuallyDrop::into_inner`]) ou bien DOIT être manuellement libérée (*unsafe*
[`ManuallyDrop::drop`]).

</div>

<!-- -->

[`mem::forget`]: https://doc.rust-lang.org/std/mem/fn.forget.html
[`mem::drop`]: https://doc.rust-lang.org/std/mem/fn.drop.html
[`Drop`]: https://doc.rust-lang.org/std/ops/trait.Drop.html
[`Box::leak`]: https://doc.rust-lang.org/std/boxed/struct.Box.html#method.leak
[`Box::into_raw`]: https://doc.rust-lang.org/std/boxed/struct.Box.html#method.into_raw
[`ManuallyDrop`]: https://doc.rust-lang.org/beta/std/mem/struct.ManuallyDrop.html
[`ManuallyDrop::into_inner`]: https://doc.rust-lang.org/beta/std/mem/struct.ManuallyDrop.html#method.into_inner
[`ManuallyDrop::drop`]: https://doc.rust-lang.org/beta/std/mem/struct.ManuallyDrop.html#method.drop

## *Raw pointers*

L'utilisation principale des pointeurs *raw* est de traduire les pointeurs C en Rust.
Comme leur nom l'indique, ces types sont *bruts* et n'ont pas toutes les capacités des
pointeurs *intelligents* (*smart pointer*) de Rust. En particulier, leur libération est
à la charge du programmeur.

<div class="reco" id="MEM-NORAWPOINTER" type="Règle" title="Pas de conversion en pointeur *raw* en Rust non-*unsafe*">

Dans un développement sécurisé en Rust non-*unsafe*, les références et les *smart pointers*
NE DOIVENT PAS être convertis en *raw pointers*. En particulier, les fonctions `into_raw` ou `into_non_null`
des *smart pointers* [`Box`], [`Rc`], [`Arc`], [`rc::Weak`] ou [`sync::Weak`] NE DOIVENT PAS être utilisées dans un code Rust non-*unsafe*.

</div>

<div class="reco" id="MEM-INTOFROMRAWALWAYS" type="Règle" title="Appel systématique à `from_raw` pour les valeurs créées avec `into_raw`">

Dans un développement sécurisé en Rust, tout pointeur créé par un appel à
`into_raw` (ou `into_non_null`) depuis un des types suivants DOIT
finalement être transformé en valeur avec l'appel à la fonction `from_raw`
correspondant, pour permettre sa libération :

- [`std::boxed::Box`] (ou [`alloc::boxed::Box`]) ;
- [`std::rc::Rc`] (ou [`alloc::rc::Rc`]) ;
- [`std::rc::Weak`] (ou [`alloc::rc::Weak`]) ;
- [`std::sync::Arc`] (ou [`alloc::sync::Arc`]) ;
- [`std::sync::Weak`] (ou [`alloc::sync::Weak`]) ;
- [`std::ffi::CString`] ;
- [`std::ffi::OsString`].

```rust align
{{#include ../../../examples/src/memory.rs:raw_pointer}}
```

</div>

La réciproque est aussi vraie, c'est-à-dire que les fonctions `from_raw` ne
devraient pas être utilisées sur des *raw pointers* qui ne sont pas issus de la fonction
`into_raw` associée. En effet, pour les cas comme [`Rc`], la documentation officielle 
[limite](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.from_raw) explicitement ces fonctions
à ce cas d'usage, et, dans le cas de [`Box`], la conversion de pointeurs C en [`Box`] 
[n'est pas sûre](https://doc.rust-lang.org/std/boxed/index.html#memory-layout), 

<div class="reco" id="MEM-INTOFROMRAWONLY" type="Règle" title="Appel de `from_raw` uniquement pour les valeurs issues de `into_raw`">

Dans un développement de sécurité en Rust, les fonctions `from_raw` NE DOIVENT être appelées QUE sur des
valeurs issues de la fonction `into_raw`

</div>

<!-- -->

<div class="note">

Dans le cas de [`Box::into_raw`], le nettoyage automatique est possible, mais
est bien plus compliqué que de *re-boxer* le pointeur brut et doit être
évité :

```rust align bad
{{#include ../../../examples/src/memory.rs:into_raw}}
```

Puisque les autres types ([`Rc`] et [`Arc`]) sont opaques et plus complexes, la
libération manuelle n'est pas possible.

</div>

[`Box`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
[`std::boxed::Box`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
[`alloc::boxed::Box`]: https://doc.rust-lang.org/alloc/boxed/struct.Box.html
[`Rc`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
[`std::rc::Rc`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
[`alloc::rc::Rc`]: https://doc.rust-lang.org/alloc/rc/struct.Rc.html
[`rc::Weak`]: https://doc.rust-lang.org/std/rc/struct.Weak.html
[`std::rc::Weak`]: https://doc.rust-lang.org/std/rc/struct.Weak.html
[`alloc::rc::Weak`]: https://doc.rust-lang.org/alloc/rc/struct.Weak.html
[`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[`std::sync::Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[`alloc::sync::Arc`]: https://doc.rust-lang.org/alloc/sync/struct.Arc.html
[`sync::Weak`]: https://doc.rust-lang.org/std/sync/struct.Weak.html
[`std::sync::Weak`]: https://doc.rust-lang.org/std/sync/struct.Weak.html
[`alloc::sync::Weak`]: https://doc.rust-lang.org/alloc/sync/struct.Weak.html
[`std::ffi::CString`]: https://doc.rust-lang.org/std/ffi/struct.CString.html
[`std::ffi::OsString`]: https://doc.rust-lang.org/std/ffi/struct.OsString.html

## Mémoire non initialisée

Par défaut, le langage Rust impose que toutes les valeurs soient initialisées, pour
prévenir l'utilisation de mémoire non initialisée (à l'exception de
l'utilisation de [`std::mem::uninitialized`] ou de [`std::mem::MaybeUninit`]).

<div class="reco" id="MEM-UNINIT" type="Règle" title="Pas de mémoire non initialisée">

La fonction [`std::mem::uninitialized`] (dépréciée depuis la version 1.38) NE DOIT PAS être utilisée.
Le type [`std::mem::MaybeUninit`] (stabilisé dans la version 1.36) NE DOIT être
utilisé QU'en fournissant une justification pour chaque cas d'usage.

</div>

L'utilisation de mémoire non initialisée peut induire deux problèmes de
sécurité distincts :

- la libération de mémoire non initialisée (étant également un problème de
  sûreté mémoire) ;
- la non-libération de mémoire initialisée.

<div class="note">

Le type [`std::mem::MaybeUninit`] est une amélioration de la fonction
[`std::mem::uninitialized`]. En effet, il rend la libération des valeurs non
initialisées bien plus difficile. Toutefois, cela ne change pas le second
problème : la non-libération de la mémoire initialisée est bien possible.
C'est problématique en particulier si l'on considère l'utilisation de [`Drop`]
pour effacer des valeurs sensibles.

</div>

[`std::mem::uninitialized`]: https://doc.rust-lang.org/std/mem/fn.uninitialized.html
[`std::mem::MaybeUninit`]: https://doc.rust-lang.org/beta/std/mem/union.MaybeUninit.html
