# Introduction

[Rust](https://www.rust-lang.org) is a multi-paradigm language with a focus on
memory safety.

It aims to be system programming oriented, allowing fine-grained memory
management without garbage collection but also without tedious and error-prone
manual memory allocations and deallocations. It achieves this goal by means of
its ownership system (mostly related to variable aliasing). At any point of a
Rust program, the compiler tracks how many variables refer to a given data, and
enforces a set of rules which enable automatic memory management, memory safety
and data-race free programs.

The language also focuses on performance, with powerful compilation
optimizations and language constructs that allow writing zero-cost abstraction
code.

Moreover, the Rust language provides some high-level programming features.
Thanks to higher-order functions, closures, iterators, etc., it allows to write
program parts in the same vein as in functional programming languages.
Besides, static typing discipline, type inference, and ad hoc polymorphism (in
the form of traits) are other ways Rust provides to build libraries and programs
in a safe manner.

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

The guide intents to group recommendations that should be applied for
application development with strong security level requirements. Anyway, it can
be followed by everyone who wants to ensure that guarantees offered by the Rust
platform are not invalidated due to unsafe, misleading or unclear feature usage.

It is not intended to be a course on how to write Rust programs, there are
already plenty of good learning resources for this purpose
(see for instance the
[Rust documentation main page](https://doc.rust-lang.org)).
The purpose is rather to guide the programmer and inform them about some pitfalls
they may encounter.
These recommendations form a complement to the good level of trust
the Rust language already provides. That said, recalls are sometimes necessary
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
Firstly, we provide some advices on how to take advantage of using tools of the
Rust ecosystem for secure development.
A following chapter focuses on precautions to take when choosing and using
external libraries.
Then, recommendations about the Rust language constructs are exposed.
<!-- TODO: Finally, we introduce advices for writing
tests for a project in Rust, and for using Rust fuzzing tools.-->
A summary of recommendations presented throughout the document is listed at the
end of this guide.
