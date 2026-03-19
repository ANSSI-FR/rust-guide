---
references:
  - type: web
    title: Specialization
    url: https://rust-lang.github.io/rfcs/1210-impl-specialization.html
    id: RFC-1210
  - type: web
    title: The Rustonomicon
    url: https://doc.rust-lang.org/stable/nomicon/
    id: nomicon
---

# Bibliothèque standard

## Les traits [`Send`] et [`Sync`]

Les traits [`Send`] et [`Sync`] (définis dans `std::marker` ou `core::marker`) sont
des marqueurs utilisés pour assurer la sûreté des accès concurrents en Rust.
Lorsqu'ils sont correctement implémentés, ils permettent au compilateur Rust de
garantir l'absence de problèmes d'accès concurrents. Leurs sémantiques sont
définies comme suit :

- Un type est [`Send`] s’il est sûr d'envoyer (*move*) des valeurs de ce type vers
  un autre fil d'exécution.
- Un type est [`Sync`] s’il est sûr de partager des valeurs de ce type par une
  référence immuable avec un autre fil d'exécution.

Ces deux traits sont des traits *unsafe*, c'est-à-dire que le compilateur Rust
ne vérifie d'aucune manière que leur implémentation est correcte. Le danger est
réel : une implémentation incorrecte peut mener à un **comportement indéfini**.

Heureusement, dans la plupart des cas, il n'est pas nécessaire de fournir une
implémentation. En Rust, la quasi-totalité des types primitifs implémente
[`Send`] et [`Sync`], et dans la majorité des cas, Rust fournit de manière
automatique une implémentation pour les types composés. Comme rappelé dans
[Rustonomicon @nomicon], quelques exceptions notables sont :

- les pointeurs `raw`, qui n'implémentent ni [`Send`], ni [`Sync`], puisqu'ils
  n'offrent aucune garantie quant à la sûreté ;
- les références [`UnsafeCell`], qui n'implémentent pas [`Sync`] (et par extension,
  les références [`Cell`] et [`RefCell`] non plus), puisqu'elles autorisent la
  mutabilité des valeurs contenues (*interior mutability*) ;
- les références [`Rc`], qui n'implémentent ni [`Send`], ni [`Sync`], puisque les
  compteurs de références seraient partagés de manière désynchronisée.

L'implémentation automatique de [`Send`] (respectivement [`Sync`]) a lieu pour les
types composés (structures ou énumérations) lorsque tous les champs contenus
implémentent [`Send`] (respectivement [`Sync`]).

Afin d’empêcher *artificiellement* qu'un type n'implémente [`Send`] ou [`Sync`],
il est possible d'utiliser un champ typé par un type fantôme ([`PhantomData`]) :

```rust,noplaypen
# use std::marker::PhantomData;
#
struct SpecialType(u8, PhantomData<*const ()>);
```

<div class="reco" id="LANG-SYNC-TRAITS" type="Règle" title="Justification de l'implémentation des traits `Send` et `Sync`">

Dans un développement sécurisé en Rust, les traits
[`Send`] et [`Sync`] NE DEVRAIENT PAS être implémentés manuellement, et, si nécessaire,
l'implémentation DOIT être justifiée et documentée.

</div>

## Les traits de comparaison : [`PartialEq`], [`Eq`], [`PartialOrd`], [`Ord`]

Les comparaisons (`==`, `!=`, `<`, `<=`, `>`, `>=`) en Rust reposent sur quatre
traits de la bibliothèque standard, disponibles dans `std::cmp` (ou `core::cmp`
pour une compilation avec `no_std`) :

- [`PartialEq<Rhs>`] qui définit la relation d'équivalence partielle entre objets
  de types `Self` et `Rhs` ;
- [`PartialOrd<Rhs>`] qui définit la relation d'ordre partiel entre les objets de
  types `Self` et `Rhs` ;
- [`Eq`] qui définit la relation d'équivalence totale entre les objets du même
  type. Il s'agit d'un trait de marquage qui requiert le trait
  `PartialEq<Self>` ;
- [`Ord`] qui définit la relation d'ordre total entre les objets du même type.
  Le trait `PartialOrd<Self>` est alors requis.

Comme stipulé dans la documentation de la bibliothèque standard, Rust présuppose
**de nombreux invariants** lors de l'implémentation de ces traits :

- Pour [`PartialEq`] :

  - *Cohérence interne* : `a.ne(b)` est équivalent à `!a.eq(b)`, c'est-à-dire, `ne`
    est le strict inverse de `eq`. Cela correspond précisément à
    l'implémentation par défaut de `ne`.

  - *Symétrie* : `a.eq(b)` et `b.eq(a)` sont équivalents. Du point de vue du
    développeur, cela signifie que :

    - `PartialEq<B>` est implémenté pour le type `A` (noté `A: PartialEq<B>`).
    - `PartialEq<A>` est implémenté pour le type `B` (noté `B: PartialEq<A>`).
    - Les deux implémentations sont cohérentes l'une avec l'autre.

  - *Transitivité* : `a.eq(b)` et `b.eq(c)` impliquent `a.eq(c)`. Cela signifie
    que :

    - `A: PartialEq<B>`.
    - `B: PartialEq<C>`.
    - `A: PartialEq<C>`.
    - Les trois implémentations sont cohérentes les unes avec les autres (ainsi
      qu'avec leurs implémentations symétriques).

- Pour [`Eq`] :

  - `PartialEq<Self>` est implémenté.

  - *Réflexivité* : `a.eq(a)`. Cela signifie que `PartialEq<Self>` est
    implémenté ([`Eq`] ne fournit aucune méthode).

- Pour [`PartialOrd`] :

  - *Consistance de la relation d'égalité* : `a.eq(b)` est équivalent à
    `a.partial_cmp(b) == Some(std::ordering::Eq)`.

  - *Consistence interne* :

    - `a.lt(b)` ssi `a.partial_cmp(b) == Some(std::ordering::Less)`.
    - `a.gt(b)` ssi `a.partial_cmp(b) == Some(std::ordering::Greater)`.
    - `a.le(b)` ssi `a.lt(b) || a.eq(b)`.
    - `a.ge(b)` ssi `a.gt(b) || a.eq(b)`.

    Il faut noter qu'en définissant seulement `partial_cmp`, la cohérence
    interne est garantie par les implémentations par défaut de `lt`, `le`, `gt`,
    et `ge`.

  - *Antisymétrie* : `a.lt(b)` (respectivement `a.gt(b)`) implique `b.gt(a)`
    (respectivement `b.lt(a)`). Du point de vue du développeur, cela signifie
    que :

    - `A: PartialOrd<B>`.
    - `B: PartialOrd<A>`.
    - Les deux implémentations sont cohérentes l'une avec l'autre.

  - *Transitivité* : `a.lt(b)` et `b.lt(c)` impliquent `a.lt(c)` (également avec
    `gt`, `le` et `ge`). Cela signifie que :

    - `A: PartialOrd<B>`.
    - `B: PartialOrd<C>`.
    - `A: PartialOrd<C>`.
    - Les trois implémentations sont cohérentes les unes avec les autres (et
      avec leurs implémentations symétriques).

- Pour [`Ord`] :

  - `PartialOrd<Self>`

  - *Totalité* : `a.partial_cmp(b) != None` est toujours vrai. En d'autres mots,
    exactement une assertion parmi `a.eq(b)`, `a.lt(b)` et `a.gt(b)`, est vraie.

  - *Cohérence avec `PartialOrd<Self>`*: `Some(a.cmp(b)) == a.partial_cmp(b)`.

Le compilateur ne vérifie aucun de ces prérequis, à l'exception des
vérifications sur les types. Toutefois, les comparaisons sont des éléments
importants puisqu'elles jouent un rôle tant dans les propriétés de vivacité
des systèmes critiques comme des ordonnanceurs ou des répartiteurs de charge
que dans les algorithmes optimisés qui peuvent éventuellement utiliser des
blocs `unsafe`. Dans le premier cas d'usage, une mauvaise relation d'ordre
peut causer des problèmes de disponibilité comme des interblocages. Dans le
second cas, cela peut mener à des problèmes classiques de sécurité liés à des
violations de propriétés de sûreté mémoire. C'est là encore un atout que de
limiter au possible l'utilisation des blocs `unsafe`.

<div class="reco" id="LANG-CMP-INV" type="Règle" title="Respect des invariants des traits de comparaison standards">

Dans un développement sécurisé en Rust, l'implémentation des traits de
comparaison standards DOIT respecter les invariants décrits dans la
documentation.

</div>

<div class="reco" id="LANG-CMP-DEFAULTS" type="Recommandation" title="Utilisation des implémentations par défaut des traits de comparaison standards">

Dans un développement sécurisé en Rust, l'implémentation des traits de
comparaison standard NE DEVRAIT être effectuée que par l'implémentation des
méthodes ne fournissant pas d'implémentation par défaut, dans le but de
réduire le risque de violation des invariants associés auxdits traits.

</div>

Il existe un *lint* Clippy qui permet de vérifier que `PartialEq::ne` n'est pas
défini lors d'une implémentation du trait [`PartialEq`].

Rust propose une façon de fournir automatiquement des implémentations par défaut
pour les traits de comparaison, au travers de l'attribut `#[derive(...)]` :

- La dérivation de [`PartialEq`] implémente `PartialEq<Self>` avec une **égalité
  structurelle** à condition que chacun des types des données membres implémente
  `PartialEq<Self>`.
- La dérivation de [`Eq`] implémente le trait de marquage [`Eq`] à condition que
  chacun des types des données membres implémente [`Eq`].
- La dérivation de [`PartialOrd`] implémente `PartialOrd<Self>` comme un **ordre
  lexicographique** à condition que chacun des types des données membres
  implémente [`PartialOrd`].
- La dérivation de [`Ord`] implémente [`Ord`] comme un **ordre lexicographique** à
  condition que chacun des types des données membres implémente [`Ord`].

Par exemple, le court extrait de code suivant montre comment comparer deux
valeurs de type `T1` facilement. Toutes les assertions sont vraies.

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct T1 {
    a: u8, b: u8
}

# fn main() {
assert!(&T1 { a: 0, b: 0 } == Box::new(T1 { a: 0, b: 0 }).as_ref());
assert!(T1 { a: 1, b: 0 } > T1 { a: 0, b: 0 });
assert!(T1 { a: 1, b: 1 } > T1 { a: 1, b: 0 });
# println!("tous les tests sont validés.");
# }
```

<div class="warning">

La dérivation des traits de comparaison pour les types composites dépend de
**l'ordre de déclaration des champs** et non de leur nom.

D'abord, cela implique que changer l'ordre des champs modifie l'ordre des
valeurs. Par exemple, en considérant le type suivant :

```rust,noplaypen
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct T2{
   b: u8, a: u8
};
```

on a `T1 {a: 1, b: 0} > T1 {a: 0, b: 1}` mais
`T2 {a: 1, b: 0} < T2 {a: 0, b: 1}`.

Ensuite, si une comparaison sous-jacente provoque un `panic`, l'ordre peut
changer le résultat à cause de l'utilisation d'un opérateur logique 
court-circuitant dans l'implémentation automatique.

Pour les énumérations, les comparaisons dérivées dépendent d'abord de
**l'ordre des variants**, puis de l'ordre des champs.

</div>

En dépit de ces avertissements sur les ordres dérivés, les comparaisons dérivées
automatiquement sont bien moins sujettes à erreurs que des implémentations
manuelles, et rendent le code plus court et plus simple à maintenir.

<div class="reco" id="LANG-CMP-DERIVE" type="Recommandation" title="Dérivation des traits de comparaison lorsque c'est possible">

Dans un développement sécurisé en Rust, l'implémentation des traits de
comparaison standard DEVRAIT être automatiquement dérivée à l'aide de
`#[derive(...)]` lorsque l'égalité structurelle et l'ordre lexicographique
sont nécessaires. Toute implémentation manuelle d'un trait de comparaison
standard DEVRAIT être justifiée et documentée.

</div>

## Trait [`Drop`] : le destructeur

Les types implémentent le trait [`std::ops::Drop`] dans le but d'effectuer
certaines opérations lorsque la mémoire associée à une valeur est réclamée.
[`Drop`] est l'équivalent Rust d'un destructeur en C++ ou un finaliseur en Java.

<div class="note">

Implémenter ce trait modifie la sémantique d'exécution du langage. 

</div>

[`Drop`] agit récursivement, depuis la valeur externe vers les valeurs imbriquées.
Lorsqu'une valeur sort du scope (ou est explicitement relâchée avec
[`mem::drop`]), elle est relâchée en deux étapes. La première étape a lieu
uniquement si le type de la valeur en question implémente le trait [`Drop`] et
consiste en l'appel de la méthode [`mem::drop`]. La seconde étape consiste en la
répétition du processus de *drop* récursivement sur tous les champs que contient
la valeur. Il est à noter que l'implémentation de [`Drop`] est
*responsable uniquement de la valeur extérieure*.

Tout d'abord, l'implémentation de [`Drop`] ne doit pas être systématique. Elle est
nécessaire uniquement lorsque le type requiert un traitement logique à la
destruction. [`Drop`] est typiquement utilisé dans le cas du relâchement des
ressources externes (connexions réseau, fichier, etc.) ou de ressources mémoire
complexes (*smart pointers* comme les `Box` ou les [`Rc`] par exemple). Au
final, il est probable que l'implémentation du trait [`Drop`] contienne des blocs
`unsafe` ainsi que d'autres opérations critiques du point de vue de la sécurité.

<div class="reco" id="LANG-DROP" type="Règle" title="Justification de l'implémentation du trait `Drop`">

Dans un développement sécurisé en Rust, l'implémentation du trait
[`std::ops::Drop`] DOIT être justifiée et documentée.

</div>

Ensuite, le système de types de Rust assure seulement la sûreté mémoire et,
du point de vue du typage, des [`mem::drop`]s peuvent tout à fait être manqués.
Plusieurs situations peuvent mener à manquer des [`mem::drop`]s, comme :

- un cycle de références (par exemple avec [`Rc`] ou [`Arc`]) ;
- un appel explicite à [`mem::forget`] (voir
  paragraphe à propos de [`mem::forget` et des fuites de mémoire](unsafe/memory.md#forget-and-memory-leaks)) ;
- un `panic` dans un [`mem::drop`] ;
- un arrêt du programme (et un `panic` lorsque `abort-on-panic` est activé).

Les [`mem::drop`]s manqués peuvent mener à l'exposition de données sensibles ou bien
encore à l'épuisement de ressources limitées et par là même à des problèmes
d'indisponibilité.

<div class="reco" id="LANG-DROP-NO-PANIC" type="Règle" title="Absence de `panic` dans l'implémentation de `Drop`">

Dans un développement sécurisé en Rust, l'implémentation du trait
[`Drop`] NE DOIT PAS causer de `panic`.

</div>

En plus des `panic`s, les [`mem::drop`]s contenant du code critique doivent être
protégés.

<div class="reco" id="LANG-DROP-NO-CYCLE" type="Règle" title="Absence de cycles de références avec valeurs `Drop`ables">

Les valeurs dont le type implémente [`Drop`] NE DOIVENT PAS être incluses,
directement ou indirectement, dans un cycle de références à compteurs.

</div>

<!-- -->

<div class="reco" id="LANG-DROP-SEC" type="Règle" title="Sécurité assurée par d'autres mécanismes en plus du trait `Drop`">

Certaines opérations liées à la sécurité d'une application à la fin d'un
traitement (comme l'effacement de secrets cryptographiques par exemple) NE DOIVENT PAS
reposer uniquement sur l'implémentation du trait [`Drop`].

</div>

## Références comptées ([`Rc`] et [`Arc`])

Les références comptées permettent de cloner à peu de frais des valeurs de n'importe quel type.
Pour cela, ces valeurs sont associées à un compteur qui dénombre le nombre de clones attachés à cette valeur, et lorsque
le compteur tombe à zéro, la valeur est détruite. 

Ce paradigme peut introduire des fuites mémoires lorsque les références comptées sont utilisées dans des cycles.

<center>

![Cycle de références](static/images/cycle-color.svg)

</center>

Dans cet exemple, les objets `A`, `B` et `C` sont chacun référencés par au moins un autre objet : tant que ces trois objets existent,
ils ne seront pas supprimés. On a donc une fuite mémoire.

La **combinaison** de la mutabilité *[intérieure](https://doc.rust-lang.org/reference/interior-mutability.html)*, des références comptées et des types récursifs peut conduire à des fuites mémoire, et donc éventuellement à des attaques par déni de service et à des fuites de secrets.

L'exemple non-`unsafe` suivant montre, la création d'une fuite mémoire en utilisant la mutabilité intérieure et les références comptées.

```rust align bad
{{#include ../../examples/src/memory.rs:cyclic}}
```

La fuite peut être mise en évidence grâce à `valgrind` :

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
==153637== 48 (24 direct, 24 indirect) bytes in 1 blocks are definitely lost
                in loss record 2 of 2
==153637==    at 0x48417B4: malloc (vg_replace_malloc.c:381)
==153637==    by 0x10F8D4: safe_rust_leak::main
                (in /home/toto/src/safe-rust-leak/target/release/safe-rust-leak)
==153637==    by 0x10F7E2: std::sys::backtrace::__rust_begin_short_backtrace
                (in /home/toto/src/safe-rust-leak/target/release/safe-rust-leak)
==153637==    by 0x10F7D8: std::rt::lang_start::{{closure}}
                (in /home/toto/src/safe-rust-leak/target/release/safe-rust-leak)
==153637==    by 0x12A90F: std::rt::lang_start_internal
                (in /home/toto/src/safe-rust-leak/target/release/safe-rust-leak)
==153637==    by 0x10FA54: main
                (in /home/toto/src/safe-rust-leak/target/release/safe-rust-leak)
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

 Les types récursifs dont la récursion se base sur l'utilisation des références comptées [`Rc`] ou [`Arc`] NE DOIVENT PAS être mutables *intérieurement*.

</div>

[`Rc`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
[`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
[`RefCell`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
[`UnsafeCell`]: https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html
[`PhantomData`]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html
[`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
[`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html
[`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[`PartialEq<Rhs>`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[`PartialOrd`]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[`PartialOrd<Rhs>`]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
[`Ord`]: https://doc.rust-lang.org/std/cmp/trait.Ord.html
[`mem::forget`]: https://doc.rust-lang.org/std/mem/fn.forget.html
[`mem::drop`]: https://doc.rust-lang.org/std/mem/fn.drop.html
[`Drop`]: https://doc.rust-lang.org/std/ops/trait.Drop.html
[`std::ops::Drop`]: https://doc.rust-lang.org/std/ops/trait.Drop.html