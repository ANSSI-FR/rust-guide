# Foreign Function Interface (FFI)

The Rust approach to interfacing with other languages relies on a strong
compatibility with C. However, this boundary is by its very nature **unsafe**
(see [Rust Book: Unsafe Rust]).

[Rust Book: Unsafe Rust]: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html

Functions that are marked `extern` are made compatible with C code during
compilation. They may be called from C code with any parameter values.
The exact syntax is `extern "<ABI>"` where ABI is a calling convention and
depends on the target platform. The default one is `C` which corresponds to
a standard C calling convention on the target platform.

```rust
// export a C-compatible function
#[no_mangle]
unsafe extern "C" fn mylib_f(param: u32) -> i32 {
    if param == 0xCAFEBABE { 0 } else { -1 }
}
```

For the function `mylib_f` to be accessible with the same name, the function
must also be annotated with the `#[no_mangle]` attribute.

Conversely, one can call C functions from Rust if they are declared in an
`extern` block:

```rust
use std::os::raw::c_int;
// import an external function from libc
extern "C" {
    fn abs(args: c_int) -> c_int;
}

fn main() {
    let x = -1;
    println!("{} {}\n", x, unsafe { abs(x) });
}
```

> ### Note
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

## Typing

Typing is the way Rust ensures memory safety. When interfacing with other
languages, which may not offer the same guarantee, the choice of types in the
binding is essential to maintain the memory safety.

### Data layout

Rust provides no short or long term guarantees with respect to how the data is
laid out in the memory. The only way to make data compatible with a foreign
language is through explicit use of a C-compatible data layout with the `repr`
attribute (see [Rust Reference: Type Layout]). For instance, the following Rust
types:

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
>
> The lone exception is types that are considered **opaque** on the foreign
> side.

The following types are considered C-compatible:

- integral or floating point primitive types,
- `repr(C)`-annotated `struct`,
- `repr(C)` or `repr(Int)`-annotated `enum` with at least one variant and only
  fieldless variants (where `Int` is an integral primitive type),
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
  (see [RFC 2195]).

[RFC 2195]: https://rust-lang.github.io/rfcs/2195-really-tagged-unions.html
[Rust Reference: Type Layout]: https://doc.rust-lang.org/reference/type-layout.html

### Type consistency

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

> ### Recommendation {{#check FFI-AUTOMATE | Use automatic binding generator tools}}
>
> In a secure Rust development, automated generation tools should be used to
> generate bindings when possible and to maintain them continually.

<!-- -->

> ### Warning
>
> For binding C/C++ to Rust, [rust-bindgen] is able to automatically generate
> the low-level binding. A high-level safe binding is highly recommended (see
> Recommendation [FFI-SAFEWRAPPING](#FFI-SAFEWRAPPING)).
> Also some options of rust-bindgen may result in dangerous translations, in
> particular `rustified_enum`.

[rust-bindgen]: https://crates.io/crates/bindgen
[cbindgen]: https://crates.io/crates/cbindgen

### Platform-dependent types

When interfacing with a foreign language, like C or C++, it is often required
to use platform-dependent types such as C's `int`, `long`, etc.

In addition to `c_void` in `std::ffi` (or `core::ffi`) for `void`, the standard
library offers portable type aliases in `std:os::raw` (or `core::os::raw`):

- `c_char` for `char` (either `i8` or `u8`),
- `c_schar` for `signed char` (always `i8`),
- `c_uchar` for `unsigned char` (always `u8`),
- `c_short` for `short`,
- `c_ushort` for `unsigned short`,
- `c_int` for `int`,
- `c_uint` for `unsigned int`,
- `c_long` for `long`,
- `c_ulong` for `unsigned long`,
- `c_longlong` for `long long`,
- `c_ulonglong` for `unsigned long long`,
- `c_float` for `float` (always `f32`),
- `c_double` for `double` (always `f64`).

The [libc] crate offers more C compatible types that cover almost exhaustively
the C standard library.

> ### Rule {{#check FFI-PFTYPE | Use portable aliases `c_*` when binding to platform-dependent types}}
>
> In a secure Rust development, when interfacing with foreign code that
> uses platform-dependent types, such as C's `int` and `long`, Rust code must
> use portable type aliases, such as provided by the standard library or the
> [libc] crate, rather than platform-specific types, except if
> the binding is automatically generated for each platform (see Note below).

<!-- -->

> ### Note
>
> Automatic binding generation tools (e.g. [cbindgen], [rust-bindgen]) are able
> to ensure type consistency on a specific platform. They should be used during
> the build process for each target to ensure that the generation is sound for
> the specific target platform.

[libc]: https://crates.io/crates/libc

### Non-robust types: references, function pointers, enums

A *trap representation* of a particular type is a representation (pattern of
bits) that respects the type's representation constraints (such as size and
alignment) but does not represent a valid value of this type and leads to
undefined behavior.

In simple terms, if a Rust variable is set to such an invalid value,
anything can happen from a simple program crash to arbitrary code execution.
When writing safe Rust, this cannot happen (except through a bug in the Rust
compiler). However, when writing unsafe Rust and in particular in FFI, it is
really easy.

In the following, **non-robust types** are types that have such trap
representations (at least one). A lot of Rust types are non-robust, even among
the C-compatible types:

- `bool` (1 byte, 256 representations, only 2 valid ones),
- references,
- function pointers,
- enums,
- floats (even if almost every language have the same understanding of what is
  a valid float),
- compound types that contain a field of a non-robust type.

On the other hand, integer types (`u*`/`i*`), packed compound types that contain
no non-robust fields, for instance are *robust types*.

Non-robust types are a difficulty when interfacing two languages. It revolves
into deciding **which language of the two is responsible in asserting the
validity of boundary-crossing values** and how to do it.

> ### Rule {{#check FFI-CKNONROBUST | Do not use unchecked non-robust foreign values}}
>
> In a secure Rust development, there must not be any use of *unchecked* foreign
> values of non-robust types.
>
> In other words, either Rust translates robust types to non-robust types
> through explicit checking or the foreign side offers strong guarantees of the
> validity of the value.

<!-- -->

> ### Recommendation {{#check FFI-CKINRUST | Check foreign values in Rust}}
>
> In a secure Rust development, the validity checks of foreign values should
> be done in Rust when possible.

Those generic rules are to be adapted to a specific foreign language or for the
associated risks. Concerning languages, C is particularly unfit to offer
guarantees about validity. However, Rust is not the only language to offer
strong guarantees. For instance, some C++ subset (without reinterpretation)
allows developers to do lot of type checking. Because Rust natively separates
the safe and unsafe segments, the recommendation is to always use Rust to check
when possible. Concerning risks, the most dangerous types are references,
function references, and enums, and are discussed below.

> ### Warning
>
> Rust's `bool` has been made equivalent to C99's `_Bool` (aliased as `bool`
> in `<stdbool.h>`) and C++'s `bool`. However, loading a value other than 0 and
> 1 as a `_Bool`/`bool` is an undefined behavior *on both sides*.
> Safe Rust ensures that. Standard-compliant C and C++ compilers ensure that no
> value but 0 and 1 can be *stored* in a `_Bool`/`bool` value but cannot
> guarantee the absence of an *incorrect reinterpretation* (e.g., union types,
> pointer cast). To detect such a bad reinterpretation, sanitizers such as
> LLVM's `-fsanitize=bool` may be used.

#### References and pointers

Although they are allowed by the Rust compiler, the use of Rust references in
FFI may break Rust's memory safety. Because their “unsafety” is more explicit,
pointers are preferred over Rust references when binding to another language.

On the one hand, reference types are very non-robust: they allow only pointers
to valid memory objects. Any deviation leads to undefined behavior.

When binding to and from C, the problem is particularly severe because C has
no references (in the sense of valid pointers) and the compiler does not offer
any safety guarantee.

When binding with C++, Rust references may be bound to C++ references in
practice even though the actual ABI of an `extern "C"` function in C++ with
references is “implementation-defined”. Also, the C++ code should be checked
against pointer/reference confusion.

Rust references may be used reasonably with other C-compatible languages
including C variants allowing for non-null type checking, e.g. Microsoft SAL
annotated code.

On the other hand, Rust's *pointer types* may also lead to undefined behaviors
but are more verifiable, mostly against `std/core::ptr::null()` (C's `(void*)0`)
but also in some context against a known valid memory range (particularly in
embedded systems or kernel-level programming). Another advantage of using Rust
pointers in FFI is that any load of the pointed value is clearly marked inside
an `unsafe` block or function.

> ### Recommendation {{#check FFI-NOREF | Do not use reference types but pointer types}}
>
> In a secure Rust development, the Rust code should not use references types
> but pointer types.
>
> Exceptions include:
>
> - Rust references that are opaque in the foreign language and only manipulated
>   from the Rust side,
> - `Option`-wrapped references (see Note below),
> - references bound to foreign safe references, e.g. from some augmented C
>   variants or from C++ compiled in an environment where `extern "C"`
>   references are encoded as pointers.

<!-- -->

> ### Rule {{#check FFI-CKREF | Do not use unchecked foreign references}}
>
> In a secure Rust development, every foreign references that is transmitted to
> Rust through FFI must be **checked on the foreign side** either automatically
> (for instance, by a compiler) or manually.
> 
> Exceptions include Rust references in an opaque wrapping that is created
> and manipulated only from the Rust side and `Option`-wrapped references
> (see Note below).

<!-- -->

> ### Rule {{#check FFI-CKPTR | Check foreign pointers}}
>
> In a secure Rust development, any Rust code that dereferences a foreign
> pointer must check their validity beforehand.
> In particular, pointers must be checked to be non-null before any use.
>
> Stronger approaches are advisable when possible. They includes checking
> pointers against known valid memory range or tagging (or signing) pointers
> (particularly applicable if the pointed value is only manipulated from Rust).

The following code a simple example of foreign pointer use in an exported Rust
function:

```rust,noplaypen
/// Add in place
#[no_mangle]
pub unsafe extern fn add_in_place(a: *mut u32, b: u32) {
    // checks for nullity of `a`
    // and takes a mutable reference on it if it's non-null
    if let Some(a) = a.as_mut() {
        *a += b
    }
}
```

Note that the methods `as_ref` and `as_mut` (for mutable pointers) allows easy
access to a reference while ensuring a null check in a very *Rusty* way.
On the other side in C, it can be used as follows:

```c
#include <stdint.h>
#include <inttypes.h>

//! Add in place
void add_in_place(uint32_t *a, uint32_t b);

int main() {
    uint32_t x = 25;
    add_in_place(&x, 17);
    printf("%" PRIu32 " == 42", x);
    return 0;
}
```

> ### Note
>
> `Option<&T>` and `Option<&mut T>` for any `T: Sized` are allowable in FFI
> instead of pointers with explicit nullity checks. Due to the Rust guaranteed
> “nullable pointer optimization”, a nullable pointer is acceptable on the C
> side. The C `NULL` is understood as `None` in Rust while a non-null
> pointer is encapsulated in `Some`. While quite ergonomic, this feature does
> not allow stronger validations such as memory range checking.

#### Function pointers

Function pointers that cross FFI boundaries may ultimately lead to arbitrary code
execution and represents a real security risks.

> ### Rule {{#check FFI-MARKEDFUNPTR | Mark function pointer types in FFI as `extern` and `unsafe`}}
>
> In a secure Rust development, any function pointer types at the FFI boundary
> must be marked `extern` (possibly with the specific ABI) and `unsafe`.

Function pointers in Rust are a lot more similar to references than they are
to normal pointers. In particular, the validity of function pointers cannot be
checked directly on the Rust side. However, Rust offers two alternative
possibilities:

- use `Option`-wrapped function pointer and check against `null`:

  ```rust,noplaypen
  #[no_mangle]
  pub unsafe extern "C" fn repeat(start: u32, n: u32, f: Option<unsafe extern "C" fn(u32) -> u32>) -> u32 {
      if let Some(f) = f {
          let mut value = start;
          for _ in 0..n {
              value = f(value);
          }
          value
      } else {
          start
      }
  }
  ```

  On the C side:

  ```c
  uint32_t repeat(uint32_t start, uint32_t n, uint32_t (*f)(uint32_t));
  ```

- use raw pointers with an `unsafe` transmutation to the function pointer type,
  allowing more powerful checks at the cost of ergonomics.

> ### Rule {{#check FFI-CKFUNPTR | Check foreign function pointers}}
>
> In a secure Rust development, any foreign function pointer must be checked at
> the FFI boundary.

When binding with C or even C++, one cannot guarantee easily the validity of the
function pointer. C++ functors are not C-compatible.

#### Enums

Usually the possible bit patterns of valid `enum` values are really small with
respect to the number of possible bit patterns of the same size. Mishandling an
`enum` value provided by a foreign code may lead to type confusion and have
severe consequences on software security. Unfortunately, checking an `enum`
value at the FFI boundary is not simple on both sides.

On the Rust side, it consists to actually use an integer type in the `extern`
block declaration, a *robust* type, and then to perform a checked conversion
to the enum type.

On the foreign side, it is possible only if the other language allows for
stricter checks than plain C. `enum class` in C++ are for instance allowable.
Note however that as for reference the actual `extern "C"` ABI of
`enum class` is implementation defined and should be verified for each
environment.

> ### Recommendation {{#check FFI-NOENUM | Do not use incoming Rust `enum` at FFI boundary}}
>
> In a secure Rust development, when interfacing with a foreign language,
> the Rust code should not accept incoming values of any Rust `enum` type.
>
> Exceptions include Rust `enum` types that are:
>
> - opaque in the foreign language and only manipulated from the
>   Rust side,
> - bound to safe enums in the foreign language, e.g. `enum class` types in C++.

Concerning fieldless enums, crates like [`num_derive`] or [`num_enum`] allows
developer to easily provide safe conversion from integer to enumeration and may
be use to safely convert an integer (provided from a C `enum`) into a Rust enum.

[num_derive]: https://crates.io/crates/num_derive
[num_enum]: https://crates.io/crates/num_enum

### Opaque types

Opacifying types is a good way to increase modularity in software development.
When doing multilingual development, it is something very common.

> ### Recommendation {{#check FFI-R-OPAQUE | Use dedicated Rust types for foreign opaque types}}
>
> In a secure Rust development, when binding foreign opaque types, one should
> use pointers to dedicated opaque types rather than `c_void` pointers.

Currently the recommended way to make a foreign opaque type is like so:

```rust,unsafe,noplaypen
#[repr(C)]
pub struct Foo {_private: [u8; 0]}
extern "C" {
    fn foo(arg: *mut Foo);
}
```

The not yet implemented [RFC 1861] proposes to facilitate the coding by allowing
to declare opaque types in `extern` blocks.

[RFC 1861]: https://rust-lang.github.io/rfcs/1861-extern-types.html

> ### Recommendation {{#check FFI-C-OPAQUE | Use incomplete C/C++ `struct` pointers to make type opaque}}
>
> In a secure Rust development, when interfacing with C or C++, Rust types that
> are to be considered opaque in C/C++ should be translated as incomplete
> `struct` type (i,e., declared without definition) and be provided with
> a dedicated constructor and destructor.

Example of opaque Rust type:

```rust,unsafe,noplaypen
# use std::panic::catch_unwind;
#
struct Opaque {
    // (...) details to be hidden
}

#[no_mangle]
pub unsafe extern "C" fn new_opaque() -> *mut Opaque {
    catch_unwind(|| // Catch panics, see below
        Box::into_raw(Box::new(Opaque {
            // (...) actual construction
        }))
    ).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn destroy_opaque(o: *mut Opaque) {
    catch_unwind(||
        if !o.is_null() {
            drop(Box::from_raw(o))
        }
    ); // Only needed if Opaque or one of its subfield is Drop
}
```

## Memory and resource management

Programming languages handle memory in various ways. As a result, it is
important to known when transmitting data between Rust and another language
which language is responsible for reclaiming the memory space for this data.
The same is true for other kind of resources such as sockets or files.

Rust tracks variable ownership and lifetime to determine at compilation time if
and when memory should be deallocated. Thanks to the `Drop` trait, one can
exploit this system to reclaim other kind of resources such as file or network
access. *Moving* some piece of data from Rust to a foreign language means also
abandoning the possible reclamations associated with it.

> ### Rule {{#check FFI-MEM-NODROP | Do not use types that implement `Drop` at FFI boundary}}
>
> In a secure Rust development, Rust code must not implement `Drop` for any
> types that are directly transmitted to foreign code  (i.e. not through a
> pointer or reference).

In fact, it is advisable to only use `Copy` types. Note that `*const T` is
`Copy` even if T is not.

However if not reclaiming memory and resources is bad, using reclaimed memory or
reclaiming twice some resources is worst from a security point of view. In order
to correctly release a resource only once, one must known which language is
responsible for allocating and deallocating memory.

> ### Rule {{#check FFI-MEM-OWNER | Ensure clear data ownership in FFI}}
>
> In a secure Rust development, when data of some type passes without copy
> through a FFI boundary, one must ensure that:
>
> - A single language is responsible for both allocation and deallocation of
>   data.
> - The other language must not allocate or free the data directly but use
>   dedicated foreign functions provided by the chosen language.

Ownership is not enough. It remains to ensure the correct lifetime, mostly that
no use occurs after reclamation. It is a lot more challenging. When the other
language is responsible for the memory, the best way is to provide a safe
wrapper around the foreign type:

> ### Recommendation {{#check FFI-MEM-WRAPPING | Wrap foreign data in memory releasing wrapper}}
>
> In a secure Rust development, any non-sensitive foreign piece of data that are
> allocated and deallocated in the foreign language should be encapsulated in a
> `Drop` type in such a way as to provide automatic deallocation in Rust,
> through an automatic call to the foreing language deallocation routine.

A simple example of Rust wrapping over an external opaque type:

```rust,ignore,noplaypen
# use std::ops::Drop;
#
/// Private “raw” opaque foreign type Foo
#[repr(C)]
struct RawFoo {
    _private: [u8; 0],
}

/// Private “raw” C API
extern "C" {
    fn foo_create() -> *mut RawFoo;
    fn foo_do_something(this: *const RawFoo);
    fn foo_destroy(this: *mut RawFoo);
}

/// Foo
pub struct Foo(*mut RawFoo);
#
impl Foo {
    /// Create a Foo
    pub fn new() -> Option<Foo> {
        let raw_ptr = unsafe { foo_create() };
        if raw_ptr.is_null() {
            None
        } else {
            Some(Foo(raw_ptr))
        }
    }
#
    /// Do something on a Foo
    pub fn do_something(&self) {
        unsafe { foo_do_something(self.0) }
    }
}
#
impl Drop for Foo {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { foo_destroy(self.0) }
        }
    }
}

# fn main() {
#     let foo = Foo::new().expect("cannot create Foo");
#     foo.do_something();
# }
```

> ### Warning
>
> Because panics may lead to not running the `Drop::drop` method this solution
> is not sufficient for sensitive deallocation (such as wiping sensitive data)
> except if the code is guaranteed to never panic.
>
> For wiping sensitive data case, one could address the issue with a dedicated
> panic handler.

When the foreign language is the one exploiting Rust allocated resources, it is
a lot more difficult to offer any guarantee.

In C for instance there is no easy way to check that the appropriate destructor
is checked. A possible approach is to exploit callbacks to ensure that the
reclamation is done.

The following Rust code is a **thread-unsafe** example of a C-compatible API
that provide callback to ensure safe resource
reclamation:

```rust,noplaypen
# use std::ops::Drop;
#
pub struct XtraResource {/*fields */}

impl XtraResource {
    pub fn new() -> Self {
        XtraResource { /* ... */}
    }
    pub fn dosthg(&mut self) {
        /*...*/
    }
}

impl Drop for XtraResource {
    fn drop(&mut self) {
        println!("xtra drop");
    }
}

pub mod c_api {
    use super::XtraResource;
    use std::panic::catch_unwind;

    const INVALID_TAG: u32 = 0;
    const VALID_TAG: u32 = 0xDEAD_BEEF;
    const ERR_TAG: u32 = 0xDEAF_CAFE;

    static mut COUNTER: u32 = 0;

    pub struct CXtraResource {
        tag: u32, // to detect accidental reuse
        id: u32,
        inner: XtraResource,
    }

    #[no_mangle]
    pub unsafe extern "C" fn xtra_with(cb: extern "C" fn(*mut CXtraResource) -> ()) {
        let inner = if let Ok(res) = catch_unwind(XtraResource::new) {
            res
        } else {
#             println!("cannot allocate resource");
            return;
        };
        let id = COUNTER;
        let tag = VALID_TAG;

        COUNTER = COUNTER.wrapping_add(1);
        // Use heap memory and do not provide pointer to stack to C code!
        let mut boxed = Box::new(CXtraResource { tag, id, inner });

#         println!("running the callback on {:p}", boxed.as_ref());
        cb(boxed.as_mut() as *mut CXtraResource);

        if boxed.id == id && (boxed.tag == VALID_TAG || boxed.tag == ERR_TAG) {
#             println!("freeing {:p}", boxed.as_ref());
            boxed.tag = INVALID_TAG; // prevent accidental reuse
                                 // implicit boxed drop
        } else {
#             println!("forgetting {:p}", boxed.as_ref());
            // (...) error handling (should be fatal)
            boxed.tag = INVALID_TAG; // prevent reuse
            std::mem::forget(boxed); // boxed is corrupted it should not be
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn xtra_dosthg(cxtra: *mut CXtraResource) {
        let do_it = || {
            if let Some(cxtra) = cxtra.as_mut() {
                if cxtra.tag == VALID_TAG {
#                     println!("doing something with {:p}", cxtra);
                    cxtra.inner.dosthg();
                    return;
                }
            }
            println!("doing nothing with {:p}", cxtra);
        };
        if catch_unwind(do_it).is_err() {
            if let Some(cxtra) = cxtra.as_mut() {
#                 println!("panicking with {:p}", cxtra);
                cxtra.tag = ERR_TAG;
            }
        };
    }
}
#
# fn main() {}
```

A compatible C call:

```c
struct XtraResource;
void xtra_with(void (*cb)(XtraResource* xtra));
void xtra_sthg(XtraResource* xtra);

void cb(XtraResource* xtra) {
    // ()...) do anything with the proposed C API for XtraResource
    xtra_sthg(xtra);
}

int main() {
    xtra_with(cb);
}
```

## Panics with foreign code

When calling Rust code from another language (e.g. C), the Rust code must
be careful to never panic.
Stack unwinding from Rust code into foreign code results in **undefined behavior**.

> ### Rule {{#check FFI-NOPANIC | Handle `panic!` correctly in FFI}}
>
> Rust code called from FFI must either ensure the function cannot panic, or use
> a panic handling mechanism (such as `std::panic::catch_unwind`,
> `std::panic::set_hook`, `#[panic_handler]`) to ensure the rust code will not
> abort or return in an unstable state.

Note that `catch_unwind` will only catch unwinding panics, not those that abort
the process.

```rust,unsafe,noplaypen,ignore
use std::panic::catch_unwind;
# use rand;

fn may_panic() {
    if rand::random() {
        panic!("panic happens");
    }
}

#[no_mangle]
pub unsafe extern "C" fn no_panic() -> i32 {
    let result = catch_unwind(may_panic);
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
[`no-panic`]: https://crates.io/crates/no-panic

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

<!--
<mark>TODO</mark> example
-->

## Binding a Rust library in another language

> ### Recommendation {{#check FFI-CAPI | Expose dedicated C-compatible API only}}
>
> In a secure Rust development, exposing a Rust library to a foreign language
> should only be done through a **dedicated C-compatible API**.

The crate [cbindgen] may be used to automatically generate C or C++ bindings to
the Rust C-compatible API of a Rust library.

### Minimal example of a C-exported Rust library

`src/lib.rs`:

```rust,noplaypen
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
pub unsafe extern "C" fn counter_create() -> *mut Counter {
    Box::into_raw(Box::new(Counter::new()))
}

#[no_mangle]
pub unsafe extern "C" fn counter_incr(counter: *mut Counter) -> std::os::raw::c_int {
    if let Some(counter) = counter.as_mut() {
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
pub unsafe extern "C" fn counter_get(counter: *const Counter) -> u32 {
    if let Some(counter) = counter.as_ref() {
        return counter.get();
    }
    return 0;
}

#[no_mangle]
pub unsafe extern fn counter_destroy(counter: *mut Counter) -> std::os::raw::c_int {
    if !counter.is_null() {
        let _ = Box::from_raw(counter); // get box and drop
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
    if (argc < 2) {
        return -1;
    }
    size_t n = (size_t)strtoull(argv[1], NULL, 10);

    Counter* c = counter_create();
    for (size_t i=0; i < n; i++) {
        if (counter_incr(c) != 0) {
            printf("overflow\n");
            counter_destroy(c);
            return -1;
        }
    }

    printf("%" PRIu32 "\n", counter_get(c));
    counter_destroy(c);

    return 0;
}
```
