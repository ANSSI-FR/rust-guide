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

<!-- ## Code *unsafe* dans les bibliothèques -->

<!--
<mark>TODO</mark>: les blocs de code `unsafe` sont discutés dans le chapitre 
suivant. Le développeur a besoin de s'assurer que ces types de blocs ne sont pas
mal utilisés dans les dépendances de son projet.
-->

<!--
> ### Recommandation {{#check LIBS-UNSAFE | Vérification du code *unsafe* dans les dépendances}}
>
> <mark>TODO</mark>: vérifier qu'il n'y a pas de bloc `unsafe` dans les
> dépendances (à l'aide d'un outil ?).
-->
