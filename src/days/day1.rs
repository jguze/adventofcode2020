use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn part1() {
    let file = File::open("inputs/day1/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut numbers: HashSet<u32> = HashSet::new();

    for line in reader.lines() {
        let num = line.expect("Read line correctly").parse::<u32>().unwrap();
        numbers.insert(num);
    }

    for num in &numbers {
        let diff = 2020 - num;
        if numbers.contains(&diff) {
            println!("Pair - {}, {}", num, diff);
            println!("Answer - {}", num * diff);
            break;
        }
    }
}

pub fn part2() {
    let file = File::open("inputs/day1/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut numbers: HashSet<i32> = HashSet::new();

    for line in reader.lines() {
        let num = line.expect("Read line correctly").parse::<i32>().unwrap();
        numbers.insert(num);
    }

    'outer: for num1 in &numbers {
        for num2 in &numbers {
            let diff = 2020 - num1 - num2;
            if numbers.contains(&diff) {
                println!("Pair - {}, {}, {}", num1, num2, diff);
                println!("Answer - {}", num1 * diff * num2);
                break 'outer;
            }
        }
    }
}
