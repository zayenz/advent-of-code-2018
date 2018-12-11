use assert_cli;

#[test]
fn sample1_1() {
    assert_cli::Assert::main_binary()
        .stdin("18")
        .stdout()
        .is("(33, 45, 3)")
        .unwrap();
}

#[test]
fn sample1_2() {
    assert_cli::Assert::main_binary()
        .stdin("42")
        .stdout()
        .is("(21, 61, 3)")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("(235, 35, 3)")
        .unwrap();
}

#[test]
fn sample2_1() {
    assert_cli::Assert::main_binary()
        .with_args(&["-p", "2"])
        .stdin("18")
        .stdout()
        .is("(90, 269, 16)")
        .unwrap();
}

#[test]
fn sample2_2() {
    assert_cli::Assert::main_binary()
        .with_args(&["-p", "2"])
        .stdin("42")
        .stdout()
        .is("(232, 251, 12)")
        .unwrap();
}

#[test]
fn puzzle2() {
    assert_cli::Assert::main_binary()
        .with_args(&["-p", "2"])
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("(142, 265, 7)")
        .unwrap();
}
