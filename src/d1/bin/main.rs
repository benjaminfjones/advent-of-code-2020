/// --- Day 1: Report Repair ---
/// After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.
/// 
/// The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish; the locals just call them stars. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.
/// 
/// To save your vacation, you need to get all fifty stars by December 25th.
/// 
/// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
/// 
/// Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.
/// 
/// Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.
/// 
/// For example, suppose your expense report contained the following:
/// 
/// 1721
/// 979
/// 366
/// 299
/// 675
/// 1456
/// In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.
/// 
/// Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?
/// 
/// Your puzzle answer was 73371.
/// 
/// --- Part Two ---
/// The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.
/// 
/// Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.
/// 
/// In your expense report, what is the product of the three entries that sum to 2020?
/// 
/// Your puzzle answer was 127642310.

use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, BufRead};

pub fn main() {
    let mut nums = BTreeSet::new();
    let lines = read_lines("inputs/d1")
        .expect("failed to open file inputs/d1");
    for line in lines {
        if let Ok(act) = line {
            let num = act.parse::<i64>().expect("failed to parse int");
            nums.insert(num);
        }
    }
    
    if let Some((x,y)) = find_sum_pair(&nums, 2020) {
        println!("Found pair {}, {} that sum to 2020, product is: {}", x, y, x * y);
    } else {
        println!("Could not find pair summing to 2020.");
    }
    
    if let Some((x,y, z)) = find_sum_triple(&nums, 2020) {
        println!(
            "Found triple ({}, {}, {}) that sum to 2020, product is {}",
            x, y, z, x * y * z
        );
    } else {
        println!("Could not find triple summing to 2020.");
    }
}

/// find a non-descreasing pair of integers in the set that adds to `sum`.
/// 
/// Note: the integers in the returned pair may coincide
fn find_sum_pair(nums: &BTreeSet<i64>, sum: i64) -> Option<(i64, i64)> {
    for num in nums {
        let comp = sum - num;
        if nums.contains(&comp) {
            // return in non-decreasing order for consistency
            if *num <= comp {
                return Some((*num, comp));
            } else {
                return Some((comp, *num));
            }
        }
    }
    None
}

// find a non-decreasing triple that adds to `sum` by iterating once over the set of numbers.
fn find_sum_triple(nums: &BTreeSet<i64>, sum: i64) -> Option<(i64, i64, i64)> {
    for num in nums {
        let sub_sum = sum - num;
        if let Some((x,y)) = find_sum_pair(&nums, sub_sum) {
            // allocate a tmp vector to sort the ints
            let mut v = vec![*num, x, y];
            v.sort();
            return Some((v[0], v[1], v[2]));
        }
    }
    None
}

fn read_lines(path: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let in_file = File::open(path)?;
    Ok(io::BufReader::new(in_file).lines())
}

#[cfg(test)]
mod test_d1 {
    use std::collections::BTreeSet;
    use super::*;

    #[test]
    fn test_pair_2020() {
        let nums = [1721, 979, 366, 299, 675, 1456].iter().cloned().collect::<BTreeSet<i64>>();
        assert_eq!(find_sum_pair(&nums, 2020), Some((299, 1721)));
    }
    
    #[test]
    fn test_coincident_pair_2020() {
        let nums = [1010, 42].iter().cloned().collect::<BTreeSet<i64>>();
        assert_eq!(find_sum_pair(&nums, 2020), Some((1010, 1010)));
    }

    #[test]
    fn test_triple_2020() {
        let nums = [1721, 979, 366, 299, 675, 1456].iter().cloned().collect::<BTreeSet<i64>>();
        assert_eq!(find_sum_triple(&nums, 2020), Some((366, 675, 979)));
    }
}