use assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin(
            "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
",
        )
        .stdout()
        .is("fgij")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("pbykrmjmizwhxlqnasfgtycdv")
        .unwrap();
}
