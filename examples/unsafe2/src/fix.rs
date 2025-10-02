/// Trait for types that can be located in and loaded from a byte buffer.
///
/// # Safety
///
/// This trait is marked unsafe because the index returned by `locate_instance_into`
/// must point to a valid instance of `Self` in the provided buffer:
///
/// - valid representation,
/// - in-bound byte range.
unsafe trait Locatable: Copy {
    /// Finds an object of type `Self` in the buffer `buf` and returns the index of
    /// the first byte representing an object of type `Self`
    fn locate_instance_into(buf: &[u8]) -> Option<usize>;
}

unsafe impl Locatable for bool {
    fn locate_instance_into(buf: &[u8]) -> Option<usize> {
        buf.iter().position(|u| *u == 0 || *u == 1)
    }
}

fn find<T: Locatable>(buf: &[u8]) -> Option<T> {
    let start = T::locate_instance_into(buf)?;
    unsafe {
        let ptr: *const T = buf.as_ptr().add(start).cast();
        Some(ptr.read_unaligned())
    }
}

fn main() {
    let buf = [4, 1, 99];
    let located_bool: Option<bool> = find(&buf);
    println!("{:?}", located_bool)
}