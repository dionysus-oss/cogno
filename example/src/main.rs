#[macro_use]
extern crate cogno;

use std::process::Command;

mod sting;

#[cogno_test(spec = "1234")]
fn test() {
    should_eq!("rfc_1234_sec_8.1", 'a', 'a');
    should_not_eq!("rfc_1234_sec_8.2", 'a', 'b');
}

#[cogno_test(spec = "1234")]
fn panic_test() {
    panic!("I'm not a great test");
}

#[cogno_test(spec = "1234")]
fn failed_assertions() {
    let child = Command::new("ls").arg("-al").spawn().unwrap();
    let mut close_handle = defer_close!(child);

    must_eq!("rfc_must", 'a', 'b');
    must_not_eq!("rfc_must_not", 'a', 'a');
    should_eq!("rfc_should", 'a', 'b');
    should_not_eq!("rfc_should_not", 'a', 'a');
    may_eq!("rfc_may", 'a', 'b');

    println!("command output {:?}", close_handle.command().stdout);
}

#[cogno_main]
fn main() {}
