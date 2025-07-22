# Language guarantees

## Undefined Behaviors (*UB*)

> The behavior of a program is *undefined* when its semantics is not described in the Rust language.

The existence of UB is considered an [error](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.general).

For example, dereferencing the null pointer is a *UB*. On the other hand, `unwrap`ing the `None` object is well defined because it is the language that processes this error (by launching a panic).

The current list of *UBs* is given in the language [reference](https://doc.rust-lang.org/reference/behavior-considered-undefined.html). Notice the following guarantees:

* No dereference of pointer to an unallocated or unaligned memory address (dangling pointer), which implies
  * No buffer overflow
  * No access to freed memory
  * No non-aligned access
* The pointed values are [consistent](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.invalid) with the pointer's type. For example, a value pointed at by a boolean pointer will be byte of value 1 or 0.
* Respect of [aliasing rules](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.alias) (see also [nomicon](https://doc.rust-lang.org/nomicon/aliasing.html)): a mutable reference cannot be shared.
* No concurrent access (reading/writing is not possible while writing) to the same memory address ([data race](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.race), see also [nomicon](https://doc.rust-lang.org/nomicon/races.html))

## Rust guarantees

> The language paradigm is to ensure the absence of a UB in a program using only the non-*unsafe* part of Rust.

However, the language does not prevent

* resource leaks (memory, IO, ...),
* numeric overflows.

## References

* https://doc.rust-lang.org/reference/unsafety.html
* https://doc.rust-lang.org/nomicon/what-unsafe-does.html
