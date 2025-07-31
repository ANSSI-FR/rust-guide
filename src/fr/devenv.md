# Environnement de développement

## Rustup

[Rustup] est l'installateur des outils de la chaîne de compilation pour Rust.
Entre autres choses, il permet de basculer entre différentes variantes de la
chaîne d'outils (_stable_, _beta_, _nightly_), de gérer l'installation des
composants additionnels et de maintenir le tout à jour.

> **Attention**
>
> Du point de vue de la sécurité, `rustup` effectue tous les téléchargements en
> HTTPS, mais ne valide pas les signatures des fichiers téléchargés. Les
> protections contre les attaques par déclassement, le _pinning_ de certificats
> et la validation des signatures sont des travaux actuellement en cours. Pour
> les cas les plus sensibles, il peut être préférable d'opter pour une méthode
> d'installation alternative comme celles listées dans la section *Install* du
> site officiel du langage Rust.

[rustup]: https://github.com/rust-lang/rustup.rs

### Éditions Rust

Il existe plusieurs variantes du langage Rust, appelées *éditions*. Le concept
d'éditions a été introduit afin de distinguer la mise en place de nouvelles
fonctionnalités dans le langage, et de rendre ce processus incrémental.
Toutefois, comme mentionné dans le *[Edition Guide]*, cela ne signifie pas que
de nouvelles fonctionnalités et améliorations ne seront incluses que dans la
dernière édition.

Certaines éditions peuvent introduire de nouvelles constructions dans le langage
et de nouveaux mots-clés. Les recommandations concernant ces fonctionnalités
deviennent alors fortement liées à une édition en particulier. Dans le reste de
ce guide, un effort sera réalisé pour mettre en évidence les règles qui ne
s'appliqueraient qu'à certaines éditions de Rust en particulier.

> **Note**
>
> Aucune édition spécifique n'est recommandée, tant que le développement se
> conforme aux recommandations exprimées à propos des fonctionnalités que
> l'édition utilisée propose.

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

> **Règle {{#check DENV-STABLE | Utilisation de la chaîne d'outils *stable*}}**
>
> Le développement d'applications sécurisées doit être mené en utilisant la
> chaîne d'outils dans sa version *stable*, afin de limiter les potentiels
> *bugs* à la compilation, à l'exécution et lors de l'utilisation d'outils
> complémentaires.

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

> **Attention**
>
> Tout comme `rustup`, `cargo` effectue tous les téléchargements en HTTPS, mais
> ne valide pas l'index du registre. Des discussions sont en cours pour
> déterminer le meilleur moyen de protéger et de valider les *crates*. Pour le
> moment, la sécurité de `cargo` repose sur la bonne sécurité du site web
> [crates.io] ainsi que celle du dépôt, hébergé sur GitHub, contenant l'index du
> registre de *crates*. Pour les cas les plus sensibles, il peut être préférable
> d'opter pour une méthode d'installation alternative pour les dépendances.

Cargo propose différentes commandes et options pour adapter le processus de
compilation aux besoins de chaque projet, principalement au travers du fichier
`Cargo.toml`. Pour une présentation complète, voir le *[Cargo Book]*.

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

> **Règle {{#check DENV-CARGO-OPTS | Conservation des valeurs par défaut des variables critiques dans les profils cargo}}**
>
> Les variables `debug-assertions` et `overflow-checks` ne doivent pas être
> modifiées dans les sections de profils de développement (`[profile.dev]` and
> `[profile.test]`).

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

> **Règle {{#check DENV-CARGO-ENVVARS | Conservation des valeurs par défaut des variables d'environnement à l'exécution de cargo}}**
>
> Les variables d'environnement `RUSTC`, `RUSTC_WRAPPER` et `RUSTFLAGS` ne
> doivent pas être modifiées lorsque Cargo est appelé pour compiler un projet.

[crates.io]: https://crates.io
[cargo]: https://doc.rust-lang.org/stable/cargo/
[cargo book]: https://doc.rust-lang.org/cargo/index.html

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

> **Règle {{#check DENV-LINTER | Utilisation régulière d'un *linter*}}**
>
> Un *linter* comme `clippy` doit être utilisé régulièrement tout au long du
> développement d'une application sécurisée.

[clippy]: https://github.com/rust-lang/rust-clippy

### Rustfmt

[Rustfmt] est un outil offrant la possibilité de formater du code en fonction
de consignes de style (*style guidelines*). La documentation de l'outil
mentionne certaines limitations parmi lesquelles un support partiel des macros
(déclaration et utilisation). L'option `--check`, qui affiche les différences
de formatage entre le code actuel et le code proposé, doit être utilisé. À la
suite de cette première utilisation, l'utilisateur doit vérifier les
changements, puis éventuellement les valider en réinvoquant l'outil sans
option.

En résumé :

```shell
$ cargo fmt -- --check
$ # review of the changes
$ cargo fmt
```

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
`rustfmt` propose, voir le [*Rust Style Guide*](https://doc.rust-lang.org/style-guide/index.html).

> **Règle {{#check DENV-FORMAT | Utilisation d'un outil de formatage (rustfmt)}}**
>
> L'outil de formatage `rustfmt` peut être utilisé pour assurer le respect de
> règles de convention de style (comme décrites dans le fichier `rustfmt.toml`)
> sur une base de code, avec l'option `--check` ainsi qu'une revue de code
> manuelle.

[rustfmt]: https://github.com/rust-lang/rustfmt

### Rustfix

Inclus dans la distribution Rust depuis la fin de l'année 2018, [Rustfix] est un
outil dédié à la réparation des avertissements de compilation et facilitant la
transition entre éditions.

```shell
$ cargo fix
```

Pour préparer la transition d'un projet de l'édition Rust 2015 à Rust 2018, il
est possible de lancer la commande suivante :

```shell
$ cargo fix --edition
```

Rustfix va soit réparer le code afin de le rendre compatible avec Rust 2018,
ou bien afficher un avertissement décrivant le problème. Le problème devra alors
être réparé manuellement. En exécutant la commande (et en réparant
potentiellement les problèmes manuellement) jusqu'à ce qu'elle n'affiche plus
aucun avertissement, il est possible de s'assurer que le code est compatible
tant avec Rust 2015 qu'avec Rust 2018.

Pour basculer définitivement le projet sous Rust 2018 :

```shell
$ cargo fix --edition-idioms
```

Il est important de noter que l'outil ne fournit que peu de garanties quant
à la correction (*soundness*) des réparations proposées. Dans une certaine
configuration, certaines réparations (comme celles proposées avec l'option
`--edition-idioms`) sont connues pour casser la compilation ou pour modifier
la sémantique d'un programme dans certains cas.

> **Règle {{#check DENV-AUTOFIX | Vérification manuelle des réparations automatiques}}**
>
> Dans le cadre du développement d'une application sécurisée, toute réparation
> automatique (comme celles appliquées par `rustfix` par exemple) doit être
> vérifiée par le développeur.

[rustfix]: https://github.com/rust-lang-nursery/rustfix

### Autres

D'autres outils ou sous-commandes `cargo` utiles pour renforcer la sécurité
d'un programme existent, par exemple, en recherchant des motifs de code
particuliers. Nous en discutons dans les chapitres suivants en fonction de leurs
portées et de leurs objectifs.

## Durcissement et binaires mixtes

Les _durcissements_ sont des mécanismes mis en place pendant la compilation
permettant de réduire l'impact ou l'exploitabilité d'un certain nombre de défaut
de sûrété mémoire. Dans le cas de Rust, ces durcissements n'ont pas beaucoup
d'intérêt (hors code _unsafe_). Toutefois, la question se pose de nouveau dans
le cas de logiciel mixte, c'est-à-dire contenant des composants écrits en Rust
et des composants écrits dans un ou les langages n'assurant pas la sûreté
mémoire. En effet, il a été montré que du code Rust peut être utilisé pour
contourner des durcissements d'un code C vulnérable.

> ** Règles {{#check DENV-MIXED | Activer les durcissements pour tous les langages d'un logiciel mixte}}**
>
> Dans le cadre du développement d'une application sécurisée comportant des
> composants dans plusieurs langages, les compilations des composants (y compris
> Rust) doivent appliquer des durcissements de manière à limiter
> l'exploitabilité des vulnérabilités présents dans les composants dont le
> langage n'assure pas la sûreté mémoire.

### Références

- _Exploiting Mixed Binaries_, Michalis Papaevripides, Elias Athanasopoulos, <https://dl.acm.org/doi/10.1145/3418898>