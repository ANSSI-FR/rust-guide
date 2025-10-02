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
{{#include ../../../examples/src/ffi.rs:mylib_f}}
```

For the function `mylib_f` to be accessible with the same name, the function
must also be annotated with the `#[unsafe(no_mangle)]` attribute.

Conversely, one can call C functions from Rust if they are declared in an
`extern` block:

```rust
{{#include ../../../examples/src/ffi.rs:import_c}}
```

<div class="note">

Any foreign function imported in Rust through an `extern` block is
**automatically `unsafe`**. That is why, any call to a foreign function
must be done from an `unsafe` context.

</div>

`extern` blocks may also contain foreign global variable declarations prefixed
with the `static` keyword:

```rust
{{#include ../../../examples/src/ffi.rs:extern_static}}
```

## Typing

Typing is the way Rust ensures memory safety. When interfacing with other
languages, which may not offer the same guarantee, the choice of types in the
binding is essential to maintain memory safety.

### Data layout

Rust provides no short or long term guarantees with respect to how the data is
laid out in the memory. The only way to make data compatible with a foreign
language is through explicit use of a C-compatible data layout with the `repr`
attribute (see [Rust Reference: Type Layout]). For instance, the following Rust
types:

```rust
{{#include ../../../examples/src/ffi.rs:extern_struct}}
```

are compatible with the following C types:

```c
{{#include ../../../examples/src/ffi.c:extern_struct}}
```

<div class="reco" id="FFI-CTYPE" type="Rule" title="Use only C-compatible types in FFI">

In a secure Rust development, only C-compatible types must be used as
parameter or return type of imported or exported functions and as types of
imported or exported global variables.

The lone exception is types that are considered **opaque** on the foreign
side.

</div>

The following types are considered C-compatible:

- integral or floating point primitive types,
- `repr(C)`-annotated `struct`,
- `repr(C)` or `repr(Int)`-annotated `enum` with at least one variant and only
  fieldless variants (where `Int` is an integral primitive type),
- pointers,
- an `Option<T>` where `T` is either
  - `core::ptr::NonNull<U>` and `U` is a `Sized` C-compatible type, then it is
      compatible to a `*const T` and `*mut T` pointer;
  - `core::num::NonZero*`, then is compatible with the corresponding integral
      primitive type;
- a `repr(transparent)`-annotated `struct` with only one field, where that
  field has a C-compatible type.

The following types are not C-compatible:

- Dynamically sized types,
- Trait objects,
- Enums with fields,
- Tuples (but `repr(C)` tuple structures are OK).

Some types are compatible with some caveats:

- Zero-sized types, which is really zero sized (which is left unspecified in C
  and contradicts the C++ specification),
- `repr(C)`, `repr(C, Int)`, or `repr(Int)`-annotated enum with fields
  (see [RFC 2195]).

[RFC 2195]: https://rust-lang.github.io/rfcs/2195-really-tagged-unions.html
[Rust Reference: Type Layout]: https://doc.rust-lang.org/reference/type-layout.html

### Type consistency

<div class="reco" id="FFI-TCONS" type="Rule" title="Use consistent types at FFI boundaries">

Types must be consistent on each side of the FFI boundary.

Although some details may be hidden on one side with respect to the other
(typically to make a type opaque), types on both sides must have the same size
and the same alignment requirement.

</div>

Concerning enums with fields in particular, the corresponding types in C (or
C++) are not obvious, cf. [RFC 2195].

Automated tools to generate bindings, such as [rust-bindgen] or
[cbindgen], may be of help in making types consistent between C and Rust.

<div class="reco" id="FFI-AUTOMATE" type="Recommendation" title="Use automatic binding generator tools">

In a secure Rust development, automated generation tools should be used to
generate bindings when possible and to maintain them continually.

</div>

<!-- -->

<div class="warning">

For binding C/C++ to Rust, [rust-bindgen] is able to automatically generate
the low-level binding. A high-level safe binding is highly recommended (see
Recommendation [FFI-SAFEWRAPPING](ffi.md#FFI-SAFEWRAPPING)).
Also some options of rust-bindgen may result in dangerous translations, in
particular `rustified_enum`.

</div>

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

<div class="reco" id="FFI-PFTYPE" type="Rule" title="Use portable aliases `c_*` when binding to platform-dependent types">

In a secure Rust development, when interfacing with foreign code that
uses platform-dependent types, such as C's `int` and `long`, Rust code must
use portable type aliases, such as the ones provided by the standard library or the
[libc] crate, rather than platform-specific types, except if
the binding is automatically generated for each platform (see Note below).

</div>

<!-- -->

<div class="note">

Automatic binding generation tools (e.g. [cbindgen], [rust-bindgen]) are able
to ensure type consistency on a specific platform. They should be used during
the build process for each target to ensure that the generation is sound for
the specific target platform.

</div>

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
- floats (even if almost all languages have the same understanding of what is
  a valid float),
- compound types that contain a field of a non-robust type.

On the other hand, integer types (`u*`/`i*`), packed compound types that contain
no non-robust fields, are instances of *robust types*.

Non-robust types are a difficulty when interfacing two languages. It revolves
around deciding **which language of the two is responsible for asserting the
validity of boundary-crossing values** and how to do it.

<div class="reco" id="FFI-CKNONROBUST" type="Rule" title="Do not use unchecked non-robust foreign values">

In a secure Rust development, there must not be any use of *unchecked* foreign
values of non-robust types.

In other words, either Rust translates robust types into non-robust types
through explicit checking or the foreign side offers strong guarantees of the
validity of the value.

</div>

<!-- -->

<div class="reco" id="FFI-CKINRUST" type="Recommendation" title="Check foreign values in Rust">

In a secure Rust development, the validity checks of foreign values should
be done in Rust when possible.

</div>

Those generic rules are to be adapted to a specific foreign language or for the
associated risks. Concerning languages, C is particularly unfit for offering
guarantees about validity. However, Rust is not the only language to offer
strong guarantees. For instance, some C++ subset (without reinterpretation)
allows developers to do lot of type checking. Because Rust natively separates
the safe and unsafe segments, the recommendation is to always use Rust to check
when possible. Concerning risks, the most dangerous types are references,
function references, and enums, and are discussed below.

<div class="warning">

Rust `bool` has been made equivalent to C99's `_Bool` (aliased as `bool`
in `<stdbool.h>`) and C++'s `bool`. However, loading a value other than 0 and
1 as a `_Bool`/`bool` is an undefined behavior *on both sides*.
Safe Rust ensures that. Standard-compliant C and C++ compilers ensure that no
value but 0 and 1 can be *stored* in a `_Bool`/`bool` value but cannot
guarantee the absence of an *incorrect reinterpretation* (e.g., union types,
pointer cast). To detect such a bad reinterpretation, sanitizers such as
LLVM's `-fsanitize=bool` may be used.

</div>

#### References and pointers

Although they are allowed by the Rust compiler, the use of Rust references in
FFI may break Rust memory safety. Because their “unsafety” is more explicit,
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

On the other hand, Rust *pointer types* may also lead to undefined behaviors
but are more verifiable, mostly against `std/core::ptr::null()` (C's `(void*)0`)
but also in some contexts against a known valid memory range (particularly in
embedded systems or kernel-level programming). Another advantage of using Rust
pointers in FFI is that any load of the pointed value is clearly marked inside
an `unsafe` block or function.

<div class="reco" id="FFI-NOREF" type="Recommendation" title="Do not use reference types but pointer types">

In a secure Rust development, the Rust code should not use reference types
but pointer types.

Exceptions include:

- Rust references that are opaque in the foreign language and only manipulated
  from the Rust side,
- `Option`-wrapped references (see Note below),
- references bound to foreign safe references, e.g. from some augmented C
  variants or from C++ compiled in an environment where `extern "C"`
  references are encoded as pointers.

</div>

<!-- -->

<div class="reco" id="FFI-CKREF" type="Rule" title="Do not use unchecked foreign references">

In a secure Rust development, every foreign reference that is transmitted to
Rust through FFI must be **checked on the foreign side** either automatically
(for instance, by a compiler) or manually.

Exceptions include Rust references in an opaque wrapping that are created
and manipulated only from the Rust side and `Option`-wrapped references
(see Note below).

</div>

<!-- -->

<div class="reco" id="FFI-CKPTR" type="Rule" title="Check foreign pointers">

In a secure Rust development, any Rust code that dereferences a foreign
pointer must check their validity beforehand.
In particular, pointers must be checked to be non-null before any use.

Stronger approaches are advisable when possible. They include checking
pointers against known valid memory range or tagging (or signing) pointers
(particularly applicable if the pointed value is only manipulated from Rust).

</div>

The following code a simple example of foreign pointer use in an exported Rust
function:

```rust,noplaypen
{{#include ../../../examples/src/ffi.rs:pointers}}
```

Note that the methods `as_ref` and `as_mut` (for mutable pointers) allows easy
access to a reference while ensuring a null check in a very *Rusty* way.
On the other side in C, it can be used as follows:

```c
{{#include ../../../examples/src/ffi.c:pointers}}
```
<div class="note">

`Option<&T>` and `Option<&mut T>` for any `T: Sized` are allowable in FFI
instead of pointers with explicit nullity checks. Due to the Rust guaranteed
“nullable pointer optimization”, a nullable pointer is acceptable on the C
side. The C `NULL` is understood as `None` in Rust while a non-null
pointer is encapsulated in `Some`. While quite ergonomic, this feature does
not allow stronger validations such as memory range checking.

</div>

#### Function pointers

Function pointers that cross FFI boundaries may ultimately lead to arbitrary code
execution and represent a real security risks.

<div class="reco" id="FFI-MARKEDFUNPTR" type="Rule" title="Mark function pointer types in FFI as `extern` and `unsafe`">

In a secure Rust development, any function pointer types at the FFI boundary
must be marked `extern` (possibly with the specific ABI) and `unsafe`.

</div>

Function pointers in Rust are a lot more similar to references than they are
to normal pointers. In particular, the validity of function pointers cannot be
checked directly on the Rust side. However, Rust offers two alternative
possibilities:

- use `Option`-wrapped function pointer and check against `null`:

  ```rust,noplaypen
  {{#include ../../../examples/src/ffi.rs:function_pointers}}
  ```

  On the C side:

  ```c
  {{#include ../../../examples/src/ffi.c:function_pointers}}
  ```

- use raw pointers with an `unsafe` transmutation to the function pointer type,
  allowing more powerful checks at the cost of ergonomics.

<div class="reco" id="FFI-CKFUNPTR" type="Rule" title="Check foreign function pointers">

In a secure Rust development, any foreign function pointer must be checked at
the FFI boundary.

</div>

When binding with C or even C++, one cannot guarantee easily the validity of 
function pointers. Moreover, C++ function objects (also known as functors) are not C-compatible.

#### Enums

Usually the possible bit patterns of valid `enum` values are really small with
respect to the number of possible bit patterns of the same size. Mishandling an
`enum` value provided by a foreign code may lead to type confusion and have
severe consequences on software security. Unfortunately, checking an `enum`
value at the FFI boundary is not simple on both sides.

On the Rust side, it consists in actually using an integer type in the `extern`
block declaration, a *robust* type, and then performing a checked conversion
to the enum type.

On the foreign side, it is possible only if the other language allows for
stricter checks than plain C. `enum class` in C++ are for instance allowable.
Note however that as for a reference, the actual `extern "C"` ABI of
`enum class` is implementation-defined and should be verified for each
environment.

<div class="reco" id="FFI-NOENUM" type="Recommendation" title="Do not use incoming Rust `enum` at FFI boundary">

In a secure Rust development, when interfacing with a foreign language,
the Rust code should not accept incoming values of any Rust `enum` type.

Exceptions include Rust `enum` types that are:

- opaque in the foreign language and only manipulated from the
  Rust side,
- bound to safe enums in the foreign language, e.g. `enum class` types in C++.

</div>

Concerning fieldless enums, crates like [`num_derive`] or [`num_enum`] allow
developers to easily provide safe conversions from integer to enumeration and may
be use to safely convert an integer (provided from a C `enum`) into a Rust enum.

[num_derive]: https://crates.io/crates/num_derive
[num_enum]: https://crates.io/crates/num_enum

### Opaque types

Opacifying types is a good way to increase modularity in software development.
When doing multilingual development, it is something very common.

<div class="reco" id="FFI-R-OPAQUE" type="Recommendation" title="Use dedicated Rust types for foreign opaque types">

In a secure Rust development, when binding foreign opaque types, one should
use pointers to dedicated opaque types rather than `c_void` pointers.

</div>

Currently the recommended way to make a foreign opaque type is like so:

```rust,unsafe,noplaypen
{{#include ../../../examples/src/ffi.rs:opaque_external}}
```

The not-yet-implemented [RFC 1861] proposes to facilitate this encoding by allowing
to declare opaque types in `extern` blocks.

[RFC 1861]: https://rust-lang.github.io/rfcs/1861-extern-types.html

<div class="reco" id="FFI-C-OPAQUE" type="Recommendation" title="Use incomplete C/C++ `struct` pointers to make type opaque">

In a secure Rust development, when interfacing with C or C++, Rust types that
are to be considered opaque in C/C++ should be translated as incomplete
`struct` types (i,e., declared without definition) and be provided with
a dedicated constructor and destructor.

</div>

Example of opaque Rust type:

```rust,unsafe,noplaypen
{{#include ../../../examples/src/ffi.rs:opaque_internal}}
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

<div class="reco" id="FFI-MEM-NODROP" type="Rule" title="Do not use types that implement `Drop` at FFI boundary">

In a secure Rust development, Rust code must not implement `Drop` for any
types that are directly transmitted to foreign code  (i.e. not through a
pointer or reference).

</div>

In fact, it is advisable to only use `Copy` types. Note that `*const T` is
`Copy` even if T is not.

However if not reclaiming memory and resources is bad, using reclaimed memory or
reclaiming twice some resources is worst from a security point of view. In order
to correctly release a resource only once, one must known which language is
responsible for allocating and deallocating memory.

<div class="reco" id="FFI-MEM-OWNER" type="Rule" title="Ensure clear data ownership in FFI">

In a secure Rust development, when data of some type passes without copy
through a FFI boundary, one must ensure that:

- A single language is responsible for both allocation and deallocation of
  data.
- The other language must not allocate or free the data directly but use
  dedicated foreign functions provided by the chosen language.

</div>

Ownership is not enough. It remains to ensure the correct lifetime, mostly that
no use occurs after reclamation. It is a lot more challenging. When the other
language is responsible for the memory, the best way is to provide a safe
wrapper around the foreign type:

<div class="reco" id="FFI-MEM-WRAPPING" type="Recommendation" title="Wrap foreign data in memory releasing wrapper">

In a secure Rust development, any non-sensitive foreign piece of data that are
allocated and deallocated in the foreign language should be encapsulated in a
`Drop` type in such a way as to provide automatic deallocation in Rust,
through an automatic call to the foreign language deallocation routine.

</div>

A simple example of Rust wrapping over an external opaque type:

```rust,ignore,noplaypen
{{#include ../../../examples/src/ffi.rs:drop_extern}}
```
<div class="warning">

Because panics may lead to not running the `Drop::drop` method this solution
is not sufficient for sensitive deallocation (such as wiping sensitive data)
except if the code is guaranteed to never panic.

For wiping sensitive data, one could address the issue with a dedicated
panic handler.

</div>

When the foreign language is the one exploiting Rust allocated resources, it is
a lot more difficult to offer any guarantee.

In C for instance, there is no easy way to check that the appropriate destructor
is called. A possible approach is to exploit callbacks to ensure that the
reclamation is done.

The following Rust code is a **thread-unsafe** example of a C-compatible API
that provides a callback to ensure safe resource
reclamation:

```rust,noplaypen
{{#include ../../../examples/src/ffi.rs:free_intern}}
```

A compatible C call:

```c
{{#include ../../../examples/src/ffi.c:free_intern}}
```

## Panics with foreign code

When calling Rust code from another language (e.g. C), the Rust code must
be careful to never panic:
stack unwinding from Rust code into foreign code results in **undefined behavior**.

<div class="reco" id="FFI-NOPANIC" type="Rule" title="Handle `panic!` correctly in FFI">

Rust code called from FFI must either ensure the function cannot panic, or use
a panic handling mechanism (such as `std::panic::catch_unwind`,
`std::panic::set_hook`, `#[panic_handler]`) to ensure the rust code will not
abort or return in an unstable state.

</div>

Note that `catch_unwind` will only catch unwinding panics, not those that abort
the process.

```rust,unsafe,noplaypen,ignore
{{#include ../../../examples/src/ffi.rs:panic}}
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

<div class="reco" id="FFI-SAFEWRAPPING" type="Recommendation" title="Provide safe wrapping to foreign library">

Interfacing a library written in another language in Rust should be done in
two parts:

- a low-level, possibly *hidden*, module that closely translates the original
  C API into `extern` blocks,
- a safe wrapping module that ensures memory safety and security invariants at
  the Rust level.

If the low-level API is exposed to the world, it should be done in a dedicated
crate with a name of the form `*-sys`.

</div>

The crate [rust-bindgen] may be used to automatically generate the low-level
part of the binding from C header files.

<!--
<mark>TODO</mark> example
-->

## Binding a Rust library in another language

<div class="reco" id="FFI-CAPI" type="Recommendation" title="Expose dedicated C-compatible API only">

In a secure Rust development, exposing a Rust library to a foreign language
should only be done through a **dedicated C-compatible API**.

</div>

The crate [cbindgen] may be used to automatically generate C or C++ bindings to
the Rust C-compatible API of a Rust library.

### Minimal example of a C-exported Rust library

`src/lib.rs`:

```rust,noplaypen
{{#include ../../../examples/src/counter.rs}}
```

Using [cbindgen] (`[cbindgen] -l c > counter.h`), one can generate a consistent
C header, `counter.h`:

```c
{{#include ../../../examples/src/counter.h}}
```

`counter_main.c`:

```c
{{#include ../../../examples/src/counter.c}}
```
