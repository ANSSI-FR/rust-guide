#![allow(dead_code)]
use std::num::Wrapping;
use std::panic;

fn use_integer() {
    let x: u8 = 242;

    let result = panic::catch_unwind(|| {
        println!("{}", x + 50); // panics in debug, prints 36 in release.
    });
    if result.is_err() {
        println!("panic");
    }
    // always prints 36.
    println!("{}", x.overflowing_add(50).0);
    // always prints 36.
    println!("{}", x.wrapping_add(50));
    // always prints 36.
    println!("{}", Wrapping(x) + Wrapping(50));

    // always panics:
    let (res, c) = x.overflowing_add(50);
    let result = panic::catch_unwind(|| {
        if c {
            panic!("custom error");
        } else {
            println!("{}", res);
        }
    });
    if result.is_err() {
        println!("panic");
    }
}
