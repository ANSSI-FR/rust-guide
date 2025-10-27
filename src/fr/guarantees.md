---
references:
  - type: web
    title: The Rust Reference
    url: https://doc.rust-lang.org/stable/reference/
    id: rust-reference
  - type: web
    title: The Rustonomicon
    url: https://doc.rust-lang.org/stable/nomicon/
    id: nomicon
---

# Garanties du langage

## Comportements indéfinis

> Le comportement d'un programme est *indéfini* (*UB* pour *Undefined Behavior*) lorsque sa sémantique n'est 
> pas décrite dans le langage Rust.

Selon [@rust-reference], l'existence d'*UB* est considéré comme une [erreur](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.general).

Par exemple le déréférencement d'un pointeur null est un *UB*.
*A contrario*, un `unwrap` sur l'objet `None` est bien *défini* car c'est le langage qui traite cette erreur
(en lançant un `panic`).

La liste actuelle des *UB* est donnée [ici](https://doc.rust-lang.org/reference/behavior-considered-undefined.html).
On notera les garanties suivantes :

* Pas de déréférencement de pointeur vers une adresse mémoire non allouée (*dangling pointer*) ou non alignée, ce qui implique
  * Pas de dépassement de tableau
  * Pas d'accès à de la mémoire libérée
  * Accès toujours aligné quelque soit la plateforme
* Les valeurs pointées sont [cohérentes](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.invalid) avec le type du pointeur. Par exemple, une valeur pointée par un pointeur booléen sera l'octet 1 ou 0.
* Respect des règles d'[*aliasing*](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.alias) (voir aussi le [@nomicon] pour des [exemples](https://doc.rust-lang.org/nomicon/aliasing.html)): une référence mutable ne peux être partagée.
* Pas d'[accès concurrent]((https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.race)) (un accès en lecture et un autre en écriture ou en lecture) à la même adresse mémoire (voir aussi le [@nomicon] pour des [exemples](https://doc.rust-lang.org/nomicon/races.html))

## Garantie de Rust

> La volonté du langage est d'assurer l'absence d'*UB* dans un programme utilisant uniquement la partie non *unsafe* de Rust.

Cependant, le langage ***ne protège pas*** contre les erreurs suivantes :

* fuites de resources (mémoire, IO, ...) ;
* dépassements numériques.

## Garantie de niveau LLVM

LLVM classe ses cibles prises en charge en différents niveaux afin d’indiquer le degré de stabilité et de tests auquel chaque moteur de génération (backend) bénéficie.

### Tier 1 - Fonctionnement garanti

La cible est entièrement examinée par la communauté LLVM. Elle réussit l’ensemble complet de la batterie de tests LLVM, fait l’objet de tests de régression réguliers et est maintenue à jour avec les nouvelles versions de LLVM. En pratique, vous pouvez compter sur une génération de code cohérente, une ABI stable et des performances prévisibles d’une version d’LLVM à l’autre.

### Tier 2 - Compilation garantie

La cible se compile correctement avec LLVM, mais elle ne bénéficie pas du même niveau de tests ni de la même maintenance que les cibles de niveau 1. Elle peut ne pas être entièrement couverte par les tests, et certaines optimisations ou fonctionnalités récentes de LLVM peuvent être absentes ou instables. Les utilisateurs peuvent tout de même générer du code pour ces backends, mais ils doivent s’attendre à d’éventuels problèmes occasionnels ou à devoir appliquer des correctifs manuels.

### Tier 3

Les cibles de niveau 3 ne sont tout simplement pas prises en charge officiellement.



La distinction entre les niveaux aide les développeurs à choisir une cible adaptée à leur tolérance au risque : le niveau 1 pour des applications de production, le niveau 2 pour des architectures expérimentales ou de niche dont le support complet n’est pas encore assuré.

## Relation entre rustc et LLVM

Le compilateur Rust (rustc) utilise LLVM comme moteur principal de génération de code. Après que le front‑end ait effectué le *borrow checking*, l’inférence de types et les optimisations MIR (intermédiaire de niveau moyen), il transforme le programme en représentation intermédiaire (IR) de LLVM. LLVM se charge ensuite du travail lourd : sélection des instructions, allocation des registres et optimisations spécifiques à la cible, avant d’émettre le code machine.

Parce que rustc délègue la génération de code à LLVM, la qualité des binaires produits dépend fortement du niveau de prise en charge (tier/niveau) de la cible choisie.
Lorsque vous compilez pour une cible de niveau 1 (par exemple : x86‑64, ARMv8), vous bénéficiez des vastes batteries de tests et des pipelines d’optimisation d’LLVM.
Cibler une architecture de niveau 2 fonctionne, mais il est garanti qu’elle se *compilera*, pas qu’elle *fonctionnera* correctement.

Par conséquent:

<div class="reco" id="LANG-ERRWRAP" type="Rule" title="Les cibles de niveau 2 ne doivent jamais être utilisées dans des systèmes critiques">
Les cibles de niveau 2 ne doivent jamais être utilisées dans des systèmes critiques pour la sureté.
</div>

Une liste complète des cibles prises en charge est disponible dans la documentation officielle:

[Plateform support]: https://doc.rust-lang.org/stable/rustc/platform-support.html

