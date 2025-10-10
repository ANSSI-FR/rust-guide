---
references:
  - DOI: 10.1145/3418898
    ISSN: 2471-2566
    url: https://doi.org/10.1145/3418898
    author:
      - family: Papaevripides
        given: Michalis
      - family: Athanasopoulos
        given: Elias
    container-title: ACM Trans. Priv. Secur.
    id: mixed-bins
    issue: '2'
    issued:
      date-parts:
        - - 2021
          - 1
    keyword: CFI, Go, Memory safety, Rust, SafeStack
    publisher: Association for Computing Machinery
    publisher-place: New York, NY, USA
    title: Exploiting Mixed Binaries
    type: article-journal
    volume: '24'
---

# Compilation

## Hardening and Mixed Binaries

_Hardening_ refers to mechanisms applied during compilation to reduce the impact
or exploitability of certain memory safety defects. In the case of Rust, these
hardening techniques are generally less relevant (except for `unsafe` code).
However, the question arises again in the context of mixed software, that is,
software containing components written in Rust and components written in one or
more languages that do not guarantee memory safety. Indeed, it has been shown (see for instance [@mixed-bins])
that Rust code can be used to bypass hardening applied to vulnerable C code.

<div class="reco" id="COMP-MIXED" type="Recommendation" title="Enable hardening for all languages in a mixed-language application">

When developing a secure application that includes components in multiple
languages, the compilation of all components (including Rust ones) should apply
hardening techniques to limit the exploitability of vulnerabilities present in
components written in languages that do not guarantee memory safety.

</div>