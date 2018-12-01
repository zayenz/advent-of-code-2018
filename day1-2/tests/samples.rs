use assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin(
            "+1
-1",
        )
        .stdout()
        .is("0")
        .unwrap();
}

#[test]
fn sample2() {
    assert_cli::Assert::main_binary()
        .stdin(
            "+3
+3
+4
-2
-4
",
        )
        .stdout()
        .is("10")
        .unwrap();
}

#[test]
fn sample3() {
    assert_cli::Assert::main_binary()
        .stdin(
            "-6
+3
+8
+5
-6",
        )
        .stdout()
        .is("5")
        .unwrap();
}

#[test]
fn sample4() {
    assert_cli::Assert::main_binary()
        .stdin(
            "+7
+7
-2
-7
-4",
        )
        .stdout()
        .is("14")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("341")
        .unwrap();
}
