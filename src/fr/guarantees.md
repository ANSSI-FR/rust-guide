# Garanties du langage

## Comportement indéfini (*UB*)

> Le comportement d'un programme est *indéfini* (*UB* pour *Undefined Behavior*) lorsque sa sémantique n'est
> pas décrite par le langage Rust.

L'existence d'un *UB* est considérée comme une [erreur](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.general).

Par exemple, le déréférencement d'un pointeur nul est un *UB*. *A contrario*, un
`unwrap` sur la valeur `None` est bien défini car c'est le langage qui traite
cette erreur (le plus souvent en lançant un `panic`).

La liste actuelle des *UB* est donnée dans la [référence du langage](https://doc.rust-lang.org/reference/behavior-considered-undefined.html).
On notera les garanties suivantes :

* Pas de déréférencement de pointeur vers une adresse mémoire non allouée
  (*dangling pointer*) ou non alignée, ce qui implique :
  * Pas de dépassement de tableau
  * Pas d'accès à de la mémoire libérée
  * Accès toujours aligné, quelle que soit la plateforme
* [Cohérence](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.invalid)
  entre les valeurs pointées et le type du pointeur. Par exemple, une valeur
  pointée par un pointeur booléen sera l'octet 1 ou 0.
* Respect des règles
  d'[*aliasing*](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.alias)
  (voir aussi le [nomicon](https://doc.rust-lang.org/nomicon/aliasing.html)) :
  une référence mutable ne peut pas être partagée.
* Pas d'accès concurrent (lecture ou écriture en cas d'écriture) à la même adresse mémoire
  ([*data race*](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.race),
  voir aussi le [nomicon](https://doc.rust-lang.org/nomicon/races.html))

## Garanties de Rust

> La volonté du langage est d'assurer l'absence d'*UB* dans un programme utilisant uniquement la partie non *unsafe* de Rust.

Cependant, le langage **ne protège pas** contre les erreurs suivantes :

* fuites de ressources (mémoire, IO, ...) ;
* dépassements numériques.

## Références

* <https://doc.rust-lang.org/reference/unsafety.html>
* <https://doc.rust-lang.org/nomicon/what-unsafe-does.html>
<!-- * https://github.com/ANSSI-FR/rust-guide/pull/3 -->
