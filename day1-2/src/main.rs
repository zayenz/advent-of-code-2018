#![allow(dead_code, unused_imports)]

use failure::bail;
use failure::Error;
use rayon::prelude::*;
use strum_macros::EnumString;

use hashbrown::{HashMap, HashSet};
use std::char;
use std::cmp::{max, min};
use std::fmt;
use std::io::BufRead;
use std::ops::*;
use std::str;
use std::str::FromStr;
use std::{io, process};

type Input = Vec<i32>;
type Output = i32;

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut result = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        result.push(line.parse()?)
    }

    Ok(result)
}

fn solve(input: &mut Input) -> Result<Output, Error> {
    let mut sum = 0;
    let mut sums = HashSet::new();
    sums.insert(sum);
    loop {
        for i in input.iter() {
            sum += *i;
            if sums.contains(&sum) {
                return Ok(sum);
            }
            sums.insert(sum);
        }
    }
}

fn run() -> Result<(), Error> {
    let mut input = read_input()?;
    //println!("Read input {:?}", input);
    let output = solve(&mut input)?;

    println!("{}", output);
    Ok(())
}

fn main() {
    match run() {
        Ok(()) => process::exit(0),
        Err(error) => {
            for cause in error.iter_causes() {
                eprintln!("{}", cause)
            }
            process::exit(1)
        }
    }
}
