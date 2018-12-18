use assert_cli;

#[test]
fn sample1_1() {
    assert_cli::Assert::main_binary()
        .stdin(
            ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.",
        )
        .stdout()
        .is("1147")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("483840")
        .unwrap();
}

#[test]
fn puzzle2() {
    assert_cli::Assert::main_binary()
        .with_args(&["-p", "2"])
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("219919")
        .unwrap();
}
