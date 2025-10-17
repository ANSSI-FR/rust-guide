---
references:
  - type: web
    title: The Rust Programming Language
    url: https://doc.rust-lang.org/stable/book/
    id: rust-book
---

# General information on `unsafe`

## *Unsafe* operations

Language capabilities can be extended using unsafe code. The full list of these features is given in the [Rust reference](https://doc.rust-lang.org/reference/unsafety.html). Notice the following ones.

* Dereference a raw pointer
* Read or write a mutable or extern static variable
* Read a field of an `union`
* Implement an `unsafe` trait
* Declare an `extern` block

More examples can be found in [nomicon](https://doc.rust-lang.org/nomicon/what-unsafe-does.html).

These capabilities may be necessary for system programming but they cause the language to lose its [safety properties](../guarantees.md#language-guarantees) and Undefined Behaviors may happen.

<div class="reco" id="UNSAFE-NOUB" type="Rule" title="No Undefined Behavior">

No Undefined Behavior is allowed.

</div>

## A keyword with two usages

The `unsafe` keyword is used both for marking unsafety in an API and unlocking unsafety in the implementation.

### `unsafe` marking

Marking with `unsafe` is a delegation of responsibility with respect to memory safety from the API author to the API user.
The use of this keyword in an API *warns* the API user about the potential harmful effects of using the API.

* In a function signature ([r-unsafe.fn]), `unsafe` means that the behavior of the function may lead to UB if the use of the function does not comply with its interface contract (informally described in its documentation).
* In a trait declaration ([r-unsafe.trait]), `unsafe` means that an erroneous implementation of this trait may lead to UB if the implementation contract (preferably documented) is not respected.

[r-unsafe.fn]: <https://doc.rust-lang.org/reference/unsafe-keyword.html#r-unsafe.fn>
[r-unsafe.trait]: <https://doc.rust-lang.org/reference/unsafe-keyword.html#r-unsafe.trait>

### `unsafe` unlocking

Unlocking with `unsafe` means taking responsibility for memory safety from the compiler to the developer.

Using an `unsafe` block in a function body or in a constant declaration is imposed by the compiler to prevent the *inadvertent use* of `unsafe` capabilities like

* using `unsafe` tagged functions
* modifying static variables
* using extern functions

Similarly, the implementation of an `unsafe` trait requires `unsafe` for the developer to explicitly take into account the memory safety contracts. The keyword `unsafe` *unlocks* the implementation of `unsafe` traits.

Lastly, Since the 2024 edition, `unsafe` is also required to unlock the following:

* `extern` blocks, which contain declarations of foreign functions and variables, for [FFI](./ffi.md),
* some attributes (for instance , no_mangle, cf. [r-attributes.safety]).

[r-attributes.safety]: <https://doc.rust-lang.org/reference/attributes.html#r-attributes.safety>

## Limitations and precautions

Paraphrasing the [Rustonomicon](https://doc.rust-lang.org/nomicon/), the fundamental principle of Rust can be summed up as follows:

> `unsafe`-free code cannot go wrong

The combined use of the type system and the ownership system enforces a high-level memory safety in Rust programs. This way, the language helps prevent memory overflows, null or invalid pointer constructions, and data races.

This promise is valid only if the code does not use `unsafe` features. When `unsafe` features are used, the compiler can no longer guarantee memory safety. The developer must then ensure that the code respects the invariants that guarantee memory safety.

That is why it is crucial to limit the use of `unsafe` features as much as possible:

<div class="reco" id="LANG-UNSAFE" type="Rule" title="Don't use unsafe blocks">

In a secure Rust development, the `unsafe` blocks must be avoided. In the following,
we list the only cases where `unsafe` may be used, provided that they come
with a proper justification:

* The Foreign Function Interface (FFI) of Rust allows for describing functions whose implementations are written in C, using the `extern "C"` prefix. To use such a function, the `unsafe` keyword is required. “Safe” wrapper shall be defined to safely and seamlessly call C code.

* For embedded device programming, registers and various other resources are often accessed through a fixed memory address. In this case, `unsafe` blocks are required to initialize and dereference those particular pointers in Rust. In order to minimize the number of unsafe accesses in the code and to allow easier identification of them by a programmer, a proper abstraction (data
structure or module) shall be provided.

* A function can be marked unsafe globally (by prefixing its declaration with the `unsafe` keyword) when it may exhibit unsafe behaviors based on its arguments, that are unavoidable. For instance, this happens when a function tries to dereference a pointer passed as an argument.

* When hitting a performance wall on a small portion of code (E.G: Zero-copy buffer modified in-place, Allocation overhead, etc.).

With the exception of these cases, `#![forbid(unsafe_code)]` must appear in the crate root (typically `main.rs` or `lib.rs`) to generate compilation errors if `unsafe` is used in the code base.

</div>

If the use of `unsafe` is necessary, it is the responsibility of the developer to:

* ensure that the use of `unsafe` unlocking does not lead to *UB*s,
* ensure that any `unsafe` markings are correctly and exhaustively documented so that no *UB* are possible if the usage conditions (invariants) are respected.

Aside from the `unsafe` code itself, it is also crucial to properly encapsulate the use of `unsafe` features in a component (crate or module) so as to restore the usual Rust memory safety guarantees:

<div class="reco" id="LANG-UNSAFE-ENCP" type="Rule" title="Encapsulation of *unsafe* features">

In secure development of a Rust software component (crate or module), all
*unsafe* code must be encapsulated in such a way that:

* either it exposes a safe behavior to the user, in which no safe interaction
  can result in UB (undefined behavior);
* or it exposes features marked as unsafe whose usage conditions
  (preconditions, sequencing, etc.) are exhaustively documented.

</div>

Thus, a function using `unsafe` operations can be *safe* if the `unsafe`
operations do not present any *UB* (undefined behavior) given the component's
invariants (typically the type invariant for a method). Conversely, a function
without an `unsafe` block must be marked as `unsafe` if it breaks these
invariants. The choice and knowledge of these invariants are therefore crucial
for secure development.

### Example 1: Preserving a type invariant

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

This function may be necessary for internal use, but it should not be exposed in the API, or it should be marked with the `unsafe` keyword, because its use can lead to UB.

### Example 2: Trust relationship between *safe* and *unsafe*

In the Rust paradigm:

> `unsafe`-free code cannot go wrong

which means it cannot result in UB.
This property is lost when developers use *unsafe* code, so they are responsible for not producing UB in any scenario.
Consequently, even *safe* functions must be handled carefully in *unsafe* contexts.

Suppose one wants to propose an API to find an object of a given type in memory.
This API could require implementing the following trait:

```rust
{{#include ../../../examples/src/generalities.rs:Locatable}}
```

This trait can be implemented **without** using `unsafe`.

For instance, the `bool` type can implement this trait as follows:

```rust,ignore align
{{#include ../../../examples/src/generalities.rs:Locatable_bool_OK}}
```

<div class="warning">

This API is harmful for two reasons:

* If the `Locatable` implementation does not give the index of an object of type `T`, the `read_unaligned` may produce UB.
* If the `Locatable` implementation gives an out-of-bounds index or an index for which part of the object is out of bounds, the subsequent buffer overflow is UB.

</div>

For instance, the following `Locatable` implementation is incorrect, **but** it is the responsibility of the API author to take it into account.

```rust align
{{#include ../../../examples/src/generalities.rs:Locatable_bool_KO}}
```

The following program produces UB.

```rust,ignore align
{{#include ../../../examples/src/generalities.rs:Locatable_UB}}
```

The UB-detecting tool `miri` reports the following:

```default
$ cargo +nightly miri r --bin overflow
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/cargo-miri runner target/miri/x86_64-unknown-linux-gnu/debug/overflow`
error: Undefined Behavior: in-bounds pointer arithmetic failed: attempting to offset pointer by 101 bytes, but got alloc249 which is only 3 bytes from the end of the allocation
  --> src/overflow.rs:16:29
   |
16 |         let ptr: *const T = buf.as_ptr().add(start).cast();
   |                             ^^^^^^^^^^^^^^^^^^^^^^^ Undefined Behavior occurred here
   |
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
help: alloc249 was allocated here:
  --> src/overflow.rs:22:9
   |
22 |     let buf = [4, 1, 99];
   |         ^^^
   = note: BACKTRACE (of the first span):
   = note: inside `find::<bool>` at src/overflow.rs:16:29: 16:52
note: inside `main`
  --> src/overflow.rs:23:38
   |
23 |     let located_bool: Option<bool> = find(&buf); // UB here!
   |                                      ^^^^^^^^^^

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to 1 previous error
```

This example shows that developers using `unsafe` blocks
cannot assume that *safe* functions or traits they use are well implemented, and thus must prevent UB in case these *safe* functions have bad behavior.

If they cannot protect their function against poorly implemented *safe* functions or traits, they have two options:

* Mark the function they *write* as `unsafe`: thus, it is the user's responsibility to provide correct arguments (by checking the `unsafe` function's documentation).
* Mark the traits they *use* as `unsafe`: thus, it is the user's responsibility to implement the trait properly (again, by reading the trait documentation).

More examples can be found in [@rust-book] (in the [Unsafe Rust](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html) chapter) or the [nomicon](https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html).