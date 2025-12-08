use std::mem::forget;

#[allow(dropping_copy_types)]
fn drop_example() {
    // ANCHOR: drop_example
    let pair = ('↑', 0xBADD_CAFEu32);
    drop(pair);
    // ANCHOR_END: drop_example
}

fn forget_example() {
    // ANCHOR: forget_example
    let s = String::from("Hello");
    forget(s); // Leak memory
    // ANCHOR_END: forget_example
}

fn raw_pointer() {
    // ANCHOR: raw_pointer
    let boxed = Box::new(String::from("Crab"));
    let raw_ptr = Box::into_raw(boxed);
    let _ = unsafe { Box::from_raw(raw_ptr) }; // will be freed
    // ANCHOR_END: raw_pointer
}

fn into_raw() {
    // ANCHOR: into_raw
    // Excerpt from the standard library documentation
    use std::alloc::{Layout, dealloc};
    use std::ptr;

    let x = Box::new(String::from("Hello"));
    let p = Box::into_raw(x);
    unsafe {
        ptr::drop_in_place(p);
        dealloc(p as *mut u8, Layout::new::<String>());
    }
    // ANCHOR_END: into_raw
}

mod cyclic {
    // ANCHOR: cyclic
    use std::{cell::Cell, rc::Rc};

    struct LinkedStruct {
        other: Cell<Option<Rc<LinkedStruct>>>,
    }

    fn main() {
        println!("Hello, world!");
        let a = Rc::new(LinkedStruct {
            other: Cell::new(None),
        });
        let b = Rc::new(LinkedStruct {
            other: Cell::new(None),
        });
        let aa = a.clone();
        let bb = b.clone();
        a.other.set(Some(bb));
        b.other.set(Some(aa));
    }
    // ANCHOR_END: cyclic
}
