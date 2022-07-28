use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"10
9
10
3
100
100
90
80
10
30
10
"#)
        .tee_output()
        .expect_success();

    assert_eq!(output.stdout_str(), r#"up 1
down 7
up 97
stay
down 10
down 10
down 70
up 20
down 20
"#);

    assert!(output.stderr_str().is_empty());
}
