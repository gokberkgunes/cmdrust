use assert_cmd::Command;
use predicates::prelude::*;

// Create a new type called TestResult
// This type uses a standart type: Result<T, E>
// Result has two generic parameters, T and E. T is returned on success, E is on error.one 
// Our type returns () on success, () means something like void. Thus, return nothing on success.
// Our type returns a Box meaning heap allocated object.
// dyn means to use dynamic dispatch which allow flexible error capture.
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    // ? is used to unpack Ok or send in Err into cmd.
    let mut cmd = Command::cargo_bin("echor")?;

    // Here we make sure that if failed, the standard error has the string USAGE.
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));

    // If we got to this point without error in this function, we return Ok.
    Ok(())
}

// Here we're using a slice of str, &[&str], because we need a fixed size structure to hold data.
fn run(arguments: &[&str], expected_file_path: &str) -> TestResult {
    // read_to_string returns Result<String, std::io::Error>, we get String with .unwrap() or ?
    // In book, it's noted that this function is convenient but dangerous. it doesn't have limits
    // on how much it should read and write to memory; thus, may crashe the computer or program.
    let expected = std::fs::read_to_string(expected_file_path)?;

    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(arguments)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn one_arg_test() -> TestResult {
    return run(&["arg1"], "tests/expected/test_1"); // may omit return and ;
}

#[test]
fn two_args_test() -> TestResult {
    run(&["arg1", "arg2"], "tests/expected/test_2")
}

#[test]
fn one_arg_no_new_line_test() -> TestResult {
    run(&["-n", "arg1"], "tests/expected/test_3")
}

#[test]
fn two_arg_no_new_line_test() -> TestResult {
    return run(&["-n", "arg1", "arg2"], "tests/expected/test_4");
}
