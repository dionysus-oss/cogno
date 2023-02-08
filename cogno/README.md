# Cogno

#### Contents
- [What is it?](#what-is-it)
- [Quick start](#quick-start)


#### What is it?

_TL;DR_ A test harness for creating RFC spec tests. 

It provides

- Specialised assertions like `must_eq` and `should_eq!` to match the language RFCs use and automatically translate
  into test outcomes.
- Tests that do not panic, so that all assertions are expected to run. This gives confidence that the test passed
  or failed for the intended reasons.
- Specification identifiers as metadata for tests so that conformance against multiple RFCs can be tested in a flexible way.
- Assertion identifiers so that specifications can evolve. This allows assertions to be explicitly changed outside of the code
  as requirements change.

#### Quick start

Create a new binary package with

```shell
$ cargo init --bin <package-name>
$ cd <package-name>
```

A good package name specifies the RFC content being tested. For example, `cogno-spec-dns` to test the DNS as opposed to `cogno-spec-rfc-1034`.

Add a dependency on the `cogno` test harness

```shell
$ cargo add --features console cogno
```

Install the Cargo plugin

```shell
$ cargo install cargo-cogno --locked
```

Add sample code to your application in `src/main.rs`

```
#[macro_use]
extern crate cogno;

#[cogno_test(spec = "<my-rfc-id>")]
fn test() {
    should_eq!("<my-assertion-id>", 'a', 'a');
}

#[cogno_main]
fn main() {}
```

This will compile and run as-is, or you can update the values in angle brackets to match your project.

Now you can run the tests

```shell
$ cargo cogno --spec '<my-rfc-id>'
```

You should see `✓ - test` as the output.

That's it. You can start writing tests, or you can check out the more detailed documentation.
