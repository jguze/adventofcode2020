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
            'F' => {
                if cur_row_min == cur_row_max - 1 {
                    cur_row_max = cur_row_min;
                } else {
                    cur_row_max = (cur_row_max - cur_row_min) / 2 + cur_row_min;
                }
            }
            'B' => {
                if cur_row_min == cur_row_max - 1 {
                    cur_row_min = cur_row_max;
                } else {
                    cur_row_min = (cur_row_max - cur_row_min) / 2 + cur_row_min;
                }
            }
            'L' => {
                if cur_col_min == cur_col_max - 1 {
                    cur_col_max = cur_col_min;
                } else {
                    cur_col_max = (cur_col_max - cur_col_min) / 2 + cur_col_min;
                }
            }
            'R' => {
                if cur_col_min == cur_col_max - 1 {
                    cur_col_min = cur_col_max;
                } else {
                    cur_col_min = (cur_col_max - cur_col_min) / 2 + cur_col_min;
                }
            }
            _ => panic!("Unexpected character {}", c),
        }
    }

    (cur_row_max, cur_col_max)
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

    let max = *seats.iter().max().unwrap();
    let min = *seats.iter().min().unwrap();

    if matches!(variant, QVariant::Part1) {
        println!("Answer - {}", max);
    } else {
        seats.sort();

        let mut misses: usize = min as usize;
        let mut answer: Option<u32> = None;
        for (e, seat) in seats.iter().enumerate() {
            if e == 0 || e == max as usize {
                continue;
            }

            let cur_iter = e + misses;
            if *seat != cur_iter as u32 {
                if seats[e - 1] == cur_iter as u32 - 1 && *seat == cur_iter as u32 + 1 {
                    answer = Some(cur_iter as u32);
                    break;
                }
                misses += 1;
            }
        }

        if answer.is_some() {
            println!("Answer {}", answer.unwrap());
        } else {
            println!("Could not find seat within the requirements");
        }
    }
}

pub fn part1() {
    run_validation(QVariant::Part1);
}

pub fn part2() {
    run_validation(QVariant::Part2);
}
