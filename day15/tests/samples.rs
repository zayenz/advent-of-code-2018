use assert_cli;

#[test]
fn sample1_1() {
    assert_cli::Assert::main_binary()
        .stdin(
            "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######",
        )
        //        .stdout()
        //        .is("27730")
        .unwrap();
}

#[test]
fn sample1_2() {
    assert_cli::Assert::main_binary()
        .stdin(
            "#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######",
        )
        //        .stdout()
        //        .is("36334")
        .unwrap();
}

#[test]
fn sample1_3() {
    assert_cli::Assert::main_binary()
        .stdin(
            "#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######",
        )
        //        .stdout()
        //        .is("39514")
        .unwrap();
}

#[test]
fn sample1_4() {
    assert_cli::Assert::main_binary()
        .stdin(
            "#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######",
        )
        //        .stdout()
        //        .is("27755")
        .unwrap();
}

#[test]
fn sample1_5() {
    assert_cli::Assert::main_binary()
        .stdin(
            "#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######",
        )
        //        .stdout()
        //        .is("28994")
        .unwrap();
}

#[test]
fn sample1_6() {
    assert_cli::Assert::main_binary()
        .stdin(
            "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########",
        )
        //        .stdout()
        //        .is("18740")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        //        .stdout()
        //        .is("1")
        .unwrap();
}

#[test]
fn puzzle2() {
    assert_cli::Assert::main_binary()
        .with_args(&["-p", "2"])
        .stdin(include_str!("../data/puzzle1.in"))
        //        .stdout()
        //        .is("2")
        .unwrap();
}
