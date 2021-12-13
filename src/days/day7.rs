use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::rc::Rc;

use lazy_static::lazy_static;
use regex::Regex;

const SHINY_GOLD: &str = "shiny gold";

enum QVariant {
    Part1,
    Part2,
}

type BagRef = Rc<RefCell<Bag>>;

#[derive(Debug)]
struct Bag {
    kind: String,
    inner_bags: Vec<(BagRef, u32)>,
}

impl Bag {
    fn new(kind: String) -> Bag {
        Bag {
            kind,
            inner_bags: vec![],
        }
    }
}

fn str_to_bag_kind(s: &str) -> String {
    s.trim().to_string()
}

// Absolutely awful parsing code. I should rewrite it to use more regex
fn parse_bags(reader: BufReader<File>) -> Vec<BagRef> {
    lazy_static! {
        static ref LINE_REGEX: Regex = Regex::new(r"^(.+) contain (.+).$").unwrap();
    }

    let mut bag_map: HashMap<_, _> = HashMap::new();

    for line in reader.lines() {
        let text = line.unwrap();
        let captures = LINE_REGEX.captures_iter(text.as_str());

        for cap in captures {
            cap[2].split(",").for_each(|b| {
                let bag_end = if b.contains("bags") {
                    b.len() - 4
                } else {
                    b.len() - 3
                };
                let group = &b[0..bag_end];
                if group.trim() == "no other" {
                    return;
                }

                let tokens: Vec<&str> = group.split_whitespace().collect();

                let total_bags = &tokens[0].parse::<usize>().unwrap();

                let bag_kind = &str_to_bag_kind(&tokens[1..3].join(" "));
                bag_map
                    .entry(bag_kind.to_string())
                    .or_insert(Rc::new(RefCell::new(Bag::new(bag_kind.to_string()))));

                let bag_end = if cap[1].contains("bags") {
                    cap[1].len() - 4
                } else {
                    cap[1].len() - 3
                };

                let main_bag_kind = &str_to_bag_kind(&cap[1][0..bag_end]);
                bag_map
                    .entry(main_bag_kind.to_string())
                    .or_insert(Rc::new(RefCell::new(Bag::new(main_bag_kind.to_string()))));

                let main_bag_rc = bag_map.get(main_bag_kind).unwrap();
                let new_bag_rc = bag_map.get(bag_kind).unwrap();

                let mut main_bag = main_bag_rc.as_ref().borrow_mut();
                main_bag
                    .inner_bags
                    .push((Rc::clone(new_bag_rc), *total_bags as u32));
            });
        }
    }

    bag_map.into_iter().map(|(_, bag)| bag).collect()
}

fn can_find_bag(kind: &str, bag: &Bag, visited: &mut HashSet<String>) -> bool {
    if bag.kind == kind {
        return true;
    }

    if visited.contains(&bag.kind) {
        return false;
    }

    visited.insert(bag.kind.to_string());

    for inner_bag in &bag.inner_bags {
        let inner_bag = inner_bag.0.as_ref().borrow();

        if can_find_bag(kind, &inner_bag, visited) {
            return true;
        }
    }

    visited.remove(&bag.kind);

    false
}

fn count_bags(bag: &Bag, visited: &mut HashMap<String, u32>) -> u32 {
    if visited.contains_key(&bag.kind) {
        return *visited.get(&bag.kind).unwrap();
    }

    let mut total_bags = 1;
    for inner_bag_rc in &bag.inner_bags {
        let inner_bag = inner_bag_rc.0.as_ref().borrow();
        total_bags += count_bags(&inner_bag, visited) * inner_bag_rc.1;
    }

    visited.insert(bag.kind.to_string(), total_bags);
    total_bags
}

fn run_validation(variant: QVariant) {
    let file = File::open("inputs/day7/input.txt").unwrap();
    let reader = BufReader::new(file);
    let bags = parse_bags(reader);

    let mut count = 0;

    match variant {
        QVariant::Part1 => {
            for bag in &bags {
                let bag = bag.as_ref().borrow();
                if bag.kind == SHINY_GOLD {
                    continue;
                }
                if can_find_bag(SHINY_GOLD, &bag, &mut HashSet::new()) {
                    count += 1;
                }
            }
        }
        QVariant::Part2 => {
            let shiny_bag = bags
                .iter()
                .find(|b| b.as_ref().borrow().kind == SHINY_GOLD)
                .unwrap()
                .as_ref()
                .borrow();
            count = count_bags(&shiny_bag, &mut HashMap::new()) - 1;
        }
    }

    println!("Answer - {}", count);
}

pub fn part1() {
    run_validation(QVariant::Part1);
}

pub fn part2() {
    run_validation(QVariant::Part2);
}
