use assert_cli;

#[test]
fn sample1_1() {
    assert_cli::Assert::main_binary()
        .stdin(
            "|
v
|
|
|
^
|",
        )
        .stdout()
        .is("(0,3)")
        .unwrap();
}

#[test]
fn sample1_2() {
    assert_cli::Assert::main_binary()
        .stdin(
            r"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ",
        )
        .stdout()
        .is("(7,3)")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("(115,138)")
        .unwrap();
}

#[test]
fn sample2_1() {
    assert_cli::Assert::main_binary()
        .with_args(&["-p", "2"])
        .stdin(
            r"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/",
        )
        .stdout()
        .is("(6,4)")
        .unwrap();
}
#[test]
fn puzzle2() {
    assert_cli::Assert::main_binary()
        .with_args(&["-p", "2"])
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("(0,98)")
        .unwrap();
}
