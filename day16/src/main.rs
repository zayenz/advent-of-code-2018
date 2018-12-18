#![allow(
    dead_code,
    unused_imports,
    clippy::needless_range_loop,
    clippy::ptr_arg,
    clippy::char_lit_as_u8,
    clippy::useless_format
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

use enum_map::*;
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

use aoc2018::dense_grid::Grid;
use aoc2018::dense_grid::*;
use aoc2018::input::*;
use aoc2018::matrix::*;
use aoc2018::position::*;
use std::collections::BTreeSet;
use std::fmt::Display;
use std::fmt::Formatter;

type Input = (Vec<TestCase>, Vec<Code>);
type Output = String;

type Word = i64;

type Registers = [Word; 4];
type Code = (Word, Word, Word, Word);
type TestCase = (Registers, Code, Registers);
type Assembly = (Instruction, Word, Word, Word);

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum ArgumentMode {
    Immediate,
    Register,
}

use crate::ArgumentMode::*;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Instruction {
    Add(ArgumentMode),
    Mul(ArgumentMode),
    BitAnd(ArgumentMode),
    BitOr(ArgumentMode),
    Set(ArgumentMode),
    GreaterThan(ArgumentMode, ArgumentMode),
    Equal(ArgumentMode, ArgumentMode),
}

use crate::Instruction::*;

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let representation = match *self {
            Add(Immediate) => "addi",
            Add(Register) => "addr",
            Mul(Immediate) => "muli",
            Mul(Register) => "mulr",
            BitAnd(Immediate) => "bani",
            BitAnd(Register) => "banr",
            BitOr(Immediate) => "bori",
            BitOr(Register) => "borr",
            Set(Immediate) => "seti",
            Set(Register) => "setr",
            GreaterThan(Immediate, Immediate) => "gtii",
            GreaterThan(Register, Immediate) => "gtri",
            GreaterThan(Immediate, Register) => "gtir",
            GreaterThan(Register, Register) => "gtrr",
            Equal(Immediate, Immediate) => "eqii",
            Equal(Register, Immediate) => "eqri",
            Equal(Immediate, Register) => "eqir",
            Equal(Register, Register) => "eqrr",
        };
        write!(f, "{}", representation)
    }
}

fn as_code(words: &[Word]) -> Result<Code, Error> {
    if words.len() == 4 {
        Ok((words[0], words[1], words[2], words[3]))
    } else {
        bail!(format!(
            "Wrong number of arguments, expected 4, got {:?}",
            words
        ))
    }
}

fn as_register(words: &[Word]) -> Result<Registers, Error> {
    if words.len() == 4 {
        Ok([words[0], words[1], words[2], words[3]])
    } else {
        bail!(format!(
            "Wrong number of arguments, expected 4, got {:?}",
            words
        ))
    }
}

fn read_input() -> Result<Input, Error> {
    let stdin = io::stdin();
    let lines = stdin
        .lock()
        .lines()
        .map(|s| s.unwrap().trim().to_owned())
        .filter(|s| !s.is_empty())
        .collect_vec();
    let mut tests = Vec::new();
    let mut program = Vec::new();
    let mut it = lines.iter();
    while let Some(line) = it.next() {
        if line.starts_with("Before") {
            let before = get_numbers::<Word>(line)?;
            let code = get_numbers::<Word>(it.next().unwrap())?;
            let after = get_numbers::<Word>(it.next().unwrap())?;
            tests.push((as_register(&before)?, as_code(&code)?, as_register(&after)?))
        } else {
            let code = get_numbers::<Word>(line)?;
            program.push(as_code(&code)?)
        }
    }

    Ok((tests, program))
}

fn get(mode: ArgumentMode, arg: Word, regs: &Registers) -> Result<Word, Error> {
    match mode {
        ArgumentMode::Immediate => Ok(arg),
        ArgumentMode::Register => {
            if 0 <= arg && arg < 4 {
                Ok(regs[arg as usize])
            } else {
                bail!(format!("Argument {} is not a valid register index", arg))
            }
        }
    }
}

fn execute(asm: Assembly, regs: Registers) -> Result<Registers, Error> {
    let mut result = regs;
    let (inst, arg1, arg2, res) = asm;
    let res = res as usize;
    match inst {
        Instruction::Add(mode) => {
            result[res] = get(Register, arg1, &regs)? + get(mode, arg2, &regs)?
        }
        Instruction::Mul(mode) => {
            result[res] = get(Register, arg1, &regs)? * get(mode, arg2, &regs)?
        }
        Instruction::BitAnd(mode) => {
            result[res] = get(Register, arg1, &regs)? & get(mode, arg2, &regs)?
        }
        Instruction::BitOr(mode) => {
            result[res] = get(Register, arg1, &regs)? | get(mode, arg2, &regs)?
        }
        Instruction::Set(mode) => result[res] = get(mode, arg1, &regs)?,
        Instruction::GreaterThan(mode1, mode2) => {
            result[res] = if get(mode1, arg1, &regs)? > get(mode2, arg2, &regs)? {
                1
            } else {
                0
            }
        }
        Instruction::Equal(mode1, mode2) => {
            result[res] = if get(mode1, arg1, &regs)? == get(mode2, arg2, &regs)? {
                1
            } else {
                0
            }
        }
    }

    Ok(result)
}

static INSTS: [Instruction; 16] = [
    Add(Immediate),
    Add(Register),
    Mul(Immediate),
    Mul(Register),
    BitAnd(Immediate),
    BitAnd(Register),
    BitOr(Immediate),
    BitOr(Register),
    Set(Immediate),
    Set(Register),
    GreaterThan(Register, Immediate),
    GreaterThan(Immediate, Register),
    GreaterThan(Register, Register),
    Equal(Register, Immediate),
    Equal(Immediate, Register),
    Equal(Register, Register),
];

fn solve1(input: &mut Input) -> Result<Output, Error> {
    let (tests, _) = input;

    let mut matches3 = 0;
    for (input, (_code, arg1, arg2, res), output) in tests.iter().cloned() {
        //        println!("Checking if ({}, {}, {}, {}) matches {:?} to {:?}",
        //                 code, arg1, arg2, res, input, output
        //        );
        let mut matches = 0;
        for &inst in INSTS.iter() {
            //            print!("Checking instruction {} ", inst);
            let asm = (inst, arg1, arg2, res);
            if let Ok(actual) = execute(asm, input) {
                if actual == output {
                    //                    print!("...matches!");
                    matches += 1;
                }
            }
            //            println!();
        }
        if matches >= 3 {
            matches3 += 1;
        }
    }

    Ok(format!("{}", matches3))
}

fn is_valid(input: Registers, output: Registers, asm: Assembly) -> bool {
    if let Ok(actual) = execute(asm, input) {
        if actual == output {
            return true;
        }
    };
    false
}

fn solve2(input: &mut Input) -> Result<Output, Error> {
    let (tests, program) = input;

    // Set up potential mappings between code-points and assembly instructions
    //
    let mut code2asm = Vec::new();
    for _op in 0..16 {
        code2asm.push(INSTS.iter().cloned().collect::<HashSet<_>>());
    }
    let mut asm2code = HashMap::new();
    for inst in INSTS.iter() {
        asm2code.insert(inst, (0..16).collect::<HashSet<Word>>());
    }

    // Use the test programs to remove potential mappings
    //
    for (input, (code, arg1, arg2, res), output) in tests.iter().cloned() {
        for &inst in INSTS.iter() {
            let asm: Assembly = (inst, arg1, arg2, res);
            if !is_valid(input, output, asm) {
                code2asm[code as usize].remove(&inst);
                asm2code.get_mut(&inst).unwrap().remove(&code);
            }
        }
    }

    // Propagate assignments
    //  * only one potential assembly instruction for a code point
    //  * only one potential code point for an assembly instruction
    //
    // From testing this is enough to solve the current problem.
    //
    let mut handled_code_value = HashSet::new();
    let mut handled_asm_value = HashSet::new();
    loop {
        let mut propagation = false;
        for op in 0..16 {
            if code2asm[op].len() == 1 && !handled_code_value.contains(&op) {
                propagation = true;
                handled_code_value.insert(op);
                let inst = *code2asm[op].iter().next().unwrap();
                for other_op in 0..15 {
                    if other_op != op {
                        code2asm[other_op].remove(&inst);
                        asm2code.get_mut(&inst).unwrap().remove(&(other_op as Word));
                    }
                }
            }
        }

        for inst in INSTS.iter() {
            if asm2code[inst].len() == 1 && !handled_asm_value.contains(inst) {
                propagation = true;
                handled_asm_value.insert(*inst);
                let op = *asm2code[inst].iter().next().unwrap();
                for other_inst in INSTS.iter() {
                    if other_inst != inst {
                        code2asm[op as usize].remove(other_inst);
                        asm2code.get_mut(other_inst).unwrap().remove(&op);
                    }
                }
            }
        }

        if !propagation {
            break;
        }
    }

    // Use the resulting mapping to execute the program
    let program = program
        .iter()
        .cloned()
        .map(|(code, arg1, arg2, res)| {
            (
                *code2asm[code as usize].iter().next().unwrap(),
                arg1,
                arg2,
                res,
            )
        })
        .collect_vec();

    let mut regs: Registers = [0; 4];

    for asm in program.iter() {
        regs = execute(*asm, regs)?;
    }

    Ok(format!("{}", regs[0]))
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

#[cfg(test)]
mod test {
    use super::*;
}
