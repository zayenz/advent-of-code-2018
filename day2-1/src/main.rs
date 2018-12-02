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

type Input = Vec<String>;
type Output = i32;

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut result = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        result.push(line.trim().to_owned())
    }

    Ok(result)
}

fn solve(input: &mut Input) -> Result<Output, Error> {
    let mut twos = 0;
    let mut threes = 0;

    for word in input.iter() {
        let mut chars = HashMap::new();
        for c in word.chars() {
            *chars.entry(c as u8).or_insert(0) += 1;
        }
        let mut doubles = 0;
        let mut triples = 0;
        for value in chars.values() {
            match value {
                2 => doubles += 1,
                3 => triples += 1,
                _ => (),
            }
        }
        if doubles >= 1 {
            twos += 1;
        }
        if triples >= 1 {
            threes += 1;
        }
    }

    Ok(twos * threes)
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
