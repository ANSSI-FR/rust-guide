# Integer operations

## Integer overflows

Although some verification is performed by Rust regarding potential integer
overflows, precautions should be taken when executing arithmetic operations on
integers.

In particular, it should be noted that using debug or release compilation
profile changes integer overflow behavior. In debug configuration, overflow
cause the termination of the program (`panic`), whereas in the release
configuration the computed value silently wraps around the maximum value that
can be stored.

This last behavior can be made explicit by using the `Wrapping` generic type,
or the `overflowing_<op>` and `wrapping_<op>` operations on integers
(the `<op>` part being `add`, `mul`, `sub`, `shr`, etc.).

```rust
{{#include ../../examples/src/integer.rs}}
```

<div class="reco" id="LANG-ARITH" type="Rule" title="Use appropriate arithmetic operations regarding potential overflows">

When assuming that an arithmetic operation can produce an overflow, the
specialized functions `overflowing_<op>`, `wrapping_<op>`, or the
`Wrapping` type must be used.

</div>
