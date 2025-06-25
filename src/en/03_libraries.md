# Libraries

In addition to the standard library, Rust provides an easy way to import other
libraries in a project, thanks to `cargo`. The libraries, known as *crates* in
the Rust ecosystem, are imported from the open-source components central
repository [crates.io](https://crates.io).

It should be noticed that the quality (in terms of security, performances,
readability, etc.) of the published crates is very variable. Moreover, their
maintenance can be irregular or interrupted. The usage of each component from
this repository should be justified, and the developer should validate the
correct application of rules from the current guide in its code. Several tools
can aid in that task.

## Cargo-outdated

[Cargo-outdated] tool allows one to easily manage dependencies versions.

For a given crate, it lists current dependencies versions (using its
`Cargo.toml`), and checks latest compatible version and also latest general
version.

> ### Rule {{#check LIBS-OUTDATED | Check for outdated dependencies versions (cargo-outdated)}}
> The `cargo-outdated` tool must be used to check dependencies status. Then,
> each outdated dependency must be updated or the choice of the version must be
> justified.

[cargo-outdated]: https://github.com/kbknapp/cargo-outdated

## Cargo-audit

[Cargo-audit] tool allows one to easily check for security vulnerabilities
reported to the RustSec Advisory Database.

> ### Rule {{#check LIBS-AUDIT | Check for security vulnerabilities report on dependencies (cargo-audit)}}
> The `cargo-audit` tool must be used to check for known vulnerabilities in
> dependencies.

[cargo-audit]: https://github.com/RustSec/cargo-audit