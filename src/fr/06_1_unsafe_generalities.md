# Généralités sur l'utilisation de `unsafe`

## Comportements ajoutés par Rust *unsafe*

* Déréférencer un [`raw pointer`](https://doc.rust-lang.org/std/primitive.pointer.html)
* Modifier une variable mutable [statique](https://doc.rust-lang.org/std/keyword.static.html)
* Accéder aux champs d'une [`union`](https://doc.rust-lang.org/std/keyword.union.html)
* Accéder à une [feature](https://doc.rust-lang.org/cargo/reference/features.html) à partir d'une autre sans que le lien entre les deux ne soit exprimé dans le fichier `Cargo.toml`
* Implémenter un [trait `unsafe`](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#implementing-an-unsafe-trait)
* Déclarer un block [externe](https://doc.rust-lang.org/std/keyword.extern.html)
* Appeler une [fonction unsafe](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#calling-an-unsafe-function-or-method)

### Références

* https://doc.rust-lang.org/reference/unsafety.html

## Précaution générale d'un code unsafe

***TODO***

### Références

* https://doc.rust-lang.org/nomicon/working-with-unsafe.html