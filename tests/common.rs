#[cfg(test)]
use std::path::PathBuf;
use std::process::{Command, Output};

#[allow(dead_code)]
pub const HELP: &str = "\
ToDor
endinferno <censhaofeng@sjtu.edu.cn>
TODO Command Line Tool

USAGE:
    ToDor [FLAGS] <INPUT> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints this message
    -V, --version    Prints version information

ARGS:
    INPUT    The input TODO content

SUBCOMMANDS:
    list    Prints TODO list
    add     Add TODO item
    delete  Delete TODO item\
";

#[allow(dead_code)]
pub fn run_cli(arg: Option<&str>) -> Output {
    let mut path = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
        .expect("Failed to read env var CARGO_MANIFEST_DIR"));
    path.push("target/debug/ToDor");
    let output: Output;
    match arg {
        None => {
            output = Command::new(path)
                .output()
                .expect("Failed to execute command");
        },
        _ => {
            output = Command::new(path)
                .arg(arg.unwrap())
                .output()
                .expect("Failed to execute command");
        },
    }

    return output;
}
