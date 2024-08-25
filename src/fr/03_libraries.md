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
> L'outil `cargo-supply-chain` doit être utilisé afin de conaître les personnes à qui vous faites implicitement confiance pour le bon fonctionnement de votre projet.

[cargo-supply-chain]: https://github.com/rust-secure-code/cargo-supply-chain
### Cargo vet / crev

[Cargo-vet] est un outil développé par la fondation mozilla et qui permet de vérifier si les librairies que vous pouvez utiliser son audité par des tiers de confiance.

> ### Règle {{#check LIBS-VET | Utilisation en priorité de librairie ayant été audité}}
>
> Il est conseillé d'utiliser l'outil `cargo-vet` afin d'utiliser en priorité des librairies ayant été audités par des tiers.

Les audits de sécurités peuvent être créés à l'aide d'un outil nommé [Cargo-crev]. L'utilisation de cet outil ne sera pas détaillée dans ce guide.

Pour plus d'information, veuillez consulter la [documentation officielle] de l'outil.

> ### Conseille
>
> Il est conseillés de faire d'audits de sécurités via l'outil `cargo-crev` afin de vérifier la sécurité 
> des librairies utilisées dans votre projet et d'en faire profiter la communauté.

[cargo-vet]: https://github.com/mozilla/cargo-vet
[cargo-crev]: https://github.com/crev-dev/cargo-crev
[documentation officielle]: https://github.com/crev-dev/cargo-crev/blob/main/cargo-crev/src/doc/getting_started.md
## Code *unsafe* dans les bibliothèques

[Cargo-geiger] est un outil maintenu par le groupe de travail permettant de sécuriser Rust.
Son but est de détecter l'utilisation du block `unsafe` dans la supply-chain d'un projet.

Les résultats possèdent trois niveaux : 
1) 🔒  = pas d'utilisation du bloc  `unsafe` trouvée, la ligne #![forbid(unsafe_code)] est déclarés
2) ❓  = pas d'utilisation du bloc `unsafe` trouvée, la ligne n'est pas  #![forbid(unsafe_code)] est déclarés
3) ☢️   = utilisation de bloc `unsafe` trouvée dans le code

> ### Règle {{#check LIBS-UNSAFE | Vérification du code *unsafe* dans les dépendances}}
>
> Utiliser l'outil `cargo-geiger` afin de vérifier que les usages du block `unsafe` respectent bien les recommandations décrites dans la section suivantes de ce guide.

> Attention
>
> To date, the `cargo-geiger` tool does not take into account when #![forbid(unsafe_code)] is in its second form in the `Cargo.toml` file.

[cargo-geiger]: https://github.com/geiger-rs/cargo-geiger