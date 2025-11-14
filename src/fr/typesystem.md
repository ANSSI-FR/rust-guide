# Système de types

## Développement dirigé par les types

Le système de type d'un langage est un outil précieux permettant d'exprimer plus finement les intentions du code au compilateur, lui permettant d'effectuer

* des optimisation lors de la compilation
* des vérifications d'invariants

C'est ce deuxième point qui nous intéresse ici. En plus des vérifications de sûreté déjà détaillées dans les autres parties de ce guide, on va brosser rapidement dans cette partie les invariants qu'il est possible d'encoder dans le système de type de Rust et qui permettent d'éviter des erreurs lors de l'exécution.

### Énumérations

L'utilisation du motif *wildcard* (de la forme `_ => {...}`) lors d'un filtrage par motif (`match`) empêche le compilateur d'avertir le développeur lorsque celui-ci ajoute un cas dans l'énumération, ce qui n'aide pas à répercuter correctement l'ajout de ce cas dans toute la base du code.

<div id="TYPE-NO-CATCHALL" title="Énumération explicite" type="Recommandation">

Lors d'une énumération de cas *via* un filtrage par motifs, il est recommandé de ne pas utiliser le motif *wildcard*.

</div>

La construction `if let` implique la même faiblesse que le motif *wildcard* d'un `match`. Il ne devrait donc n'être utilisé que pour marquer une exception dans le traitements des cas de l'énumération.

### Préciser des invariants sémantiques dans le système de type

#### Un type par usage

<div id="TYPE-NEWTYPE" title="Limitation de la surface d'usage d'un même type" type="Recommandation">

Un type ne devrait recouvrir qu'une seule unité sémantique.

</div>

Dans la réalisation de cette recommendation, si un type `T` recouvre différentes notions, on pourra utiliser le motif *newtype* qui consiste à cacher `T` derrière des types facades `T1`, `T2`, ..., `Tn` :

```rust
struct T1(T);
struct T2(T);
...
struct Tn(T);
```

Ces nouveaux types sont considérés différents dans système de types, mais sont compilés de manière identique à `T`.

La recommandation précédente trouve un usage important dans la recommandation suivante.

<div id="TYPE-NEWTYPE-ARGS" title="Utilisation d'un type propre à chaque argument" type="Recommandation">

Lors de la définition d'une fonction, chacun de ses arguments devrait avoir un type différent afin de ne pas risquer d'en intervertir deux.

</div>

Par exemple, la définition


```rust align
{{#include ../../examples/src/typesystem.rs:add_user_bad}}
```

devrait être remplacée par 

```rust align
{{#include ../../examples/src/typesystem.rs:add_user_good}}
```

#### Utiliser les *lifetimes*

Si elles sont utilisées pour assurer la sûreté mémoire des programmes *safe*, les *lifetimes* peuvent aussi être utilisées pour protéger des invariants métiers, comme par exemple :

* l'unique usage d'une clef éphémère
* l'accès en lecture seule à une resource
* l'enchaînement d'états dans un protocole réseau
* les anneaux de protection d'un système

Des exemples *jouets* de ces implémentations sont données dans le [dépôt](https://github.com/ANSSI-FR/rust-guide) de ce guide.

<!-- ## À propos du système de types de Rust -->

<!--
<mark>TODO</mark> : identifier les pièges du système de types (par exemple,
des confusions à propos du code qui est vraiment exécuté à la suite d'une
résolution de contraintes de traits complexes).
-->
