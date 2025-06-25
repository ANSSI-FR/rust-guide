# Memory management

<!-- ## About Rust memory safety -->

<!--
<mark>TODO</mark>: explain safe allocations/deallocations, ownership/borrowing,
and identify language constructs that may break memory safety (for instance,
unsound behaviors in older versions of the compiler).
-->

## Forget and memory leaks

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

In particular, using `forget` may result in not releasing critical resources
leading to deadlocks or not erasing sensitive data from the memory. That is why,
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

> **Rule {{#check MEM-LEAK | Do not leak memory}}**
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

> **Rule {{#check MEM-INTOFROMRAW | Always call `from_raw` on `into_raw`ed value}}**
>
> In a secure Rust development, any pointer created with a call to `into_raw`
> (or `into_raw_nonnull`) from one of the following types:
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
uninitialized memory (except if using `std::mem::uninitialized` or
`std::mem::MaybeUninit`).

> **Rule {{#check MEM-UNINIT | Do not use uninitialized memory}}**
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

## Secure memory zeroing for sensitive information

Zeroing memory is useful for sensitive variables, especially if the
Rust code is used through FFI.

> **Rule {{#check MEM-ZERO | Zero out memory of sensitive data after use}}**
>
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
