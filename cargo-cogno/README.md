## cargo-cogno

A Cargo plugin for running Cogno tests. It can be installed using

```shell
$ cargo install cargo-cogno --locked
```

You will need to add the associated [test harness](https://crates.io/crates/cogno) to your project 
and follow the [setup instruction](https://crates.io/crates/cogno#quick-start).

#### Usage

You can select the specification to test using the `--spec` flag

```shell
$ cargo cogno --spec 'spec-1234'
```

Or test multiple specifications at the same time

```shell
$ cargo cogno --spec 'spec-1234' --spec 'spec-1235'
```

---

You can select a reporter to use with the `--reporter` flag

```shell
$ cargo cogno --spec 'spec-1234' --reporter raw
```

The `console` reporter is the default if available (it is an optional feature of the Cogno test harness). Otherwise, the `raw` reporter is the default.
The `raw` reporter dumps the raw test results as a JSON file in the current directory.

---

You can load a modifier configuration file using the `--modifier` flag

```shell
$ cargo cogno --spec 'spec-1234' --modifier modifier-spec-1234.toml
```

Or load multiple modifier configuration files at the same time

```shell
$ cargo cogno --spec 'spec-1234' --spec 'spec-1235' --modifier modifier-spec-1234.toml --modifier modifier-spec-1235.toml
```

---

You can enable tracing for the Cogno test harness using the `--trace` flag.

```shell
$ cargo cogno --spec 'spec-1234' --trace
```

This will print detailed information to the console which can be useful to debug issues. This is most useful to Cogno test harness developers.
However, you can add additional [tracing](https://docs.rs/tracing/latest/tracing/) information to your tests if necessary that consumers of your tests can use to debug issues.

---

You can pass extra arguments to `cargo run` when calling `cargo cogno`. For example, to pass the `--quiet` flag to Cargo
you can use

```shell
$ cargo cogno --spec 'spec-1234' -- --quiet
```

Any `cargo run` CLI options can be specified after a `--` to separate the arguments for `cogno` and `run`.

You can think of the `cogno` Cargo plugin as a pre-processor for the project. The implementation replaces the plugin process with a
`cargo run` process once the required setup is done.
