/// --- Day 2: Password Philosophy ---
/// Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.
///
/// The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.
///
/// Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.
///
/// To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.
///
/// For example, suppose you have the following list:
///
/// 1-3 a: abcde
/// 1-3 b: cdefg
/// 2-9 c: ccccccccc
/// Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.
///
/// In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.
///
/// How many passwords are valid according to their policies?
///
/// --- Part Two ---
/// While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.
///
/// The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy actually works a little differently.
///
/// Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.
///
/// Given the same example list from above:
///
/// 1-3 a: abcde is valid: position 1 contains a and position 3 does not.
/// 1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
/// 2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
/// How many passwords are valid according to the new interpretation of the policies?
extern crate aoc_2020;

use std::str::FromStr;
use aoc_2020::util;

pub struct OldPolicy {
    letter: char,
    min: usize,
    max: usize,
}

impl OldPolicy {
    pub fn new(letter: char, min: usize, max: usize) -> Self {
        if min > max {
            panic!("min > max for policy");
        }
        OldPolicy {
            letter: letter,
            min: min,
            max: max,
        }
    }

    /// Return true iff. given password satisfies the policy
    pub fn check(&self, password: &str) -> bool {
        let count = password.chars().filter(|c| *c == self.letter).count();
        self.min <= count && count <= self.max
    }
}

impl FromStr for OldPolicy {
    // TODO: use a more expressive error type
    type Err = ();

    /// Parse patterns of the form: `N-M L` where N, M are non-negative
    /// integers, and L is a character, into a `Policy`.
    ///
    /// Parsing is insensitive to whitespace before N and after L.
    fn from_str(policy_str: &str) -> Result<Self, Self::Err> {
        let (num1, num2, letter) = parse_prefix(policy_str)?;
        Ok(OldPolicy::new(letter, num1, num2))
    }
}

pub struct NewPolicy {
    letter: char,
    pos1: usize,
    pos2: usize,
}

impl NewPolicy {
    pub fn new(letter: char, pos1: usize, pos2: usize) -> Self {
        NewPolicy {
            letter: letter,
            pos1: pos1,
            pos2: pos2,
        }
    }

    /// Return true iff. given password satisfies the *new* policy
    pub fn check(&self, password: &str) -> bool {
        let cs: Vec<char> = password.chars().collect();
        (cs[self.pos1-1] == self.letter) ^ (cs[self.pos2-1] == self.letter)
    }
}

impl FromStr for NewPolicy {
    // TODO: use a more expressive error type
    type Err = ();

    /// Parse patterns of the form: `N-M L` where N, M are non-negative
    /// integers, and L is a character, into a `Policy`.
    ///
    /// Parsing is insensitive to whitespace before N and after L.
    fn from_str(policy_str: &str) -> Result<Self, Self::Err> {
        let (num1, num2, letter) = parse_prefix(policy_str)?;
        Ok(NewPolicy::new(letter, num1, num2))
    }
}

/// Parse prefixes of the form `N-M L` where N, M are non-negative integers
/// and L is a character.
fn parse_prefix(pref: &str) -> Result<(usize, usize, char), ()> {
    let splits: Vec<&str> = pref.trim().split(' ').collect();
    if splits.len() != 2 {
        return Err(());
    }
    let min_max_split: Vec<&str> = splits[0].split('-').collect();
    if min_max_split.len() != 2 {
        return Err(());
    }
    let num1 = min_max_split[0].parse::<usize>().map_err(|_e| ())?;
    let num2 = min_max_split[1].parse::<usize>().map_err(|_e| ())?;

    let letter = splits[1].parse::<char>().map_err(|_e| ())?;
    Ok((num1, num2, letter))
}

/// Parse lines of the form `1-3 a: password` into a (new or old) policy
/// (before the :) and / a word (after the :).
fn parse_line<F, T>(line: &str, prefix_parser: F) -> Result<(T, String), ()>
    where F: Fn(&str) -> Result<T, ()> {
    let splits: Vec<&str> = line.trim().split(':').collect();
    if splits.len() != 2 {
        return Err(());
    }
    let policy = prefix_parser(splits[0])?;
    let word = splits[1].trim().to_string();
    if word.is_empty() {
        return Err(());
    }
    Ok((policy, word))
}

fn main() {
    let lines = util::read_lines("inputs/d2").unwrap();

    // count passwords that satisfy the *old* policy scheme
    let mut old_count = 0;
    let mut new_count = 0;
    for line in lines {
        let ln = line.unwrap();
        // panic if lines fails to parse!
        let (policy, word) = parse_line(&ln, OldPolicy::from_str).unwrap();
        if policy.check(&word) {
            // println!("line {} -- password {} passes old check!", ln, word);
            old_count += 1;
        }

        // check the *new* policy
        let (policy, word) = parse_line(&ln, NewPolicy::from_str).unwrap();
        if policy.check(&word) {
            // println!("line {} -- password {} passes new check!", ln, word);
            new_count += 1;
        }
    }
    println!("*** OLD count = {}", old_count);
    println!("*** NEW count = {}", new_count);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_policy() {
        let policy1 = OldPolicy::new('a', 1, 3);
        assert!(policy1.check("abc"));
        assert!(policy1.check("abarac"));
        assert!(!policy1.check("abaracadabra"));
        assert!(!policy1.check("foo quux"));
    }

    /// 1-3 a: abcde
    /// 1-3 b: cdefg
    /// 2-9 c: ccccccccc
    #[test]
    fn test_given_cases() {
        let policy1 = OldPolicy::new('a', 1, 3);
        assert!(policy1.check("abcde"));
        let policy2 = OldPolicy::new('b', 1, 3);
        assert!(!policy2.check("cdefg"));
        let policy3 = OldPolicy::new('c', 2, 9);
        assert!(policy3.check("ccccccccc"));
    }

    #[test]
    fn test_parse_policy() {
        let policy_res = OldPolicy::from_str("1-3 a");
        assert!(policy_res.is_ok());
        let policy = policy_res.unwrap();
        assert!(policy.check("abc"));
        assert!(policy.check("aaa"));
        assert!(!policy.check("bbb"));
    }

    #[test]
    fn test_parse_ignores_ws() {
        assert!(OldPolicy::from_str("  1-2 a ").is_ok());
        assert!(OldPolicy::from_str("1 - 2 a").is_err());
        assert!(OldPolicy::from_str("1-2  a").is_err());
    }

    #[test]
    fn test_parse_line() {
        assert!(parse_line("1-3 a: foobar", OldPolicy::from_str).is_ok());
        assert!(parse_line("1- a: foobar", OldPolicy::from_str).is_err());
        assert!(parse_line("1-3 a: ", OldPolicy::from_str).is_err());
        let (policy, word) = parse_line("1-3 a: foobar", OldPolicy::from_str).unwrap();
        assert_eq!(word, "foobar".to_string());
        assert!(policy.check(&word));

        // test parsing new policies
        assert!(parse_line("1-3 a: foobar", NewPolicy::from_str).is_ok());
    }
}
