use assert_cli;

#[test]
fn sample1_1() {
    assert_cli::Assert::main_binary()
        .stdin("9 players; last marble is worth 25 points")
        .stdout()
        .is("32")
        .unwrap();
}

#[test]
fn sample1_2() {
    assert_cli::Assert::main_binary()
        .stdin("10 players; last marble is worth 1618 points")
        .stdout()
        .is("8317")
        .unwrap();
}

#[test]
fn sample1_3() {
    assert_cli::Assert::main_binary()
        .stdin("13 players; last marble is worth 7999 points")
        .stdout()
        .is("146373")
        .unwrap();
}

#[test]
fn sample1_4() {
    assert_cli::Assert::main_binary()
        .stdin("17 players; last marble is worth 1104 points")
        .stdout()
        .is("2764")
        .unwrap();
}

#[test]
fn sample1_5() {
    assert_cli::Assert::main_binary()
        .stdin("21 players; last marble is worth 6111 points")
        .stdout()
        .is("54718")
        .unwrap();
}

#[test]
fn sample1_6() {
    assert_cli::Assert::main_binary()
        .stdin("30 players; last marble is worth 5807 points")
        .stdout()
        .is("37305")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("404611")
        .unwrap();
}

#[test]
fn sample2_1() {
    assert_cli::Assert::main_binary()
        .with_args(&["-p", "2"])
        .stdin("9 players; last marble is worth 25 points")
        .stdout()
        .is("22563")
        .unwrap();
}

#[test]
fn puzzle2() {
    assert_cli::Assert::main_binary()
        .with_args(&["-p", "2"])
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("3350093681")
        .unwrap();
}
