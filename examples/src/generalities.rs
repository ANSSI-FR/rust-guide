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
    pub fn make_room(&mut self) {
        // grow the capacity
        self.cap += 1;
    }
}
// ANCHOR_END: make_room
