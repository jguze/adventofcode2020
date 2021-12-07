use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

enum QVariant {
    Part1,
    Part2,
}

fn parse_boards(reader: BufReader<File>) -> Vec<(HashMap<char, u32>, u32)> {
    let mut boards = vec![];
    let mut line_iter = reader.lines().peekable();

    while line_iter.peek().is_some() {
        let mut board = HashMap::new();
        let mut people = 0;
        while let Some(line_opt) = line_iter.next() {
            let line = line_opt.unwrap();
            if line.len() == 0 {
                break;
            }

            line.chars().for_each(|c| {
                let count = board.entry(c).or_insert(0);
                *count += 1;
            });
            people += 1;
        }
        boards.push((board, people));
    }

    boards
}

fn run_validation(variant: QVariant) {
    let file = File::open("inputs/day6/input.txt").unwrap();
    let reader = BufReader::new(file);

    let boards = parse_boards(reader);

    let mut sum = 0;

    match variant {
        QVariant::Part1 => {
            for (board, _) in boards {
                sum += board.len();
            }
        }
        QVariant::Part2 => {
            for (board, people) in boards {
                for (_, value) in board {
                    if value == people {
                        sum += 1;
                    }
                }
            }
        }
    }
    println!("Answer - {}", sum);
}

pub fn part1() {
    run_validation(QVariant::Part1);
}

pub fn part2() {
    run_validation(QVariant::Part2);
}
