---
references:
  - genre: Guide
    id: anssi-risque-numerique
    issued:
      date-parts:
        - - 2019
          - 11
    note: https://cyber.gouv.fr/en/publications/controlling-digital-risk-trust-advantage
    url: https://cyber.gouv.fr/en/publications/controlling-digital-risk-trust-advantage
    number: ANSSI-PA-070-EN v1.0
    publisher: ANSSI
    title: Controlling the digital risk - The trust advantage
    type: report
---

# Introduction

[Rust](https://www.rust-lang.org) is a multi-paradigm language with a focus on
memory safety.

It aims to be system programming oriented, for instance allowing fine-grained memory
management without garbage collection and without tedious and error-prone
manual memory allocations and deallocations, or preventing data-races.

It achieves this goal by means of
its ownership system (mostly related to variable aliasing). At any point of a
Rust program, the compiler tracks how many variables refer to a given data, and
enforces a set of rules which enable automatic memory management, memory safety
and data-race free programs.

The language also focuses on performance, with powerful compilation
optimizations and language constructs that allow writing *zero-cost abstraction*
code.

Moreover, the Rust language provides some high-level programming features.
Thanks to higher-order functions, closures, iterators, etc., it allows to write
program parts in the same vein as in functional programming languages.
Besides, static typing discipline, type inference, and ad hoc polymorphism (in
the form of traits) are other ways Rust provides to build libraries and programs
in a safe manner.

Finally, the toolchain (rustup, cargo) greatly facilitate the use of Rust by simplifying
the configuration of the software construction, while giving priority to good compilation
safety practices.

Nevertheless, due to its versatility, the language possibly offers some
constructions that, if not used properly, can introduce security problems,
by making code misinterpreted by the programmer or a reviewer. In addition, as
for every tool in the compilation or software verification field, the tools used
 to develop, compile and execute programs can expose certain features or
 configurations that, if misused, may lead to vulnerabilities.

Thus, the objective of this document is to compile hints and recommendations to
stay in a safe zone for secure applications development while taking advantage
of the range of possibilities the Rust language can offer.

## Target Audience

The guide intends to group recommendations that should be applied for
application development with strong security level requirements. Anyway, it can
be followed by everyone who wants to ensure that guarantees offered by the Rust
platform are not invalidated due to unsafe, misleading or unclear feature usage.

It is not intended to be a course on how to write Rust programs, there are
already plenty of good learning resources for this purpose
(see for instance the
[Rust documentation main page](https://doc.rust-lang.org)).
The purpose is rather to guide programmers and inform them about some pitfalls
they may encounter.
These recommendations form a complement to the good level of trust
the Rust language already provides. That being said, recalls are sometimes necessary
for clarity, and the experienced Rust programmer may rely solely on
highlighted inserts (*Rule*, *Recommendation*, *Warning*, etc.).

## Contributions

This guide is written in a collaborative and open manner, via the GitHub
platform
([https://github.com/ANSSI-FR/rust-guide](https://github.com/ANSSI-FR/rust-guide)).
All contributions for future versions are welcome, whether in the form of direct
propositions (*pull requests*) or in the form of suggestions and discussions
(*issues*).

## Structure of the Document

This document considers separately different phases of a typical (and simplified)
development process.
Firstly, we provide some advice on how to take advantage of using tools of the
Rust ecosystem for secure development.
A following chapter focuses on precautions to take when choosing and using
external libraries.
Then, recommendations about the Rust language constructs are presented.
<!-- TODO: Finally, we introduce advices for writing
tests for a project in Rust, and for using Rust fuzzing tools.-->
A summary of recommendations presented throughout the document is listed at the
end of this guide.

## Reading guidelines

For each of the recommendations in this guide, the use of the verb *must* is deliberately
more prescriptive than the wording *it is recommended*.

Considering the threats identified during the drafting of
this guide, for some recommendations several solutions are proposed, which differ in the level of security they provide. This allows readers to choose the solution that offers the best protection
based on their context and security objectives.

The recommendations are therefore presented as follows:

<div class="examplereco" id="EXAMPLE-RULE" type="Rule" title="State-of-the-art recommandation">

This recommendation enables the implementation of state-of-the-art security.

</div>

<div class="examplereco" id="EXAMPLE-RECO" type="Recommandation" title="Enforced recommandation">

This recommendation supports the deployment of advanced security.
It targets entities with mature information security capabilities.

</div>

As part of an ongoing process of digital risk management and continuous improvement of
information system security [^1], the relevance of implementing the recommendations
described in this document must be periodically reassessed.

The checklist can be found <span class="gotochecklist"></span>.

[^1]: For further information, please refer to the ANSSI's digital risk management guide [@anssi-risque-numerique]
