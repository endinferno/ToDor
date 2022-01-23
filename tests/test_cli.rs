#[cfg(test)]
mod common;
use common::*;

fn test_raw_cli() {
    let help_str = format!("{}\n", HELP);
    let output = run_cli(None);

    assert_eq!(&output.stdout[..], help_str.as_bytes());
}

fn test_help_cli() {
    let help_str = format!("{}\n", HELP);
    let output = run_cli(Some("--help"));

    assert_eq!(&output.stdout[..], help_str.as_bytes());
}

fn test_h_cli() {
    let help_str = format!("{}\n", HELP);
    let output = run_cli(Some("-h"));

    assert_eq!(&output.stdout[..], help_str.as_bytes());
}

fn test_v_cli() {
    let version_str = format!("{} {}\n", "ToDor", env!("CARGO_PKG_VERSION"));
    let output = run_cli(Some("-V"));

    assert_eq!(&output.stdout[..], version_str.as_bytes());
}

fn test_version_cli() {
    let version_str = format!("{} {}\n",  "ToDor", env!("CARGO_PKG_VERSION"));
    let output = run_cli(Some("--version"));

    assert_eq!(&output.stdout[..], version_str.as_bytes());
}

#[test]
fn test_cli() {
    // ToDor
    test_raw_cli();
    // ToDor --help
    test_help_cli();
    // ToDor -h
    test_h_cli();
    // ToDor --version
    test_version_cli();
    // ToDor -V
    test_v_cli();
}
