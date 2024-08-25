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

## Checking the supply chain

Through its security working group, Rust offers a number of tools for checking the security of a program's supply-chain at library level.

### Cargo supply-chain

[Cargo-supply-chain] is the tool developed by the rust foundation's official working group, which collects all the people who can work on the libraries used by the project.

> ### Rule {{#check LIBS-SUPPLY-CHAIN | Check developers implicitly trusted}}
>
> The `cargo-supply-chain` tool must be used to find out who your organisation implicitly trust to run your project.

[cargo-supply-chain]: https://github.com/rust-secure-code/cargo-supply-chain
### Cargo vet / crev

[Cargo-vet] is a tool developed by the Mozilla Foundation that allows you to check whether the libraries you can use have been audited by trusted third parties.

> Rule {{#check LIBS-VET | Priority use of libraries that have been audited}}
>
> It is advisable to use the `cargo-vet` tool to prioritise the use of libraries which have been audited by third parties.

Security audits can be created using a tool called [Cargo-crev]. The use of this tool will not be detailed in this guide.

For more information, please consult the tool's [official documentation].

> ### Advises
>
> We recommend that you carry out security audits using the `cargo-crev` tool in order to check the security of the 
> of the libraries used in your project and to share them with the community.

[cargo-vet]: https://github.com/mozilla/cargo-vet
[cargo-crev]: https://github.com/crev-dev/cargo-crev
[official documentation]: https://github.com/crev-dev/cargo-crev/blob/main/cargo-crev/src/doc/getting_started.md

## Unsafe code in libraries

[Cargo-geiger] is a tool maintained by the Rust security working group.
Its aim is to detect the use of the `unsafe` block in a project's supply chain.

The results have three levels: 
1) 🔒  = No `unsafe` usage found, declares #![forbid(unsafe_code)]
2) ❓  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
3) ☢️   = `unsafe` usage found

> ### Rule {{#check LIBS-UNSAFE | Check *unsafe* code in dependencies}}
>
> Use the `cargo-geiger` tool to check that uses of the `unsafe` block comply with the recommendations described in the following section of this guide.


[cargo-geiger]: https://github.com/geiger-rs/cargo-geiger