extern crate aoc_2020;

use aoc_2020::util;
use std::collections::HashSet;

pub fn main() {
    let mut max = 0;
    let mut seen_seats: HashSet<usize> = HashSet::new();
    for line in util::read_lines("inputs/d5").unwrap() {
        let line = line.unwrap();
        let idx = line.find(|c| c == 'R' || c == 'L').unwrap();
        let (row_cmd, seat_cmd) = line.split_at(idx);
        let row = bsp_find(row_cmd, 0, 127);
        let seat = bsp_find(seat_cmd, 0, 7);
        let seat_id = row * 8 + seat;
        // println!("{} -> row {} seat {} seat ID {}", line, row, seat, seat_id);
        if seat_id > max {
            max = seat_id;
        }
        seen_seats.insert(seat_id);
    }
    println!("MAX {}", max);
    for s in seen_seats.iter() {
        if !seen_seats.contains(&(s-1)) || !seen_seats.contains(&(s+1)) {
            println!("suspect seat {}", s);
        }
    }
}

fn bsp_find(command: &str, lower: usize, upper: usize) -> usize {
    let command_cs: Vec<char> = command.to_ascii_lowercase().chars().collect();
    assert!((1 << command_cs.len()) == upper - lower + 1);
    let mut lower = lower;
    let mut upper = upper;
    for cmd in command_cs.iter() {
        assert!((upper - lower  + 1) % 2 == 0);
        let delta = (upper - lower + 1) / 2 - 1;
        match cmd {
            'f' | 'l' => {
                upper = lower + delta;
            },
            'b' | 'r' => {
                lower = upper - delta;
            },
            _ => panic!("unknown command {}", cmd),
        }
    }
    if lower != upper {
        panic!("search ended with ({}, {})", lower, upper)
    }
    lower
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bsp_find() {
        assert_eq!(bsp_find("FBF", 0, 7), 2);
        assert_eq!(bsp_find("FBFBBFF", 0, 127), 44);
    }
}
