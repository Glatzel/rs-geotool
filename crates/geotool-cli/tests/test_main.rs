use assert_cmd::Command;
use predicates::prelude::*; // For more readable assertions

#[test]
fn test_main_with_args() {
    // Use Command to run the binary
    let mut cmd = Command::cargo_bin("geotool").unwrap();

    // Pass in command-line arguments, for example: `program_name arg1 arg2`
    cmd.arg("--help");

    // Assert the output printed to stdout (modify expected output as needed)
    cmd.assert().success().stdout(predicate::str::contains("transform"));
}
