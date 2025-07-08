# Généralités sur l'utilisation de `unsafe`

## Comportements ajoutés par Rust *unsafe*

Les capacités du langages peuvent être étendues en utilisant du code *unsafe*. La liste complète de ces capacités est donnée dans le [manuel de Rust](https://doc.rust-lang.org/reference/unsafety.html). On notera les capacités suivantes.

* Déréférencer un *raw pointer*
* Modifier une variable mutable statique
* Accéder aux champs d'une `union`
* Déclarer un block `extern`

Si ces capacités sont nécessaires à la programmation système, elles font perdre au langage ses [propriétés de sûreté](04_language.md#garanties-du-langage).

## Utilisation de Rust *unsafe*

L'utilisation conjointe du système de types et du système d'*ownership* vise à
apporter un haut niveau de sûreté quant à la gestion de la mémoire dans les
programmes écrits en Rust. Le langage permet alors d'éviter les débordements
mémoire, la construction de pointeurs nuls ou invalides, et les problèmes
d'accès concurrents à la mémoire.
Pour effectuer des actions considérées risquées comme des appels système, des
conversions de types ou la manipulation directe de pointeurs mémoire, le
langage fournit le mot-clé `unsafe`.

> **Règle {{#check LANG-UNSAFE | Non-utilisation des blocs *unsafe*}}**
>
> Pour un développement sécurisé, les blocs `unsafe` doivent être évités.
> Ci-dessous, nous listons les seuls cas pour lesquels des blocs `unsafe`
> peuvent être utilisés, à la condition que leur usage soit justifié :
>
>  - L'interfaçage entre Rust et d'autres langages (FFI) permet la déclaration
>  de fonctions dont l'implantation est faite en C, en utilisant le préfixe
>  `extern "C"`. Pour utiliser une telle fonction, le mot-clé `unsafe` est
>  requis. Un *wrapper* "sûr" doit être défini pour que le code C soit
>  finalement appelé de façon souple et sûre.
>
>  - Pour la programmation des systèmes embarqués, on accède souvent aux
>  registres et à d'autres ressources au travers d'adresses mémoire fixées
>  Dans ce cas, des blocs `unsafe` sont nécessaires afin de pouvoir initialiser
>  et déréférencer des pointeurs en Rust pour ces adresses. Afin de minimiser le
>  nombre de déclarations `unsafe` pour permettre au développeur de facilement
>  identifier les accès critiques, une abstraction adaptée (structure de
>  données ou module) doit être mise en place.
>
>  - Une fonction peut être marquée globalement comme non sûre (en préfixant sa
>  déclaration par le mot-clé `unsafe`) lorsqu'elle exhibe inévitablement des
>  comportements non sûrs en fonction de ses arguments. Par exemple, cela arrive
>  lorsqu'une fonction doit déréférencer un pointeur passé en argument.
>
> À l'exception de l'un ou plusieurs de ces cas `#![forbid(unsafe_code)]` doit
> apparaître dans à la racine de la *crate* (typiquement `main.rs` ou `lib.rs`)
> afin de générer des erreurs de compilation dans le cas ou le mot-clé `unsafe`
> est utilisé dans le projet.