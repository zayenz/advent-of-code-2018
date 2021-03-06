use assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin("dabAcCaCBAcCcaDA")
        .stdout()
        .is("4")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("6118")
        .unwrap();
}
