---
references:
  - type: web
    title: The Rust Edition Guide
    url: https://doc.rust-lang.org/edition-guide/
    id: rust-edition-guide
  - type: web
    title: The Cargo Book
    url: https://doc.rust-lang.org/cargo/index.html
    id: cargo-book
  - type: web
    title: Rust Style Guide
    url: https://doc.rust-lang.org/style-guide/index.html
    id: rust-style
  - type: web
    title: The rustc book
    url: https://doc.rust-lang.org/stable/rustc/index.html
    id: rustc-book
---
# Environnement de développement

## Rustup

[Rustup] est l'installateur des outils de la chaîne de compilation pour Rust.
Entre autres choses, il permet de basculer entre différentes variantes de la
chaîne d'outils (_stable_, _beta_, _nightly_), de gérer l'installation des
composants additionnels et de maintenir le tout à jour.

<div class="warning">

Du point de vue de la sécurité, `rustup` effectue tous les téléchargements en
HTTPS, mais ne valide pas les signatures des fichiers téléchargés. Les
protections contre les attaques par déclassement, le _pinning_ de certificats
et la validation des signatures sont des travaux actuellement en cours. Pour
les cas les plus sensibles, il peut être préférable d'opter pour une méthode
d'installation alternative comme celles listées dans la section *Install* du
site officiel du langage Rust.

</div>

[rustup]: https://github.com/rust-lang/rustup.rs

### Éditions Rust

Il existe plusieurs variantes du langage Rust, appelées *éditions*. Le concept
d'éditions a été introduit afin de distinguer la mise en place de nouvelles
fonctionnalités dans le langage, et de rendre ce processus incrémental.
Toutefois, comme mentionné dans le [@rust-edition-guide], cela ne signifie pas que
de nouvelles fonctionnalités et améliorations ne seront incluses que dans la
dernière édition.

Certaines éditions peuvent introduire de nouvelles constructions dans le langage
et de nouveaux mots-clés. Les recommandations concernant ces fonctionnalités
deviennent alors fortement liées à une édition en particulier. Dans le reste de
ce guide, un effort sera réalisé pour mettre en évidence les règles qui ne
s'appliqueraient qu'à certaines éditions de Rust en particulier.

<div class="note">

Aucune édition spécifique n'est recommandée, tant que le développement se
conforme aux recommandations exprimées à propos des fonctionnalités que
l'édition utilisée propose.

</div>

[edition guide]: https://doc.rust-lang.org/edition-guide/

### Chaînes d'outils *stable*, *nightly* et *beta*

De manière orthogonale aux éditions qui permettent d'opter pour une variante du
langage en termes de fonctionnalités, la chaîne d'outils du langage Rust est
déclinée en trois variantes appelées *release channels*.

- La version *nightly* est produite une fois par jour.
- La version *nightly* est promue en version *beta* toutes les six semaines.
- La version *beta* est promue en version *stable* toutes les six semaines.

Lors du développement d'un projet, il est important de vérifier non seulement
la version de la chaîne d'outils couramment sélectionnée par défaut, mais aussi
les potentielles surcharges qui peuvent être définies en fonction des
répertoires :

```shell
$ pwd
/tmp/foo
$ rustup toolchain list
stable-x86_64-unknown-linux-gnu (default)
beta-x86_64-unknown-linux-gnu
nightly-x86_64-unknown-linux-gnu
$ rustup override list
/tmp/foo                                    nightly-x86_64-unknown-linux-gnu
$
```

<!-- -->

<div class="reco" id="DENV-STABLE" type="Règle" title="Utilisation de la chaîne d'outils *stable*">

Le développement d'applications sécurisées DOIT être mené en utilisant la
chaîne d'outils dans sa version *stable*, afin de limiter les potentiels
*bugs* à la compilation, à l'exécution et lors de l'utilisation d'outils
complémentaires.

</div>

Enfin, lorsque l'utilisation d'un outil (par exemple, une sous-commande `cargo`)
requiert l'installation d'un composant dans une chaîne d'outils non *stable*,
le basculement de variante doit être effectué de la façon la plus locale
possible (idéalement, uniquement pendant la commande concernée) au lieu
d'explicitement basculer la version courante. Par exemple, pour lancer la
version *nightly* de la commande `rustfmt` :

```shell
$ rustup toolchain list
stable-x86_64-unknown-linux-gnu (default)
beta-x86_64-unknown-linux-gnu
nightly-x86_64-unknown-linux-gnu
$ rustup run nightly cargo fmt
$ # or
$ cargo +nightly fmt
$
```
### Garantie de niveau pour Rustc

Rustc utilise LLVM comme backend, il hérite donc du support de ce dernier et classe ses cibles prises en charge en différents niveaux afin d’indiquer le degré de stabilité et de tests effectué.

#### Tier 1 - Fonctionnement garanti

La cible est entièrement examinée par la communauté. Elle réussit l’ensemble complet de la batterie de tests, fait l’objet de tests de régression réguliers et est maintenue à jour avec les nouvelles versions. En pratique, vous pouvez compter sur une génération de code cohérente, une ABI stable et des performances prévisibles d’une version à l’autre. Les cibles de tier 1 offrent une garantie de **fonctionnement**.

#### Tier 2 - Compilation garantie

La cible se compile correctement, mais elle ne bénéficie pas du même niveau de tests ni de la même maintenance que les cibles de niveau 1. Elle peut ne pas être entièrement couverte par les tests, et certaines optimisations ou fonctionnalités récentes peuvent être absentes ou instables. Les utilisateurs peuvent tout de même générer du code pour ces cibles, mais ils doivent s’attendre à d’éventuels problèmes occasionnels ou à devoir appliquer des correctifs manuels. Les cibles de tier 2 offrent une garantie de **compilation** mais pas de **fonctionnement**.

#### Tier 3

Les cibles de niveau 3 ne sont tout simplement pas prises en charge officiellement.


La distinction entre les différents niveaux aide les développeurs à choisir une cible adaptée à leur tolérance au risque : le niveau 1 pour des applications de production, le niveau 2 pour des architectures plus expérimentales ou de niche dont le support complet n’est pas encore assuré.


<div class="reco" id="DENV-TIERS" type="Rule" title="Utilisation exclusive du tier 1 de `rustc` pour les logiciels de sûreté critiques">

Les cibles Rustc de niveau 1 et les chaînes de compilation certifiées DOIVENT être utilisées pour les systèmes de sûreté critiques.

</div>


Une liste complète des cibles prises en charge est disponible dans [@rustc-book].


## Cargo

Une fois que la chaîne d'outils appropriée a été sélectionnée avec Rustup,
l'outil [Cargo] est disponible pour exécuter ces différents outils en
fournissant la commande `cargo`. Cargo est le gestionnaire de paquetages de Rust.
Il joue plusieurs rôles fondamentaux tout au long d'un développement en Rust. Il
permet notamment de :

- structurer le projet en fournissant un squelette de projet (`cargo new`) ;
- lancer la compilation du projet (`cargo build`) ;
- lancer la génération de la documentation (`cargo doc`) ;
- lancer les tests (`cargo test`) et les *benchmarks* (`cargo bench`) ;
- gérer le téléchargement des dépendances ;
- rendre le projet distribuable et le publier sur [crates.io]
  (`cargo publish`) ;
- lancer des outils complémentaires tels que ceux décrits ci-après, sous la
  forme de sous-commandes.

Cargo permet la récupération automatique des dépendances avant compilation.
L'utilitaire permet de vérifier l'intégrité des dépendances après téléchargement.
Il utilise pour cela un fichier `Cargo.lock` qui, s'il est présent au moment de la compilation,
contraint les sommes de contrôle des dépendances. En cas de différence entre
les sources téléchargées et le fichier `Cargo.lock`, un erreur apparaît.

```
error: checksum for `sha256 v1.6.0` changed between lock files

this could be indicative of a few possible errors:

    * the lock file is corrupt
    * a replacement source in use (e.g., a mirror) returned a different checksum
    * the source itself may be corrupt in one way or another

unable to verify that `sha256 v1.6.0` is the same as when the lockfile was generated
```

En cas d'absence du fichier `Cargo.lock`, il est créé automatiquement à la première compilation en suivant les
sommes de contrôle des sources téléchargées (selon le principe TOFU : *Trust On First Use*).
Ce fichier peut également être créé manuellement avec `cargo generate-lockfile`, et s'il
est déjà présent, un nouveau fichier est créé avec les dernières versions compatibles de chaque crate.

<div class="reco" id="DENV-CARGO-LOCK" type="Règle" title="Mise en dépôt du fichier Cargo.lock">

Le fichier `Cargo.lock` DOIT être versionné avec le code source du programme Rust.

</div>

<div class="warning">

Des discussions sont en cours pour
déterminer le meilleur moyen de protéger et de valider les *crates* lors de leur ajout au projet
(les téléchargements suivants sont vérifié par le fichier `Cargo.lock`). Pour le
moment, la sécurité des premiers téléchargements de `cargo` repose sur la bonne sécurité du site web
[crates.io] ainsi que celle du dépôt, hébergé sur GitHub, contenant l'index du
registre de *crates*. Pour les cas les plus sensibles, il peut être préférable
d'opter pour une méthode d'installation alternative pour les dépendances.

</div>

Cargo propose différentes commandes et options pour adapter le processus de
compilation aux besoins de chaque projet, principalement au travers du fichier
`Cargo.toml`. Pour une présentation complète, voir le [@cargo-book].

Certaines de ces options requièrent une attention particulière.

La section `[profile.*]` permet de configurer la façon dont le compilateur est
invoqué. Par exemple :

- La variable `debug-assertions` contrôle l'activation des assertions de
  *debug*.
- La variable `overflow-checks` contrôle l'activation de la vérification des
  dépassements d'entiers lors d'opérations arithmétiques.

Changer les options par défaut pour ces variables peut entraîner l'apparition de
*bugs* non détectés, même si le profil de *debug* qui active normalement les
vérifications (par exemple, les
[vérifications de dépassements d'entiers](./04_language.html#integer-overflows))
est utilisé.

<div class="reco" id="DENV-CARGO-OPTS" type="Règle" title="Conservation des valeurs par défaut des variables critiques dans les profils cargo">

Les variables `debug-assertions` et `overflow-checks` NE DOIVENT PAS être
modifiées dans les sections de profils de développement (`[profile.dev]` and
`[profile.test]`).

</div>

Cargo propose d'autres moyens de configuration afin de modifier son comportement
sur un système donné. Cela peut être très pratique, mais il peut alors aussi
être difficile de connaître et de se souvenir de toutes les options qui sont
effectivement passées à `cargo`, et en particulier passées ensuite au
compilateur Rust. Finalement, cela peut affecter la robustesse du processus de
compilation et la confiance qu'on lui accorde. Il est préférable de centraliser
les options de compilation dans le fichier de configuration `Cargo.toml`. Pour
le cas spécifique de la variable d'environnement `RUSTC_WRAPPER`, utilisée par
exemple pour générer une partie du code ou pour invoquer un outil externe avant
la compilation, il est préférable d'utiliser la fonctionnalité de *scripts de
compilation* de Cargo.

<div class="reco" id="DENV-CARGO-ENVVARS" type="Règle" title="Conservation des valeurs par défaut des variables d'environnement à l'exécution de cargo">

Les variables d'environnement `RUSTC`, `RUSTC_WRAPPER` et `RUSTFLAGS` NE
DOIVENT PAS être modifiées lorsque Cargo est appelé pour compiler un projet.

</div>

[crates.io]: https://crates.io
[cargo]: https://doc.rust-lang.org/stable/cargo/

### Rustfmt

[Rustfmt] est un outil offrant la possibilité de formater du code en fonction
de consignes de style (*style guidelines*).

Les règles de convention de style peuvent être personnalisées au besoin dans
le fichier `rustfmt.toml` ou `.rustfmt.toml` à la racine du projet. Il sera
utilisé par l'outil en écrasant les préférences par défaut. Par exemple :

```toml
# Set the maximum line width to 120
max_width = 120
# Maximum line length for single line if-else expressions
single_line_if_else_max_width = 40
```

Pour plus d'informations à propos des règles de convention de style que
`rustfmt` propose, voir [@rust-style].

<div class="reco" id="DENV-FORMAT" type="Recommandation" title="Utilisation d'un outil de formatage (rustfmt)">

L'outil de formatage `rustfmt` DEVRAIT être utilisé pour assurer le respect de
règles de convention de style sur une base de code.

</div>

[rustfmt]: https://github.com/rust-lang/rustfmt

### Cargo fix

La commande `cargo fix` est un
outil dédié à la réparation des avertissements de compilation et facilitant la
transition entre éditions.

```shell
$ cargo fix
```

Pour préparer la transition d'un projet de l'édition Rust 2021 à Rust 2024, il
est possible de lancer la commande suivante :

```shell
$ cargo fix --edition
```

Rustfix va soit réparer le code afin de le rendre compatible avec Rust 2024,
ou bien afficher un avertissement décrivant le problème. Le problème devra alors
être réparé manuellement. En exécutant la commande (et en réparant
potentiellement les problèmes manuellement) jusqu'à ce qu'elle n'affiche plus
aucun avertissement, il est possible de s'assurer que le code est compatible
tant avec Rust 2021 qu'avec Rust 2024.

Pour basculer définitivement le projet sous Rust 2024 :

```shell
$ cargo fix --edition-idioms
```

Il est important de noter que l'outil ne fournit que peu de garanties quant
à la correction (*soundness*) des réparations proposées. Dans une certaine
configuration, certaines réparations (comme celles proposées avec l'option
`--edition-idioms`) sont connues pour casser la compilation ou pour modifier
la sémantique d'un programme dans certains cas.

[rustfix]: https://github.com/rust-lang-nursery/rustfix

### Clippy

[Clippy] est un outil permettant la vérification de nombreux *lints* (*bugs*,
style et lisibilité du code, problèmes de performances, etc.). Depuis la chaîne
d'outils stable en version 1.29, `clippy` peut être installé dans
l'environnement `rustup` stable. Il est aussi recommandé d'installer `clippy` en
tant que composant (`rustup component add clippy`) dans la chaîne d'outils
stable plutôt que de l'installer comme une dépendance de chaque projet.

L'outil fournit plusieurs catégories de *lints*, selon le type de problème qu'il
vise à détecter dans le code. Les avertissements doivent être revérifiés par le
développeur avant d'appliquer la réparation suggérée par `clippy`, en
particulier dans le cas des *lints* de la catégorie `clippy::nursery` puisque
ceux-ci sont encore en cours de développement et de mise au point.

`clippy` dispose maintenant d'un outils `fix` similaire à celui de `rustfix`.

[clippy]: https://github.com/rust-lang/rust-clippy

<div class="reco" id="DENV-LINTER" type="Règle" title="Utilisation régulière d'un *linter*">

Un *linter* comme `clippy` DOIT être utilisé régulièrement tout au long du
développement d'une application sécurisée.

</div>

<div class="reco" id="DENV-AUTOFIX" type="Règle" title="Vérification manuelle des réparations automatiques">

Dans le cadre du développement d'une application sécurisée, toute réparation
automatique (comme celles appliquées par `rustfix` ou `clippy` par exemple) DOIT être
vérifiée par le développeur.

</div>

### Autres

D'autres outils ou sous-commandes `cargo` utiles pour renforcer la sécurité
d'un programme existent, par exemple, en recherchant des motifs de code
particuliers. Nous en discutons dans les chapitres suivants en fonction de leurs
portées et de leurs objectifs.
