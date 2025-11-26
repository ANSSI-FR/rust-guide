# Traitements des entiers

## Dépassement d'entiers

Bien que des vérifications soient effectuées par Rust en ce qui concerne les
potentiels dépassements d'entiers, des précautions doivent être prises lors de
l'exécution d'opérations arithmétiques sur les entiers.

En particulier, il faut noter que le profil de compilation (généralement *dev*, la compilation de débogage par défaut, ou *release*, la compilation optimisée standard) modifie le comportement en cas de dépassement d'entier. En configuration *dev*, un dépassement provoque l'arrêt du programme (`panic`), tandis qu'en configuration *release*, la valeur calculée est silencieusement tronquée au nombre de bits du type numérique, ce qui donne une sémantique d'arithmétique circulaire (*wrap-around*).

Lorsqu'un dépassement est possible, le comportement peut être rendu explicite soit en utilisant des méthodes spécifiques, soit en utilisant des types enveloppants spécifiques.

Les méthodes sont de la forme `<mode>_<op>`, où `<mode>` est `checked`, `overflowing`, `wrapping` ou `saturating`, et `<op>` est `add`, `mul`, `sub`, `shr`, etc. Les sémantiques sont les suivantes :

- `checked_<op>` retourne `None` en cas de dépassement,
- `overflowing_<op>` retourne à la fois un résultat selon l'arithmétique circulaire et un booléen indiquant si un dépassement a eu lieu,
- `wrapping_<op>` retourne toujours le résultat selon l'arithmétique circulaire,
- `saturating_<op>` retourne toujours le résultat saturé.

Les types enveloppants sont `Wrapping<T>` et `Saturating<T>` (de `std::num`), où `T` est un type entier. Le premier fournit une sémantique d'arithmétique circulaire pour toutes les opérations arithmétiques, tandis que le second fournit une sémantique de saturation. Une fois les valeurs enveloppées, toutes les opérations suivantes sont effectuées avec la sémantique correspondante.

```rust
{{#include ../../examples/src/integer.rs}}
```

<div class="reco" id="LANG-ARITH" type="Règle" title="Utilisation des opérations arithmétiques appropriées au regard des potentiels dépassements">

Lorsqu'une opération arithmétique peut produire un dépassement, les opérateurs classiques sur les entiers ne doivent pas être utilisés. Les méthodes spécialisées comme `checked_<op>`, `overflowing_<op>`, `wrapping_<op>`, ou `saturating_<op>`, ou des types enveloppants spécialisés comme `Wrapping` ou `Saturating`, doivent être utilisés pour rendre le comportement explicite et homogène, quel que soit le profil de compilation.

</div>
