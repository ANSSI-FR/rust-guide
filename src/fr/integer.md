# Traitements des entiers

## Dépassement d'entiers

Bien que des vérifications soient effectuées par Rust en ce qui concerne les
potentiels dépassements d'entiers, des précautions doivent être prises lors de
l'exécution d'opérations arithmétiques sur les entiers.

En particulier, il doit être noté que les profils de compilation *debug* et
*release* produisent des variations de comportements quant à la gestion des
dépassements d'entiers. Dans la configuration *debug*, un dépassement provoque
la terminaison du programme (`panic`), tandis que dans la configuration
*release* la valeur calculée est silencieusement tronquée en fonction de la
valeur maximum qui peut être stockée pour le type considéré.

Ce comportement peut être rendu explicite en utilisant le type générique
`Wrapping`, ou les opérations sur les entiers `overflowing_<op>` et
`wrapping_<op>` (la partie `<op>` étant remplacée par le type de calcul :
`add`, `mul`, `sub`, `shr`, etc.).

```rust
{{#include ../../examples/src/integer.rs}}
```

<div class="reco" id="LANG-ARITH" type="Règle" title="Utilisation des opérations arithmétiques appropriées au regard des potentiels dépassements">

Lorsqu'une opération arithmétique peut produire un dépassement d'entier, les
fonctions spécialisées `overflowing_<op>`, `wrapping_<op>` ou le type
`Wrapping` doivent être utilisés.

</div>
