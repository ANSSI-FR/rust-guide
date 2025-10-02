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
---
# Development environment

## Rustup

[Rustup] is the Rust toolchain installer. Among other things, it enables
switching between different flavors of the toolchain (stable, beta, nightly),
managing additional components installation and keeping them up to date.

<div class="warning">

From a security perspective, `rustup` does perform all downloads over HTTPS,
but does not yet validate signatures of downloads. Protection against
downgrade attacks, certificate pinning, validation of signatures are still
works in progress.
In some cases, it may be preferable to opt for an alternative installation
method listed in the *Install* section of the official Rust website.

</div>

[rustup]: https://github.com/rust-lang/rustup.rs

### Rust Editions

Several flavors, called *editions*, of the Rust language coexist.

The concept of editions has been introduced to clarify the
implementation of new features and to make them incremental. A new edition will be produced
every two or three years, as stated in [@rust-edition-guide], but this doesn’t
mean that new features and improvements will only be shipped in a new edition.

Some editions bring new keywords and language constructs. Recommendations for
secure applications development then remain closely linked to features of the
language, that are used in such applications, rather than to Rust editions.
In the rest of this guide, best effort will be made to highlight constructions
and language features that are specific to a particular Rust edition.

<div class="note">

No specific edition is recommended, as long as users follow the
recommendations related to the features offered by the edition that has been
chosen.

</div>

[edition guide]: https://doc.rust-lang.org/edition-guide/


### Stable, nightly and beta toolchains

Independently from the choice of edition that allows one to select a flavor (a set of features)
of the Rust language, the Rust toolchain is provided in three different
versions, called *release channels*:

- *nightly* releases are created once a day,
- *beta* releases are created every six weeks, from promoted *nightly* releases,
- *stable* releases are created every six weeks, from promoted *beta* releases.

When playing with different toolchains, it is important to check not only what
the default toolchain is, but also if overrides are currently set for some
directories.

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

<div class="reco" id="DENV-STABLE" type="Rule" title="Use a stable compilation toolchain">

Development of a secure application must be done using a fully stable
toolchain, for limiting potential compiler, runtime or tool bugs.

</div>

When using a specific `cargo` subcommand that requires a nightly component,
it is preferable to run it by switching the toolchain only locally, instead
of explicitly switching the complete toolchain. For example, to run the
(nightly) latest `rustfmt`:

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

Once Rustup has set up the appropriate Rust toolchain, [Cargo] is available
through the command line program `cargo`. Cargo is the Rust package manager.
It has a fundamental role in most Rust developments:

- It structures the project by providing the project's skeleton (`cargo new`),
- It compiles the project (`cargo build`),
- It generates the project's documentation (`cargo doc`),
- It runs tests (`cargo test`) and benchmarks (`cargo bench`),
- It manages and downloads dependencies,
- It makes packages distributable and publishes them on [crates.io],
- It’s also a front-end to run complementary tools such as those that are
  described below, in the form of sub-commands.

Cargo enables automatic dependencies resolution before compilation 
by checking their checksums.
File `Cargo.lock` contains all dependencies checksums which are compared to
the one downloaded.
If a difference is detected, compilation fails:

```
error: checksum for `sha256 v1.6.0` changed between lock files

this could be indicative of a few possible errors:

    * the lock file is corrupt
    * a replacement source in use (e.g., a mirror) returned a different checksum
    * the source itself may be corrupt in one way or another

unable to verify that `sha256 v1.6.0` is the same as when the lockfile was generated
```

When the `Cargo.lock` file is missing, it is generated on first build and records the
dependencies' checksums, adhering to a TOFU (*Trust On First Use*) policy.
This file can also be created manually with `cargo generate-lockfile`. If it already exists,
the file is overwritten with the latest available version of every crate.

<div class="reco" id="DENV-CARGO-LOCK" type="Rule" title="Track Cargo.lock in version control system">

`Cargo.lock` files must be tracked by version control system.

</div>

<div class="warning">

Ongoing discussions occur on how to best protect
and verify crates *on their first download* (according TOFU rule).
For now, the security of the first download relies on the good security of the
website [crates.io] and the GitHub hosted repository containing the
registry index. In some cases, it may be preferable to opt for an alternative
installation method for dependencies.

</div>

Cargo proposes many different commands and options to adapt the build process to
your project needs, mainly through the manifest file `Cargo.toml`. For a
complete presentation, see [@cargo-book].

During the development of a secure application, some of the options may require
some attention. The `[profile.*]` sections allow configuring how the compiler is
invoked. For example:

- the `debug-assertions` variable controls whether debug assertions are enabled,
- the `overflow-checks` variable controls whether overflows are checked for
  integer arithmetic.

Overriding the default options may cause bugs not being detected, even when
using the debug profile that normally enables runtime checks (for example it does not enable
[integer overflow checks](./04_language.html#integer-overflows)).

<div class="reco" id="DENV-CARGO-OPTS" type="Rule" title="Keep default values for critical variables in cargo profiles">

The variables `debug-assertions` and `overflow-checks` must not be overridden
in development profiles' sections (`[profile.dev]` and `[profile.test]`).

</div>

Cargo proposes other ways to setup its configuration and change its behavior on
a given system. This can be very useful, but it may also be difficult to know
and remember at a given time all the options that are effectively used, and
in particular passed to the compiler. At the end, this can affect the confidence
and robustness of the build process. It is preferable to centralize compiler
options and flags in the configuration file `Cargo.toml`. For the case of
environment variable `RUSTC_WRAPPER`, for example, that may be used to generate
part of the code or to run external tools before Rust compilation, it is preferable
to use the Cargo build scripts feature.

<div class="reco" id="DENV-CARGO-ENVVARS" type="Rule" title="Keep default values for compiler environment variables when running cargo">

The environment variables `RUSTC`, `RUSTC_WRAPPER` and `RUSTFLAGS` must not
be overriden when using Cargo to build the project.

</div>

[crates.io]: https://crates.io
[cargo]: https://doc.rust-lang.org/stable/cargo/

### Rustfmt

Included with Rust, [Rustfmt] is a tool that formats your code according to style guidelines.

These guidelines can be customized to your needs by creating a `rustfmt.toml` or
`.rustfmt.toml` file at the root of your project. It will be used to override
the default settings, for instance:

```toml
# Set the maximum line width to 120
max_width = 120
# Maximum line length for single line if-else expressions
single_line_if_else_max_width = 40
```

For more information about the guidelines that `rustfmt` will check, have a look
at the [@rust-style].

<div class="reco" id="DENV-FORMAT" type="Rule" title="Use Rust formatter (rustfmt)">

The tool `rustfmt` can be used to ensure that the codebase respects style
guidelines (as described in `rustfmt.toml` file).

</div>

[rustfmt]: https://github.com/rust-lang/rustfmt

### Cargo fix

The `cargo fix` command is a tool dedicated in
fixing compiler warnings as well as easing transitions between editions.

```shell
$ cargo fix
```

To prepare a Rust 2021 project to transition to Rust 2024, one can run:

```shell
$ cargo fix --edition
```

Rustfix will either fix the code to be compatible with Rust 2024 or print a
warning that explains the problem. This problem will have to be fixed manually.
By running the command (and possibly fixing manually some issues) until there
is no warning, one can ensure the code is compatible with both Rust 2021 and
Rust 2024.

To switch definitely to Rust 2024, one may run:

```shell
$ cargo fix --edition-idioms
```

Be advised that this tool provides few guarantees on the soundness of the
proposed fixes. In particular, some corrections (such as some of those
provided with the `--edition-idioms`) are known to break the compilation
or change the program semantics in some cases.

[rustfix]: https://github.com/rust-lang-nursery/rustfix

### Clippy

[Clippy] is a tool that provides and checks many lints (bugs, styling, performance
issues, etc.). Since version 1.29, `clippy` can be used within the stable
`rustup` environment. It is recommended to install `clippy` as a component
(`rustup component add clippy`) in the stable toolchain instead of installing it
as a project dependency.

The tool comes with some lint categories regarding the kind of issues it aims to
detect. The warnings should be re-checked by the programmer before committing
the fix that is suggested by `clippy`, especially in the case of lints of the
category `clippy::nursery` since those hints are still under development.

`clippy` now has similar `fix` tool as `rustfix`

[clippy]: https://github.com/rust-lang/rust-clippy

<div class="reco" id="DENV-LINTER" type="Rule" title="Use linter regularly">

A linter, such as `clippy`, must be used regularly during the development of
a secure application.

</div>

<div class="reco" id="DENV-AUTOFIX" type="Rule" title="Manually check automatic fixes">

In a secure Rust development, any automatic fix (for instance, provided by
`rustfix` or `clippy`) must be verified by the developer.

</div>

### Others

There exist other useful tools or `cargo` subcommands for enforcing program
security whether by searching for specific code patterns or by providing
convenient commands for testing or fuzzing. They are discussed in the following
chapters, according to their goals.
