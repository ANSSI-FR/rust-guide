# Central traits

## `Drop` trait, the destructor

Types implement the trait `std::ops::Drop` to perform some operations when the
memory associated with a value of this type is to be reclaimed. `Drop` is the
Rust equivalent of a destructor in C++ or a finalizer in Java.

Dropping is done recursively from the outer value to the inner values.
When a value goes out of scope (or is explicitly dropped with `std::mem::drop`),
the value is dropped in two steps. The first step happens only if the type of
this value implements `Drop`. It consists in calling the `drop` method on it.
The second step consists in repeating the dropping process recursively on any
field the value contains. Note that a `Drop` implementation is
**only responsible for the outer value**.

First and foremost, implementing `Drop` should not be systematic.
It is only needed if the type requires some destructor logic. In fact, `Drop` is
typically used to release external resources (network connections, files, etc.)
or to release memory (e.g. in smart pointers such as `Box` or `Rc`).
As a result, `Drop` trait implementations are likely to contain `unsafe` code
blocks as well as other security-critical operations.

<div class="reco" id="LANG-DROP" type="Rule" title="Justify `Drop` implementation">

In a Rust secure development, the implementation of the `std::ops::Drop` trait
MUST be justified and documented.

</div>

Second, Rust type system only ensures memory safety and, from the type system's
standpoint, missing drops is allowed. In fact, several things may lead to
missing drops, such as:

- a reference cycle (for instance, with `Rc` or `Arc`),
- an explicit call to `std::mem::forget` (or `core::mem::forget`) (see paragraph
  on [`forget` and memory leaks](unsafe/memory.md#forget-and-memory-leaks)),
- a panic during drop,
- program aborts (and panics when abort-on-panic is on).


And missing drops may lead to exposing sensitive data or to lock limited
resources leading to availability issues.

<div class="reco" id="LANG-DROP-NO-PANIC" type="Rule" title="Do not panic in `Drop` implementation">

In a Rust secure development, the implementation of the `std::ops::Drop` trait
MUST not panic.

</div>

Beside panics, secure-critical drop should be protected.

<div class="reco" id="LANG-DROP-NO-CYCLE" type="Rule" title="Do not allow cycles of reference-counted `Drop`">

A value whose type implements `Drop` MUST NOT be embedded directly or indirectly
in a cycle of reference-counted references.

</div>

<div class="reco" id="LANG-DROP-SEC" type="Rule" title="Do not rely only on `Drop` to ensure security">

Ensuring security operations at the end of some treatment (such as key erasure
at the end of a cryptographic encryption) MUST NOT rely only on the `Drop`
trait implementation.

</div>

