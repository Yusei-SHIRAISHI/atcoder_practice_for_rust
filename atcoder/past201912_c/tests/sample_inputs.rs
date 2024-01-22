use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin("4 18 25 20 9 13")
        .tee_output()
        .expect_success();

    assert_eq!(output.stdout_str(), "18");

    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin("95 96 97 98 99 100")
        .tee_output()
        .expect_success();

    assert_eq!(output.stdout_str(), "98");

    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin("19 92 3 35 78 1")
        .tee_output()
        .expect_success();

    assert_eq!(output.stdout_str(), "35");

    assert!(output.stderr_str().is_empty());
}
