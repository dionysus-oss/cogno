#[macro_use]
extern crate cogno;

mod sting;

#[cogno_test]
fn test() {
    println!("I'm a test");
}

#[cogno_main]
fn main() {}
