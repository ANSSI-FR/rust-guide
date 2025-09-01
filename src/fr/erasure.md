# Effacement sécurisé

L'effacement sécurisé (mise à zéro) est nécessaire pour les variables sensibles,
en particulier dans lorsque le code Rust est utilisé *via* des FFI.

> **Règle {{#check MEM-ZERO | Mise à zéro des données sensibles après utilisation}}**
>
> Les variables contenant des données sensibles doivent être mises à zéro après
> utilisation.

<!-- justification ? -->

<div class=warning>

Les opérations d'effacement de la mémoire son compliquées à réaliser en général, et 
avec Rust en particulier. En effet, comme en C, la sécurité de la mémoire n'est pas
un comportement *observable*, c'est à dire dont le compilateur se porte garant.

Du fait des nombreuses optimisations[^note] du compilateur,
les recommendations suivantes ne garantissent donc pas l'effacement complet des secrets
après leur utilisation, mais sont données en *best-effort*.

</div>

[^note]: ... et éventuellement de celles futures !



## Forcer l'effacement

Une première difficulté naît des optimisations du compilateur : en général, il considère inutiles des opérations 
d'écriture en mémoire sans lecture subséquente et peut les supprimer.

Par exemple, dans le code suivant, l'écriture en mémoire suivante n'est pas effectuée

```rust
pub fn main() {
    let mut n: u64 = 0x4953534e41;
    println!("{}", n);
    n = 0; // optimized away!
}
```

Il existe dans la bibliothèque standard Rust des fonctions `unsafe` forçant l'écriture en mémoire malgré les optimisations
du compilateur. Par exemple, la fonction `std::ptr::write_volatile` ne sera jamais supprimée
par le compilateur. Par exemple, la fonction suivante appliquera toujours l'effacement de la mémoire d'un entier.

```rust
fn erase(mut n: u8) {
    println!("zeroing memory");
    unsafe { ::std::ptr::write_volatile(&mut n, 0) };
}
```

Cependant, cette fonction étant `unsafe`, on lui préférera si possible la crate `zeroize`.

> **Règle {{#check MEM-ZERO-EFFECTIVE | Effectivité de l'effacement de la mémoire}}**
>
> Les développements concernant la mise à zéro de variables sensibles
> s'assureront de l'exécution effective de l'opération.

 <!-- ## Éviter les duplications de secrets

Plus le nombre de secrets est grand plus il est difficile de s'assurer de leur non divulgation. 
Et même en supposant que leur effacement est automatisé (en utilisant `Drop`, ce qui,
on le verra dans la suite, ne garantie par un effacement systématique), la multiplication
des emplacements augmente le risque de divulgation -->

<!-- je n'en suis pas si sûr finalement -->


## Déplacements de valeurs sensibles

La [sémantique par déplacement](semantics.md#déplacement-des-valeurs) de Rust induit une contrainte
supplémentaire dans le contrôle des variables sensibles. Le déplacement est une simple copie
bit à bit d'un emplacement à un autre, sans action paramétrable par l'utilisateur (excluant par exemple
d'éventuel recours à `drop`).

De plus, le déplacement étant un détail d'implémentation du compilateur il est difficile de
savoir à quel moment il interviendra.

> **Règle {{#check MEM-ZERO-NOMOVE | Absence de déplacements de valeurs sensibles}}**
>
> Les valeurs sensibles ne devront pas être déplacées après leur création.

Il s'en suit la recommendation suivante découlant directement de la précédente

> **Règle {{#check MEM-ZERO-BYREF | Absence de transfert de propriété d'une valeur sensible}}**
>
> La propriété d'un secret n'est pas transferable. Une fois le secret créé (même vide), les seules façons
> permettant d'accéder à un secret sont
> 
> * la copie (trait `Copy`) ou le clonage (trait `Clone`) uniquement si le type du secret implémente `Drop` dans laquelle le secret est effacé,
> * la référence (`&T` ou `&mut T`)

<div class="warning">

Il est à noter que ces précautions ne garantissent pas le bonne effacement d'un secret : en effet,
il est possible dans certaines situations que le compilateur ajoute des optimisations cassant les invariants
mis en place pour empêcher le déplacement d'un secret.

Par exemple, il peut choisir de transformer un passage par référence en passage par valeur, ce
qui met à mal la précaution [MEM-ZERO-BYREF](#MEM-ZERO-BYREF).

</div>

<!-- ### Fixer les valeurs sensibles en mémoire

Afin de se prémunir d'un déplacement involontaire, on utilisera le système de types pour vérifier à la
compilation qu'une valeur est *fixée* en mémoire.

> **Règle {{#check MEM-ZERO-NOMOVE-TYPED | Épinglage des valeurs sensibles}}**
>
> Les fonctions manipulant les secrets devront, dans la mesure du possible, prendre en paramètre une version *épinglée* 
> du secret, s'assurant ainsi que le secret ne pourra être déplacé en mémoire.
>
> Par exemple, si `T` est le type d'un secret, on privilégiera la fonction 
>
> ```rust
> fn plop()
> ``` -->

> **Règle {{#check MEM-ZERO-WRAPPED-INIT | Création de valeurs sensible dans un environnement sécurisé}}**
>
> Les valeurs sensibles seront initialisées dans un environnement s'assurant que ces valeurs seront bien
> effacées après leur usage.

L'initialisation suivante est **mauvaise** car la valeur du secret est d'abord créée dans un environnement où l'effacement n'est pas assuré

```rust
struct Secret {...}

impl Secret {
    /// This initialisation function is no secure since the content
    /// of the secret is first generated outside the secure wrapper [`Secret`]
    fn new(content : [u8; 16])
}

impl Drop for Secret {
    fn drop(&self) {
        // securely erase the content of a [`Secret`]
        ...
    }
}
```
<!-- 
## Exemple d'implementation d'un type de secret

On pourra s'inspirer de la librairie [secrust](https://github.com/hg-anssi/secrust) pour
gérer ses secrets en suivant les recommendations de ce guide. -->