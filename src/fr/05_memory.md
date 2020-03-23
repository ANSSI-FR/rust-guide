# Gestion de la mémoire

<!-- ## À propos de la sûreté mémoire en Rust -->

<!--
<mark>TODO</mark> : expliquer les allocations/désallocations sûres,
l'ownership/borrowing, et identifier les constructions de langage qui peuvent
casser la sûreté mémoire (par exemple, comportements *unsounds* dans des
versions plus anciennes du compilateur).
-->

## `Forget` et fuites de mémoire

En général, la mémoire est automatiquement récupérée en Rust lorsqu'une variable
sort de la portée lexicale courante. En complément de ce mécanisme, Rust fournit
des fonctions spéciales pour réclamer manuellement la mémoire : les fonctions
`forget` et `drop` du module `std::mem` (ou `core::mem`). `drop` déclenche
simplement une récupération prématurée de la mémoire tout en appelant les
destructeurs associés lorsque nécessaire, `forget` quant à elle n'appelle pas
ces destructeurs.

```rust
let pair = ('↑', 0xBADD_CAFEu32);
drop(pair); // ici, `forget` serait équivalent (pas de destructeur à appeler)
```

Les deux fonctions sont considérées comme **sûres du point de vue mémoire** par
Rust. Toutefois, `forget` rendra toute ressource gérée par la valeur libérée
inaccessible, mais non libérée.

```rust
# use std::mem::forget;
let s = String::from("Hello");
forget(s); // fuite mémoire
```

En particulier, l'utilisation de `forget` peut causer la rétention en mémoire de
ressources critiques, menant à des interblocages et à la persistance de données
sensibles en mémoire. C'est pourquoi `forget` doit être considérée comme
**non sécurisée**.

> ### Règle {{#check MEM-FORGET | Non-utilisation de `forget`}}
>
> Dans un développement sécurisé en Rust, la fonction `forget` de `std::mem`
> (`core::mem`) ne doit pas être utilisée.

<!-- -->

> ### Recommandation {{#check MEM-FORGET-LINT | Utilisation du *lint* clippy pour détecter l'utilisation de `forget`}}
>
> Le *lint* `mem_forget` de Clippy peut être utilisé pour automatiquement
> détecter toute utilisation de la fonction `forget`. Pour s'assurer de l'absence
> d'appel à `forget`, ajouter la directive suivante en début de fichier racine
> (en général `src/lib.rs` ou `src/main.rs`) :
>
> ```rust,noplaypen,ignore
> #![deny(clippy::mem_forget)]
> ```

La bibliothèque standard inclut d'autres moyens d'*oublier* une valeur :

- `Box::leak` pour libérer une ressource ;
- `Box::into_raw` pour exploiter une valeur dans un bloc *unsafe*, notamment
  dans une FFI ;
- `ManuallyDrop` (dans `std::mem` ou `core::mem`) pour assurer la libération
  manuelle d'une valeur.

Ces alternatives peuvent mener au même type de problème de sécurité, mais ont
l'avantage de faire apparaître explicitement leur but.

> ### Règle {{#check MEM-LEAK | Absence de fuite mémoire}}
>
> Dans un développement sécurisé en Rust, le code ne doit pas faire fuiter de la
> mémoire ou des ressources *via* `Box::leak`.

`ManuallyDrop` et `Box::into_raw` passent la responsabilité de la libération de
la ressource concernée du compilateur au développeur.

> ### Règle {{#check MEM-MANUALLYDROP | Libération des valeurs *wrappées* dans `ManuallyDrop`}}
>
> Dans un développement sécurisé en Rust, toute valeur *wrappée* dans le type
> `ManuallyDrop` doit être *unwrapped* pour permettre sa libération automatique
> (`ManuallyDrop::into_inner`) ou bien doit être manuellement libérée (*unsafe*
> `ManuallyDrop::drop`).

<!-- -->

> ### Règle {{#check MEM-INTOFROMRAW | Appel systématique à `from_raw` pour les valeurs créées avec `into_raw`}}
>
> Dans un développement sécurisé en Rust, tout pointeur créé par un appel à
> `into_raw` (ou `into_raw_nonnull`) depuis un des types suivants doit
> finalement être transformé en valeur avec l'appel à la fonction `from_raw`
> correspondant, pour permettre sa libération :
> 
> - `std::boxed::Box` (ou `alloc::boxed::Box`) ;
> - `std::rc::Rc` (ou `alloc::rc::Rc`) ;
> - `std::rc::Weak` (ou `alloc::rc::Weak`) ;
> - `std::sync::Arc` (ou `alloc::sync::Arc`) ;
> - `std::sync::Weak` (ou `alloc::sync::Weak`) ;
> - `std::ffi::CString` ;
> - `std::ffi::OsString`.
>
> ```rust
> let boxed = Box::new(String::from("Crab"));
> let raw_ptr = unsafe { Box::into_raw(boxed) };
> let _ = unsafe { Box::from_raw(raw_ptr) }; // sera libéré
> ```

<!-- -->

> ### Note
>
> Dans le cas de `Box::into_raw`, le nettoyage automatique est possible, mais
> est bien plus compliqué que de *re-boxer* le pointeur brut et doit être
> évité :
>
> ```rust
> // extrait de la documentation de la bibliothèque standard
> use std::alloc::{dealloc, Layout};
> use std::ptr;
>
> let x = Box::new(String::from("Hello"));
> let p = Box::into_raw(x);
> unsafe {
>     ptr::drop_in_place(p);
>     dealloc(p as *mut u8, Layout::new::<String>());
> }
> ```
>
> Puisque les autres types (`Rc` et `Arc`) sont opaques et plus complexes, la
> libération manuelle n'est pas possible.

## Mémoire non initialisée

Par défaut, le langage Rust impose que toutes les valeurs soient initialisées, pour
prévenir l'utilisation de mémoire non initialisée (à l'exception de
l'utilisation de `std::mem::uninitialized` ou de `std::mem::MaybeUninit`).

> ### Règle {{#check MEM-UNINIT | Pas de mémoire non initialisée}}
>
> La fonction `std::mem::uninitialized` (dépréciée depuis la version 1.38) ou
> le type `std::mem::MaybeUninit` (stabilisé dans la version 1.36) ne doivent
> pas être utilisés, ou bien explicitement justifiés si nécessaire.

L'utilisation de mémoire non initialisée peut induire deux problèmes de
sécurité distincts :

- la libération de mémoire non initialisée (étant également un problème de
  sûreté mémoire) ;
- la non-libération de mémoire initialisée.

> ### Note
>
> Le type `std::mem::MaybeUninit` est une amélioration de la fonction
> `std::mem::uninitialized`. En effet, il rend la libération des valeurs non
> initialisées bien plus difficile. Toutefois, cela ne change pas le second
> problème : la non-libération de la mémoire initialisée est bien possible.
> C'est problématique en particulier si l'on considère l'utilisation de `Drop`
> pour effacer des valeurs sensibles.

## Effacement sécurisé des informations sensibles

L'effacement sécurisé (mise à zéro) est nécessaire pour les variables sensibles,
en particulier dans lorsque le code Rust est utilisé *via* des FFI.

> ### Règle {{#check MEM-ZERO | Mise à zéro des données sensibles après utilisation}}
>
> Les variables contenant des données sensibles doivent être mises à zéro après
> utilisation, en utilisant des fonctions dont les appels ne seront pas
> supprimés par les optimisations du compilateur, comme
> `std::ptr::write_volatile` ou bien la *crate* `zeroize`.

Le code suivant montre comment définir un type entier qui sera remis à zéro
à sa libération, en utilisant le trait `Drop` :

```rust
/// Exemple : newtype pour u32, réécrit à 0 quand libéré
pub struct ZU32(pub u32);

impl Drop for ZU32 {
    fn drop(&mut self) {
        println!("zeroing memory");
        unsafe{ ::std::ptr::write_volatile(&mut self.0, 0) };
    }
}

# fn main() {
{
    let i = ZU32(42);
    // ...
} // i est libéré ici
# }
```
