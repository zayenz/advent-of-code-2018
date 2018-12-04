#![allow(dead_code, unused_imports, clippy::needless_range_loop)]

use failure::bail;
use failure::err_msg;
use failure::Error;
use rayon::prelude::*;
use strum_macros::EnumString;
use tap::{TapOps, TapOptionOps, TapResultOps};

use hashbrown::{HashMap, HashSet};
use std::char;
use std::cmp::{max, min};
use std::fmt;
use std::io::BufRead;
use std::ops::*;
use std::str;
use std::str::FromStr;
use std::{io, process};

use aoc2018::input::*;
use aoc2018::matrix::*;
use std::collections::BTreeSet;

type Input = Vec<Duty>;
type Output = usize;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Default, Hash, Clone)]
struct Duty {
    id: usize,
    sleep_start: usize,
    sleep_end: usize,
}

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?.trim().to_owned();
        lines.push(line);
    }
    lines.sort();

    let mut result = Vec::new();
    let mut duty = Duty::default();
    for line in lines {
        let nums = get_numbers::<usize>(&line.replace('-', " "))?;
        if nums.len() == 6 {
            duty.id = nums[5];
        } else if line.ends_with("asleep") {
            duty.sleep_start = nums[4];
        } else if line.ends_with("up") {
            duty.sleep_end = nums[4];
            result.push(duty.clone());
        } else {
            bail!(format!("Could not parse line: \"{}\"", line));
        }
    }

    Ok(result)
}

fn solve(input: &mut Input) -> Result<Output, Error> {
    let guards = input.iter().map(|d| d.id).collect::<BTreeSet<_>>();
    let guards = guards.into_iter().collect::<Vec<_>>();

    let mut schedule = HashMap::new();
    let mut total_sleeps = HashMap::new();
    for guard in guards.iter() {
        schedule.insert(*guard, [0; 60]);
        total_sleeps.insert(*guard, 0);
    }

    for duty in input.iter() {
        let minutes = schedule.get_mut(&duty.id).unwrap();
        for minute in duty.sleep_start..duty.sleep_end {
            minutes[minute] += 1;
        }
        let time_asleep = duty.sleep_end - duty.sleep_start;
        *total_sleeps.get_mut(&duty.id).unwrap() += time_asleep;
    }

    let (guard, minute, _total_sleep) = schedule
        .iter()
        .map(|(&guard, minutes)| {
            let (minute, total_sleep) = minutes
                .iter()
                .enumerate()
                .max_by_key(|&(_minute, &total_sleep)| total_sleep)
                .unwrap();
            (guard, minute, total_sleep)
        })
        .max_by_key(|&(_guard, _max_minute, &total_sleep)| total_sleep)
        .unwrap();

    Ok(guard * minute)
}

fn run() -> Result<(), Error> {
    let mut input = read_input()?;
    let output = solve(&mut input)?;

    println!("{}", output);
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
