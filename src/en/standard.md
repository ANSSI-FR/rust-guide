# Standard library

## `Send` and `Sync` traits

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

The negative implementation of `Send` or `Sync` are also used in the Rust
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

<div class="reco" id="LANG-SYNC-TRAITS" type="Recommendation" title="Justify `Send` and `Sync` implementation">

In a Rust secure development, the manual implementation of the `Send` and
`Sync` traits should be avoided and, if necessary, should be justified,
documented and peer-reviewed.

</div>

## Comparison traits (`PartialEq`, `Eq`, `PartialOrd`, `Ord`)

Comparisons (`==`, `!=`, `<`, `<=`, `>`, `>=`) in Rust rely on four standard
traits available in `std::cmp` (or `core::cmp` for `no_std` compilation):

- `PartialEq<Rhs>` that defines a partial equivalence between
  objects of types `Self` and `Rhs`,
- `PartialOrd<Rhs>` that defines a partial order between objects of types
  `Self` and `Rhs`,
- `Eq` that defines a total equivalence between objects of the same
  type. It is only a marker trait that requires `PartialEq<Self>`!
- `Ord` that defines a total order between objects of the same type.
  It requires that `PartialOrd<Self>` is implemented.

As documented in the standard library, Rust assumes **many invariants**
about each implementation of those traits:

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

The compiler does not check any of those requirements except for the type checking
itself. However, comparisons are critical because they intervene both in
liveness critical systems such as schedulers and load balancers, and in
optimized algorithms that may use `unsafe` blocks.
In the first use, a bad ordering may lead to availability issues such as
deadlocks.
In the second use, it may lead to classical security issues linked to memory
safety violations. That is again a factor in the practice of limiting the use
of `unsafe` blocks.

<div class="reco" id="LANG-CMP-INV" type="Rule" title="Respect the invariants of standard comparison traits">

In a Rust secure development, the implementation of standard comparison traits
must respect the invariants described in the documentation.

</div>

<div class="reco" id="LANG-CMP-DEFAULTS" type="Recommendation" title="Use the default method implementation of standard comparison traits">

In a Rust secure development, the implementation of standard comparison traits
should only define methods with no default implementation, so as to reduce
the risk of violating the invariants associated with the traits.

</div>

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

<div class="warning">

Derivation of comparison traits for compound types depends on the
**field order**, and not on their names.

First, it means that changing the order of declaration of two fields changes
the resulting lexicographical order. For instance, provided this second
ordered type:

```rust,noplaypen
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct T2{
   b: u8, a: u8
};
```

we have `T1 {a: 1, b: 0} > T1 {a: 0, b: 1}` but
`T2 {a: 1, b: 0} < T2 {a: 0, b: 1}`.

Second, if one of the underlying comparisons panics, the order may change the
result due to the use of short-circuit logic in the automatic implementation.

For enums, the derived comparisons depend first on the **variant order**, then
on the field order.

</div>

Despite the ordering caveat, derived comparisons are a lot less error-prone
than manual ones and make the code shorter and easier to maintain.

<div class="reco" id="LANG-CMP-DERIVE" type="Recommendation" title="Derive comparison traits when possible">

In a secure Rust development, the implementation of standard comparison traits
should be automatically derived with `#[derive(...)]` when structural equality
and lexicographical comparison is needed. Any manual implementation of
standard comparison traits should be documented and justified.

</div>
