use assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin(
            "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
",
        )
        .stdout()
        .is("12")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("6000")
        .unwrap();
}
