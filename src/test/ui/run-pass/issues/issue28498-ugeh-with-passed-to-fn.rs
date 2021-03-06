// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
#![allow(deprecated)] // FIXME: switch to `#[may_dangle]` below.

// Demonstrate the use of the unguarded escape hatch with a type param in negative position
// to assert that destructor will not access any dead data.
//
// Compare with compile-fail/issue28498-reject-lifetime-param.rs

// Demonstrate that a type param in negative position causes dropck to reject code
// that might indirectly access previously dropped value.
//
// Compare with run-pass/issue28498-ugeh-with-passed-to-fn.rs

#![feature(dropck_parametricity)]

#[derive(Debug)]
struct ScribbleOnDrop(String);

impl Drop for ScribbleOnDrop {
    fn drop(&mut self) {
        self.0 = format!("DROPPED");
    }
}

struct Foo<T>(u32, T, Box<for <'r> fn(&'r T) -> String>);

impl<T> Drop for Foo<T> {
    #[unsafe_destructor_blind_to_params]
    fn drop(&mut self) {
        // Use of `unsafe_destructor_blind_to_params` is sound,
        // because destructor never passes a `self.1` to the callback
        // (in `self.2`) despite having it available.
        println!("Dropping Foo({}, _)", self.0);
    }
}

fn callback(s: & &ScribbleOnDrop) -> String { format!("{:?}", s) }

fn main() {
    let (last_dropped, foo0);
    let (foo1, first_dropped);

    last_dropped = ScribbleOnDrop(format!("last"));
    first_dropped = ScribbleOnDrop(format!("first"));
    foo0 = Foo(0, &last_dropped, Box::new(callback));
    foo1 = Foo(1, &first_dropped, Box::new(callback));

    println!("foo0.1: {:?} foo1.1: {:?}", foo0.1, foo1.1);
}
