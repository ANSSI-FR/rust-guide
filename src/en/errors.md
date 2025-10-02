# Error handling

<!-- <mark>TODO</mark>: explicit good practices in error handling. -->

The `Result` type is the preferred way of handling functions that can fail.
A `Result` object must be tested, and never ignored.

<div class="reco" id="LANG-ERRWRAP" type="Recommendation" title="Implement custom `Error` type wrapping all possible errors">

A crate can implement its own `Error` type, wrapping all possible errors.
It must be careful to make this type exception-safe (RFC 1236), and implement
`Error + Send + Sync + 'static` as well as `Display`.

</div>

<div class="reco" id="LANG-ERRDO" type="Recommendation" title="Use the `?` operator and do not use the `try!` macro">

The `?` operator should be used to improve readability of code.
The `try!` macro should not be used.

</div>

Third-party crates may be used to facilitate error handling. Most of them
(notably [failure], [snafu], [thiserror]) address the creation of new custom
error types that implement the necessary traits and allow wrapping other
errors.

Another approach (notably proposed in the [anyhow] crate) consists in an automatic
wrapping of errors into a single universal error type. Such wrappers should not
be used in libraries and complex systems because they do not allow developers to
provide context to the wrapped error.

[failure]: https://crates.io/crates/failure
[snafu]: https://crates.io/crates/snafu
[thiserror]: https://crates.io/crates/thiserror
[anyhow]: https://crates.io/crates/anyhow

## Panics

Explicit error handling (`Result`) should always be preferred instead of calling
`panic`.  The cause of the error should be available, and generic errors should
be avoided.

Crates providing libraries should never use functions or instructions that can
fail and cause the code to panic.

Common patterns that can cause panics are:

- using `unwrap` or `expect`,
- using `assert`,
- an unchecked access to an array,
- integer overflow (in debug mode),
- division by zero,
- large allocations,
- string formatting using `format!`.

<div class="reco" id="LANG-NOPANIC" type="Rule" title="Don't use functions that can cause `panic!`">

Functions or instructions that can cause the code to panic at runtime must not
be used.

</div>

<div class="reco" id="LANG-ARRINDEXING" type="Rule" title="Test properly array indexing or use the `get` method">

Array indexing must be properly tested, or the `get` method should be used to
return an `Option`.

</div>

<!--
<mark>TODO</mark> Check if the [no_panic](https://github.com/dtolnay/no-panic)
crate can catch all cases. Drawback: all functions need to be marked as
`#[no_panic]`.
-->
<!--
<mark>TODO</mark> Another possibility:
[rustig](https://github.com/Technolution/rustig) (doesn't build here)
-->

## FFI and panics

When calling Rust code from another language (for ex. C), the Rust code must
be careful to never panic.
Stack unwinding from Rust code into foreign code results in undefined behavior.

<div class="reco" id="LANG-FFIPANIC" type="Rule" title="Handle correctly `panic!` in FFI">

Rust code called from FFI must either ensure the function cannot panic, or use
`catch_unwind` or the `std::panic` module to ensure the rust code will not
abort or return in an unstable state.

</div>

Note that `catch_unwind` will only catch unwinding panics, not those that abort
the process.
