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

## Mise à jour des dépendances

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

## Cargo-audit

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
