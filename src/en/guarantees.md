---
references:
  - type: web
    title: The Rust Reference
    url: https://doc.rust-lang.org/stable/reference/
    id: rust-reference
  - type: web
    title: The Rustonomicon
    url: https://doc.rust-lang.org/stable/nomicon/
    id: nomicon
---

# Language guarantees

## Undefined Behaviors (*UB*)

> The behavior of a program is *undefined* when its semantics is not described in the Rust language.

Considering [@rust-reference], the existence of UB is considered an [error](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.general).

For example, dereferencing the null pointer is a *UB*. On the other hand, `unwrap`ing the `None` object is well defined because it is the language that processes this error (by launching a panic).

The current list of *UBs* is given in the language [reference](https://doc.rust-lang.org/reference/behavior-considered-undefined.html). Notice the following guarantees:

* No dereference of pointer to an unallocated or unaligned memory address (dangling pointer), which implies
  * No buffer overflow
  * No access to freed memory
  * No non-aligned access
* The pointed values are [consistent](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.invalid) with the pointer's type. For example, a value pointed at by a boolean pointer will be byte of value 1 or 0.
* Respect of [aliasing rules](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.alias) (see also [@nomicon] for [examples](https://doc.rust-lang.org/nomicon/aliasing.html)): a mutable reference cannot be shared.
* No [concurrent access](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.race) (reading/writing is not possible while writing), to the same memory address (see also [@nomicon] for [examples](https://doc.rust-lang.org/nomicon/races.html))

## Rust guarantees

> The language paradigm is to ensure the absence of a UB in a program using only the non-*unsafe* part of Rust.

However, the language does not prevent

* resource leaks (memory, IO, ...),
* numeric overflows.

## Rustc Tier guarantees

As Rustc's backend is based on LLVM, it inherits and classifies its supported targets into tiers to communicate how much stability and testing each backend receives.

### Tier 1 - guaranteed to work

The target is fully vetted by the community. It passes the entire test suite, receives regular regression testing, and is kept up‑to‑date with new releases. In practice, you can rely on consistent code generation, stable ABI, and predictable performance. Tier 1 targets can be thought of as "guaranteed to work".

### Tier 2 - guaranteed to build

The target’s source code compiles cleanly, but it does not receive the same level of testing or maintenance as Tier 1. It may lack full coverage in the test suite, and certain optimizations or newer LLVM features could be missing or unstable. Users can still generate code for these backends, but they should expect occasional breakage or the need for manual patches. Tier 1 targets can be thought of as "guaranteed to build" but not as "guaranteed to work".

### Tier 3 

Tier 3 targets are simply not officially supported.



The tier distinction helps developers choose a target that matches their risk tolerance: Tier 1 for production‑grade workloads, Tier 2 for experimental or niche architectures where full support isn’t yet met.

<div class="reco" id="TEST_TOOLCHAINS" type="Rule" title="Tier 1 targets and certified toolchains should be prioritized">
Tier 1 targets and certified toolchains should be prioritized.
</div>

A comprehensive list of supported targets is available in the official documentation:

[Plateform support]: https://doc.rust-lang.org/stable/rustc/platform-support.html
