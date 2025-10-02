# Système de types

## Sémantique typée

Certains langage (comme OCaml, Java ou C) ont une sémantique majoritairement[^syntax_directed_exceptions] *non typée* car leur exécution n'a pas besoin d'information de type. D'autres langages comme Haskell, Rust ou C++[^cpp_typed] ont majoritairement une sémantique *typée* dans laquelle la sémantique d'une opération dépend de son type.

[^syntax_directed_exceptions]: Les types primitifs de Java utilisent des informations de type pour déterminer leur sémantique. De même, l’arithmétique des tableaux en C utilise les informations de type pour calculer les *offsets*.
[^cpp_typed]: On excepte le cas des méthodes virtuelles.

Par exemple, dans les programmes suivants, l'expression `"127.0.0.1:8080".parse()` dépend du type attendu :

```rust
let r : Result<u8, _> = "127.0.0.1:8080".parse();
println!("{:?}", r);
```

affiche

```
Err(ParseIntError { kind: InvalidDigit })
```

alors que

```rust
let r : Result<std::net::SocketAddr, _> = "127.0.0.1:8080".parse();
println!("{:?}", r);
```

affiche

```
Ok(127.0.0.1:8080)
```

Le compilateur Rust utilise ici l'information de type pour résoudre la fonction `parse`.

<!-- Attention aux implémentations de la même fonction pour un smart pointer et le type qu'il wrap -->
<!-- Attention aux implémentations qui se chevauchent mais pas trop dérangeant car si on ne sait pas qu'un type implémente un trait, on ne va pas l'utiliser dans le cadre de ce trait -->