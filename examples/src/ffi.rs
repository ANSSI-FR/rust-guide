#![allow(dead_code)]
use std::panic::catch_unwind;

/////////////////////// ANCHOR: mylib_f
/// Export C-compatible function
#[unsafe(no_mangle)]
extern "C" fn mylib_f(param: u32) -> i32 {
    if param == 0xCAFEBABE { 0 } else { -1 }
}
/////////////////////// ANCHOR_END: mylib_f

/////////////////////// ANCHOR: import_c
use std::os::raw::c_int;
// import C function
unsafe extern "C" {
    fn abs(args: c_int) -> c_int;
}

fn use_abs() {
    let x = -1;
    println!("{} {}\n", x, unsafe { abs(x) });
}
/////////////////////// ANCHOR_END: import_c

/////////////////////// ANCHOR: extern_static
// A direct way to access environment variables (on Unix)
// should not be used! Not thread safe, have a look at `std::env`!

unsafe extern "C" {
    // Global variable
    #[link_name = "environ"]
    static libc_environ: *const *const std::os::raw::c_char;
}

fn use_static_extern() {
    let mut next = unsafe { libc_environ };
    while !next.is_null() && !unsafe { *next }.is_null() {
        let env = unsafe { std::ffi::CStr::from_ptr(*next) }
            .to_str()
            .unwrap_or("<invalid>");
        println!("{}", env);
        next = unsafe { next.offset(1) };
    }
}
/////////////////////// ANCHOR_END: extern_static

/////////////////////// ANCHOR: extern_struct
#[repr(C)]
struct Data {
    a: u32,
    b: u16,
    c: u64,
}
#[repr(C, packed)]
struct PackedData {
    a: u32,
    b: u16,
    c: u64,
}
/////////////////////// ANCHOR_END: extern_struct

/////////////////////// ANCHOR: pointers
/// Add in place
#[unsafe(no_mangle)]
unsafe extern "C" fn add_in_place(a: *mut u32, b: u32) {
    // checks for nullity of `a`
    // and takes a mutable reference on it if it's non-null
    if let Some(a) = unsafe { a.as_mut() } {
        *a += b
    }
}
/////////////////////// ANCHOR_END: pointers

/////////////////////// ANCHOR: function_pointers
#[unsafe(no_mangle)]
pub unsafe extern "C" fn repeat(
    start: u32,
    n: u32,
    f: Option<unsafe extern "C" fn(u32) -> u32>,
) -> u32 {
    if let Some(f) = f {
        let mut value = start;
        for _ in 0..n {
            value = unsafe { f(value) };
        }
        value
    } else {
        start
    }
}
/////////////////////// ANCHOR_END: function_pointers

/////////////////////// ANCHOR: opaque_external
#[repr(C)]
pub struct OpaqueFoo {
    _private: [u8; 0],
}
unsafe extern "C" {
    fn handle_opaque_foo(arg: *mut OpaqueFoo);
}
/////////////////////// ANCHOR_END: opaque_external

/////////////////////// ANCHOR: opaque_internal
pub struct Opaque {
    // (...) hide details
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn new_opaque() -> *mut Opaque {
    catch_unwind(|| // Catch panics, see below
        Box::into_raw(Box::new(Opaque {
            // (...) construction
        })))
    .unwrap_or(std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn destroy_opaque(o: *mut Opaque) {
    catch_unwind(|| {
        if !o.is_null() {
            // only necessary when `Opaque` or any of its fields is `Drop`
            drop(unsafe { Box::from_raw(o) })
        }
    })
    .unwrap_or_default();
}
/////////////////////// ANCHOR_END: opaque_internal

/////////////////////// ANCHOR: drop_extern
/// Private “raw” opaque foreign type Foo
#[repr(C)]
struct RawFoo {
    _private: [u8; 0],
}

// Private “raw” C API
unsafe extern "C" {
    fn foo_create() -> *mut RawFoo;
    fn foo_do_something(this: *const RawFoo);
    fn foo_destroy(this: *mut RawFoo);
}

/// Foo
pub struct Foo(*mut RawFoo);

impl Foo {
    /// Create a Foo
    pub fn new() -> Option<Foo> {
        let raw_ptr = unsafe { foo_create() };
        if raw_ptr.is_null() {
            None
        } else {
            Some(Foo(raw_ptr))
        }
    }

    /// Do something on a Foo
    pub fn do_something(&self) {
        unsafe { foo_do_something(self.0) }
    }
}

impl Drop for Foo {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { foo_destroy(self.0) }
        }
    }
}
/////////////////////// ANCHOR_END: drop_extern

/////////////////////// ANCHOR: free_intern
pub struct XtraResource {/* fields */}

impl XtraResource {
    pub fn new() -> Self {
        XtraResource { /* ... */ }
    }
    pub fn dosthg(&mut self) {
        /* ... */
    }
}

impl Drop for XtraResource {
    fn drop(&mut self) {
        println!("xtra drop");
    }
}

pub mod c_api {
    use super::XtraResource;
    use std::panic::catch_unwind;

    const INVALID_TAG: u32 = 0;
    const VALID_TAG: u32 = 0xDEAD_BEEF;
    const ERR_TAG: u32 = 0xDEAF_CAFE;

    static mut COUNTER: u32 = 0;

    pub struct CXtraResource {
        tag: u32, // to detect accidental reuse
        id: u32,
        inner: XtraResource,
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xtra_with(cb: Option<unsafe extern "C" fn(*mut CXtraResource) -> ()>) {
        let inner = if let Ok(res) = catch_unwind(XtraResource::new) {
            res
        } else {
            return;
        };
        let id = unsafe { COUNTER };
        let tag = VALID_TAG;

        unsafe { COUNTER = COUNTER.wrapping_add(1) };
        // Use heap memory and do not provide pointer to stack to C code!
        let mut boxed = Box::new(CXtraResource { tag, id, inner });

        if let Some(cb) = cb {
            unsafe { cb(boxed.as_mut() as *mut CXtraResource) };
        }

        if boxed.id == id && (boxed.tag == VALID_TAG || boxed.tag == ERR_TAG) {
            boxed.tag = INVALID_TAG; // prevent accidental reuse
        // implicit boxed drop
        } else {
            // (...) error handling (should be fatal)
            boxed.tag = INVALID_TAG; // prevent reuse
            std::mem::forget(boxed); // boxed is corrupted it should not be
        }
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn xtra_dosthg(cxtra: *mut CXtraResource) {
        let do_it = || {
            if let Some(cxtra) = unsafe { cxtra.as_mut() }
                && cxtra.tag == VALID_TAG
            {
                cxtra.inner.dosthg();
                return;
            }
            println!("do noting with {:p}", cxtra);
        };
        if catch_unwind(do_it).is_err()
            && let Some(cxtra) = unsafe { cxtra.as_mut() }
        {
            cxtra.tag = ERR_TAG;
        };
    }
}
/////////////////////// ANCHOR_END: free_intern

/////////////////////// ANCHOR: panic
fn may_panic() {
    /* Something may panic... */
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn no_panic() -> i32 {
    let result = catch_unwind(may_panic);
    match result {
        Ok(_) => 0,
        Err(_) => -1,
    }
}
/////////////////////// ANCHOR_END: panic
