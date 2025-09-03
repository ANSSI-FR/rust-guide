# General information on `unsafe`

## *Unsafe* operations

Language capabilities can be extended using unsafe code. The full list of these features is given in the [Rust reference](https://doc.rust-lang.org/reference/unsafety.html). Notice the following one's.

* Dereference a raw pointer
* Read or write a mutable or extern static variable
* Read a field of an `union`
* Implement an `unsafe` trait
* Declare an `extern` block

These capabilities may be necessary for system programming but they cause the language to lose its [security properties](../guarantees.md).

## A keyword with two usages

The `unsafe` keyword is used both as for marking unsafety in an API and unlocking unsafety in the implementation.

### `unsafe` marking

Marking with `unsafe` is a delegation of responsibility w.r.t. to the memory safety from the API maker to the API user.
The use of this keyword in an API *warns* the API user against the potential harmful effects of the use of the API.

* In a function signature ([r-unsafe.fn]), `unsafe` means that the behavior of the function may lead to *UB*s if the use of the function does not comply with its interface contract (informally described in its documentation).
* In a trait declaration ([r-unsafe.trait]), `unsafe` means that an erroneous implementation of this trait may lead to *UB*s if the implementation contract (preferrably documented) is not respected.

[r-unsafe.fn]: <https://doc.rust-lang.org/reference/unsafe-keyword.html#r-unsafe.fn>
[r-unsafe.trait]: <https://doc.rust-lang.org/reference/unsafe-keyword.html#r-unsafe.trait>

### `unsafe` unlokcing

Unlocking with `unsafe` is taking responsibility for the memory safety from the compiler to the developer.

Using an `unsafe` block in a function body or in a constant declaration is imposed by the compiler to prevent the *inadvertent use* of `unsafe` capabilities.
The keyword `unsafe` *unlocks* the use of these capabilities.

In a similar manner, the implementation of an `unsafe` trait requires `unsafe` for the developer to explictly take into account the memory safety contracts. The keyword `unsafe` *unlocks* the implementation of `unsafe` traits.

Since the 2024 edition, `unsafe` is also required to unlock the following:

* `extern` blocks, which contains declarations of foreign functions and variables, for [FFI](./ffi.md),
* some attributes (for instance , no_mangle, cf. [r-attributes.safety]).

[r-attributes.safety]: <https://doc.rust-lang.org/reference/attributes.html#r-attributes.safety>

## Limitations and precautions

Paraphrasing the [Rustonomicon](https://doc.rust-lang.org/nomicon/), the fundamental principle of Rust can be sumed up as follows:

> `unsafe`-free code cannot go wrong

The combined use of the type system and the ownership system enforces a high-level memory safety in Rust programs. This way, the language helps prevent memory overflows, null or invalid pointer constructions, and data races.

This promise is valid only if the code does not use `unsafe` features. When `unsafe` features are used, the compiler cannot guarantee memory safety anymore. The developer must then ensure that the code respects the invariants that guarantee memory safety.

That's why, it is crucial to limit the use of `unsafe` features as much as possible:

> **Rule {{#check LANG-UNSAFE | Don't use unsafe blocks}}**
>
> In a secure Rust development, the `unsafe` blocks must be avoided. In the following,
> we list the only cases where `unsafe` may be used, provided that they come
> with a proper justification:
>
> * The Foreign Function Interface (FFI) of Rust allows for describing functions whose implementations are written in C, using the `extern "C"` prefix. To use such a function, the `unsafe` keyword is required. “Safe” wrapper shall be defined to safely and seamlessly call C code.
>
> * For embedded device programming, registers and various other resources are often accessed through a fixed memory address. In this case, `unsafe` blocks are required to initialize and dereference those particular pointers in Rust. In order to minimize the number of unsafe accesses in the code and to allow easier identification of them by a programmer, a proper abstraction (data
> structure or module) shall be provided.
>
> * A function can be marked unsafe globally (by prefixing its declaration with the `unsafe` keyword) when it may exhibit unsafe behaviors based on its arguments, that are unavoidable. For instance, this happens when a function tries to dereference a pointer passed as an argument.
>
> With the exception of these cases, `#![forbid(unsafe_code)]` must appear in the crate root (typically `main.rs` or `lib.rs`) to generate compilation errors if `unsafe` is used in the code base.

In case the use of `unsafe` is necessary, it is of the responsability of the developer to:

* ensure that the use of `unsafe` unlocking does not lead to *UB*s,
* ensure that any `unsafe` markings are correctly and exhaustively documented so that no *UB* are possible if the usage conditions (invariants) are respected.

Aside from the `unsafe` code itself, it is also crucial to properly encapsulate the use of `unsafe` features in a component (crate or module) so as to restore the usual Rust memory safety guarantees:

> **Rule {{#check LANG-UNSAFE-ENCP | Encapsulation of *unsafe* features}}**
>
> In secure development of a Rust software component (crate or module), all
> *unsafe* code must be encapsulated in such a way that:
>
> * either it exposes a safe behavior to the user, in which no safe interaction
>   can result in UB (undefined behavior);
> * or it exposes features marked as unsafe whose usage conditions
>   (preconditions, sequencing, etc.) are exhaustively documented.

Thus, a function using `unsafe` operations can be *safe* if the `unsafe`
operations do not present any *UB* (undefined behavior) given the component's
invariants (typically the type invariant for a method). Conversely, a function
without an `unsafe` block must be marked as `unsafe` if it breaks these
invariants. The choice and knowledge of these invariants are therefore crucial
for secure development.

### Example 1: Preservering a type invariant

The following code comes from the [Rustonomicon](https://doc.rust-lang.org/nomicon/working-with-unsafe.html).
It could be used to implement a custom `Vec` type.

```rust
use std::ptr;

pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

// Note this implementation does not correctly handle zero-sized types.
impl<T> Vec<T> {
    pub fn push(&mut self, elem: T) {
        if self.len == self.cap {
            // not important for this example
            self.reallocate();
        }
        unsafe {
            ptr::write(self.ptr.add(self.len), elem);
            self.len += 1;
        }
    }
}
```

Soundness and safety of this code rely on the fact that bytes from address `self.ptr` to `self.ptr + self.cap * size_of<T>()` are allocated.

This invariant can be broken with *safe* code. For instance

```rust
impl<T> Vec<T> {
    fn make_room(&mut self) {
        // grow the capacity
        self.cap += 1;
    }
}
```

This function my be necessary for internal use, but it should not be exposed in the API, or it should be marked with `unsafe` keyword, because its use can lead to *UB*.

> **Rule {{#check LANG-UNSAFE-ENCP | Encapsulation of *unsafe* features}}**
>
> In secure development of a Rust software component (crate or module), all
> *unsafe* code must be encapsulated in such a way that:
>
> * either it exposes a safe behavior to the user, in which no safe interaction
>   can result in UB (undefined behavior);
> * or it exposes features marked as unsafe whose usage conditions
>   (preconditions, sequencing, etc.) are exhaustively documented.

Thus, a function using `unsafe` operations can be *safe* if the `unsafe`
operations do not present any *UB* (undefined behavior) given the component's
invariants (typically the type invariant for a method). Conversely, a function
without an `unsafe` block must be marked as `unsafe` if it breaks these
invariants. The choice and knowledge of these invariants are therefore crucial
for secure development.

### Trust relation between *safe* and *unsafe*

#### Principle

In Rust paradigm,

> `unsafe`-free code cannot go wrong

which means it cannot result in *UB*.
This property is lost when the developers use *unsafe* code, that is why they are responsible of not producing *UB* in any scenario.
Consequently, even *safe* function must be handled carefully in *unsafe* context.

#### Example

Suppose one wants to propose an API find object of a given type in the memory.
This API could ask implementing the following trait

```rust
trait Locatable {
    /// Find object of type `Self` in the buffer `buf`.
    /// Returns the index of the first byte representing
    /// an object of type `Self`
    fn locate_instance_into(buf: &[u8]) -> Option<usize>;
}
```

This trait can be implemented **without** `unsafe`.

For instance, `bool` type can implement this trait as follows:

```rust,ignore
{{#include ../../../examples/unsafe2/src/ok.rs:7:11}}
```

Moreover the function searching a `Locatable` type in the memory could be the following:

```rust,ignore
{{#include ../../../examples/unsafe2/src/ok.rs:13:19}}
```

<div class="warning">

This implementation is harmful for two reasons:

* If the `Locatable` implementation does not give the index of an object of type `T`, the `read_unaligned` may produce an *UB*.
* If the `Locatable` implementation gives an out-of-bounds index or an index for which part of the object is out of bound, the subsequent buffer overflow is an *UB*.

</div>

For instance, the following `Locatable` implementation is wrong **but** it is the responsibility of the API maker to take it into account.

```rust
{{#include ../../../examples/unsafe2/src/overflow.rs:7:11}}
```

The following program produces a *UB*.

```rust,ignore
{{#include ../../../examples/unsafe2/src/overflow.rs:21:25}}
```

The UB detecting tool `miri` reports the following :

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
cannot assume *safe* functions/traits they
use are well implemented, and thus must prevent *UB*
in case these *safe* functions have bad behavior.

If they cannot protect their function against badly implemented *safe* functions/traits, they could either

* mark the function they *write* as `unsafe`: thus it is the user's responsibility to feed this function with correct arguments (by checking *unsafe* function documentation),
* mark the traits they *use* as `unsafe` : thus it is user's responsibility to implement the trait properly (again reading the trait documentation).

#### References

* <https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html>
