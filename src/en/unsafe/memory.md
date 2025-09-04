# Memory management

<!-- ## About Rust memory safety -->

<!--
<mark>TODO</mark>: explain safe allocations/deallocations, ownership/borrowing,
and identify language constructs that may break memory safety (for instance,
unsound behaviors in older versions of the compiler).
-->

In the vast majority of cases, in non-`unsafe` Rust (i.e., code that does not use `unsafe`), the compiler **automatically** determines when it can release the memory occupied by a value in the program.
But, as [noted earlier](04_language.md#rust-guarantees), this is not
a guarantee: code without `unsafe` can still lead to memory leaks. Therefore, some of the
rules presented in this chapter are not strictly related to the `unsafe` keyword. However,

> even if a function in the following is not `unsafe`,
> it should only be used in Rust *unsafe*.

## `forget` and memory leaks

While the usual way for memory to be reclaimed is for a variable to go out of
scope, Rust provides special functions to manually reclaim memory: `forget` and
`drop` of the `std::mem` module (or `core::mem`). While `drop` simply triggers
an early memory reclamation that calls associated destructors when needed,
`forget` skips any call to the destructors.

```rust
let pair = ('↑', 0xBADD_CAFEu32);
drop(pair); // here `forget` would be equivalent (no destructor to call)
```

Both functions are **memory safe** in Rust. However, `forget` will make any
resource managed by the value unreachable and unclaimed.

```rust
# use std::mem::forget;
let s = String::from("Hello");
forget(s); // Leak memory
```

In particular, using `forget` may result in not releasing critical resources,
leading to deadlocks or not erasing sensitive data from the memory. This is why
`forget` is **unsecure**.

> **Rule {{#check MEM-FORGET | Do not use `forget`}}**
>
> In a secure Rust development, the `forget` function of `std::mem`
> (`core::mem`) must not be used.

<!-- -->

> **Recommendation {{#check MEM-FORGET-LINT | Use clippy lint to detect use of `forget`}}**
>
> The lint `mem_forget` of Clippy may be used to automatically detect any use of
> `forget`. To enforce the absence of `forget` in a crate, add the following
> line at the top of the root file (usually `src/lib.rs` or `src/main.rs`):
>
> ```rust,noplaypen,ignore
> #![deny(clippy::mem_forget)]
> ```

The standard library includes other way to *forget* dropping values:

- `Box::leak` to leak a resource,
- `Box::into_raw` to exploit the value in some unsafe code, notably in FFI,
- `ManuallyDrop` (in `std::mem` or `core::mem`) to enforce manual release of some value.

Those alternatives may lead to the same security issue but they have the
additional benefit of making their goal obvious.

> **Rule {{#check MEM-LEAK | Do not use `leak` function}}**
>
> In a secure Rust development, the code must not leak memory or resource in
> particular via `Box::leak`.

`ManuallyDrop` and `Box::into_raw` shift the release responsibility from the
compiler to the developer.

> **Rule {{#check MEM-MANUALLYDROP | Do release value wrapped in `ManuallyDrop`}}**
>
> In a secure Rust development, any value wrapped in `ManuallyDrop` must be
> unwrapped to allow for automatic release (`ManuallyDrop::into_inner`)
> or manually released (unsafe `ManuallyDrop::drop`).

<!-- -->

## Raw pointers

These pointers are mainly used for C pointers. They do not have the same protections
as *smart pointers* and often have to be used in `unsafe` context. For instance, freeing 
raw pointers must be done manually without Rust guaranties.

> **Rule {{#check MEM-NORAWPOINTER | Do no convert smart pointer into raw pointer in Rust without `unsafe`}}**
>
> In a secure Rust development without `unsafe`, references and *smart pointers*
> should not be converted into *raw pointers*. For instance, functions `into_raw` ou `into_non_null`
> of smart pointers `Box`, `Rc`, `Arc` or `Weak` should not be used.

> **Rule {{#check MEM-INTOFROMRAW | Always call `from_raw` on `into_raw`ed value}}**
>
> In a secure Rust development, any pointer created with a call to `into_raw`
> (or `into_non_null`) from one of the following types:
>
> - `std::boxed::Box` (or `alloc::boxed::Box`),
> - `std::rc::Rc` (or `alloc::rc::Rc`),
> - `std::rc::Weak` (or `alloc::rc::Weak`),
> - `std::sync::Arc` (or `alloc::sync::Arc`),
> - `std::sync::Weak` (or `alloc::sync::Weak`),
> - `std::ffi::CString`,
> - `std::ffi::OsString`,
>
> must eventually be transformed into a value with a call to the respective
> `from_raw` to allow for their reclamation.
>
> ```rust
> let boxed = Box::new(String::from("Crab"));
> let raw_ptr = unsafe { Box::into_raw(boxed) };
> let _ = unsafe { Box::from_raw(raw_ptr) }; // will be freed
> ```

The converse is also true! That is, `from_raw` should be call **only** on `into_raw`ed value. For instance,
`Rc` smart pointers [explicitly request for this condition](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.from_raw)
and, for `Box` smart pointers, conversion of C pointers into `Box` is [discouraged](https://doc.rust-lang.org/std/boxed/index.html#memory-layout).

> **Rule {{#check MEM-INTOFROMRAW | Call `from_raw` *only* on `into_raw`ed value}}**
>
> In a secure Rust development, `from_raw` should only be called on `into_raw`ed values.

<!-- -->

> **Note**
>
> In the case of `Box::into_raw`, manual cleanup is possible but a lot more
> complicated than re-boxing the raw pointer and should be avoided:
>
> ```rust
> // Excerpt from the standard library documentation
> use std::alloc::{dealloc, Layout};
> use std::ptr;
>
> let x = Box::new(String::from("Hello"));
> let p = Box::into_raw(x);
> unsafe {
>     ptr::drop_in_place(p);
>     dealloc(p as *mut u8, Layout::new::<String>());
> }
> ```
>
> Because the other types (`Rc` and `Arc`) are opaque and more complex, manual
> cleanup is not possible.

## Uninitialized memory

By default, Rust forces all values to be initialized, preventing the use of
uninitialized memory (except when using `std::mem::uninitialized` or
`std::mem::MaybeUninit`).

> **Rule {{#check MEM-UNINIT | Do not use uninitialized memory}}**
>
> The `std::mem::uninitialized` function (deprecated 1.38) must never be used.
> Each usage of the `std::mem::MaybeUninit` type (stabilized 1.36) must be explicitly
> justified when necessary.

The use of uninitialized memory may result in two distinct security issues:

- drop of uninitialized memory (also a memory safety issue),
- non-drop of initialized memory.

> **Note**
>
> `std::mem::MaybeUninit` is an improvement over `std::mem::uninitialized`.
> Indeed, it makes dropping uninitialized values a lot more difficult.
> However, it does not change the second issue: the non-drop of an initialized
> memory remains. It is problematic, in particular when considering
> the use of `Drop` to erase sensitive memory.

## Cyclic reference counted pointers (`Rc` and `Arc`)

Combining [interior mutability](https://doc.rust-lang.org/reference/interior-mutability.html), recursivity and reference counted pointer into type definitions is unsafe. It can produce memory leaks which can result in DDoS attacks or leaking secrets.

The following example shows such a memory leak in safe Rust:

```rust
use std::{cell::Cell, rc::Rc};

struct LinkedStruct {
    other: Cell<Option<Rc<LinkedStruct>>>,
}

fn main() {
    println!("Hello, world!");
    let a = Rc::new(LinkedStruct {
        other: Cell::new(None),
    });
    let b = Rc::new(LinkedStruct {
        other: Cell::new(None),
    });
    let aa = a.clone();
    let bb = b.clone();
    a.other.set(Some(bb));
    b.other.set(Some(aa));
}
```

Memory leak is shown with `valgrind`:

```
$ valgrind --leak-check=full target/release/safe-rust-leak 
==153637== Memcheck, a memory error detector
==153637== Copyright (C) 2002-2022, and GNU GPL'd, by Julian Seward et al.
==153637== Using Valgrind-3.19.0 and LibVEX; rerun with -h for copyright info
==153637== Command: target/release/safe-rust-leak
==153637== 
Hello, world!
==153637== 
==153637== HEAP SUMMARY:
==153637==     in use at exit: 48 bytes in 2 blocks
==153637==   total heap usage: 10 allocs, 8 frees, 3,144 bytes allocated
==153637== 
==153637== 48 (24 direct, 24 indirect) bytes in 1 blocks are definitely lost in loss record 2 of 2
==153637==    at 0x48417B4: malloc (vg_replace_malloc.c:381)
==153637==    by 0x10F8D4: safe_rust_leak::main (in /home/toto/src/safe-rust-leak/target/release/safe-rust-leak)
==153637==    by 0x10F7E2: std::sys::backtrace::__rust_begin_short_backtrace (in /home/toto/src/safe-rust-leak/target/release/safe-rust-leak)
==153637==    by 0x10F7D8: std::rt::lang_start::{{closure}} (in /home/toto/src/safe-rust-leak/target/release/safe-rust-leak)
==153637==    by 0x12A90F: std::rt::lang_start_internal (in /home/toto/src/safe-rust-leak/target/release/safe-rust-leak)
==153637==    by 0x10FA54: main (in /home/toto/src/safe-rust-leak/target/release/safe-rust-leak)
==153637== 
==153637== LEAK SUMMARY:
==153637==    definitely lost: 24 bytes in 1 blocks
==153637==    indirectly lost: 24 bytes in 1 blocks
==153637==      possibly lost: 0 bytes in 0 blocks
==153637==    still reachable: 0 bytes in 0 blocks
==153637==         suppressed: 0 bytes in 0 blocks
==153637== 
==153637== For lists of detected and suppressed errors, rerun with: -s
==153637== ERROR SUMMARY: 1 errors from 1 contexts (suppressed: 0 from 0)
```

> **Rule {{#check MEM-MUT-REC-RC | Avoid cyclic reference counted pointers}}**
>
> Avoid recursive types whose recursivity uses reference counted pointers together with interior mutability.