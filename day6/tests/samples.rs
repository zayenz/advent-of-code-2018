use assert_cli;

#[test]
fn sample1_1() {
    assert_cli::Assert::main_binary()
        .stdin(
            "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9",
        )
        .stdout()
        .is("17")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("4829")
        .unwrap();
}

#[test]
fn puzzle2() {
    assert_cli::Assert::main_binary()
        .with_args(&["-p", "2"])
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("46966")
        .unwrap();
}
