# Bibliothèque standard

## Les traits `Send` et `Sync`

Les traits `Send` et `Sync` (définis dans `std::marker` ou `core::marker`) sont
des marqueurs utilisés pour assurer la sûreté des accès concurrents en Rust.
Lorsqu'ils sont correctement implémentés, ils permettent au compilateur Rust de
garantir l'absence de problèmes d'accès concurrents. Leurs sémantiques sont
définies comme suit :

- Un type est `Send` s’il est sûr d'envoyer (*move*) des valeurs de ce type vers
  un autre fil d'exécution.
- Un type est `Sync` s’il est sûr de partager des valeurs de ce type par une
  référence immutable avec un autre fil d'exécution.

Ces deux traits sont des traits *unsafe*, c'est-à-dire que le compilateur Rust
ne vérifie d'aucune manière que leur implémentation est correcte. Le danger est
réel : une implémentation incorrecte peut mener à un **comportement indéfini**.

Heureusement, dans la plupart des cas, il n'est pas nécessaire de fournir une
implémentation. En Rust, la quasi-totalité des types primitifs implémente
`Send` et `Sync`, et dans la majorité des cas, Rust fournit de manière
automatique une implémentation pour les types composés. Quelques exceptions
notables sont :

- les pointeurs `raw`, qui n'implémentent ni `Send`, ni `Sync`, puisqu'ils
  n'offrent aucune garantie quant à la sûreté ;
- les références `UnsafeCell`, qui n'implémentent pas `Sync` (et par extensions,
  les références `Cell` et `RefCell` non plus), puisqu'elles autorisent la
  mutabilité des valeurs contenues (*interior mutability*) ;
- les références `Rc`, qui n'implémentent ni `Send`, ni `Sync`, puisque les
  compteurs de références seraient partagés de manière désynchronisée.

L'implémentation automatique de `Send` (respectivement `Sync`) a lieu pour les
types composés (structures ou énumérations) lorsque tous les champs contenus
implémentent `Send` (respectivement `Sync`). Une fonctionnalité notable, mais
**instable**, de Rust (depuis 1.37.0) permet d'empêcher cette implémentation
automatique en annotant explicitement le type considéré avec une
_négation d'implementation_ :

```rust,ignore,noplaypen
#![feature(option_builtin_traits)]

struct SpecialType(u8);
impl !Send for SpecialType {}
impl !Sync for SpecialType {}
```
L'implémentation négative de `Send` ou `Sync` est également utilisée dans la
bibliothèque Rust pour les exceptions, et est automatiquement implémentée
lorsque cela est approprié. En résultat, la documentation générée est toujours
explicite : un type implémente soit `Send` (respectivement `Sync`), soit
`!Send` (respectivement `!Sync`).

En guise d'alternative *stable* à l'implémentation négative, il est possible
d'utiliser un champ typé par un type fantôme (`PhantomData`) :

```rust,noplaypen
# use std::marker::PhantomData;
#
struct SpecialType(u8, PhantomData<*const ()>);
```

<div class="reco" id="LANG-SYNC-TRAITS" type="Règle" title="Justification de l'implémentation des traits `Send` et `Sync`">

Dans un développement sécurisé en Rust, l'implémentation manuelle des traits
`Send` et `Sync` devrait être évitée, et, si nécessaire, doit être justifiée
et documentée.

</div>

## Les traits de comparaison : `PartialEq`, `Eq`, `PartialOrd`, `Ord`

Les comparaisons (`==`, `!=`, `<`, `<=`, `>`, `>=`) en Rust reposent sur quatre
traits de la bibliothèque standard disponibles dans `std::cmp` (ou `core::cmp`
pour une compilation avec `no_std`) :

- `PartialEq<Rhs>` qui définit la relation d'équivalence partielle entre objets
  de types `Self` et `Rhs` ;
- `PartialOrd<Rhs>` qui définit la relation d'ordre partiel entre les objets de
  types `Self` et `Rhs` ;
- `Eq` qui définit la relation d'équivalence totale entre les objets du même
  type. Il s'agit d'un trait de marquage qui requiert le trait
  `PartialEq<Self>` ;
- `Ord` qui définit la relation d'ordre total entre les objets du même type.
  Le trait `PartialOrd<Self>` est alors requis.

Comme stipulé dans la documentation de la bibliothèque standard, Rust présuppose
**de nombreux invariants** lors de l'implémentation de ces traits :

- Pour `PartialEq` :

  - *Cohérence interne* : `a.ne(b)` est équivalent à `!a.eq(b)`, c.-à-d., `ne`
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

- Pour `Eq` :

  - `PartialEq<Self>` est implémenté.

  - *Réflexivité* : `a.eq(a)`. Cela signifie que `PartialEq<Self>` est
    implémenté (`Eq` ne fournit aucune méthode).

- Pour `PartialOrd` :

  - *Consistance de la relation d'égalité* : `a.eq(b)` est équivalent à
    `a.partial_cmp(b) == Some(std::ordering::Eq)`.

  - *Consistence interne* :

    - `a.lt(b)` ssi `a.partial_cmp(b) == Some(std::ordering::Less)`.
    - `a.gt(b)` ssi `a.partial_cmp(b) == Some(std::ordering::Greater)`.
    - `a.le(b)` ssi `a.lt(b) || a.eq(b)`.
    - `a.ge(b)` ssi `a.gt(b) || a.eq(b)`.

    Il faut noter qu'en définissant seulement `partial_cmp`, la consistance
    interne est garantie par les implémentations par défaut de `lt`, `le`, `gt`,
    and `ge`.

  - *Antisymétrie* : `a.lt(b)` (respectivement `a.gt(b)`) implique `b.gt(a)`
    (respectivement `b.lt(b)`). Du point de vue du développeur, cela signifie
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

- Pour `Ord` :

  - `PartialOrd<Self>`

  - *Totalité* : `a.partial_cmp(b) != None` est toujours vrai. En d'autres mots,
    exactement une assertion parmi `a.eq(b)`, `a.lt(b)` et `a.gt(b)` est vraie.

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
comparaison standards doit respecter les invariants décrits dans la
documentation.

</div>

<div class="reco" id="LANG-CMP-DEFAULTS" type="Recommandation" title="Utilisation des implémentations par défaut des traits de comparaison standards">

Dans un développement sécurisé en Rust, l'implémentation des traits de
comparaison standard ne devrait être effectuée que par l'implémentation des
méthodes ne fournissant pas d'implémentation par défaut, dans le but de
réduire le risque de violation des invariants associés auxdits traits.

</div>

Il existe un *lint* Clippy qui permet de vérifier que `PartialEq::ne` n'est pas
défini lors d'une implémentation du trait `PartialEq`.

Rust propose une façon de fournir automatiquement des implémentations par défaut
pour les traits de comparaison, au travers de l'attribut `#[derive(...)]` :

- La dérivation de `PartialEq` implémente `PartialEq<Self>` avec une **égalité
  structurelle** à condition que chacun des types des données membres implémente
  `PartialEq<Self>`.
- La dérivation de `Eq` implémente le trait de marquage `Eq` à condition que
  chacun des types des données membres implémente `Eq`.
- La dérivation de `PartialOrd` implémente `PartialOrd<Self>` comme un **ordre
  lexicographique** à condition que chacun des types des données membres
  implémente `PartialOrd`.
- La dérivation de `Ord` implémente `Ord` comme un **ordre lexicographique** à
  condition que chacun des types des données membres implémente `Ord`.

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
changer le résultat à cause de l'utilisation d'un opérateur logique court-
circuitant dans l'implémentation automatique.

Pour les énumérations, les comparaisons dérivées dépendent d'abord de
**l'ordre des variants**, puis de l'ordre des champs.

</div>

En dépit de ces avertissements sur les ordres dérivés, les comparaisons dérivées
automatiquement sont bien moins sujettes à erreurs que des implémentations
manuelles, et rendent le code plus court et plus simple à maintenir.

<div class="reco" id="LANG-CMP-DERIVE" type="Recommandation" title="Dérivation des traits de comparaison lorsque c'est possible">

Dans un développement sécurisé en Rust, l'implémentation des traits de
comparaison standard devrait être automatiquement dérivée à l'aide de
`#[derive(...)]` lorsque l'égalité structurelle et l'ordre lexicographique
sont nécessaires. Toute implémentation manuelle d'un trait de comparaison
standard devrait être justifiée et documentée.

</div>
