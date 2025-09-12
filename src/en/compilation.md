# Compilation

## Hardening and Mixed Binaries

_Hardening_ refers to mechanisms applied during compilation to reduce the impact
or exploitability of certain memory safety defects. In the case of Rust, these
hardening techniques are generally less relevant (except for `unsafe` code).
However, the question arises again in the context of mixed software, that is,
software containing components written in Rust and components written in one or
more languages that do not guarantee memory safety. Indeed, it has been shown
that Rust code can be used to bypass hardening applied to vulnerable C code.

> **Rule {{#check DENV-MIXED | Enable hardening for all languages in a mixed-language application}}**
>
> When developing a secure application that includes components in multiple
> languages, the compilation of all components (including Rust ones) must apply
> hardening techniques to limit the exploitability of vulnerabilities present in
> components written in languages that do not guarantee memory safety.

### References

- _Exploiting Mixed Binaries_, Michalis Papaevripides, Elias Athanasopoulos, <https://dl.acm.org/doi/10.1145/3418898>