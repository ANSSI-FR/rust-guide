# Integer operations

## Integer overflows

Although Rust performs some checks for potential integer overflows, precautions
should be taken when executing arithmetic operations on integers.

In particular, note that the compilation profile (typically, *dev*, the default
debug build, or *release*, the standard optimized build) changes integer
overflow behavior. In the *dev* configuration, overflow causes the termination
of the program (`panic`), whereas in the *release* configuration, the computed
value is silently truncated to the number of bits of the numeric type, giving it
this wrap-around semantics.

When an overflow is possible, the behavior can be made explicit by using
specific methods `<mode>_<op>`, where `<op>` can be `add`, `mul`, `sub`, `shr`,
etc.:

- `checked_<op>` returns `None` in case of overflow,
- `overflowing_<op>` returns both a possibly wrapped result and a Boolean
  indicating whether overflow occurred,
- `wrapping_<op>` always returns the wrapped result,
- `saturating_<op>` always returns the saturated result.

For the last two choices, an alternative is to use the generic types `Wrapping`
and `Saturating` (from `std::num`) to accomplish the same thing in a more
concise way. Indeed, once the values are wrapped inside all subsequent
operations are made with the given semantics.

```rust
use std::num::{Saturating, Wrapping};
# use std::panic;

# fn main() {
let x: u8 = 242;

# let result = panic::catch_unwind(|| {
println!("{}", x + 50);     // panics in debug, prints 36 in release.
# });
# if result.is_err() { println!("panic"); }
println!("{:?}", x.checked_add(50));            // always prints None.
println!("{}", x.overflowing_add(50).0);        // always prints 36.
println!("{}", x.wrapping_add(50));             // always prints 36.
println!("{}", x.saturating_add(50));           // always prints 255.
println!("{}", Wrapping(x) + Wrapping(50));     // always prints 36.
println!("{}", Saturating(x) + Saturating(50)); // always prints 255.

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
> When an arithmetic operation may produce an overflow, specialized methods like
> `checked_<op>`, `overflowing_<op>`, `wrapping_<op>`, or `saturating_<op>`, or
> specialized wrapper types like `Wrapping` or `Saturating` must be used.
