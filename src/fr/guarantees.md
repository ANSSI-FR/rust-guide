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

<div class="reco" id="UB-NOUB" type="Rule" title="No Undefined Behavior">

No Undefined Behavior is allowed.

</div>
