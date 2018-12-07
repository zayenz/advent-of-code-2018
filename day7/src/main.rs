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
use rayon::prelude::*;
use stats::Frequencies;
use structopt::StructOpt;
use strum_macros::EnumString;
use tap::{TapOps, TapOptionOps, TapResultOps};

use aoc2018::input::*;
use aoc2018::matrix::*;
use std::collections::BTreeSet;

type Input = Vec<(char, char)>;
type Output = String;

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let mut result = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        let line = line.trim();
        if !line.is_empty() {
            result.push((line.chars().nth(5).unwrap(), line.chars().nth(36).unwrap()));
        }
    }

    Ok(result)
}

fn solve1(input: &mut Input) -> Result<Output, Error> {
    let tasks: HashSet<char> = input.iter().flat_map(|&(a, b)| vec![a, b]).collect();
    let mut done: HashSet<char> = HashSet::new();
    let mut prereqs = HashMap::new();
    for ch in tasks.iter() {
        prereqs.insert(ch, HashSet::new());
    }
    let mut todo = tasks.clone();
    for &(req, task) in input.iter() {
        prereqs.get_mut(&task).unwrap().insert(req);
    }

    let mut result: Vec<char> = Vec::new();
    while !todo.is_empty() {
        let ready = todo
            .iter()
            .cloned()
            .filter(|c| prereqs[c].is_empty())
            .collect::<Vec<_>>()
            .tap(|v| v.sort());
        let next = *ready.first().ok_or_else(|| err_msg("Ready is emtpy"))?;
        result.push(next);
        for prereq in prereqs.values_mut() {
            prereq.remove(&next);
        }
        todo.remove(&next);
        done.insert(next);
    }

    Ok(result.iter().collect())
}

fn solve2(input: &mut Input, workers: usize, base_time: u8) -> Result<Output, Error> {
    let tasks: HashSet<char> = input.iter().flat_map(|&(a, b)| vec![a, b]).collect();
    let mut done: HashSet<char> = HashSet::new();
    let mut prereqs = HashMap::new();
    for ch in tasks.iter() {
        prereqs.insert(ch, HashSet::new());
    }
    let mut todo = tasks.clone();
    for &(req, task) in input.iter() {
        prereqs.get_mut(&task).unwrap().insert(req);
    }

    let mut result: Vec<char> = Vec::new();
    let mut in_progress: HashMap<char, u8> = HashMap::new();
    let mut time = 0;
    while result.len() < tasks.len() {
        for (_task, remaining) in in_progress.iter_mut() {
            *remaining -= 1;
        }
        let completed = in_progress
            .iter()
            .filter(|&(_task, &remaining)| remaining == 0)
            .map(|(&task, _remaining)| task)
            .collect::<Vec<_>>()
            .tap(|v| v.sort());
        for task in completed.iter() {
            in_progress.remove(task);
            result.push(*task);
            done.insert(*task);
            for prereq in prereqs.values_mut() {
                prereq.remove(task);
            }
        }

        if in_progress.len() < workers {
            let ready = todo
                .iter()
                .cloned()
                .filter(|c| prereqs[c].is_empty())
                .collect::<Vec<_>>()
                .tap(|v| v.sort());
            for task in ready {
                let work_time = base_time + (task as u8 - 'A' as u8) + 1;
                in_progress.insert(task, work_time);
                todo.remove(&task);
                if in_progress.len() == workers {
                    break;
                }
            }
        }
        time += 1;
    }

    Ok(format!("{}", time - 1))
}

#[derive(StructOpt, Debug)]
#[structopt(name = "day6")]
struct Opt {
    /// Part to solve, either 1 or 2
    #[structopt(short = "-p", long = "--part", default_value = "1")]
    part: u8,
    /// Number of workers
    #[structopt(short = "-w", long = "--workers", default_value = "5")]
    workers: usize,
    /// Base time
    #[structopt(short = "-b", long = "--base", default_value = "60")]
    base_time: u8,
}

fn run() -> Result<(), Error> {
    let mut input = read_input()?;

    let options: Opt = Opt::from_args();

    let output = if options.part == 1 {
        solve1(&mut input)?
    } else {
        solve2(&mut input, options.workers, options.base_time)?
    };

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
