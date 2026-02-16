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

<div class="definition">

The behavior of a program is [*undefined*](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) when its semantics is not described in the Rust language.

</div>

the existence of UB is considered a [programming error](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.general) and must be avoided.

<div class="example">

Dereferencing the null pointer is a *UB*. On the other hand, `unwrap`ing the `None` object is well defined because it is the language that processes this error (by launching a panic).

</div>

A list of programming errors leading to UBs is [given](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
in the [Rust reference @rust-reference]. Among them, the following errors are noteworthy:

* No dereference of pointer to an unallocated or unaligned memory address (dangling pointer), which implies
  * No buffer overflow
  * No access to freed memory
  * No non-aligned access
* The pointed values are [consistent](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.invalid) with the pointer's type. For example, a value pointed at by a boolean pointer will be byte of value 1 or 0.
* Respect of [aliasing rules](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.alias) (see also the [Rustonomicon @nomicon] for [examples](https://doc.rust-lang.org/nomicon/aliasing.html)): a mutable reference cannot be shared.
* No [concurrent access](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.race) (reading/writing is not possible while simultaneously writing), to the same memory address (see also the [Rustonomicon @nomicon] for [examples](https://doc.rust-lang.org/nomicon/races.html))

## Rust guarantees

<div class="important">

The language paradigm is to ensure the absence of a UB in a program using only the non-*unsafe* part of Rust.

</div>

<div class="note">

Despite these memory safety guarantees, the language does not prevent

* resource leaks (memory, IO, ...) (see the [memory management section](unsafe/memory.md#chapter-memory)),
* numeric overflows (see the [integer operations section](integer.md#chapter-integer)).

</div>
