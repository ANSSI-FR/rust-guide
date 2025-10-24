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

## LLVM Tier guarantees

LLVM classifies its supported targets into tiers to communicate how much stability and testing each backend receives.

### Tier 1 - “guaranteed to work”
The target is fully vetted by the LLVM community. It passes the entire LLVM test suite, receives regular regression testing, and is kept up‑to‑date with new LLVM releases. In practice, you can rely on consistent code generation, stable ABI, and predictable performance across LLVM versions.

### Tier 2 - “guaranteed to build”
The target’s source code compiles cleanly with LLVM, but it does not receive the same level of testing or maintenance as Tier 1. It may lack full coverage in the test suite, and certain optimizations or newer LLVM features could be missing or unstable. Users can still generate code for these backends, but they should expect occasional breakage or the need for manual patches.
### Tier 3 
Tier 3 targets are simply not officially supported.


The tier distinction helps developers choose a target that matches their risk tolerance: Tier 1 for production‑grade workloads, Tier 2 for experimental or niche architectures where full support isn’t yet supported.

## Rustc’s Relationship to LLVM

The Rust compiler (rustc) uses LLVM as its primary code‑generation backend. After the front‑end performs borrow checking, type inference, and MIR (Mid‑level IR) optimizations, it lowers the program to LLVM’s intermediate representation (IR). From there, LLVM handles the heavy lifting of instruction selection, register allocation, and target‑specific optimizations before emitting machine code.

Because rustc delegates to LLVM, the quality of the generated binaries depends heavily on the tier status of the chosen target. When you compile for a Tier 1 target (e.g., x86‑64, ARMv8), you benefit from LLVM’s extensive testing and optimization pipelines. Targeting a Tier 2 architecture works, but it is *guaranteed* to *build* not to *work*.
As a consequence: 

<div class="reco" id="LANG-ERRWRAP" type="Recommendation" title="Tier 2 targets should not be used on safety critical systems">
Tier 2 targets should not be used on safety critical systems. 
</div>

A comprehensive list of supported targets is available in the official documentation:

[Plateform support]: https://doc.rust-lang.org/stable/rustc/platform-support.html
