use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use regex::Regex;

enum QVariant {
    Part1,
    Part2,
}

fn parse_input(reader: BufReader<File>) -> Vec<HashMap<String, String>> {
    let mut line_iter = reader.lines().peekable();
    let mut passports: Vec<HashMap<String, String>> = vec![];

    while line_iter.peek().is_some() {
        let mut passport = HashMap::new();

        while let Some(line_opt) = line_iter.next() {
            let line = line_opt.unwrap();
            if line.len() == 0 {
                break;
            }

            let fields = line.split_whitespace();
            for field in fields {
                let tokens: Vec<&str> = field.split(":").collect();

                // Too lazy to make a struct, so we'll just work with a set
                passport.insert(tokens[0].to_string(), tokens[1].to_string());
            }
        }

        passports.push(passport);
    }

    passports
}

fn is_valid(
    passport: &HashMap<String, String>,
    variant: &QVariant,
    hcl_regex: &Regex,
    pid_regex: &Regex,
) -> bool {
    let ecl_set: HashSet<&str> = HashSet::from(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]);

    match variant {
        QVariant::Part1 => {
            passport.len() == 8 || passport.len() == 7 && !passport.contains_key("cid")
        }
        QVariant::Part2 => {
            if passport.len() < 7 || passport.len() == 7 && passport.contains_key("cid") {
                return false;
            }

            let byr = passport.get("byr").unwrap().parse::<u32>().unwrap();
            if byr < 1920 || byr > 2002 {
                return false;
            }

            let iyr = passport.get("iyr").unwrap().parse::<u32>().unwrap();
            if iyr < 2010 || iyr > 2020 {
                return false;
            }

            let eyr = passport.get("eyr").unwrap().parse::<u32>().unwrap();
            if eyr < 2020 || eyr > 2030 {
                return false;
            }

            let hgt = passport.get("hgt").unwrap();
            if !hgt.ends_with("cm") && !hgt.ends_with("in") {
                return false;
            }

            let hgt_num_max_len = hgt.len() - 2;
            let hgt_num = &hgt[0..hgt_num_max_len].parse::<u32>().unwrap();

            let hgt_result = match &hgt[hgt_num_max_len..hgt.len()] {
                "cm" => *hgt_num >= 150 && *hgt_num <= 193,
                "in" => *hgt_num >= 59 && *hgt_num <= 76,
                _ => false,
            };

            if !hgt_result {
                return false;
            }

            let hcl = passport.get("hcl").unwrap();
            if !hcl_regex.is_match(hcl) {
                return false;
            }

            let ecl = passport.get("ecl").unwrap();
            if !ecl_set.contains(ecl.as_str()) {
                return false;
            }

            let pid = passport.get("pid").unwrap();
            if !pid_regex.is_match(pid) {
                return false;
            }

            true
        }
    }
}

fn run_validation(variant: QVariant) {
    let file = File::open("inputs/day4/input.txt").unwrap();
    let reader = BufReader::new(file);

    let passports = parse_input(reader);
    let mut valid_count = 0;

    // Code runs slow if we generate the regex within the loop, so we'll
    // just pass it into the validation
    let hcl_regex: Regex = Regex::new(r"^#[a-z0-9]{6}$").unwrap();
    let pid_regex: Regex = Regex::new(r"^[0-9]{9}$").unwrap();

    for passport in &passports {
        if is_valid(passport, &variant, &hcl_regex, &pid_regex) {
            valid_count += 1;
        }
    }

    println!("Answer - {}", valid_count);
}

pub fn part1() {
    run_validation(QVariant::Part1);
}

pub fn part2() {
    run_validation(QVariant::Part2);
}
