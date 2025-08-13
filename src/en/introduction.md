# Introduction

[Rust](https://www.rust-lang.org) is a multi-paradigm language focused on
memory safety.

It is designed for system programming, allowing fine-grained memory management
without garbage collection and without tedious or error-prone manual memory
allocations and deallocations.

Rust achieves this through its ownership system (primarily related to variable
aliasing). At any point in a Rust program, the compiler tracks how many
variables refer to a given piece of data, enforcing rules that enable automatic
memory management, memory safety, and data-race-free programs.

The language also emphasizes performance, with powerful compilation
optimizations and language constructs that allow writing _zero-cost abstraction_
code.

Additionally, Rust provides several high-level programming features. Thanks to
higher-order functions, closures, iterators, etc., it allows writing parts of
programs in a style similar to functional programming languages. Furthermore,
static typing, type inference, and ad hoc polymorphism (in the form of traits)
are other ways Rust enables building libraries and programs safely.

Nevertheless, due to its versatility, the language offers some
constructs that, if not used properly, can introduce security problems,
by making code misinterpreted by the programmer or a reviewer. In addition, as
with any tool in the compilation or software verification field, tools used
to develop, compile, and execute Rust programs can expose certain features or
configurations that, if misused, may lead to vulnerabilities.

Thus, the objective of this document is to compile hints and recommendations to
stay in a safe zone for secure application development while taking advantage
of the range of possibilities the Rust language offers.

## Target Audience

This guide aims to group recommendations that should be applied for
application development with strong security requirements. However, it can
be followed by anyone who wants to ensure that the guarantees offered by the Rust
platform are not invalidated due to unsafe, misleading, or unclear feature usage.

It is not intended to be a course on how to write Rust programs; there are
already plenty of good learning resources for this purpose
(see, for instance, the
[Rust documentation main page](https://doc.rust-lang.org)).
The purpose is rather to guide programmers and inform them about some pitfalls
they may encounter.
These recommendations complement the high level of trust
the Rust language already provides. That said, reminders are sometimes necessary
for clarity, and experienced Rust programmers may rely solely on
highlighted inserts (_Rule_, _Recommendation_, _Warning_, etc.).

## Contributions

This guide is written in a collaborative and open manner, via the GitHub
platform (<https://github.com/ANSSI-FR/rust-guide>). All contributions for
future versions are welcome, whether in the form of direct proposals (_pull
requests_) or in the form of suggestions and discussions (_issues_).

## Structure of the Document

This document considers separately the different phases of a typical (and
simplified) development process. First, we provide advice on how to take
advantage of tools in the Rust ecosystem for secure development. A following
chapter focuses on precautions to take when choosing and using external
libraries. Then, recommendations about Rust language constructs are presented.
<!-- TODO: Finally, we introduce advice for writing
tests for a project in Rust, and for using Rust fuzzing tools.-->
A summary of recommendations presented throughout the document is listed at the
end of this guide.
