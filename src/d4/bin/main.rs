/// --- Part Two ---
/// The line is moving more quickly now, but you overhear airport security talking about how passports with invalid data are getting through. Better add some data validation, quick!
///
/// You can continue to ignore the cid field, but each other field has strict rules about what values are valid for automatic validation:
///
/// byr (Birth Year) - four digits; at least 1920 and at most 2002.
/// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
/// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
/// hgt (Height) - a number followed by either cm or in:
/// If cm, the number must be at least 150 and at most 193.
/// If in, the number must be at least 59 and at most 76.
/// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
/// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
/// pid (Passport ID) - a nine-digit number, including leading zeroes.
/// cid (Country ID) - ignored, missing or not.
/// Your job is to count the passports where all required fields are both present and valid according to the above rules. Here are some example values:
///
/// byr valid:   2002
/// byr invalid: 2003
///
/// hgt valid:   60in
/// hgt valid:   190cm
/// hgt invalid: 190in
/// hgt invalid: 190
///
/// hcl valid:   #123abc
/// hcl invalid: #123abz
/// hcl invalid: 123abc
///
/// ecl valid:   brn
/// ecl invalid: wat
///
/// pid valid:   000000001
/// pid invalid: 0123456789
/// Here are some invalid passports:
///
/// eyr:1972 cid:100
/// hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
///
/// iyr:2019
/// hcl:#602927 eyr:1967 hgt:170cm
/// ecl:grn pid:012533040 byr:1946
///
/// hcl:dab227 iyr:2012
/// ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
///
/// hgt:59cm ecl:zzz
/// eyr:2038 hcl:74454a iyr:2023
/// pid:3556412378 byr:2007
/// Here are some valid passports:
///
/// pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
/// hcl:#623a2f
///
/// eyr:2029 ecl:blu cid:129 byr:1989
/// iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
///
/// hcl:#888785
/// hgt:164cm byr:2001 iyr:2015 cid:88
/// pid:545766238 ecl:hzl
/// eyr:2022
///
/// iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
/// Count the number of valid passports - those that have all required fields and valid values.
/// Continue to treat cid as optional. In your batch file, how many passports are valid?

extern crate aoc_2020;

use std::collections::HashMap;
use aoc_2020::util::read_to_string;

const REQ_FIELDS: [&'static str; 7] = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
];

const EYE_COLORS: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

#[derive(Debug)]
struct Passport<'a>(HashMap<&'a str, &'a str>);

impl<'a> Passport<'a> {
    fn new(fields: HashMap<&'a str, &'a str>) -> Self {
        Passport(fields)
    }

    /// Return true iff. all required fields are present:
    ///
    /// byr (Birth Year) - four digits; at least 1920 and at most 2002.
    /// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    /// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    /// hgt (Height) - a number followed by either cm or in:
    /// If cm, the number must be at least 150 and at most 193.
    /// If in, the number must be at least 59 and at most 76.
    /// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    /// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    /// pid (Passport ID) - a nine-digit number, including leading zeroes.
    /// cid (Country ID) - ignored, missing or not.
    fn validate(&self) -> bool {
        if !REQ_FIELDS.iter().all(|req| self.0.contains_key(req)) {
            return false;
        }
        for (key, value) in self.0.iter() {
            let valid_value = match *key {
                "byr" => value.parse::<usize>()
                    .and_then(|v| Ok(v >= 1920 && v <= 2002)).unwrap_or(false),
                "iyr" => value.parse::<usize>()
                    .and_then(|v| Ok(v >= 2010 && v <= 2020)).unwrap_or(false),
                "eyr" => value.parse::<usize>()
                    .and_then(|v| Ok(v >= 2020 && v <= 2030)).unwrap_or(false),
                "hgt" => {
                    // ad-hoc parse height
                    let unit_pos = value.find(char::is_alphabetic);
                    if unit_pos.is_none() { return false; }
                    let unit_pos = unit_pos.unwrap();
                    let (num_str, unit_str) = value.split_at(unit_pos);
                    let num = num_str.parse::<usize>();
                    if num.is_err() { return false; }
                    let num = num.unwrap();
                    match unit_str {
                        "cm" => num >= 150 && num <= 193,
                        "in" => num >= 59 && num <= 76,
                        _ => false,
                    }
                },
                "hcl" => {
                    let chars: Vec<char> = value.chars().collect();
                    chars.len() == 7 && chars[0] == '#' && chars[1..].iter().all(char::is_ascii_hexdigit)
                },
                "ecl" => {
                    EYE_COLORS.iter().any(|color| color == value)
                },
                "pid" => {
                    let chars: Vec<char> = value.chars().collect();
                    chars.len() == 9 && chars.iter().all(|c| c.is_digit(10))
                }
                _ => true,  // if field is unknown, we ignore it
            };
            if !valid_value {
                return false;
            }
        }
        true
    }
}

pub fn main() {
    let input = read_to_string("inputs/d4").expect("failed to read inputs/d4");
    let parse_result = parse_passports(&input).expect("failed to parse input");
    let mut count = 0;
    for passwd in parse_result.iter() {
        if passwd.validate() {
            count += 1;
        } else {
            println!("Invalid passport: {:?}", passwd)
        }
    }
    println!("Number of valid passports: {}", count);
}

fn parse_passports(input: &str) -> Result<Vec<Passport>, &'static str> {
    let mut result = Vec::new();
    let mut map: HashMap<&str, &str> = HashMap::new();
    let input = input.trim();
    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() == 0 {
            // blank line encountered
            let tmp_map = map.clone();
            map.clear();
            result.push(Passport::new(tmp_map));
            continue;
        }

        // parse the tokens and insert into `map`
        for tok in tokens.iter() {
            let (key, val) = parse_token(tok)?;
            if let Some(_) = map.insert(key, val)  {
                return Err("duplicate key encounterd");
            }
        }
    }
    // push last entry before EOF
    if map.len() != 0 {
        result.push(Passport::new(map));
    }
    Ok(result)
}

fn parse_token(tok: &str) -> Result<(&str, &str), &'static str> {
    let splits: Vec<&str> = tok.split(':').collect();
    if splits.len() == 2 {
        Ok((splits[0], splits[1]))
    } else {
        Err("error parsing token")
    }
}

#[cfg(test)]
mod test_d4 {
    use crate::parse_passports;

    const TEST_INPUT: &'static str =
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
         byr:1937 iyr:2017 cid:147 hgt:183cm

         iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
         hcl:#cfa07d byr:1929

         hcl:#ae17e1 iyr:2013
         eyr:2024
         ecl:brn pid:760753108 byr:1931
         hgt:179cm

         hcl:#cfa07d eyr:2025 pid:166559648
         iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test_parse() {
        let p = parse_passports(TEST_INPUT);
        assert!(p.is_ok());
        let p = p.unwrap();
        assert!(p[0].0.contains_key("ecl"));
        assert!(p[0].validate());
        assert!(!p[1].validate());
        assert!(p[2].validate());
        assert!(!p[3].validate());
    }
}
