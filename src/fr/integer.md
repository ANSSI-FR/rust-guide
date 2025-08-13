# Traitements des entiers

## Dépassement d'entiers

Bien que des vérifications soient effectuées par Rust en ce qui concerne les
potentiels dépassements d'entiers, des précautions doivent être prises lors de
l'exécution d'opérations arithmétiques sur les entiers.

En particulier, il doit être noté que les profils de compilation (typiquement,
*dev*, la compilation de debug par défaut, ou *release*, la compilation
optimisée standard) produisent des variations de comportements quant à la
gestion des dépassements d'entiers. Dans la configuration *dev*, un dépassement
provoque la terminaison du programme (`panic`), tandis que dans la configuration
*release*, la valeur calculée est silencieusement tronquée au nombre de bits du
type numérique, lui conférant cette sémantique d'arithmétique modulaire (*wrap
around*).

Ce comportement peut être rendu explicite en utilisant des méthodes spécifiques
`<mode>_<op>`, où `<op>` peut être `add`, `mul`, `sub`, `shr`, etc. :

- `checked_<op>` retourne `None` en cas de dépassement,
- `overflowing_<op>` retourne à la fois le résultat éventuellement tronqué et un
  booléen indiquant si un dépassement a eu lieu,
- `wrapping_<op>` retourne toujours le résultat tronqué,
- `saturating_<op>` retourne toujours le résultat saturé.

Pour les deux derniers choix, il est possible d'utiliser les types génériques
`Wrapping` et `Saturating` (de `std::num`) pour obtenir le même comportement de
manière plus concise. En effet, une fois les valeurs encapsulées, toutes les
opérations suivantes sont effectuées avec la sémantique choisie.

```rust
use std::num::{Saturating, Wrapping};
# use std::panic;

# fn main() {
let x: u8 = 242;

# let result = panic::catch_unwind(|| {
println!("{}", x + 50);     // panique en dev, affiche 36 en release.
# });
# if result.is_err() { println!("panic"); }
println!("{:?}", x.checked_add(50));            // affiche toujours None.
println!("{}", x.overflowing_add(50).0);        // affiche toujours 36.
println!("{}", x.wrapping_add(50));             // affiche toujours 36.
println!("{}", x.saturating_add(50));           // affiche toujours 255.
println!("{}", Wrapping(x) + Wrapping(50));     // affiche toujours 36.
println!("{}", Saturating(x) + Saturating(50)); // affiche toujours 255.

// panique toujours :
let (res, c) = x.overflowing_add(50);
# let result = panic::catch_unwind(|| {
if c { panic!("custom error"); }
else { println!("{}", res); }
# });
# if result.is_err() { println!("panic"); }
# }
```

> **Règle {{#check LANG-ARITH | Utilisation des opérations arithmétiques appropriées au regard des potentiels dépassements}}**
>
> Lorsqu'une opération arithmétique peut produire un dépassement d'entier, il
> faut utiliser les méthodes spécialisées `checked_<op>`, `overflowing_<op>`,
> `wrapping_<op>`, ou `saturating_<op>`, ou les types génériques spécialisés
> `Wrapping` ou `Saturating`.
