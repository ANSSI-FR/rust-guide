# Rust safety: common programming pitfalls

Rust's moto is "Hack without fear", and the ownership + borrow checker + `Send`/`Sync` make a good job at catching bugs at compile time (far better than other programming languages, for sure!).

Still, it is not a reason to be reckless; here is a **non-exhaustive** list of things to pay attention to when programming in Rust.

## With `unsafe` Rust

Correct usage of `unsafe` code requires a good understanding of ALL the possible pitfalls (and who knows them?), for instance (but not limited to):

- standard raw memory management pitfalls (typical C pitfalls):
    - buffer overflow,

    - use-after-free (e.g. double-free),

    - dereferencing an invalid pointer (e.g. the result of (non-`unsafe`)pointer-based arithmetic (which may involve indices) that underflows/overflows),

    - explicitly using (reading) uninitialized memory,

    - and so on;


- FFI pitfalls:
    - allocating/freeing memory across the different sides of FFI boundaries (unless the same allocator is used, but that is a very fragile and error-prone assumption to rely on). For instance, calling C `free()` on a pointer obtained from Rust `Box::into_raw()`;

    - `panic!`-ing (unwinding the stack) across FFI boundaries.

      Since many Rust functions may `panic!` (e.g. `.unwrap()`, `.expect()`, integer overflow with `overflow-checks` enabled, out-of-bound index accesses, even `println!()` if the underlying `::std::io::stdout().write()` call fails!), Rust-C FFI wrappers should call all rust code within [`::std::panic::catch_unwind()`](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html);

    - `transmute`-ing a C `enum` (that is, an `int`) to a Rust `enum` (e.g. if the Rust `enum` has `n` variants, `transmute`-ing any `int` value not in the `[0; n[` range is UB). Instead, use a `match` or the [`enum-primitive-derive` crate](https://crates.io/crates/enum-primitive-derive)


- (Too) generic code pitfalls:
    - [generic memory management code handling ZST](https://doc.rust-lang.org/nomicon/vec-zsts.html) or references to generic data converting it to raw pointers without thinking about references to DST being fat pointers and thus containing more data than just a single memory address;

    - [uninitialized/zeroed memory](https://doc.rust-lang.org/std/mem/fn.uninitialized.html) vs. compiler optimizations based on bit-layouts (i.e. **the compiler may implicitly rely on the properties of data** that you have `unsafe`-ly (un)initialized. For instance, with a non-nullable type such as a reference, [`Some::<&_>(::std::mem::zeroed())`](https://play.rust-lang.org/?version=stable&mode=release&edition=2018&gist=3e8dd3dde698d820bdb93d531eb9c7be) is UB).

      If this situation seems unlikely, know that all it takes is using `mem::uninitialized()` / `mem::zeroed()`, a generic type and an `enum` somewhere. See also the aforementioned `transmute`-ing of `int`egers into Rust `enum`s;

- ["exception" safety](https://doc.rust-lang.org/nomicon/exception-safety.html);

## Within non-`unsafe` Rust (⚠ non-`unsafe` does not imply "safe" ⚠)
Non-`unsafe` code has its own share of bugs (and thus security issues) as well, such as:

- logic bugs (no language can prevent that);

- integer overflows: (e.g. downcasting, sign-extending instead of zero-extending or vice-versa, UB from overflowing integer arithmetic (unchecked by default in `release` code for those not aware of [the available tools to handle them](https://huonw.github.io/blog/2016/04/myths-and-legends-about-integer-overflow-in-rust/))), which may, in turn, lead to logic bugs.

  It is especially dangerous with indices (when you think about it, there is little difference between an index and a raw pointer: just the fact that "dereferencing" an invalid index usually causes a panic instead of straight UB (and only when the index-accessing function does bound-check);

- Deadlocks;

- [Poisoning](https://doc.rust-lang.org/nomicon/poisoning.html);

- OS-related data races (e.g., files and folders). These may, in turn, lead to logic bugs;

- Hash-based collections using a DOS-vulnerable hashing logic ([`::std::hash::Hasher`](https://doc.rust-lang.org/std/hash/trait.Hasher.html));

- Memory leaks (which may, in turn, lead to memory exhaustion and a crash (not even a panic!));
