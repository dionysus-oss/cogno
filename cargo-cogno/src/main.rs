use anyhow::{anyhow, Result};
use cargo::util::command_prelude::ArgMatchesExt;
use cargo::util::command_prelude::*;
use cargo::{CliResult, Config};
use cargo_util::{ProcessBuilder, ProcessError};
use clap::{ArgMatches, Command, CommandFactory, Parser};
use std::ffi::OsStr;
use std::process::exit;

mod discover;

fn main() -> Result<()> {
    let config = Config::default()?;

    let (app, args) = parse_args()?;
    if args.flag("version") {
        println!("cogno-{}", app.get_version().unwrap());
        return Ok(());
    }

    let ws = args.workspace(&config)?;

    let fs = ws.target_dir();
    let path = fs.as_path_unlocked();

    // TODO how to detect/handle a workspace with multiple projects
    let current_project = ws.current_opt().unwrap().root();

    let found = discover::discover(&current_project.join("src"))?;
    let manifest_path = path.join("cogno-manifest.json");
    core::dump_manifest(found, &manifest_path)?;

    std::env::set_var("COGNO_MANIFEST", manifest_path.to_str().unwrap());

    let run_result = call_cargo_run();
    if run_result.is_err() {
        let e = run_result.unwrap_err();
        if e.error.is_some() {
            println!("call run failed {:?}", e.exit_code);
        }
        exit(e.exit_code);
    }

    Ok(())
}

#[derive(Parser)]
struct CognoCli {
    #[clap(long)]
    version: bool,
}

fn make_command() -> Command {
    CognoCli::command()
        .name("cogno")
        .about("Run conformance tests")
        // Taken from Cargo's `src/bin/cargo/commands/run.rs`
        .about("Run a binary or example of the local package")
        .arg_quiet()
        .arg(
            Arg::new("args")
                .value_parser(value_parser!(std::ffi::OsString))
                .num_args(0..)
                .trailing_var_arg(true),
        )
        .arg_targets_bin_example(
            "Name of the bin target to run",
            "Name of the example target to run",
        )
        .arg_package("Package with the target to run")
        .arg_jobs()
        .arg_release("Build artifacts in release mode, with optimizations")
        .arg_profile("Build artifacts with the specified profile")
        .arg_features()
        .arg_target_triple("Build for the target triple")
        .arg_target_dir()
        .arg_manifest_path()
        .arg_message_format()
        .arg_unit_graph()
        .arg_ignore_rust_version()
        .arg_timings()
        .after_help("Run `cargo help run` for more detailed information.\n")
}

fn parse_args() -> Result<(Command, ArgMatches)> {
    let command = make_command();

    let mut app = clap::command!().subcommand(command);

    let args = app.clone().get_matches();

    match args.subcommand() {
        Some(("cogno", args)) => Ok((app, args.clone())),
        _ => {
            app.render_usage();
            return Err(anyhow!("Invalid usage"));
        }
    }
}

pub fn call_cargo_run() -> CliResult {
    let mut args = vec![OsStr::new("run")];

    // Forward args, skipping `cargo cogno`
    let args1: Vec<String> = std::env::args().skip(2).collect();
    args.extend(args1.iter().map(|f| OsStr::new(f)));

    let err = match ProcessBuilder::new("cargo").args(&args).exec_replace() {
        Ok(()) => return Ok(()),
        Err(e) => e,
    };

    if let Some(p_err) = err.downcast_ref::<ProcessError>() {
        if let Some(code) = p_err.code {
            return Err(CliError::code(code));
        }
    }

    Err(CliError::new(err, 1))
}
