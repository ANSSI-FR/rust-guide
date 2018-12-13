# Libraries

In addition to a standard library, Rust provides an easy way to import libraries
in a project, thanks to `cargo`. The libraries, known as *crates* in the Rust
ecosystem, are imported from the open-source components central repository
[crates.io](https://crates.io).

It should be noticed that the quality (in terms of security, performances,
readability, etc.) of the published crates is very variable. Moreover, their
maintenance can be irregular or interrupted. The usage of each component from
this repository should be justified, and the developer should validate the
correct application of rules from the current guide in its code. Several tools
can aid in that task.

## cargo-outdated

Cargo-outdated tool allows one to easily manage dependencies versions.

For a given crate, it lists current dependencies versions (using its
`Cargo.toml`), and checks latest compatible version and also latest general
version.

> ### Recommendation:
> <mark>TODO</mark>: run cargo-outdated to check dependencies status

## Unsafe code in libraries

<mark>TODO</mark>: `unsafe` blocks are discussed in the following chapter.
One needs to ensure that this kind of block is not misused in project
dependencies.

> ### Recommendation:
> <mark>TODO</mark>: check that no `unsafe` blocks appear in the imported
> dependencies (with a tool?).
