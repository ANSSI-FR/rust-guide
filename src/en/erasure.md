# Secure erasure

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