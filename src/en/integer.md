# Integer operations

## Integer overflows

Although Rust performs some checks for potential integer overflows, precautions should be taken when executing arithmetic operations on integers.

In particular, note that the compilation profile (typically *dev*, the default debug build, or *release*, the standard optimized build) changes integer overflow behavior. In the *dev* configuration, overflow causes the termination of the program (`panic`), whereas in the *release* configuration, the computed value is silently truncated to the number of bits of the numeric type, giving it wrap-around semantics.

When an overflow is possible, the behavior can be made explicit either by using specific methods or by using specific wrapper types.

The methods are of the form `<mode>_<op>`, where `<mode>` is `checked`, `overflowing`, `wrapping`, or `saturating`, and `<op>` is `add`, `mul`, `sub`, `shr`, etc. The semantics are as follows:

- `checked_<op>` returns `None` in case of overflow,
- `overflowing_<op>` returns both a possibly wrapped result and a Boolean indicating whether overflow occurred,
- `wrapping_<op>` always returns the wrapped result,
- `saturating_<op>` always returns the saturated result.

The wrapper types are `Wrapping<T>` and `Saturating<T>` (from `std::num`), where `T` is an integer type. The former provides wrap-around semantics for all arithmetic operations, whereas the latter provides saturation semantics. Once the values are wrapped, all subsequent operations are performed with the given semantics.

```rust
{{#include ../../examples/src/integer.rs}}
```

<div class="reco" id="LANG-ARITH" type="Rule" title="Use appropriate arithmetic operations regarding potential overflows">

When an arithmetic operation can produce an overflow, the usual operators MUST NOT be used directly.
Instead, specialized methods such as `checked_<op>`, `overflowing_<op>`, `wrapping_<op>`, or `saturating_<op>`, or specialized wrapper types like `Wrapping` or `Saturating`, MUST be used to ensure explicit and consistent behavior regardless of the compilation profile.

</div>
