# Type system

<!-- ## About Rust type system -->

<!--
<mark>TODO</mark>: identify pitfalls with the type system (for instance,
misunderstanding of which code is actually executed when implementing complex
patterns with traits).
-->

## Standard library traits

### `Drop` trait, the destructor

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

> ### Recommendation {{#check LANG-DROP | Justify `Drop` implementation}}
>
> In a Rust secure development, the implementation of the `std::ops::Drop` trait
> should be justified, documented and peer-reviewed.

Second, Rust type system only ensures memory safety and, from the type system's
standpoint, missing drops is allowed. In fact, several things may lead to
missing drops, such as:

- a reference cycle (for instance, with `Rc` or `Arc`),
- an explicit call to `std::mem::forget` (or `core::mem::forget`) (see paragraph
  on [Forget and memory leaks](05_memory.html#forget-and-memory-leaks),
- a panic in drop,
- program aborts (and panics when abort-on-panic is on).


And missing drops may lead to exposing sensitive data or to lock limited
resources leading to unavailability issues.

> ### Rule {{#check LANG-DROP-NO-PANIC | Do not panic in `Drop` implementation}}
>
> In a Rust secure development, the implementation of the `std::ops::Drop` trait
> must not panic.

Beside panics, secure-critical drop should be protected.

> ### Rule {{#check LANG-DROP-NO-CYCLE | Do not allow cycles of reference-counted `Drop`}}
>
> Value whose type implements `Drop` must not be embedded directly or indirectly
> in a cycle of reference-counted references.

> ### Recommendation {{#check LANG-DROP-SEC | Do not rely only on `Drop` to ensure security}}
>
> Ensuring security operations at the end of some treatment (such as key erasure
> at the end of a cryptographic encryption) should not rely only on the `Drop`
> trait implementation.

### `Send` and `Sync` traits

The `Send` and `Sync` traits (defined in `std::marker` or `core::marker`) are
marker traits used to ensure the safety of concurrency in Rust. When implemented
correctly, they allow the Rust compiler to guarantee the absence of data races.
Their semantics is as follows:

- A type is `Send` if it is safe to send (move) it to another thread.
- A type is `Sync` if it is safe to share a immutable reference to it with
  another thread.

Both traits are _unsafe traits_, i.e., the Rust compiler does not verify in any
way that they are implemented correctly. The danger is real: an incorrect
implementation may lead to **undefined behavior**.

Fortunately, in most cases, one does not need to implement it. In Rust,
almost all primitive types are `Send` and `Sync`, and for most compound types
the implementation is automatically provided by the Rust compiler.
Notable exceptions are:

- Raw pointers are neither `Send` nor `Sync` because they offer no safety
  guards.
- `UnsafeCell` is not `Sync` (and as a result `Cell` and `RefCell` aren't
  either) because they offer interior mutability (mutably shared value).
- `Rc` is neither `Send` nor `Sync` because the reference counter is shared and
  unsynchronized.

Automatic implementation of `Send` (resp. `Sync`) occurs for a compound type
(structure or enumeration) when all fields have `Send` types (resp. `Sync`
types). Using an unstable feature (as of Rust 1.37.0), one can block the
automatic implementation of those traits with a manual
_negative implementation_:

```rust,ignore,noplaypen
#![feature(option_builtin_traits)]

struct SpecialType(u8);
impl !Send for SpecialType {}
impl !Sync for SpecialType {}
```

The negative implementation of `Send` or `Sync` are also used in the standard
library for the exceptions, and are automatically implemented when appropriate.
As a result, the generated documentation is always explicit: a type implements
either `Send` or `!Send` (resp. `Sync` or `!Sync`).

As a stable alternative to negative implementation, one can use a `PhantomData`
field:

```rust,noplaypen
# use std::marker::PhantomData;
#
struct SpecialType(u8, PhantomData<*const ()>);
```

> ### Recommendation {{#check LANG-SYNC-TRAITS | Justify `Send` and `Sync` implementation}}
>
> In a Rust secure development, the manual implementation of the `Send` and
> `Sync` traits should be avoided and, if necessary, should be justified,
> documented and peer-reviewed.

### Comparison traits (`PartialEq`, `Eq`, `PartialOrd`, `Ord`)

Comparisons (`==`, `!=`, `<`, `<=`, `>`, `>=`) in Rust relies on four standard
traits available in `std::cmp` (or `core::cmp` for `no_std` compilation):

- `PartialEq<Rhs>` that defines a partial equivalence between
  objects of types `Self` and `Rhs`,
- `PartialOrd<Rhs>` that defines a partial order between objects of types
  `Self` and `Rhs`,
- `Eq` that defines a total equivalence between objects of the same
  type. It is only a marker trait that requires `PartialEq<Self>`!
- `Ord` that defines the total order between objects of the same type.
  It requires that `PartialOrd<Self>` is implemented.

As documented in the standard library, Rust assumes **a lot of invariants**
about the implementations of those traits:

- For `PartialEq`

  - *Internal consistency*: `a.ne(b)` is equivalent to `!a.eq(b)`, i.e., `ne` is
    the strict inverse of `eq`. The default implementation of `ne` is precisely
    that.

  - *Symmetry*: `a.eq(b)` and `b.eq(a)`, are equivalent. From the developer's
    point of view, it means:

    - `PartialEq<B>` is implemented for type `A` (noted `A: PartialEq<B>`),
    - `PartialEq<A>` is implemented for type `B` (noted `B: PartialEq<A>`),
    - both implementations are consistent with each other.

  - *Transitivity*: `a.eq(b)` and `b.eq(c)` implies `a.eq(c)`. It means that:

    - `A: PartialEq<B>`,
    - `B: PartialEq<C>`,
    - `A: PartialEq<C>`,
    - the three implementations are consistent with each other (and their
      symmetric implementations).

- For `Eq`

  - `PartialEq<Self>` is implemented.

  - *Reflexivity*: `a.eq(a)`. This stands for `PartialEq<Self>` (`Eq` does not
    provide any method).

- For `PartialOrd`

  - *Equality consistency*:
    `a.eq(b)` is equivalent to `a.partial_cmp(b) == Some(std::ordering::Eq)`.

  - *Internal consistency*:

    - `a.lt(b)` iff `a.partial_cmp(b) == Some(std::ordering::Less)`,
    - `a.gt(b)` iff `a.partial_cmp(b) == Some(std::ordering::Greater)`,
    - `a.le(b)` iff `a.lt(b) || a.eq(b)`,
    - `a.ge(b)` iff `a.gt(b) || a.eq(b)`.

    Note that by only defining `partial_cmp`, the internal consistency is
    guaranteed by the default implementation of `lt`, `le`, `gt`, and `ge`.

  - *Antisymmetry*: `a.lt(b)` (respectively `a.gt(b)`) implies `b.gt(a)`
    (respectively, `b.lt(b)`). From the developer's standpoint, it also means:

    - `A: PartialOrd<B>`,
    - `B: PartialOrd<A>`,
    - both implementations are consistent with each other.

  - *Transitivity*: `a.lt(b)` and `b.lt(c)` implies `a.lt(c)` (also with `gt`,
    `le` and `ge`). It also means:

    - `A: PartialOrd<B>`,
    - `B: PartialOrd<C>`,
    - `A: PartialOrd<C>`,
    - the implementations are consistent with each other (and their symmetric).

- For `Ord`

  - `PartialOrd<Self>`

  - *Totality*: `a.partial_cmp(b) != None` always. In other words,
    exactly one of `a.eq(b)`, `a.lt(b)`, `a.gt(b)` is true.

  - *Consistency with `PartialOrd<Self>`*: `Some(a.cmp(b)) == a.partial_cmp(b)`.

The compiler do not check any of those requirements except for the type checking
itself. However, comparisons are critical because they intervene both in
liveness critical systems such as schedulers and load balancers, and in
optimized algorithms that may use `unsafe` blocks.
In the first use, a bad ordering may lead to availability issues such as
deadlocks.
In the second use, it may lead to classical security issues linked to memory
safety violations. That is again a factor in the practice of limiting the use
of `unsafe` blocks.

> ### Rule {{#check LANG-CMP-INV | Respect the invariants of standard comparison traits}}
>
> In a Rust secure development, the implementation of standard comparison traits
> must respect the invariants described in the documentation.

> ### Recommendation {{#check LANG-CMP-DEFAULTS | Use the default method implementation of standard comparison traits}}
>
> In a Rust secure development, the implementation of standard comparison traits
> should only define methods with no default implementation, so as to reduce
> the risk of violating the invariants associated with the traits.

There is a Clippy lint to check that `PartialEq::ne` is not defined in
`PartialEq` implementations.

Rust comes with a standard way to automatically construct implementations of the
comparison traits through the `#[derive(...)]` attribute:

- Derivation `PartialEq` implements `PartialEq<Self>` with a
  **structural equality** providing that each subtype is `PartialEq<Self>`.
- Derivation `Eq` implements the `Eq` marker trait providing that each subtype
  is `Eq`.
- Derivation `PartialOrd` implements `PartialOrd<Self>` as a
  **lexicographical order** providing that each subtype is `PartialOrd`.
- Derivation `Ord` implements `Ord` as a **lexicographical order**
  providing that each subtype is `Ord`.

For instance, the short following code shows how to compare two `T1`s easily.
All the assertions hold.

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct T1 {
    a: u8, b: u8
}

# fn main() {
assert!(&T1 { a: 0, b: 0 } == Box::new(T1 { a: 0, b: 0 }).as_ref());
assert!(T1 { a: 1, b: 0 } > T1 { a: 0, b: 0 });
assert!(T1 { a: 1, b: 1 } > T1 { a: 1, b: 0 });
# println!("all tests passed.");
# }
```

> ### Warning
>
> Derivation of comparison traits for compound types depends on the
> **field order**, and not on their names.
>
> First, it means that changing the order of declaration of two fields change
> the resulting lexicographical order. For instance, provided this second
> ordered type:
>
> ```rust,noplaypen
> #[derive(PartialEq, Eq, PartialOrd, Ord)]
> struct T2{
>    b: u8, a: u8
> };
> ```
>
> we have `T1 {a: 1, b: 0} > T1 {a: 0, b: 1}` but
> `T2 {a: 1, b: 0} < T2 {a: 0, b: 1}`.
>
> Second, if one of the underlying comparison panics, the order may change the
> result due to the use of short-circuit logic in the automatic implementation.
>
> For enums, the derived comparisons depends first on the **variant order** then
> on the field order.

Despite the ordering caveat, derived comparisons are a lot less error-prone
than manual ones and makes code shorter and easier to maintain.

> ### Recommendation {{#check LANG-CMP-DERIVE | Derive comparison traits when possible}}
>
> In a secure Rust development, the implementation of standard comparison traits
> should be automatically derived with `#[derive(...)]` when structural equality
> and lexicographical comparison is needed. Any manual implementation of
> standard comparison traits should be documented and justified.
