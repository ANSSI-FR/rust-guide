# General information on `unsafe`

## *Unsafe* capacities

Language capabilities can be extended using unsafe code. The full list of these capacities is given in the [Rust reference](https://doc.rust-lang.org/reference/unsafety.html). Notice the following one's.

* Dereference a raw pointer
* Modify a static mutable variable
* Access to the fields of a `union`
* Declaring an `extern` block

These capabilities may be necessary for system programming but they cause the language to lose its [security properties](04_language.md#language-guarantees).

## Unsafe code

The combined use of the type system and the ownership system
enforces a high-level memory safety in Rust programs. This way, the language helps prevent memory overflows, null or invalid pointer constructions, and data
races.
To perform risky actions such as system calls, type coercions, or direct
manipulations of memory pointers, the language provides the `unsafe` keyword.

> **Rule {{#check LANG-UNSAFE | Don't use unsafe blocks}}**
>
> For a secured development, the `unsafe` blocks must be avoided. Afterward,
> we list the only cases where `unsafe` may be used, provided that they come
> with a proper justification:
>
>  - The Foreign Function Interface (FFI) of Rust allows for describing
>  functions whose implementations are written in C, using the `extern "C"`
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
> With the exception of these cases, `#![forbid(unsafe_code)]` must appear in
> the crate root (typically `main.rs` or `lib.rs`) to generate compilation
> errors if `unsafe` is used in the code base.