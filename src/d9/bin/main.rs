#![feature(test)]
extern crate aoc_2020;
extern crate test;

use aoc_2020::util;
use std::collections::HashSet;

pub fn main() {
    let mut nums = Vec::new();
    for line in util::read_lines("inputs/d9").unwrap() {
        let line_str = line.unwrap();
        nums.push(line_str.trim().parse::<i64>().unwrap());
    }
    println!("num of nums {}", nums.len());

    for pos in 25..nums.len() {
        if let None = find_pair_in_slice(&nums[pos-25..pos], nums[pos]) {
            println!("first invalid number: {}", nums[pos]);
        }
    }

    let target: i64 = 57195069;
    let (start, end) = find_contig_range_prune(&nums, target).expect("can't find range");
    println!("contig range: {:?}", (start, end));
    let min = nums[start..end].into_iter().min().unwrap();
    let max = nums[start..end].into_iter().max().unwrap();
    println!("min {} max {} min+max {}", min, max, *min+*max);
}

fn find_contig_range_naive(nums: &Vec<i64>, target: i64) -> Option<(usize, usize)> {
    let n = nums.len();
    for start in 0..n-1 {
        for end in start+2..n {
            let sum: i64 = nums[start..end].iter().sum();
            if sum == target {
                return Some((start, end))
            }
        }
    }
    None
}

fn find_contig_range_prune(nums: &Vec<i64>, target: i64) -> Option<(usize, usize)> {
    let n = nums.len();
    for start in 0..n-1 {
        for end in start+2..n {
            let sum: i64 = nums[start..end].iter().sum();
            if sum == target {
                return Some((start, end))
            } else if sum > target {
                break;
            }
        }
    }
    None
}

fn find_pair_in_slice(window: &[i64], target: i64) -> Option<(i64, i64)> {
    let set: HashSet<i64> = window.into_iter().map(|x| *x).collect();
    for k in set.iter() {
        let dual = target - *k;
        if dual == *k {
            let mut set_minus_key = set.clone();
            set_minus_key.remove(k);
            if set_minus_key.contains(k) {
                return Some((*k, *k))
            }
        } else {
            if set.contains(&dual) {
                return Some((*k, dual))
            }
        }
    }
    None
}


#[cfg(test)]
mod test_d9 {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_find_contig_native(b: &mut Bencher) {
        let mut nums = Vec::new();
        for line in util::read_lines("inputs/d9").unwrap() {
            let line_str = line.unwrap();
            nums.push(line_str.trim().parse::<i64>().unwrap());
        }
        b.iter(|| find_contig_range_naive(&nums, 57195069));
    }

    #[bench]
    fn bench_find_contig_prune(b: &mut Bencher) {
        let mut nums = Vec::new();
        for line in util::read_lines("inputs/d9").unwrap() {
            let line_str = line.unwrap();
            nums.push(line_str.trim().parse::<i64>().unwrap());
        }
        b.iter(|| find_contig_range_prune(&nums, 57195069));
    }
}
