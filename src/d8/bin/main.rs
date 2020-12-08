/// --- Day 8: Handheld Halting ---
/// Your flight to the major airline hub reaches cruising altitude without incident. While you
/// consider checking the in-flight menu for one of those drinks that come with a little umbrella,
/// you are interrupted by the kid sitting next to you.
///
/// Their handheld game console won't turn on! They ask if you can take a look.
///
/// You narrow the problem down to a strange infinite loop in the boot code (your puzzle input) of
/// the device. You should be able to fix it, but first you need to be able to run the code in
/// isolation.
///
/// The boot code is represented as a text file with one instruction per line of text. Each
/// instruction consists of an operation (acc, jmp, or nop) and an argument (a signed number like
/// +4 or -20).
///
/// acc increases or decreases a single global value called the accumulator by the value given in
/// the argument. For example, acc +7 would increase the accumulator by 7. The accumulator starts
/// at 0. After an acc instruction, the instruction immediately below it is executed next. jmp
/// jumps to a new instruction relative to itself. The next instruction to execute is found using
/// the argument as an offset from the jmp instruction; for example, jmp +2 would skip the next
/// instruction, jmp +1 would continue to the instruction immediately below it, and jmp -20 would
/// cause the instruction 20 lines above to be executed next. nop stands for No OPeration - it does
/// nothing. The instruction immediately below it is executed next. For example, consider the
/// following program:
///
/// nop +0
/// acc +1
/// jmp +4
/// acc +3
/// jmp -3
/// acc -99
/// acc +1
/// jmp -4
/// acc +6
///
/// These instructions are visited in this order:
///
/// nop +0  | 1
/// acc +1  | 2, 8(!)
/// jmp +4  | 3
/// acc +3  | 6
/// jmp -3  | 7
/// acc -99 |
/// acc +1  | 4
/// jmp -4  | 5
/// acc +6  |
///
/// First, the nop +0 does nothing. Then, the accumulator is increased from 0 to 1 (acc +1) and jmp
/// +4 sets the next instruction to the other acc +1 near the bottom. After it increases the
/// accumulator from 1 to 2, jmp -4 executes, setting the next instruction to the only acc +3. It
/// sets the accumulator to 5, and jmp -3 causes the program to continue back at the first acc +1.
///
/// This is an infinite loop: with this sequence of jumps, the program will run forever. The moment
/// the program tries to run any instruction a second time, you know it will never terminate.
///
/// Immediately before the program would run an instruction a second time, the value in the
/// accumulator is 5.
///
/// Run your copy of the boot code. Immediately before any instruction is executed a second time,
/// what value is in the accumulator?
///
/// --- Part Two ---
///
/// After some careful analysis, you believe that exactly one instruction is corrupted.
///
/// Somewhere in the program, either a jmp is supposed to be a nop, or a nop is supposed to be a
/// jmp. (No acc instructions were harmed in the corruption of this boot code.)
///
/// The program is supposed to terminate by attempting to execute an instruction immediately after
/// the last instruction in the file. By changing exactly one jmp or nop, you can repair the boot
/// code and make it terminate correctly.
///
/// For example, consider the same program from above:
///
/// nop +0
/// acc +1
/// jmp +4
/// acc +3
/// jmp -3
/// acc -99
/// acc +1
/// jmp -4
/// acc +6
///
/// If you change the first instruction from nop +0 to jmp +0, it would create a single-instruction
/// infinite loop, never leaving that instruction. If you change almost any of the jmp
/// instructions, the program will still eventually find another jmp instruction and loop forever.
///
/// However, if you change the second-to-last instruction (from jmp -4 to nop -4), the program
/// terminates! The instructions are visited in this order:
///
/// nop +0  | 1
/// acc +1  | 2
/// jmp +4  | 3
/// acc +3  |
/// jmp -3  |
/// acc -99 |
/// acc +1  | 4
/// nop -4  | 5
/// acc +6  | 6
///
/// After the last instruction (acc +6), the program terminates by attempting to run the
/// instruction below the last instruction in the file. With this change, after the program
/// terminates, the accumulator contains the value 8 (acc +1, acc +1, acc +6).
///
/// Fix the program so that it terminates normally by changing exactly one jmp (to nop) or nop (to
/// jmp). What is the value of the accumulator after the program terminates?
extern crate aoc_2020;

use aoc_2020::util;
use std::collections::HashSet;

pub fn main() {
    let input = util::read_to_string("inputs/d8").unwrap();
    let prog = Program::from_str(&input).unwrap();
    println!("program exec: {:?}", prog.exec(0));

    for pos in 0..prog.0.len() {
        let mut new_prog = prog.clone();
        let res = mutate_and_exec(&mut new_prog, pos);
        match res {
            Ok(acc) => println!("*** mutation at pos {} succeeded, acc = {}", pos, acc),
            Err(e) => println!("fail: pos {}, res = {:?}", pos, e),
        }
    }
}

/// Tries to mutate a Nop -> Jmp or a Jmp -> Nop at given instruction pos and exec. If the
/// mutation executes successfully, the final accumulator is returned.
fn mutate_and_exec(prog: &mut Program, pos: usize) -> Result<i32, ExecError> {
    match prog.0[pos] {
        ISA::Nop(x) => {
            prog.0[pos] = ISA::Jmp(x);
            prog.exec(0)
        },
        ISA::Acc(_) => { prog.exec(0) },
        ISA::Jmp(x) => {
            prog.0[pos] = ISA::Nop(x);
            prog.exec(0)
        },
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ISA {
    Nop(i32),  // integer is ignored
    Acc(i32),
    Jmp(i32),
}

#[derive(Clone, Debug)]
pub struct Program(Vec<ISA>);

impl Program {
    pub fn new(ins: Vec<ISA>) -> Self { Program(ins) }

    /// Parse a program
    pub fn from_str(input: &str) -> Result<Self, String> {
        let mut res = Vec::new();
        for line in input.lines() {
            let splits: Vec<&str> = line.trim().split(' ').collect();
            if splits.len() != 2 {
                return Err(format!("did not find two columns: {}", line))
            }
            let lower_ins = splits[0].to_ascii_lowercase();
            let param = splits[1].parse::<i32>().map_err(|_e| format!("couldn't parse param: {}", splits[1]))?;
            let ins = match &lower_ins[..] {
                "nop" => Ok(ISA::Nop(param)),
                "acc" => Ok(ISA::Acc(param)),
                "jmp" => Ok(ISA::Jmp(param)),
                _ => Err(format!("unknown instruction: {}", lower_ins)),
            }?;
            res.push(ins);
        }
        Ok(Program(res))
    }

    pub fn exec(&self, init_state: i32) -> Result<i32, ExecError> {
        let mut trace: HashSet<(i32, ISA)> = HashSet::new();
        let mut acc: i32 = init_state;  // execution state
        let mut pp: i32 = 0;  // program pointer
        let prog_len = self.0.len() as i32;
        loop {
            let ins = self.0[pp as usize];
            if trace.contains(&(pp, ins)) {
                return Err(ExecError::InfiniteLoop(acc));
            }
            trace.insert((pp, ins));
            match &ins {
                ISA::Nop(_) => {
                    pp += 1;
                }
                ISA::Acc(x) => {
                    acc += x;
                    pp += 1;
                }
                ISA::Jmp(x) => {
                    pp = pp + x;
                }
            };
            if pp < 0 || pp > prog_len {
                return Err(ExecError::JumpOutOfBounds(pp));
            } else if pp == prog_len {
                return Ok(acc)
            }
        }
    }
}

#[derive(Debug)]
pub enum ExecError {
    JumpOutOfBounds(i32),
    InfiniteLoop(i32),
}

#[cfg(test)]
mod test_d8 {
    use super::*;

    const EXAMPLE: &'static str =
        "nop +0
         acc +1
         jmp +4
         acc +3
         jmp -3
         acc -99
         acc +1
         jmp -4
         acc +6";

    #[test]
    fn test_parse_program() {
        let prog = Program::from_str(EXAMPLE);
        assert!(prog.is_ok());
        assert_eq!(prog.unwrap().0.len(), 9);
    }

    #[test]
    fn test_parse_input() {
        let input = util::read_to_string("inputs/d8").unwrap();
        let prog = Program::from_str(&input);
        assert!(prog.is_ok());
    }
}
