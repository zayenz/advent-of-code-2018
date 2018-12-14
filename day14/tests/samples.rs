use assert_cli;

#[test]
fn sample1_1() {
    assert_cli::Assert::main_binary()
        .stdin("9")
        .stdout()
        .is("5158916779")
        .unwrap();
}

#[test]
fn sample1_2() {
    assert_cli::Assert::main_binary()
        .stdin("5")
        .stdout()
        .is("0124515891")
        .unwrap();
}

#[test]
fn sample1_3() {
    assert_cli::Assert::main_binary()
        .stdin("18")
        .stdout()
        .is("9251071085")
        .unwrap();
}

#[test]
fn sample1_4() {
    assert_cli::Assert::main_binary()
        .stdin("2018")
        .stdout()
        .is("5941429882")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("2111113678")
        .unwrap();
}

#[test]
fn sample2_1() {
    assert_cli::Assert::main_binary()
        .with_args(&["-p", "2"])
        .stdin("51589")
        .stdout()
        .is("9")
        .unwrap();
}

#[test]
fn sample2_2() {
    assert_cli::Assert::main_binary()
        .with_args(&["-p", "2"])
        .stdin("01245")
        .stdout()
        .is("5")
        .unwrap();
}

#[test]
fn sample2_3() {
    assert_cli::Assert::main_binary()
        .with_args(&["-p", "2"])
        .stdin("92510")
        .stdout()
        .is("18")
        .unwrap();
}

#[test]
fn sample2_4() {
    assert_cli::Assert::main_binary()
        .with_args(&["-p", "2"])
        .stdin("59414")
        .stdout()
        .is("2018")
        .unwrap();
}

#[test]
fn puzzle2() {
    assert_cli::Assert::main_binary()
        .with_args(&["-p", "2"])
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("20195114")
        .unwrap();
}
