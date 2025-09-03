# Libraries

## Dependency Repositories

The management of external libraries is integrated into the `Cargo` tool. There are several ways to specify the source of these libraries, some of which are presented below.

It is important to note that accurately tracking the versions of these libraries is a critical condition for the security of software written in Rust. This requirement is embodied in the rule [DENV-CARGO-LOCK](devenv.md#DENV-CARGO-LOCK).

### Crates

In addition to the standard library, Rust provides an easy way to import other
libraries in a project, thanks to `cargo`. The libraries, known as *crates* in
the Rust ecosystem, are imported from an open-source components central
repository.

An example of dependency declaration in the `Cargo.toml` file:

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

The default repository is [crates.io](https://crates.io). It is also possible to use [your own registry](https://doc.rust-lang.org/cargo/reference/registries.html).

### Git Dependencies

Each dependency in the `Cargo.toml` file can also refer to [a GIT repository](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories). For example:

```toml
[dependencies]
regex = { git = "https://github.com/rust-lang/regex.git" }
```

It is possible to specify the desired version in more detail by providing either a branch, a tag, or a commit hash.

The [dependency lock system](devenv.md#cargo) operates even in the case of a GIT repository: if the dependency does not specify a particular commit, the most recent commit matching the criteria in the `Cargo.toml` file is fetched during the first compilation and is recorded in the `Cargo.lock` file. All subsequent compilations will use the same commit (unless the `Cargo.lock` file is updated).

## Dependency Security

Regardless of the method used to retrieve dependencies (*crate* or GIT commit), if they come from external organizations, the dependencies must be validated.

<div class="reco" id="LIBS-VETTING-DIRECT" type="Rule" title="Validation of Direct Third-Party Dependencies">

Each direct third-party dependency must be properly validated, and each validation must be tracked.

</div>

With regard to transitive dependencies, it is also recommended to validate them individually.

<div class="reco" id="LIBS-VETTING-TRANSITIVE" type="Recommendation" title="Validation of Transitive Third-Party Dependencies">

Each third-party dependency should be properly validated, and each validation should be tracked.

</div>

## Dependency validation tools

### Dependency Updates

Updating third-party libraries is essential to ensure that any potential vulnerabilities are properly addressed.

The dependency resolution mechanism relies on the `Cargo.toml` and `Cargo.lock` files.

* The `Cargo.toml` file lists the constraints defined by the developers.
* The `Cargo.lock` file records the resolution of these constraints by Cargo *at a given point in time*.

Therefore, updating dependencies occurs at several levels.

* The `Cargo.toml` file can be updated to use a new version of a dependency.
  On the next build, the `Cargo.lock` file will be updated accordingly.
* For a given `Cargo.toml` file, dependency resolution by Cargo can change between two points in time.
  Indeed, new versions of dependencies may have been published in the meantime.
  Thus, to update dependencies while keeping the constraints from `Cargo.toml`, you can update the `Cargo.lock` file using the `cargo update` command,
  which is equivalent to deleting the `Cargo.lock` file and rebuilding it, or applying `cargo generate-lockfile`. <!-- verified by the test, but is this always the case? -->

  The `cargo update` command also allows you to update only a subset of dependencies. For example:

  ```
  cargo update serde clap
  ```


### Cargo-audit

[Cargo-audit] tool allows one to easily check for security vulnerabilities
reported to the RustSec Advisory Database.

<div class="reco" id="LIBS-AUDIT" type="Rule" title="Check for security vulnerabilities report on dependencies (cargo-audit)">

The `cargo-audit` tool must be used to check for known vulnerabilities in
dependencies.

</div>

[cargo-audit]: https://github.com/RustSec/cargo-audit

<!-- ## Unsafe code in libraries -->

<!--
<mark>TODO</mark>: `unsafe` blocks are discussed in the following chapter.
One needs to ensure that this kind of block is not misused in project
dependencies.
-->

<!--
<div class="reco" id="LIBS-UNSAFE" type="Recommendation" title="Check for unsafe code in dependencies">

<mark>TODO</mark>: check that no `unsafe` blocks appear in the imported
dependencies (with a tool?).

</div>

-->
