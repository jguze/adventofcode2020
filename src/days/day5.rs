use std::fs::File;
use std::io::{prelude::*, BufReader};

enum QVariant {
    Part1,
    Part2,
}

const MAX_ROW: u32 = 128;
const MAX_COL: u32 = 8;

fn find_seat(input: &str) -> (u32, u32) {
    let mut cur_row_min = 0;
    let mut cur_row_max = MAX_ROW - 1;
    let mut cur_col_min = 0;
    let mut cur_col_max = MAX_COL - 1;

    for c in input.chars() {
        match c {
            'F' => cur_row_max = (cur_row_max - cur_row_min) / 2 + cur_row_min,
            'B' => cur_row_min = (cur_row_max - cur_row_min) / 2 + cur_row_min,
            'L' => cur_col_max = (cur_col_max - cur_col_min) / 2 + cur_col_min,
            'R' => cur_col_min = (cur_col_max - cur_col_min) / 2 + cur_col_min,
            _ => panic!("Unexpected character {}", c),
        }
    }

    (cur_row_min + 1, cur_col_min + 1)
}

fn run_validation(variant: QVariant) {
    let file = File::open("inputs/day5/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut seats = vec![];
    for line in reader.lines() {
        let input = line.unwrap();
        let (row, col) = find_seat(&input);

        seats.push(row * 8 + col);
    }

    let max = seats.iter().max().unwrap();

    println!("Answer - {}", max);
}

pub fn part1() {
    run_validation(QVariant::Part1);
}

pub fn part2() {
    run_validation(QVariant::Part2);
}
