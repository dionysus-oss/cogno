# Cogno

#### Contents
- [What is it?](#what-is-it)
- [Quick start](#quick-start)
- [Usage notes](#usage-notes)
- [Evolving specifications with modifiers](#evolving-specifications-with-modifiers)
- [Advice for creating tests and modifiers](#advice-for-creating-tests-and-modifiers)
- [A note on abuse](#a-note-on-abuse)

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

That's it. You can start writing tests, or you can check out the more detailed documentation below.

#### Usage notes

The following are suggestions and restrictions to get the best out of Cogno

- The crate must be a binary with the `cogno_main` and `cogno_test` attributes used as documented.
- All source code must live inside the `src` directory of the crate. You can organise your code any way you like but tests
  imported from other local crates in a workspace or otherwise, will not be found.
- Panic as rarely as possible. The harness should catch and handle panics but your test will of course, not complete. 
  This is not desirable and is best avoided.
- Because the provided assertions do not panic, exiting a test without panicking is not the same as a successful test. You should always aim
  to include at least one assertion in your test. This is not currently enforced but may become at least a warning in the future.
- Use an IDE to write your code. The Rust source generation in the macros is imperfect and will be tripped up by syntax errors.
  The compiler errors you get for syntax and Cogno macro errors will not necessarily be reported as well as usual by the Rust compiler. 
  Rely on your IDE for these issues. For other types of error the compiler errors should appear as expected. 

#### Evolving specifications with modifiers

As specifications evolve, many requirements stay valid and others need to change. The tests themselves should not change
because they should continue to describe the original specification.

Given two tests for spec `1234`, the original specification, and `1235`, the new specification

```
#[cogno_test(spec = "1234")]
fn test_original() {
    must_eq!("rfc_1234_assertion_id", 'a', 'a');
}

#[cogno_test(spec = "1235")]
fn test_new() {
    must_eq!("rfc_1235_assertion_id", 'b', 'b');
}
```

Where the specification for `1235` no longer makes `rfc_1234_assertion_id` a hard requirement. Implementations should 
continue to support the feature but a valid implementation `1235` does not require `rfc_1234_assertion_id`.

You can provide a _modifier_ configuration to describe this

```toml
[[spec_modifiers]]
spec_id = "1234"

test_modifiers = [
  { test_id = "test_original", assertion_modifiers = [
    { assertion_id = "rfc_1234_assertion_id", assertion_type = "Should" },
  ] },
]
```

This is saved in a file named after the spec that requires them, such as `modifier-rfc-1235.toml`. It is then provided
to `cogno` when testing for compliance with `1235`

```shell
$ cargo cogno --spec 1234 --spec 1235 --modifier modifier-rfc-1235.toml
```

The result of the original assertion will be transformed to apply the `Should` assertion in place of the `Must` assertion.

#### Advice for creating tests and modifiers

It is up to the end-user of your tests which specifications they are implementing and how to use your tests and modifiers.
With this in mind, you should aim to make your tests flexible.

You should aim to group tests which are expected to be run against the same program or library. For example, the `DNS` specification
is made up of [many specifications](https://en.wikipedia.org/wiki/Domain_Name_System#RFC_documents) where there is expected to be
a `name server` and a `resolver` component. This could be written as one or two test suites. While `HTTP` is also formed of multiple
specifications, versions 1, 2 and 3, it is entirely reasonable for programs to only implement one of these. In this case it may make sense
to split test suites by HTTP.

Similarly, for modifiers you should aim for flexibility. Where possible it is desirable to provide modifiers which are independent.
Say specification `1234` is updated by both `1235` and `1236`, then you should provide `modifier-rfc-1235.toml` and `modifier-rfc-1236.toml` which
to not update the same assertions. Cogno will detect it if you do, and stop the problem test. If `1236` conflicts with or is an update to `1235`
then you should include the modifiers relevant to `1235` in the modifier toml for `1236` and document for your users that they should use either 
`cargo cogno --spec 1234 --spec 1235 --modifier modifier-rfc-1235.toml` or `cargo cogno --spec 1234 --spec 1235 --spec 1236 --modifier modifier-rfc-1236.toml`.

Flexibility and documentation are important but the end goal is to provide tests that can be used as widely as possible.
The aim should be to describe the specification as completely as possible, without being more restrictive than the specification.

#### A note on abuse

This crate exposes more than it strictly should as a library. It does this for good reason - to expose functionality to your tests
via code generated by the `cogno_main` and `cogno_test` attributes.

The following code will compile and run

```rust
#[cogno_test]
fn test() {
    should_eq!("test_assertion_id", 'a', 'a');

    controller_thread_ref.lock().unwrap().register("naughty", "test");
}
```

Please don't do this! There should be no need to interact with the generated code. If you experience a bug then please raise it on
GitHub and I will try to fix it. If you are curious how the test harness works, feel free to fork it and experiment.

The result of modifying the `TestController` state or accessing anything else which is documented as 'internal' is likely to result in unreliable tests
or undefined behaviour.

You should only need to use the exported macros to build you tests.
