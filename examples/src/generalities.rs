// ANCHOR: naive_vec
use std::ptr;

pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

// Note this implementation does not correctly handle zero-sized types.
impl<T> Vec<T> {
    pub fn push(&mut self, elem: T) {
        if self.len == self.cap {
            // reallocate new array with bigger capacity
        }
        unsafe {
            ptr::write(self.ptr.add(self.len), elem);
            self.len += 1;
        }
    }
}

// ANCHOR_END: naive_vec

// ANCHOR: make_room
impl<T> Vec<T> {
    fn make_room(&mut self) {
        // grow the capacity
        self.cap += 1;
    }
}
// ANCHOR_END: make_room

// ANCHOR: Locatable
trait Locatable {
    /// Find object of type `Self` in the buffer `buf`.
    /// Returns the index of the first byte representing
    /// an object of type `Self`
    fn locate_instance_into(buf: &[u8]) -> Option<usize>;
}

fn find<T: Locatable>(buf: &[u8]) -> Option<T> {
    let start = T::locate_instance_into(buf)?;
    unsafe {
        let ptr: *const T = buf.as_ptr().add(start).cast();
        Some(ptr.read_unaligned())
    }
}
// ANCHOR_END: Locatable
#[cfg(feature = "OK")]
mod ok {
    use super::*;
    // ANCHOR: Locatable_bool_OK
    impl Locatable for bool {
        fn locate_instance_into(buf: &[u8]) -> Option<usize> {
            buf.iter().position(|u| *u == 0 || *u == 1)
        }
    }
    // ANCHOR_END: Locatable_bool_OK
}
#[cfg(feature = "")]
mod bad {
    use super::*;
    // ANCHOR: Locatable_bool_KO
    impl Locatable for bool {
        fn locate_instance_into(buf: &[u8]) -> Option<usize> {
            buf.iter().position(|u| *u == 0 || *u == 1).map(|n| n + 100)
        }
    }
    // ANCHOR_END: Locatable_bool_KO

    // ANCHOR: Locatable_UB
    fn use_locatable() {
        let buf = [4, 1, 99];
        let located_bool: Option<bool> = find(&buf); // UB here!
        println!("{:?}", located_bool)
    }
    // ANCHOR_END: Locatable_UB
}
