use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct PasswordAttempt {
    password: String,
    key_character: char,
    policy_range: (usize, usize),
}

fn map_password_attempt(input: &str) -> PasswordAttempt {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    let split: Vec<&str> = tokens[0].split("-").collect();

    let policy_range: (usize, usize) = (
        split[0].parse::<usize>().unwrap(),
        split[1].parse::<usize>().unwrap(),
    );

    let key_character = tokens[1].chars().nth(0).unwrap();
    let password = tokens[2].to_owned();

    PasswordAttempt {
        policy_range,
        key_character,
        password,
    }
}

fn is_attempt_valid_p1(p_attempt: &PasswordAttempt) -> bool {
    let mut char_count: HashMap<char, usize> = HashMap::new();

    for c in p_attempt.password.chars() {
        let count = char_count.entry(c).or_insert(0);
        *count += 1;
    }

    let count = char_count.get(&p_attempt.key_character).unwrap_or(&0);
    return *count >= p_attempt.policy_range.0 && *count <= p_attempt.policy_range.1;
}

fn is_attempt_valid_p2(p_attempt: &PasswordAttempt) -> bool {
    let plen = p_attempt.password.len();

    let mut count = 0;
    if p_attempt.policy_range.0 <= plen
        && p_attempt
            .password
            .chars()
            .nth(p_attempt.policy_range.0 - 1)
            .unwrap()
            == p_attempt.key_character
    {
        count += 1;
    }

    if p_attempt.policy_range.1 <= plen
        && p_attempt
            .password
            .chars()
            .nth(p_attempt.policy_range.1 - 1)
            .unwrap()
            == p_attempt.key_character
    {
        count += 1;
    }

    count == 1
}

pub fn part1() {
    let file = File::open("inputs/day2/part1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut valid_pwds = 0;

    for line in reader.lines() {
        let password_attempt = map_password_attempt(&line.unwrap());
        if is_attempt_valid_p1(&password_attempt) {
            valid_pwds += 1;
        }
    }

    println!("Answer - {}", valid_pwds);
}

pub fn part2() {
    let file = File::open("inputs/day2/part2.txt").unwrap();
    let reader = BufReader::new(file);

    let mut valid_pwds = 0;

    for line in reader.lines() {
        let password_attempt = map_password_attempt(&line.unwrap());
        if is_attempt_valid_p2(&password_attempt) {
            valid_pwds += 1;
        }
    }

    println!("Answer - {}", valid_pwds);
}
