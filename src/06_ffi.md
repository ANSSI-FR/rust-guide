# Interfacing (FFI)

Rust approach to interfacing with other programs relies on a strong
compatibility with C. However, this boundary is by its very nature `unsafe`.

Functions that are marked `extern` are made compatible with C code at the
compilation. They may be called from C code with any parameter.
The exact syntax is `extern "<ABI>"` where ABI is a calling convention and
depends on the target platform. The default one is `C` which corresponds to
a standard C calling convention on the target platform.

```rust
// export a C-compatible function
#[no_mangle]
pub extern "C" fn mylib_f(param: u32) -> i32 {
    if param == 0xCAFEBABE { 0 } else { -1 }
}
```

For the function `mylib_f` to be accessible with the same name, the function
must also be annotated with the `#[no_mangle]` attribute.

Conversely, one can call C functions from Rust if they are declared in an
`extern` block:

```rust
// import an external function from libc
extern "C" {
    fn abs(args: i32) -> i32;
}

fn main() {
    let x = -1;
    println!("{} {}\n", x, unsafe { abs(x) });
}
```

> **Note**
>
> Any foreign function imported in Rust through an `extern` block is
> **automatically `unsafe`**. That is why, any call to a foreign function
> must be done from an `unsafe` context.

`extern` blocks may also contain foreign global variable declarations prefixed
with the `static` keyword:

```rust
//! A direct way to access environment variables (on Unix).
//! Should not be used! Not thread safe, have a look at `std::env`!

extern {
    // Libc global variable
    #[link_name = "environ"]
    static libc_environ: *const *const std::os::raw::c_char;
}

fn main() {
    let mut next = unsafe { libc_environ };
    while !next.is_null() && !unsafe { *next }.is_null() {
        let env = unsafe { std::ffi::CStr::from_ptr(*next) }
            .to_str()
            .unwrap_or("<invalid>");
        println!("{}", env);
        next = unsafe { next.offset(1) };
    }
}
```

## Data layout

Rust provides no short or long term guarantees with respect to how the data is
laid out in the memory. The only way to make data compatible with a foreign
language is through explicit use of a C-compatible data layout with the `repr`
attribute. For instance, the following Rust types:

```rust
#[repr(C)]
struct Data {
    a: u32,
    b: u16,
    c: u64,
}
#[repr(C, packed)]
struct PackedData {
    a: u32,
    b: u16,
    c: u64,
}
```

are compatible with the following C types:

```c
struct Data {
    uint32_t a;
    uint16_t b;
    uint64_t c;
};
__attribute__((packed))
struct PackedData {
    uint32_t a;
    uint16_t b;
    uint64_t c;
}
```

> ### Rule {{#check FFI-CTYPE | Use only C-compatible types in FFI}}
>
> In a secure Rust development, only C-compatible types must be used as
> parameter or return type of imported or exported functions and as types of
> imported or exported global variables.

The following types are considered C-compatible:

- integral or floating point primitive types,
- `repr(C)`-annotated `struct`,
- `repr(C)` or `repr(Int)`-annotated `enum` with only fieldless variants (where
  `Int` is an integral primitive type),
- pointers.

The following types are not C-compatible:

- Dynamically sized types,
- Trait objects,
- Enums with fields,
- Tuples (but `repr(C)` tuple structures are OK).

Some types are compatibles with some caveats:

- Zero-sized types, which is really zero sized (which is let unspecified in C
  and contradicts the C++ specification),
- `repr(C)`, `repr(C, Int)`, or `repr(Int)`-annotated enum with fields
  ([RFC 2195]).

References:

- [RFC 2195 (Really tagged unions)][RFC 2195]
- [Rust Reference: Type Layout]
- [Rust Book: Unsafe Rust]

[RFC 2195]: https://github.com/rust-lang/rfcs/blob/master/text/2195-really-tagged-unions.md
[Rust Reference: Type Layout]: https://doc.rust-lang.org/reference/type-layout.html
[Rust Book: Unsafe Rust]: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html

## Pointers and references

Although there are allowed by the Rust compiler, the use of Rust references in
FFI should be avoided. It is particularly true when binding to and from C,
because C has no references (in the sense of valid pointers):

> ### Rule {{#check FFI-NOREF | Do not use unchecked references in FFI}}
>
> In a secure Rust development, there must be no Rust reference types when
> interfacing with C, in particular, in:
>
> - the prototype of an imported or exported function,
> - the type of an imported or exported global variables,
>
> directly or indirectly (in a non-opaque subtype).

In case of binding to or from C++, it is possible to use Rust references on one
side, and C++ references on the other. However, the C++ code should be checked
against pointer/reference confusion.

> ### Rule {{#check FFI-CKPTR | Check foreign pointers}}
>
> In a secure Rust development, any Rust code that dereferences a foreign
> pointer must check their validity beforehand.
> In particular, pointers must be checked to be non-null before any use.

Stronger approaches such as _tagged pointers_ are possible if the pointed value
is only manipulated from the Rust side of an FFI boundary.

> ### Note
>
> It is possible to use the type `Option<&T>` (resp. `Option<&mut T>`) instead
> of pointers with nullity checks. However, because it is less explicit and
> relies on Rust “nullable pointer optimization”, it is not advisable at this
> point.

## Type consistency

> ### Rule {{#check FFI-TCONS | Use consistent types at FFI boundaries}}
>
> Types must be consistent on each side of the FFI boundary.
>
> Although some details may be hidden on one side with respect to the other
> (typically to make a type opaque), types on both sides must have the same size
> and the same alignment requirement.

Concerning enums with fields in particular, the corresponding types in C (or
C++) are not obvious, cf. [RFC 2195].

Automated tools to generate bindings, such as [rust-bindgen] or
[cbindgen], may be of help in making types consistent between C and Rust.

[rust-bindgen]: https://github.com/rust-lang/rust-bindgen
[cbindgen]: https://github.com/eqrion/cbindgen

## Panics with foreign code

When calling Rust code from another language (e.g. C), the Rust code must
be careful to never panic.
Unwinding from Rust code into foreign code results is **undefined behavior**.

> ### Rule {{#check FFI-NOPANIC | Handle `panic!` correctly in FFI}}
>
> Rust code called from FFI must either ensure the function cannot panic, or use
> a panic handling mechanism (such as `std::panic::catch_unwind`,
> `std::panic::set_hook`, `#[panic_handler]`) to ensure the rust code will not
> abort or return in an unstable state.

Note that `catch_unwind` will only catch unwinding panics, not those that abort
the process.

```rust ignore
use std::panic::catch_unwind;
use rand;

pub fn may_panic() {
    if rand::random() {
        panic!("panic happens");
    }
}

#[no_mangle]
pub extern fn no_panic() -> i32 {
    let result = catch_unwind(||may_panic());
    match result {
        Ok(_) => 0,
        Err(_) => -1,
    }
}
```

### `no_std`

In the case of `#![no_std]` program, a panic handler (`#[panic_handler]`) must
be defined to ensure security. The panic handler should be written with great
care in order to ensure both the safety and security of the program.

Another approach is to simply ensure that there is no use of `panic!` with the
[`panic-never`] crate. Like [`no-panic`], [`panic-never`] relies on a linking
trick: the linker fails if a non-trivially-dead branch leads to `panic!`.

[`panic-never`]: https://crates.io/crates/panic-never
[`no-panic`]: https://github.com/dtolnay/no-panic

## Binding a foreign library in Rust

> ### Recommendation {{#check FFI-SAFEWRAPPING | Provide safe wrapping to foreign library}}
>
> Interfacing a library written in another language in Rust should be done in
> two parts:
>
> - a low-level, possibly *hidden*, module that closely translates the original
>   C API into `extern` blocks,
> - a safe wrapping module that ensures memory safety and security invariants at
>   the Rust level.
>
> If the low-level API is exposed to the world, it should be done in a dedicated
> crate with a name of the form `*-sys`.

The crate [rust-bindgen] may be used to automatically generate the low-level
part of the binding from C header files.

<mark>TODO</mark> example

## Binding a Rust library in another language

> ### Recommendation {{#check FFI-CAPI | Expose dedicated C-compatible API only}}
>
> In a secure Rust development, exposing a Rust library to a foreign language
> should only be done through a **dedicated C-compatible API**.

The crate [cbindgen] may be used to automatically generate C or C++ bindings to
the Rust C-compatible API of a Rust library.

### Minimal example of a C-exported Rust library

`src/lib.rs`:

```rust
/// Opaque counter
pub struct Counter(u32);

impl Counter {
    /// Create a counter (initially at 0)
    fn new() -> Self {
        Self(0)
    }
    /// Get the current value of the counter
    fn get(&self) -> u32 {
        self.0
    }
    /// Increment the value of the counter if there's no overflow
    fn incr(&mut self) -> bool {
        if let Some(n) = self.0.checked_add(1) {
            self.0 = n;
            true
        } else {
            false
        }
    }
}

// C-compatible API

#[no_mangle]
pub extern fn counter_create() -> *mut Counter {
    Box::into_raw(Box::new(Counter::new()))
}

#[no_mangle]
pub extern fn counter_incr(counter: *mut Counter) -> std::os::raw::c_int {
    if let Some(counter) = unsafe { counter.as_mut() } {
        if counter.incr() {
            0
        } else {
            -1
        }
    } else {
        -2
    }
}

#[no_mangle]
pub extern fn counter_get(counter: *const Counter) -> u32 {
    if let Some(counter) = unsafe { counter.as_ref() } {
        return counter.get();
    }
    return 0;
}

#[no_mangle]
pub extern fn counter_destroy(counter: *mut Counter) -> std::os::raw::c_int {
    if !counter.is_null() {
        let _ = unsafe { Box::from_raw(counter) }; // get box and drop
        return 0;
    }
    return -1;
}
```

Using [cbindgen] (`[cbindgen] -l c > counter.h`), one can generate a consistent
C header, `counter.h`:

```c
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Counter Counter;

Counter *counter_create(void);

int counter_destroy(Counter *counter);

uint32_t counter_get(const Counter *counter);

int counter_incr(Counter *counter);
```

`counter_main.c`:

```c
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>

#include "counter.h"

int main(int argc, const char** argv) {
    Counter* c = counter_create();

    if (argc < 2) {
        return -1;
    }
    size_t n = (size_t)strtoull(argv[1], NULL, 10);

    for (size_t i=0; i < n; i++) {
        if (counter_incr(c) != 0) {
            printf("overflow\n");
            return -1;
        }
    }

    printf("%" PRIu32 "\n", counter_get(c));
    counter_destroy(c);

    return 0;
}
```
