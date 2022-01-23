#[cfg(test)]
mod common;
use common::*;

fn test_subcommand_list(output_str: &str) {
    let list_str = "list";
    let output = run_cli(Some(list_str));

    assert_eq!(&output.stdout[..], output_str.as_bytes());
}

fn test_subcommand_add(add_str: &str, output_str: &str) {
    let add = "add ".to_string() + add_str;
    let output = run_cli(Some(&add));

    assert_eq!(&output.stdout[..], output_str.as_bytes());
}

fn test_subcommand_delete(del_str: &str, output_str: &str) {
    let del = "del ".to_string() + del_str;
    let output = run_cli(Some(&del));

    assert_eq!(&output.stdout[..], output_str.as_bytes());
}

#[test]
fn test_subcommand() {
    // ToDor list
    test_subcommand_list("");
    // ToDor add
    test_subcommand_add("", "Please Input ToDo Description");
    // ToDor add "123"
    test_subcommand_add("\"123\"", "");
    // ToDor list
    test_subcommand_list("0 123");
    // ToDor delete 1
    test_subcommand_delete("1", "No such ToDo Item");
    // ToDor delete 0
    test_subcommand_delete("0", "");
    // ToDor list
    test_subcommand_list("");
    // ToDor delete 0
    test_subcommand_delete("0", "No such ToDo Item");
}
