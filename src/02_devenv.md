# Development Environment

## `rustup`

`rustup` is the Rust toolchain installer. Among other things, it enables
switching between different flavors of the toolchain (stable, beta, nightly),
managing additional components installation and keeping them up to date.

> ### Warning:
> From a security perspective, `rustup` does perform all downloads over HTTPS,
> but doesn’t validate signatures of downloads. Protection against downgrade
> attacks, certificate pinning, validation of signatures are works that are
> currently in progress. In some cases, it may be preferable to opt for
> an alternative installation method listed in the *Install* section of the
> official rust website.

### Rust Editions

Several flavors, called *editions*, of the Rust language coexist.
The concept of editions has been introduced to clarify new features
implementation and to make them incremental. But as stated in the
[Edition Guide](https://rust-lang-nursery.github.io/edition-guide/editions/index.html),
this doesn’t mean that new features and improvements will be shipped on
the last edition only.

However, some editions could bring new keywords and language constructs.
Recommendations for secure applications development then remain closely
linked to features that are used in such applications rather than the actual
edition that is declared in it.
In the rest of this guide, best effort will be made to highlight constructions
and language features that are specific to a particular Rust edition.

> ### Note:
> No specific edition is recommended, as long as users follow recommendations
> that are expressed in relation to features offered by edition that has been
> chosen.

### Stable, nightly and beta toolchains

Orthogonally to editions that allows one to select a flavor (a set of features)
of the Rust language, the Rust toolchain is provided in three different
versions, called *release channels*:

- *nightly* releases are created once a day,
- *nightly* releases are promoted every six weeks to *beta* releases,
- *beta* releases are promoted every six weeks to *stable* releases.

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
/tmp/foo                                	nightly-x86_64-unknown-linux-gnu
$
```

> ### Rule {{#check DENV-STABLE | Use a stable compilation toolchain}}:
> Development of a secure application must be done using a fully stable
> toolchain, for limiting potential compiler, runtime or tool bugs.

When using a specific cargo subcommand that requires a nightly component,
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

## `cargo`

Once `rustup` has been used to set up the appropriate Rust toolchain, the
tool `cargo` has been made available. It’s the Rust package manager, that
provides ways to structure and build projects, managing on its own dependencies
download among other tasks. It’s also a front-end to run complementary tools such
as those that are described below, in the form of sub-commands.

<mark>TODO</mark>: identify unsafe features and risky environment variables.

### clippy

Clippy is a tool that provides and checks many lints (bugs, styling, performance
issues, etc.). Since the stable toolchain has reached version 1.29, `clippy` can
be used within the stable rustup environment. It is also recommended
to install `clippy` as a component (`rustup component add clippy`) in the
stable toolchain instead of installing it as a project dependency.

The tool comes with some lint categories regarding the kind of issue it aims to
detect. The warnings should be re-checked by the programmer before committing
the fix that is suggested by `clippy`, especially in the case of lints of the
category `clippy::nursery` since those hints are still under development.

> ### Rule {{#check DENV-LINTER | Use Rust linter (cargo-clippy)}}:
> The tool `clippy` must be used at various times during a secure application
> development process.

### rustfmt

`rustfmt` is a tool that formats your code according to style guidelines.

To launch it:

```shell
$> cargo fmt
```

These guidelines can be customized to your needs by creating a `rustfmt.toml` or `.rustfmt.toml` file at the root of your project. It will be used to override the default settings, for instance:

```toml
# Set the maximum line width to 120
max_width = 120
# Maximum line length for single line if-else expressions
single_line_if_else_max_width = 40
```


For more information about the guidelines that `rustfmt` will check, have a look at the [Rust Style Guide](https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md).


### Others

There exist other useful tools or cargo subcommands for enforcing program
security whether by searching for specific code patterns or by providing
convenient commands for testing or fuzzing. They are discussed in the following
chapters, according to their goals.
