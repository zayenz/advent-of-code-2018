#![allow(
    dead_code,
    unused_imports,
    clippy::needless_range_loop,
    clippy::ptr_arg,
    clippy::char_lit_as_u8
)]

use std::char;
use std::cmp::{max, min};
use std::fmt;
use std::io::BufRead;
use std::iter::*;
use std::ops::*;
use std::str;
use std::str::FromStr;
use std::{io, process};

use failure::bail;
use failure::err_msg;
use failure::Error;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rayon::prelude::*;
use stats::Frequencies;
use structopt::StructOpt;
use strum_macros::EnumString;
use tap::{TapOps, TapOptionOps, TapResultOps};

use aoc2018::input::*;
use aoc2018::matrix::*;
use std::collections::BTreeSet;
use std::fmt::Display;
use std::fmt::Formatter;

type Input = usize;
type Output = (usize, usize, usize);

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?.trim().to_owned();
        if !line.is_empty() {
            let serial = line.parse()?;
            return Ok(serial);
        }
    }

    bail!("No input found")
}

fn grid(serial: usize) -> Vec<Vec<i64>> {
    let mut grid = vec![vec![0; 300]; 300];

    for x in 0..300 {
        for y in 0..300 {
            grid[x][y] = power_level(serial, x + 1, y + 1);
        }
    }

    grid
}

fn power_level(serial: usize, x: usize, y: usize) -> i64 {
    let rack_id = x + 10;
    let power = y * rack_id;
    let power_start = power + serial;
    let power_indication = (power_start) * rack_id;
    let power_base = (power_indication / 100) % 10;
    power_base as i64 - 5
}

fn solve1(input: &mut Input) -> Result<Output, Error> {
    let grid = grid(*input);

    let mut res_x = 0;
    let mut res_y = 0;
    let mut res_power = -10000;

    for x in 0..(300 - 2) {
        for y in 0..(300 - 2) {
            let mut power = 0;
            for dx in 0..3 {
                for dy in 0..3 {
                    power += grid[x + dx][y + dy];
                }
            }
            if power > res_power {
                res_power = power;
                res_x = x;
                res_y = y;
            }
        }
    }

    Ok((res_x + 1, res_y + 1, 3))
}

fn solve2(input: &mut Input) -> Result<Output, Error> {
    let grid = grid(*input);

    let mut res_x = 0;
    let mut res_y = 0;
    let mut res_power = -10000;
    let mut res_d = 0;

    // Cumulative sums for box from bottom left corner up to (x,y)
    let mut sums = vec![vec![0; 300]; 300];
    sums[299][299] = grid[299][299];
    for i in (0..299).rev() {
        sums[299][i] = grid[299][i] + sums[299][i + 1];
        sums[i][299] = grid[i][299] + sums[i + 1][299];
    }
    let mut y_sum = vec![0; 300];
    for x in (0..299).rev() {
        y_sum[299] = grid[x][299];
        for y in (0..299).rev() {
            y_sum[y] = y_sum[y + 1] + grid[x][y];
        }
        for y in (0..299).rev() {
            sums[x][y] = y_sum[y] + sums[x + 1][y];
        }
    }

    for x in 0..300 {
        for y in 0..300 {
            let max_d = 300 - max(x, y);
            for d in 1..max_d {
                let power = sums[x][y] - sums[x + d][y] - sums[x][y + d] + sums[x + d][y + d];
                if power >= res_power {
                    res_power = power;
                    res_x = x;
                    res_y = y;
                    res_d = d;
                }
            }
        }
    }

    Ok((res_x + 1, res_y + 1, res_d))
}

#[derive(StructOpt, Debug)]
#[structopt(name = "day6")]
struct Opt {
    /// Part to solve, either 1 or 2
    #[structopt(short = "-p", long = "--part", default_value = "1")]
    part: u8,
}

fn run() -> Result<(), Error> {
    let mut input = read_input()?;

    let options: Opt = Opt::from_args();

    let output = if options.part == 1 {
        solve1(&mut input)?
    } else {
        solve2(&mut input)?
    };

    println!("{:?}", output);
    Ok(())
}

fn main() {
    match run() {
        Ok(()) => process::exit(0),
        Err(error) => {
            eprintln!("Error while solving problem: {}", error);
            for cause in error.iter_causes() {
                eprintln!("{}", cause)
            }
            process::exit(1)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_power_level() {
        assert_eq!(power_level(8, 3, 5), 4);
        assert_eq!(power_level(57, 122, 79), -5);
        assert_eq!(power_level(39, 217, 196), 0);
        assert_eq!(power_level(71, 101, 153), 4);
    }
}
