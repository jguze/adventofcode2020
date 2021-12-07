use std::fs::File;
use std::io::{prelude::*, BufReader};

const TREE: char = '#';

fn parse_input(reader: BufReader<File>) -> Vec<Vec<char>> {
    let mut field: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        let hill = line.unwrap();
        field.push(hill.chars().collect());
    }

    field
}

fn get_wrapped_value(field: &Vec<Vec<char>>, pos: &(usize, usize)) -> Option<char> {
    let height = field.len();
    let width = field[0].len();

    if pos.0 >= height {
        return None;
    }

    return Some(field[pos.0][pos.1 % width]);
}

fn count_trees(field: &Vec<Vec<char>>, start_pos: (usize, usize), slope: (usize, usize)) -> u32 {
    let mut count = 0;

    let mut current_pos = start_pos;

    while let Some(item) = get_wrapped_value(&field, &current_pos) {
        if item == TREE {
            count += 1;
        }

        current_pos = (current_pos.0 + slope.0, current_pos.1 + slope.1);
    }

    count
}

pub fn part1() {
    let file = File::open("inputs/day3/input.txt").unwrap();
    let reader = BufReader::new(file);

    // Field is accessible as row, col.
    let field = parse_input(reader);

    // Slope is set as row, col
    let slope: (usize, usize) = (1, 3);

    println!("Answer - {}", count_trees(&field, slope, slope));
}

pub fn part2() {
    let file = File::open("inputs/day3/input.txt").unwrap();
    let reader = BufReader::new(file);

    // Field is accessible as row, col.
    let field = parse_input(reader);

    let slopes: [(usize, usize); 5] = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    let mut answer = 1;
    for slope in slopes {
        answer *= count_trees(&field, slope, slope);
    }

    println!("Answer - {}", answer);
}
