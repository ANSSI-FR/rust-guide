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

<div class="definition">

Le comportement d'un programme est *indéfini* (*UB* pour [*Undefined Behavior*](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)) lorsque sa sémantique n'est 
pas décrite dans le langage Rust.

</div>

L'existence d'*UB* est considéré comme une 
[erreur de programmation](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.general) et doit être évitée.

<div class="example">

Le déréférencement d'un pointeur null est un *UB*.
*A contrario*, un `unwrap` sur l'objet `None` est bien *défini* car c'est le langage qui traite cette erreur
(en lançant un `panic`).

</div>

Une liste d'erreurs de programmation conduisant à un *UB* est [donnée](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
dans la [référence Rust @rust-reference]. Parmi elles, on notera les erreurs suivantes :

* Pas de déréférencement de pointeur vers une adresse mémoire non allouée (*dangling pointer*) ou non alignée, ce qui implique
  * Pas de dépassement de tableau
  * Pas d'accès à de la mémoire libérée
  * Accès toujours aligné quelque soit la plateforme
* Les valeurs pointées sont [cohérentes](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.invalid) avec le type du pointeur. Par exemple, une valeur pointée par un pointeur booléen sera l'octet 1 ou 0.
* Respect des règles d'[*aliasing*](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.alias) (voir aussi le [Rustonomicon @nomicon] pour des [exemples](https://doc.rust-lang.org/nomicon/aliasing.html)): une référence mutable ne peux être partagée.
* Pas d'[accès concurrent](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.race) (deux accès simultanés non atomiques, l'un en écriture et l'autre en écriture ou en lecture) à la même adresse mémoire (voir aussi le [Rustonomicon @nomicon] pour des [exemples](https://doc.rust-lang.org/nomicon/races.html))


## Garantie de Rust

<div class="important">

Le langage Rust est conçu dans le but de garantir l'absence d'*UB* dans un programme n'utilisant pas de fonctionnalités *unsafe*.

</div>

<div class="note">

On notera que cette garantie qu'offre le langage Rust ***ne protège pas*** contre les erreurs suivantes :

* fuites de resources (mémoire, IO, ...) (voir la section sur la [gestion mémoire](unsafe/memory.md#chapter-memory)) ;
* dépassements numériques (voir la section sur le traitement des [entiers](integer.md#chapter-integer)).

</div>
