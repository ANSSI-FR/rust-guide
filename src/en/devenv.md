# Development Environment

## Rustup

[Rustup] is the Rust toolchain installer. Among other things, it enables
switching between different flavors of the toolchain (stable, beta, nightly),
managing the installation of additional components, and keeping them up to date.

> **Warning**
>
> From a security perspective, `rustup` performs all downloads over HTTPS, but
> does not yet validate signatures of downloads. Protection against downgrade
> attacks, certificate pinning, and validation of signatures are still works in
> progress.
>
> In some cases, it may be preferable to opt for an alternative installation
> method listed in the *Install* section of the official Rust website.

[rustup]: https://github.com/rust-lang/rustup.rs

### Rust Editions

Several flavors, called *editions*, of the Rust language coexist.

The concept of editions was introduced to clarify the implementation of new
features and to make them incremental. A new edition is produced every two or
three years, as stated in the [Edition Guide], but this doesn’t mean that new
features and improvements are only shipped in a new edition.

Some editions bring new keywords and language constructs. Recommendations for
secure application development remain closely linked to the features of the
language used in such applications, rather than to Rust editions. In the rest of
this guide, best effort will be made to highlight constructions and language
features that are specific to a particular Rust edition.

> **Note**
>
> No specific edition is recommended, as long as users follow the
> recommendations related to the features offered by the chosen edition.

[edition guide]: https://doc.rust-lang.org/edition-guide/

### Stable, Nightly, and Beta Toolchains

Orthogonally to editions, which allow one to select a flavor (a set of features)
of the Rust language, the Rust toolchain is provided in three different
versions, called *release channels*:

- *nightly* releases are created once a day,
- *beta* releases are created every six weeks, from promoted *nightly* releases,
- *stable* releases are created every six weeks, from promoted *beta* releases.

When working with different toolchains, it is important to check not only what
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

> **Rule {{#check DENV-STABLE | Use a stable compilation toolchain}}**
>
> Development of a secure application must be done using a fully stable
> toolchain, to limit potential compiler, runtime, or tool bugs.

When using a specific `cargo` subcommand that requires a nightly component, it
is preferable to run it by switching the toolchain only locally, instead of
explicitly switching the complete toolchain. For example, to run the (nightly)
latest `rustfmt`:

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
through the command line program `cargo`. Cargo is the Rust package manager. It
plays a fundamental role in most Rust development:

- It structures projects by providing the project skeleton (`cargo new`),
- It compiles the project (`cargo build`),
- It generates the project's documentation (`cargo doc`),
- It runs tests (`cargo test`) and benchmarks (`cargo bench`),
- It manages and downloads dependencies,
- It makes packages distributable and publishes them on [crates.io],
- It’s also a front-end to run complementary tools, such as those described
  below, in the form of subcommands.

> **Warning**
>
> Like `rustup`, `cargo` performs all downloads over HTTPS, but does not
> validate the registry index. Ongoing discussions are occurring on how to best
> protect and verify crates. For now, security relies on the good security of
> the website [crates.io] and the GitHub-hosted repository containing the
> registry index. In some cases, it may be preferable to opt for an alternative
> installation method for dependencies.

Cargo provides many different commands and options to adapt the build process to
your project needs, mainly through the manifest file `Cargo.toml`. For a
complete presentation, see [The Cargo Book].

During the development of a secure application, some options may require
attention. The `[profile.*]` sections allow configuring how the compiler is
invoked. For example:

- The `debug-assertions` variable controls whether debug assertions are enabled,
- The `overflow-checks` variable controls whether overflows are checked for
  integer arithmetic.

Overriding the default options may cause bugs to go undetected, even when
using the debug profile that normally enables runtime checks (for example,
[integer overflow checks](./04_language.html#integer-overflows)).

> **Rule {{#check DENV-CARGO-OPTS | Keep default values for critical variables in cargo profiles}}**
>
> The variables `debug-assertions` and `overflow-checks` must not be overridden
> in development profile sections (`[profile.dev]` and `[profile.test]`).

Cargo offers other ways to set up its configuration and change its behavior on a
given system. This can be very useful, but it may also be difficult to know and
remember at a given time all the options that are effectively used, particularly
those passed to the compiler. Ultimately, this can affect the confidence and
robustness of the build process. It is preferable to centralize compiler options
and flags in the configuration file `Cargo.toml`. For example, the environment
variable `RUSTC_WRAPPER` may be used to generate part of the code or to run
external tools before Rust compilation; it is preferable to use the Cargo build
scripts feature.

> **Rule {{#check DENV-CARGO-ENVVARS | Keep default values for compiler environment variables when running cargo}}**
>
> The environment variables `RUSTC`, `RUSTC_WRAPPER`, and `RUSTFLAGS` must not
> be overridden when using Cargo to build the project.

[crates.io]: https://crates.io
[cargo]: https://doc.rust-lang.org/stable/cargo/
[the cargo book]: https://doc.rust-lang.org/cargo/index.html

### Clippy

[Clippy] is a tool that provides and checks many lints (bugs, styling,
performance issues, etc.). Since version 1.29, `clippy` can be used within the
stable `rustup` environment. It is recommended to install `clippy` as a
component (`rustup component add clippy`) in the stable toolchain instead of
installing it as a project dependency.

The tool comes with several lint categories regarding the kind of issues it aims
to detect. The warnings should be re-checked by the programmer before committing
the fix suggested by `clippy`, especially in the case of lints in the category
`clippy::nursery`, since those hints are still under development.

> **Rule {{#check DENV-LINTER | Use linter regularly}}**
>
> A linter, such as `clippy`, must be used regularly during the development of
> a secure application.

[clippy]: https://github.com/rust-lang/rust-clippy

### Rustfmt

[Rustfmt] is a tool that formats your code according to style guidelines. The
documentation of the tool states some limitations, among others partial support
for macro declarations and uses. One should use the `--check` option, which
prints proposed changes, review these changes, and finally apply them if the
code readability is not affected.

So, to launch it:

```shell
cargo fmt -- --check
# review the changes
cargo fmt
```

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
at the [Rust Style Guide](https://doc.rust-lang.org/style-guide/index.html).

> **Rule {{#check DENV-FORMAT | Use Rust formatter (rustfmt)}}**
>
> The tool `rustfmt` can be used to ensure that the codebase respects style
> guidelines (as described in the `rustfmt.toml` file), with the `--check`
> option and manual review.

[rustfmt]: https://github.com/rust-lang/rustfmt

### Rustfix

Included with Rust since the end of 2018, [Rustfix] is a tool dedicated to
fixing compiler warnings as well as easing transitions between editions.

```shell
cargo fix
```

To prepare a Rust 2015 project to transition to Rust 2018, one can run:

```shell
cargo fix --edition
```

Rustfix will either fix the code to be compatible with Rust 2018 or print a
warning that explains the problem. This problem will have to be fixed manually.
By running the command (and possibly fixing some issues manually) until there
is no warning, one can ensure the code is compatible with both Rust 2015 and
Rust 2018.

To switch definitely to Rust 2018, one may run:

```shell
cargo fix --edition-idioms
```

Be advised that this tool provides few guarantees on the soundness of the
proposed fixes. In particular, some corrections (such as those provided with
the `--edition-idioms` option) are known to break compilation
or change program semantics in some cases.

> **Rule {{#check DENV-AUTOFIX | Manually check automatic fixes}}**
>
> In secure Rust development, any automatic fix (for instance, provided by
> `rustfix`) must be verified by the developer.

[rustfix]: https://github.com/rust-lang-nursery/rustfix

### Others

There exist other useful tools or `cargo` subcommands for enforcing program
security, whether by searching for specific code patterns or by providing
convenient commands for testing or fuzzing. They are discussed in the following
chapters, according to their goals.
