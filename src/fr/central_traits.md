# Traits centraux

Implémenter ces traits modifie la sémantique d'exécution du langage.

## Trait `Drop` : le destructeur

Les types implémentent le trait `std::ops::Drop` dans le but d'effectuer
certaines opérations lorsque la mémoire associée à une valeur est réclamée.
`Drop` est l'équivalent Rust d'un destructeur en C++ ou un finaliseur en Java.

`Drop` agit récursivement, depuis la valeur externe vers les valeurs imbriquées.
Lorsqu'une valeur sort du scope (ou est explicitement relâchée avec
`std::mem::drop`), elle est relâchée en deux étapes. La première étape a lieu
uniquement si le type de la valeur en question implémente le trait `Drop` et
consiste en l'appel de la méthode `drop`. La seconde étape consiste en la
répétition de processus de *drop* récursivement sur tous les champs que contient
la valeur. Il est à noter que l'implémentation de `Drop` est
*responsable uniquement de la valeur extérieure*.

Tout d'abord, l'implémentation de `Drop` ne doit pas être systématique. Elle est
nécessaire uniquement lorsque le type requiert un traitement logique à la
destruction. `Drop` est typiquement utilisé dans le cas du relâchement des
ressources externes (connexions réseau, fichier, etc.) ou de ressources mémoire
complexes (*smart pointers* comme les `Box` ou les `Rc` par exemple). Au
final, il est probable que l'implémentation du trait `Drop` contienne des blocs
`unsafe` ainsi que d'autres opérations critiques du point de vue de la sécurité.

<div class="reco" id="LANG-DROP" type="Règle" title="Justification de l'implémentation du trait `Drop`">

Dans un développement sécurisé en Rust, l'implémentation du trait
`std::ops::Drop` doit être justifiée, documentée et examinée par des pairs.

</div>

Ensuite, le système de types de Rust assure seulement la sûreté mémoire et,
du point de vue du typage, des `drop`s peuvent tout à fait être manqués.
Plusieurs situations peuvent mener à manquer des `drop`s, comme :

- un cycle dans la référence (par exemple avec `Rc` ou `Arc`) ;
- un appel explicite à `std::mem::forget` (ou `core::mem::forget`) (voir
  paragraphe à propos de [`forget` et des fuites de mémoire](unsafe/memory.md#forget-and-memory-leaks)) ;
- un `panic` dans un `drop` ;
- un arrêt du programme (et un `panic` lorsque `abort-on-panic` est activé).

Les `drop`s manqués peuvent mener à l'exposition de données sensibles ou bien
encore à l'épuisement de ressources limitées et par là même à des problèmes
d'indisponibilité.

<div class="reco" id="LANG-DROP-NO-PANIC" type="Règle" title="Absence de `panic` dans l'implémentation de `Drop`">

Dans un développement sécurisé en Rust, l'implémentation du trait
`std::ops::Drop` ne doit pas causer de `panic`.

</div>

En plus des `panic`s, les `drop`s contenant du code critique doivent être
protégés.

<div class="reco" id="LANG-DROP-NO-CYCLE" type="Règle" title="Absence de cycles de références avec valeurs `Drop`ables">

Les valeurs dont le type implémente `Drop` ne doivent pas être incluses,
directement ou indirectement, dans un cycle de références à compteurs.

</div>

<!-- -->

<div class="reco" id="LANG-DROP-SEC" type="Règle" title="Sécurité assurée par d'autres mécanismes en plus du trait `Drop`">

Certaines opérations liées à la sécurité d'une application à la fin d'un
traitement (comme l'effacement de secrets cryptographiques par exemple) ne
doivent pas reposer uniquement sur l'implémentation du trait `Drop`.

</div>
