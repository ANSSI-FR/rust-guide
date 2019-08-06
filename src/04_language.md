# The Rust language

## Generalities

### Naming

As of now, the standard library is the de facto standard for naming things in
the Rust world. However, an effort has been made to formalize it, first in
[RFC 430], then in the [Rust API Guidelines].

The basic rule consists in using :

- `UpperCamelCase` for types, traits, enum variants,
- `snake_case` for functions, methods, macros, variables and modules,
- `SCREAMING_SNAKE_CASE` for statics and constants,
- `'lowercase` for lifetimes.

The [Rust API Guidelines] also prescribes more precise naming conventions for
some particular constructions:

- (C-CONV) for conversion methods (`as_`, `to_`, `into_`),
- (C-GETTER) for getters,
- (C-ITER) for iterator-producing methods,
- (C-ITER-TY) for iterator types,
- (C-FEATURE) for feature naming,
- (C-WORD-ORDER) for word order consistency.

> ### Rule {{#check LANG-NAMING | Respect naming conventions}}
>
> Development of a secure application must follow the naming conventions
> outlined in the [Rust API Guidelines].

[rfc 430]: https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md
[rust api guidelines]: https://rust-lang-nursery.github.io/api-guidelines/

### Memory management

<mark>TODO</mark>: explain safe allocations/deallocations, ownership/borrowing,
and identify language constructs that may break memory safety (for instance,
unsound behaviors in older versions of the compiler).

#### Uninitialized memory

By default, Rust forces all values to be initialized, preventing the use of
uninitialized memory (except if using `std::mem::uninitialized` or
`std::mem::MaybeUninit`).

> ### Rule {{#check MEM-UNINIT | Do not use uninitialized memory }}
>
> The `std::mem::uninitialized` function (deprecated 1.38) or the
> `std::mem::MaybeUninit` type (stabilized 1.36) must not be used, or explicitly
> justified when necessary.

The use of uninitialized memory may result in two distinct security issues:

- drop of uninitialized memory (also a memory safety issue),
- non-drop of initialized memory.

> **Note**
>
> `std::mem::MaybeUninit` is an improvement over `std::mem::uninitialized`.
> Indeed, it makes dropping uninitialized values a lot more difficult.
> However, it does not change the second issue: the non-drop of an initialized
> memory is as much likely. It is problematic, in particular when considering
> the use of `Drop` to erase sensitive memory.

#### Secure memory zeroing for sensitive information

Zeroing memory is useful for sensitive variables, especially if the
Rust code is used through FFI.

> ### Rule {{#check MEM-ZERO | Zero out memory of sensitive data after use}}
> Variables containing sensitive data must be zeroed out after use, using
> functions that will not be removed by the compiler optimizations, like
> `std::ptr::write_volatile` or the `zeroize` crate.

The following code shows how to define an integer type that will be set to
0 when freed, using the `Drop` trait:

```rust
/// Example: u32 newtype, set to 0 when freed
pub struct ZU32(pub u32);

impl Drop for ZU32 {
    fn drop(&mut self) {
        println!("zeroing memory");
        unsafe{ ::std::ptr::write_volatile(&mut self.0, 0) };
    }
}

# fn main() {
{
    let i = ZU32(42);
    // ...
} // i is freed here
# }
```

### Unsafe code

The joint utilization of the type system and the ownership system aims to
enforce safety regarding the memory management in Rust's programs. So the
language aims to avoid memory overflows, null or invalid pointer constructions,
and data races.
To perform risky actions such as system calls, type coercions, or memory
pointers direct manipulations, the language provides the `unsafe` keyword.

> ### Rule {{#check LANG-UNSAFE | Don't use unsafe blocks}}
> For a secured development, the `unsafe` blocks must be avoided. Afterward,
> we list the only cases where `unsafe` may be used, provided that they come
> with a proper justification:
> 
>  - The Foreign Function Interface (FFI) of Rust allows for describing
>  functions whose implementation is written in C, using the `extern "C"`
>  prefix. To use such a function, the `unsafe` keyword is required. “Safe”
>  wrapper shall be defined to safely and seamlessly call C code.
> 
>  - For embedded device programming, registers and various other resources are
>  often accessed through a fixed memory address. In this case, `unsafe` blocks
>  are required to initialize and dereference those particular pointers in Rust.
>  In order to minimize the number of unsafe accesses in the code and to allow
>  easier identification of them by a programmer, a proper abstraction (data
>  structure or module) shall be provided.
> 
>  - A function can be marked unsafe globally (by prefixing its declaration with
>  the `unsafe` keyword) when it may exhibit unsafe behaviors based on its
>  arguments, that are unavoidable. For instance, this happens when a function
>  tries to dereference a pointer passed as an argument.
> 
> With the exception of these cases, `#[forbid(unsafe_code)]` must appear in
> `main.rs` to generate compilation errors if `unsafe` is used in the code base.

### Integer overflows

Although some verification is performed by Rust regarding the potential
integer overflows, precautions should be taken when executing arithmetic
operations on integers.

In particular, it should be noted that using debug or release compilation
profile changes integer overflow behavior. In debug configuration, overflow
cause the termination of the program (`panic`), whereas in the release configuration
the computed value silently wraps around the maximum value that can be stored.

This last behavior can be made explicit by using the `Wrapping` generic type,
or the `overflowing_<op>` and `wrapping_<op>` operations on integers
(the `<op>` part being `add`, `mul`, `sub`, `shr`, etc.).

```rust
use std::num::Wrapping;
# use std::panic;

# fn main() {
let x: u8 = 242;

# let result = panic::catch_unwind(|| {
println!("{}", x + 50);                      // panics in debug, prints 36 in release.
# });
# if result.is_err() { println!("panic"); }
println!("{}", x.overflowing_add(50).0);     // always prints 36.
println!("{}", x.wrapping_add(50));          // always prints 36.
println!("{}", Wrapping(x) + Wrapping(50));  // always prints 36.

// always panics:
let (res, c) = x.overflowing_add(50);
# let result = panic::catch_unwind(|| {
if c { panic!("custom error"); }
else { println!("{}", res); }
# });
# if result.is_err() { println!("panic"); }
# }
```

> ### Rule {{#check LANG-ARITH | Use appropriate arithmetic operations regarding potential overflows}}
> When assuming that an arithmetic operation can produce an overflow, the
> specialized functions `overflowing_<op>`, `wrapping_<op>`, or the
> `Wrapping` type must be used.

## Type system

<mark>TODO</mark>: identify pitfalls with the type system (for instance,
misunderstanding of which code is actually executed when implementing complex
patterns with traits).

## Error handling

<mark>TODO</mark>: explicit good practices in error handling.

The `Result` type is the preferred way of handling functions that can fail.
A `Result` object must be tested, and never ignored.

> ### Recommendation {{#check LANG-ERRWRAP | Implement custom `Error` type, wrapping all possible errors}}
> A crate can implement its own Error type, wrapping all possible errors.
> It must be careful to make this type exception-safe (RFC 1236), and implement
> `Error + Send + Sync + 'static` as well as `Display`.

> ### Recommendation {{#check LANG-ERRDO | Use the `?` operator and do not use the `try!` macro}}
> The `?` operator should be used to improve readability of code.
> The `try!` macro should not be used.

The [error-chain] and [failure] crates can be used to wrap errors.

[error-chain]: https://crates.io/crates/error-chain
[failure]: https://crates.io/crates/failure

### Panics

Explicit error handling (`Result`) should always be preferred instead of calling
`panic`.  The cause of the error should be available, and generic errors should
be avoided.

Crates providing libraries should never use functions or instructions that can
fail and cause the code to panic.

Common patterns that can cause panics are:

- using `unwrap` or `expect`,
- using `assert`,
- an unchecked access to an array,
- integer overflow (in debug mode),
- division by zero,
- large allocations,
- string formatting using `format!`.

> ### Rule {{#check LANG-NOPANIC | Don't use functions that can cause `panic!`}}
> Functions or instructions that can cause the code to panic at runtime must not
> be used.

> ### Rule {{#check LANG-ARRINDEXING | Test properly array indexing or use the `get` method}}
> Array indexing must be properly tested, or the `get` method should be used to
> return an `Option`.

<mark>TODO</mark> Check if the [no_panic](https://github.com/dtolnay/no-panic)
crate can catch all cases. Drawback: all functions need to be marked as
`#[no_panic]`.
<!--
<mark>TODO</mark> Another possibility:
[rustig](https://github.com/Technolution/rustig) (doesn't build here)
-->

### FFI and panics

When calling Rust code from another language (for ex. C), the Rust code must
be careful to never panic.
Unwinding from Rust code into foreign code results in undefined behavior.

> ### Rule {{#check LANG-FFIPANIC | Handle correctly `panic!` in FFI}}
> Rust code called from FFI must either ensure the function cannot panic, or use
> `catch_unwind` or the `std::panic` module to ensure the rust code will not
> abort or return in an unstable state.

Note that `catch_unwind` will only catch unwinding panics, not those that abort
the process.

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

> ### Recommendation {{#check LANG-DROP | Justify `Drop` impl.}}
>
> In a Rust secure development, the implementation of the `std::ops::Drop` trait
> should be justified, documented and peer-reviewed.

Second, Rust type system only ensures memory safety and, from the type system's
standpoint, missing drops is allowed. In fact, several things may lead to
missing drops, such as:

- a reference cycle (for instance, with `Rc` or `Arc`),
- an explicit call to `std::mem::forget` (or `core::mem::forget`) (see Rule
  <mark>TODO</mark>),
- a panic in drop,
- program aborts (and panics when abort-on-panic is on).

And missing drops may lead to exposing sensitive data or to lock limited
resources leading to unavailability issues.

> ### Rule {{#check LANG-DROP-NO-PANIC | Do not panic in `Drop` impl.}}
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

Both traits are _unsafe traits_, i.e., the Rust compiler do not verify in any
way that they are implemented correctly. The danger is real: an incorrect
implementation may lead to **undefined behavior**.

Fortunately, in most case, one does not need to implement it. In Rust,
almost all primitive types are `Send` and `Sync`, and for most compound types
the implementation is automatically provided by the Rust compiler.
Notable exceptions are:

- Raw pointers are neither `Send` nor `Sync` because they offer no safety guards.
- `UnsafeCell` is not `Sync` (and as a result `Cell` and `RefCell` aren't
  either) because they offer interior mutability (mutably shared value).
- `Rc` is neither `Send` nor `Sync` because the reference counter is shared and
  unsynchronized.

Automatic implementation of `Send` (resp. `Sync`) occurs for a compound type
(structure or enumeration) when all fields have `Send` types (resp. `Sync`
types). Using an unstable feature (as of Rust 1.36.0), one can block the automatic
implementation of those traits with a manual _negative implementation_:

```rust,ignore
#![feature(option_builtin_traits)]

struct SpecialType(u8);
impl !Send for SpecialType {}
impl !Sync for SpecialType {}
```

The negative implementation of `Send` or `Sync` are also used in the standard
library for the exceptions, and are automatically implemented when appropriate.
As a result, the generated documentation is always explicit: a type implements
either `Send` or `!Send` (resp. `Sync` or `Sync`).

> ### Recommendation {{#check LANG-SYNC-TRAITS | Justify `Send` and `Sync` impl.}}
>
> In a Rust secure development, the manual implementation (or negative
> implementation) of the `Send` and `Sync` traits should be avoided and, if
> necessary, should be justified, documented and peer-reviewed.

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
    the strict inverse of `eq`. The default implementation of `ne` is precisely that.

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

<!-- -->

> ### Recommendation {{#check LANG-CMP-DEFAULTS | Use the default method impl. of standard comparison traits}}
>
> In a Rust secure development, the implementation of standard comparison traits
> should only define methods with no default implementation, so as to reduce
> the risk of violating the invariants associated with the traits.

There is a Clippy lint to check that `PartialEq::ne` is not defined in
`PartialEq` implementations.

<mark>TODO</mark> Recommendation: Derive when possible?

## Macros

<mark>TODO</mark>: cyclomatic complexity of the macro expanded code, recursion
limits, ...
