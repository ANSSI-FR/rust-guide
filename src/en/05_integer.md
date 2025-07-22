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
use std::num::Wrapping;
# use std::panic;

# fn main() {
let x: u8 = 242;

# let result = panic::catch_unwind(|| {
println!("{}", x + 50);                      // panics in debug, prints 36 in release.
# });
# if result.is_err() { println!("panic"); }
println!("{}", x.overflowing_add(50).0);     // always prints 36.
println!("{}", x.wrapping_add(50));          // always prints 36.
println!("{}", Wrapping(x) + Wrapping(50));  // always prints 36.

// always panics:
let (res, c) = x.overflowing_add(50);
# let result = panic::catch_unwind(|| {
if c { panic!("custom error"); }
else { println!("{}", res); }
# });
# if result.is_err() { println!("panic"); }
# }
```

> **Rule {{#check LANG-ARITH | Use appropriate arithmetic operations regarding potential overflows}}**
>
> When assuming that an arithmetic operation can produce an overflow, the
> specialized functions `overflowing_<op>`, `wrapping_<op>`, or the
> `Wrapping` type must be used.
