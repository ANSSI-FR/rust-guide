# Bibliothèques

## Dépôts de dépendances

La gestion de bibliothèques externes est intégrée dans l'outils `Cargo`. Plusieurs moyens existent pour définir la provenance de ces bibliothèques, certains sont donnés dans la suite.

On rappelle que le traçage exact des versions de ces bibliothèques est une condition important de la bonne sécurité des logiciels écrits en Rust. Ce besoin est matérialisé par la règle [DENV-CARGO-LOCK](devenv.md#DENV-CARGO-LOCK).

### Crates

En complément de la bibliothèque standard du langage, l'outil `cargo` fournit un
moyen pratique d'intégrer des bibliothèques tierces dans un projet en Rust. Ces
bibliothèques, appelées *crates* dans l'écosystème Rust, sont importées depuis
un dépôt central de composants en sources ouvertes.

Un exemple de déclaration de dépendances dans le fichier `Cargo.toml`.

```toml
[dependencies]
mdbook = { version = "0.4.52" }
anyhow = "1.0.99"
clap = { version = "4.5.47", features = ["derive"] }
markdown = { version = "1.0.0", features = ["serde"] }
semver = "1.0.26"
serde_json = "1.0.143"
serde = "1.0.219"
```

Le dépôt par défaut est [crates.io](https://crates.io). Il est aussi possible d'utiliser [son propre dépôt](https://doc.rust-lang.org/cargo/reference/registries.html).

### Dépendances Git

Chaque dépendance du fichier `Cargo.toml` peut également faire référence à [un dépôt GIT](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories). Par exemple :

```toml
[dependencies]
regex = { git = "https://github.com/rust-lang/regex.git" }
```

Il est possible de spécifier plus en détails la version souhaitée en donnant soit une branche, soit un tag, soit un hash de commit.

Le système de [verrou des dépendances](devenv.md#cargo) est opérant même dans le cas d'un dépôt GIT : dans le cas où la dépendance ne spécifie pas un commit en particulier, le commit le plus récent répondant aux critères du fichier `Cargo.toml` est récupéré au moment de la première compilation et est pérennisé dans le fichier `Cargo.lock`. Toutes les compilations suivantes utiliseront le même commit (sauf si le ficher `Cargo.lock` est mis à jour).

## Sécurité des dépendances

Quelque soit la méthode de récupération des dépendances (*crate* ou commit GIT), si elles proviennent d'organisation extérieures, les dépendances doivent faire l'objet d'une validation.

<div class="reco" id="LIBS-VETTING-DIRECT" type="Règle" title="Validation des dépendances tierces directes">

Chaque dépendance tierce directe DOIT être dûment validée, et chaque validation DOIT être tracée.

</div>

Concernant les dépendances transitives, il est également recommandé de les valider individuellement.

<div class="reco" id="LIBS-VETTING-TRANSITIVE" type="Recommandation" title="Validation des dépendances tierces transitives">

Chaque dépendance tierce DEVRAIT être dûment validée, et chaque validation DEVRAIT être tracée.

</div>

## Outils de vérification des dépendances

### Cargo-outdated

L'outil [Cargo-outdated] permet de faciliter la gestion des versions des
dépendances.

Pour une *crate* donnée, l'outil liste les versions utilisées des dépendances
(dépendances listées dans le fichier `Cargo.toml`), et vérifie s'il s'agit de la
dernière version compatible disponible ainsi que la dernière version en général.

<div class="reco" id="LIBS-OUTDATED" type="Règle" title="Vérification des dépendances obsolètes (cargo-outdated)">

L'outil `cargo-outdated` DOIT être utilisé pour vérifier le statut des
dépendances. Ensuite, chaque dépendance importée en version obsolète DEVRAIT
être mise à jour ou bien, le cas échéant, le choix de la version DOIT être
justifié.

</div>

[cargo-outdated]: https://github.com/kbknapp/cargo-outdated

### Cargo-audit

[Cargo-audit] est un outil permettant de vérifier s'il existe des vulnérabilités
connues dans la *RustSec Advisory Database* pour les dépendances utilisées dans
un projet.

<div class="reco" id="LIBS-AUDIT" type="Règle" title="Vérification des vulnérabilités connues pour les dépendances (cargo-audit)">

L'outil `cargo-audit` DOIT être utilisé pour rechercher des vulnérabilités
connues dans les dépendances d'un projet.

</div>

[cargo-audit]: https://github.com/RustSec/cargo-audit

<!-- ## Code *unsafe* dans les bibliothèques -->

<!--
<mark>TODO</mark>: les blocs de code `unsafe` sont discutés dans le chapitre 
suivant. Le développeur a besoin de s'assurer que ces types de blocs ne sont pas
mal utilisés dans les dépendances de son projet.
-->

<!--
<div class="reco" id="LIBS-UNSAFE" type="Recommandation" title="Vérification du code *unsafe* dans les dépendances">

<mark>TODO</mark>: vérifier qu'il n'y a pas de bloc `unsafe` dans les
dépendances (à l'aide d'un outil ?).

</div>
-->
