# Language Guarantees

## Undefined Behavior (*UB*)

> The behavior of a program is *undefined* when its semantics is not described
> by the Rust language.

The existence of UB is considered an [error][reference ub].

For example, dereferencing a null pointer is *UB*. On the other hand,
`unwrap`ping the `None` value is well defined because the language handles this
error (most often by triggering a panic).

The current list of *UBs* is given in the language [reference]. Note the
following guarantees:

- No dereferencing of pointers to unallocated or unaligned memory addresses
  (dangling pointers), which implies:
  - No buffer overflows
  - No access to freed memory
  - No unaligned access
- [Consistency] between pointer types and pointed-to values (on read or write). For example, a
  value pointed to by a Boolean pointer will be a byte with value 1 or 0.
- Respect for [aliasing rules][reference aliasing]
  (see also [nomicon][nomicon aliasing]): a
  mutable reference cannot be shared.
- No concurrent access (reading/writing is not possible while writing) to the
  same memory address ([data race][reference data race],
  see also [nomicon][nomicon data race]).

## Rust Guarantees

> The language paradigm is to ensure the absence of UB in a program using only
> the non-*unsafe* parts of Rust.

However, **the language does not prevent**:

- resource leaks (memory, IO, etc.),
- numeric overflows.

## References

- <https://doc.rust-lang.org/reference/unsafety.html>
- <https://doc.rust-lang.org/nomicon/what-unsafe-does.html>

[consistency]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.invalid
[nomicon aliasing]: https://doc.rust-lang.org/nomicon/aliasing.html
[nomicon data race]: https://doc.rust-lang.org/nomicon/races.html
[reference]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[reference aliasing]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.alias
[reference data race]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.race
[reference ub]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html#r-undefined.general
