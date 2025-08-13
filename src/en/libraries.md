# Libraries

In addition to the standard library, Rust provides an easy way to import other
libraries into a project, thanks to `cargo`. The libraries, known as *crates* in
the Rust ecosystem, are imported from the open-source central repository
[crates.io](https://crates.io).

It should be noted that the quality (in terms of security, performance,
readability, etc.) of published crates is highly variable. Moreover, their
maintenance can be irregular or discontinued. The use of each component from
this repository should be justified, and the developer should validate the
correct application of rules from this guide in its code. Several tools can
assist with that task.

## Cargo-outdated

The [cargo-outdated] tool allows one to easily manage dependency versions.

For a given crate, it lists the current dependency versions (using its
`Cargo.toml`), checks the latest compatible version, and also the latest general
version.

> **Rule {{#check LIBS-OUTDATED | Check for outdated dependencies (cargo-outdated)}}**
>
> The `cargo-outdated` tool must be used to check the status of dependencies.
> Each outdated dependency must be updated, or the choice of version must be
> justified.

[cargo-outdated]: https://github.com/kbknapp/cargo-outdated

## Cargo-audit

The [cargo-audit] tool allows one to easily check for security vulnerabilities
reported in the RustSec Advisory Database.

> **Rule {{#check LIBS-AUDIT | Check for security vulnerability reports on dependencies (cargo-audit)}}**
>
> The `cargo-audit` tool must be used to check for known vulnerabilities in
> dependencies.

[cargo-audit]: https://github.com/RustSec/cargo-audit

<!-- ## Unsafe code in libraries -->

<!--
<mark>TODO</mark>: `unsafe` blocks are discussed in the following chapter.
One needs to ensure that this kind of block is not misused in project
dependencies.
-->

<!--
> **Recommendation {{#check LIBS-UNSAFE | Check for unsafe code in dependencies}}**
>
> <mark>TODO</mark>: Check that no `unsafe` blocks appear in the imported
> dependencies (with a tool?).
-->
