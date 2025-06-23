# Bibliothèques

En complément de la bibliothèque standard du langage, l'outil `cargo` fournit un
moyen pratique d'intégrer des bibliothèques tierces dans un projet en Rust. Ces
bibliothèques, appelées *crates* dans l'écosystème Rust, sont importées depuis
le dépôt central de composants en sources ouvertes [crates.io](https://crates.io).

Il doit être noté que la qualité (en termes de sécurité, de performances, de
lisibilité, etc.) des *crates* publiées est très variable. De plus, leur
maintenance peut être irrégulière ou interrompue. L'usage de chaque composant
de ce dépôt doit donc être justifié, et le développeur doit également valider le
bon respect des règles du présent guide sur le code correspondant. Plusieurs
outils peuvent l'aider dans cette tâche.

## Cargo-outdated

L'outil [Cargo-outdated] permet de faciliter la gestion des versions des
dépendances.

Pour une *crate* donnée, l'outil liste les versions utilisées des dépendances
(dépendances listées dans le fichier `Cargo.toml`), et vérifie s'il s'agit de la
dernière version compatible disponible ainsi que la dernière version en général.

> ### Règle {{#check LIBS-OUTDATED | Vérification des dépendances obsolètes (cargo-outdated)}}
>
> L'outil `cargo-outdated` doit être utilisé pour vérifier le statut des
> dépendances. Ensuite, chaque dépendance importée en version obsolète doit
> être mise à jour ou bien, le cas échéant, le choix de la version doit être
> justifié.

[cargo-outdated]: https://github.com/kbknapp/cargo-outdated

## Cargo-audit

[Cargo-audit] est un outil permettant de vérifier s'il existe des vulnérabilités
connues dans la *RustSec Advisory Database* pour les dépendances utilisées dans
un projet.

> ### Règle {{#check LIBS-AUDIT | Vérification des vulnérabilités connues pour les dépendances (cargo-audit)}}
>
> L'outil `cargo-audit` doit être utilisé pour rechercher des vulnérabilités
> connues dans les dépendances d'un projet.

[cargo-audit]: https://github.com/RustSec/cargo-audit
## Vérification de la supply-chain

Rust propose, via son groupe de travail sur la sécurité un certain nombre d'outils permettant de s'assurer de la sécurité de la supply-chain d'un programme au niveau de ses bibliothèques.

### Cargo supply-chain

[Cargo-supply-chain] est l'outil développé par le groupe de travail officiel de la fondation rust et qui collecte l'ensemble des personnes qui peuvent intervenir sur les bibliothèques utilisées par le projet.

> ### Règle {{#check LIBS-SUPPLY-CHAIN | Vérification des développeurs implicitement de confiance}}
>
> L'outil `cargo-supply-chain` devrait être utilisé afin de connaître les contributeurs des différentes dépendances que votre projet utilise.

[cargo-supply-chain]: https://github.com/rust-secure-code/cargo-supply-chain

## Code *unsafe* dans les bibliothèques

[Cargo-geiger] est un outil maintenu par le groupe de travail permettant de sécuriser Rust.
Son but est de détecter l'utilisation du block `unsafe` dans la supply-chain d'un projet. Les résultats possèdent trois
niveaux :

- `🔒` lorsqu'il n'y a pas d'utilisation du bloc  `unsafe` trouvée et la ligne #![forbid(unsafe_code)] est déclarés
- `❓` lorsqu'il n'y a pas d'utilisation du bloc `unsafe` trouvée et la ligne #![forbid(unsafe_code)] est déclarés
- `☢️`   = utilisation de bloc `unsafe` trouvée dans le code

> ### Règle {{#check LIBS-AUDIT-UNSAFE | Utiliser des librairies proprement auditées}}
> Il est fortement conseillé de n'utiliser que des bibliothèques qui ont été proprement auditées par des tiers de confiances ou des entités de votre organisation.
>
> Une attention toute particulière doit être donnée aux bibliothèques utilisant du code `unsafe`. En effet, les codes `unsafe` ne bénéficiant pas des mécanismes de protection de la gestion mémorielles du langage, ils sont plus susceptibles de contenir des failles de sécurités.

> Attention
>
> A ce jour, l'outil `cargo-geiger` ne prend pas en compte quand # ![forbid(unsafe_code)] est dans le fichier `Cargo.toml`.

[cargo-geiger]: https://github.com/geiger-rs/cargo-geiger