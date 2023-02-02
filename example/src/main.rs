#[macro_use]
extern crate cogno;

mod sting;

#[cogno_test]
fn test() {
    println!("I'm a test");

    should_eq!("rfc_1234_sec_8.1", 'a', 'a');
    should_not_eq!("rfc_1234_sec_8.2", 'a', 'b');
}

#[cogno_test]
fn panic_test() {
    panic!("I'm not a great test")
}

#[cogno_test]
fn failed_assertions() {
    must_eq!("rfc_must", 'a', 'b');
    must_not_eq!("rfc_must", 'a', 'a');
    should_eq!("rfc_must", 'a', 'b');
    should_not_eq!("rfc_must", 'a', 'a');
    may_eq!("rfc_must", 'a', 'b');
}

#[cogno_main]
fn main() {}
