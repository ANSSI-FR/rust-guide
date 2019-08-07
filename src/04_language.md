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

By default, Rust forces all values to be initialized, preventing the use of
uninitialized memory (except if using `std::mem::uninitialized`).
However, zeroing memory is useful for sensitive variables, especially if the
Rust code is used through FFI.

> ### Rule {{#check MEM-ZERO | Zeroize memory of sensitive data after use}}:
> Variables containing sensitive data must be zeroized after use, using
> functions that will not be removed by the compiler optimizations, like
> `std::ptr::write_volatile` or the `zeroize` crate.
>
> The `std::mem::uninitialized` function must not be used, or explicitly
> justified when necessary.

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

> ### Rule {{#check LANG-UNSAFE | Don't use unsafe blocks}}:
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

> ### Rule {{#check LANG-ARITH | Use appropriate arithmetic operations regarding potential overflows}}:
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

> ### Recommendation {{#check LANG-ERRWRAP | Implement custom `Error` type, wrapping all possible errors}}:
> A crate can implement its own Error type, wrapping all possible errors.
> It must be careful to make this type exception-safe (RFC 1236), and implement
> `Error + Send + Sync + 'static` as well as `Display`.

> ### Recommendation {{#check LANG-ERRDO | Use the `?` operator and do not use the `try!` macro}}:
> The `?` operator should be used to improve readability of code.
> The `try!` macro should not be used.

The [error-chain](https://crates.io/crates/error-chain) and
[failure](https://crates.io/crates/failure) crates can be used to wrap errors.

### Panics

Explicit error handling (`Result`) should always be preferred instead of calling
`panic`.  The cause of the error should be available, and generic errors should
be avoided.

Crates providing libraries should never use functions or instructions that can
fail and cause the code to panic.

Common patterns that can cause panics are:

 - using `unwrap` or `expect`
 - using `assert`
 - an unchecked access to an array
 - integer overflow (in debug mode)
 - division by zero
 - large allocations
 - string formatting using `format!`

> ### Rule {{#check LANG-NOPANIC | Don't use functions that can cause `panic!`}}:
> Functions or instructions that can cause the code to panic at runtime must not
> be used.

> ### Rule {{#check LANG-ARRINDEXING | Test properly array indexing or use the `get` method}}:
> Array indexing must be properly tested, or the `get` method should be used to
> return an `Option`.

<mark>TODO</mark> Check if the [no_panic](https://github.com/dtolnay/no-panic)
crate can catch all cases. Drawback: all functions need to be marked as
`#[no_panic]`.
<!--
<mark>TODO</mark> Another possibility:
[rustig](https://github.com/Technolution/rustig) (doesn't build here)
-->

## Standard library traits

 - Drop <mark>TODO</mark>

 - The `Send` and `Sync` traits are marker traits that allow implementors
 to specify respectively that a value can be sent (moved) to another thread and
 that a value be shared through a shared reference. They both are unsafe traits,
 that should be used. Moreover, the `unsafe` keyword is necessary for a type
 to implement those traits.

 - PartialOrd, Ord, Eq <mark>TODO</mark>

## Macros

<mark>TODO</mark>: cyclomatic complexity of the macro expanded code, recursion
limits, ...
