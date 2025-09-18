/// Opaque counter
pub struct Counter(u32);

impl Counter {
    /// Create a counter (initially at 0)
    fn new() -> Self {
        Self(0)
    }
    /// Get the current value of the counter
    fn get(&self) -> u32 {
        self.0
    }
    /// Increment the value of the counter if there's no overflow
    fn incr(&mut self) -> bool {
        if let Some(n) = self.0.checked_add(1) {
            self.0 = n;
            true
        } else {
            false
        }
    }
}

// C-compatible API

#[unsafe(no_mangle)]
pub unsafe extern "C" fn counter_create() -> *mut Counter {
    Box::into_raw(Box::new(Counter::new()))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn counter_incr(counter: *mut Counter) -> std::os::raw::c_int {
    if let Some(counter) = unsafe { counter.as_mut() } {
        if counter.incr() { 0 } else { -1 }
    } else {
        -2
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn counter_get(counter: *const Counter) -> u32 {
    if let Some(counter) = unsafe { counter.as_ref() } {
        counter.get()
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn counter_destroy(counter: *mut Counter) -> std::os::raw::c_int {
    if !counter.is_null() {
        let _ = unsafe { Box::from_raw(counter) }; // get box and drop
        0
    } else {
        -1
    }
}
