# The Rust language

## Generalities

### Naming

<mark>TODO</mark>: recommendations about correctly naming things (RFC 430).

### Memory management

<mark>TODO</mark>: explain safe allocations/deallocations, ownership/borrowing,
and identify language constructs that may break memory safety (for instance,
unsound behaviors in older versions of the compiler).

### Unsafe code

The joint utilization of the type system and the ownership system aims to
enforce safety regarding the memory management in Rust's programs. So the
language aims to avoid memory overflows, null or invalid pointer constructions,
and data races.
To perform risky actions such as system calls, type coercions, or memory
pointers direct manipulations, the language provides the `unsafe` keyword.

> ### Recommendation:
> For a secured development, the `unsafe` blocks shall be avoided. Afterward,
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
> With the exception of these cases, `#[forbid(unsafe_code)]` shall appear in
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

> ### Recommendation:
> When assuming that an arithmetic operation can produce an overflow, the
> specialized functions `overlapping_<op>`, `wrapping_<op>`, or the
> `Wrapping` type must be used.

## Type system

<mark>TODO</mark>: identify pitfalls with the type system (for instance,
misunderstanding of which code is actually executed when implementing complex
patterns with traits).

## Error handling

<mark>TODO</mark>: explicit good practices in error handling.

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
