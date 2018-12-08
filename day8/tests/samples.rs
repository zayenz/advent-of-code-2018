use assert_cli;

#[test]
fn sample1_1() {
    assert_cli::Assert::main_binary()
        .stdin("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2")
        .stdout()
        .is("138")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("36027")
        .unwrap();
}

#[test]
fn sample2_1() {
    assert_cli::Assert::main_binary()
        .with_args(&["-p", "2"])
        .stdin("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2")
        .stdout()
        .is("66")
        .unwrap();
}

#[test]
fn puzzle2() {
    assert_cli::Assert::main_binary()
        .with_args(&["-p", "2"])
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("23960")
        .unwrap();
}
