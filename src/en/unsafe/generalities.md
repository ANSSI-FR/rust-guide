---
references:
  - type: web
    title: The Rust Programming Language
    url: https://doc.rust-lang.org/stable/book/
    id: rust-book
---

# General information on `unsafe`

## *Unsafe* capacities

Language capabilities can be extended using unsafe code. The full list of these capacities is given in the [Rust reference](https://doc.rust-lang.org/reference/unsafety.html). Notice the following ones.

* Dereference a raw pointer
* Modify a static mutable variable
* Access to the fields of a `union`
* Declaring an `extern` block

These capabilities may be necessary for system programming but they cause the language to lose its [safety properties](../guarantees.md#language-guarantees). More examples can be found in [nomicon](https://doc.rust-lang.org/nomicon/what-unsafe-does.html).

## `unsafe` keyword

The `unsafe` keyword is used both for API and implementation.

### `unsafe` in API

The use of this keyword in an API *warns* the API user against the potential harmful effects of the use of the API.

* In a function signature, `unsafe` means that the behavior of the function may lead to *UB*s if the use of the function does not comply with its interface contract (informally described in its documentation).
* In a trait definition, `unsafe` means that an erroneous implementation of this trait may lead to *UB*s.

### `unsafe` in implementation

Using this keyword in an implementation (a code block) is imposed by the compiler to prevent the inadvertent use of `unsafe` functions.

## Unsafe code

The combined use of the type system and the ownership system
enforces a high-level memory safety in Rust programs. This way, the language helps prevent memory overflows, null or invalid pointer constructions, and data
races.
To perform risky actions such as system calls, type coercions, or direct
manipulations of memory pointers, the language provides the `unsafe` keyword.

<div class="reco" id="LANG-UNSAFE" type="Rule" title="Don't use unsafe blocks">

In a secure Rust development, the `unsafe` blocks must be avoided. In the following,
we list the only cases where `unsafe` may be used, provided that they come
with a proper justification:

 - The Foreign Function Interface (FFI) of Rust allows describing
 functions whose implementations are written in C, using the `extern "C"`
 prefix. To use such a function, the `unsafe` keyword is required. “Safe”
 wrappers shall be defined to safely and seamlessly call C code.

 - For embedded device programming, registers and various other resources are
 often accessed through a fixed memory address. In this case, `unsafe` blocks
 are required to initialize and dereference those particular pointers in Rust.
 In order to minimize the number of unsafe accesses in the code and to allow
 easier identification of them by a programmer, a proper abstraction (data
 structure or module) shall be provided.

 - A function can be marked unsafe globally (by prefixing its declaration with
 the `unsafe` keyword) when it may exhibit unsafe behaviors based on its
 arguments, that are unavoidable. For instance, this happens when a function
 tries to dereference a pointer passed as an argument.

With the exception of these cases, `#![forbid(unsafe_code)]` must appear in
the crate root (typically `main.rs` or `lib.rs`) to generate compilation
errors if `unsafe` is used in the code base.

</div>

## General warnings 

### Invariants and wrapping *unsafe*

Exposed APIs are responsible for preserving invariants to avoid bugs in general and, when handling `unsafe` code, *UB*s in particular.

The following code comes from the [Rustonomicon](https://doc.rust-lang.org/nomicon/working-with-unsafe.html).
It could be used to implement a custom `Vec` type.

```rust
{{#include ../../../examples/src/generalities.rs:naive_vec}}
```

Soundness and safety of this code rely on the fact that bytes from address `self.ptr` to `self.ptr + self.cap * size_of<T>()` are allocated.

This invariant can be broken with *safe* code. For instance

```rust
{{#include ../../../examples/src/generalities.rs:make_room}}
```

This function my be necessary for internal use, but it should not be exposed in the API, or it should be marked with `unsafe` keyword, because its use can lead to *UB*.

### Trust relation between *safe* and *unsafe*

#### Principle

In Rust paradigm, 

> `unsafe`-free code cannot go wrong

which means it cannot result in *UB*.
This property is lost when the developers use *unsafe* code, that is why they are responsible of not producing *UB* in any scenario.
Consequently, even *safe* function must be handled carefully in *unsafe* context.

#### Example

Suppose one wants to propose an API to find objects of a given type in the memory.
This API could ask implementing the following trait

```rust
{{#include ../../../examples/src/generalities.rs:Locatable}}
```

This trait can be implemented **without** `unsafe`.

For instance, `bool` type can implement this trait as follows:

```rust,ignore align
{{#include ../../../examples/src/generalities.rs:Locatable_bool_OK}}
```

<div class="warning">

This API is harmful for two reasons:

* If the `Locatable` implementation gives the wrong index, the `as_ref` function produce an *UB*.
* If the `Locatable` implementation gives an out-of-bounds index, the subsequent buffer overflow is an *UB*.

</div>

For instance, the following `Locatable` implementation is wrong **but** it is the responsibility of the API maker to take it into account.

```rust align
{{#include ../../../examples/src/generalities.rs:Locatable_bool_KO}}
```

The following program produces a *UB*.

```rust,ignore align
{{#include ../../../examples/src/generalities.rs:Locatable_UB}}
```

The error can be shown with `valgrind`

```
$ valgrind ./target/release/rust-unsafe
==123651== Memcheck, a memory error detector
==123651== Copyright (C) 2002-2022, and GNU GPL'd, by Julian Seward et al.
==123651== Using Valgrind-3.19.0 and LibVEX; rerun with -h for copyright info
==123651== Command: ./target/release/rust-unsafe
==123651== 
==123651== valgrind: Unrecognised instruction at address 0x10f860.
==123651==    at 0x10F860: rust_unsafe::main (in /home/toto/src/rust-unsafe/target/release/rust-unsafe)
==123651==    by 0x10F842: std::sys::backtrace::__rust_begin_short_backtrace (in /home/toto/src/rust-unsafe/target/release/rust-unsafe)
==123651==    by 0x10F838: std::rt::lang_start::{{closure}} (in /home/toto/src/rust-unsafe/target/release/rust-unsafe)
==123651==    by 0x129F0F: std::rt::lang_start_internal (in /home/toto/src/rust-unsafe/target/release/rust-unsafe)
==123651==    by 0x10F894: main (in /home/toto/src/rust-unsafe/target/release/rust-unsafe)
==123651== Your program just tried to execute an instruction that Valgrind
==123651== did not recognise.  There are two possible reasons for this.
==123651== 1. Your program has a bug and erroneously jumped to a non-code
==123651==    location.  If you are running Memcheck and you just saw a
==123651==    warning about a bad jump, it's probably your program's fault.
==123651== 2. The instruction is legitimate but Valgrind doesn't handle it,
==123651==    i.e. it's Valgrind's fault.  If you think this is the case or
==123651==    you are not sure, please let us know and we'll try to fix it.
==123651== Either way, Valgrind will now raise a SIGILL signal which will
==123651== probably kill your program.
==123651== 
==123651== Process terminating with default action of signal 4 (SIGILL)
==123651==  Illegal opcode at address 0x10F860
==123651==    at 0x10F860: rust_unsafe::main (in /home/toto/src/rust-unsafe/target/release/rust-unsafe)
==123651==    by 0x10F842: std::sys::backtrace::__rust_begin_short_backtrace (in /home/toto/src/rust-unsafe/target/release/rust-unsafe)
==123651==    by 0x10F838: std::rt::lang_start::{{closure}} (in /home/toto/src/rust-unsafe/target/release/rust-unsafe)
==123651==    by 0x129F0F: std::rt::lang_start_internal (in /home/toto/src/rust-unsafe/target/release/rust-unsafe)
==123651==    by 0x10F894: main (in /home/toto/src/rust-unsafe/target/release/rust-unsafe)
==123651== 
==123651== HEAP SUMMARY:
==123651==     in use at exit: 0 bytes in 0 blocks
==123651==   total heap usage: 7 allocs, 7 frees, 2,072 bytes allocated
==123651== 
==123651== All heap blocks were freed -- no leaks are possible
==123651== 
==123651== For lists of detected and suppressed errors, rerun with: -s
==123651== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```

#### Conclusion

This example shows that developers using `unsafe` blocks 
cannot assume *safe* functions/traits they
use are well implemented, and thus must prevent *UB* 
in case these *safe* functions have bad behavior.

If they cannot protect their function against badly implemented *safe* functions/traits, they could either

* mark the function they *write* as `unsafe`: thus it is the user's responsibility to feed this function with correct arguments (by checking *unsafe* function documentation).
* mark the traits they *use* as `unsafe` : thus it is user's responsibility to implement the trait properly (again reading the trait documentation).

More examples can be found in [@rust-book] (in the [Unsafe Rust](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html) chapter) or the [nomicon](https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html).