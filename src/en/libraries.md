# Libraries

In addition to the standard library, Rust provides an easy way to import other
libraries in a project, thanks to `cargo`. The libraries, known as *crates* in
the Rust ecosystem, are imported from the open-source components central
repository [crates.io](https://crates.io).

It should be noticed that the quality (in terms of security, performance,
readability, etc.) of the published crates is very variable. Moreover, their
maintenance can be irregular or interrupted. The usage of each component from
this repository should be justified, and developers should validate the
correct application of rules from the current guide in their code. Several tools
can aid in that task.

## Dependency Updates

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


## Cargo-audit

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
