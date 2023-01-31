#[macro_use]
extern crate cogno;

mod sting;

#[cogno_test]
fn test() {
    println!("I'm a test");

    should_eq!("rfc_1234_sec_8.1", 'a', 'a');
    should_not_eq!("rfc_1234_sec_8.2", 'a', 'b');
}

#[cogno_main]
fn main() {}
