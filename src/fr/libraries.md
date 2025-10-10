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

Chaque dépendance tierce directe doit être dûment validée, et chaque validation doit être tracée.

</div>

Concernant les dépendances transitives, il est également recommandé de les valider individuellement.

<div class="reco" id="LIBS-VETTING-TRANSITIVE" type="Recommandation" title="Validation des dépendances tierces transitives">

Chaque dépendance tierce devrait être dûment validée, et chaque validation devrait être tracée.

</div>

## Outils de vérification des dépendances

### Mise à jour des dépendances

La mise à jour des bibliothèques tierces est primordiale pour assurer la bonne 
correction d'éventuelles vulnérabilités.

Le mécanisme de résolution des dépendances s'appuie sur les fichiers `Cargo.toml` et
`Cargo.lock`.

* Le fichier `Cargo.toml` liste les contraintes imposées par les développeur
* Le fichier `Cargo.lock` trace la résolution de ces contraintes par Cargo *à un moment donnée*.

Aussi, la mise à jour des dépendances intervient à plusieurs niveau.

* Le fichier `Cargo.toml` peut être mis à jour pour utiliser une nouvelle version d'une dépendance.
  À la prochaine compilation, le fichier `Cargo.lock` sera mis à jour en conséquence.
* Pour un fichier `Cargo.toml` donné, la résolution des dépendances par cargo peut changer entre deux instants.
  En effet, de nouvelles version de dépendances on pu être publiées dans ce lapse de temps.
  Aussi, pour mettre à jour les dépendances tout en conservant les contraintes de `Cargo.toml`, on 
  peut mettre à jour le fichier `Cargo.lock` grâce à la commande `cargo update`,
  ce qui revient à supprimer le fichier `Cargo.lock` et le reconstruire ou d'appliquer `cargo generate-lockfile`. <!-- vérifié par le test mais est-ce tout le temps le cas ? -->

  La commande `cargo update` permet aussi de ne mettre à jour qu'une partie des dépendances. Par exemple

  ```
  cargo update serde clap
  ```

### Cargo-audit

[Cargo-audit] est un outil permettant de vérifier s'il existe des vulnérabilités
connues dans la *RustSec Advisory Database* pour les dépendances utilisées dans
un projet.

<div class="reco" id="LIBS-AUDIT" type="Règle" title="Vérification des vulnérabilités connues pour les dépendances (cargo-audit)">

L'outil `cargo-audit` doit être utilisé pour rechercher des vulnérabilités
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
