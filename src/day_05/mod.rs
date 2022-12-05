use std::collections::HashMap;

use itertools::Itertools;

use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_05/input.txt") {
        println!("part 1: {}", part1(&contents));
        println!("part 2: {}", part2(&contents));
    }
}

pub fn part1(contents: &str) -> String {
    let (top, bottom) = contents.split_once("\n\n").unwrap();
    let mut stacks = crate_stacks(top);
    parse_moves(bottom, &mut stacks, false);
    let mut x = 0;
    let mut result: String = "".to_string();
    while stacks.get(&x).is_some() {
        let letter = stacks[&x].last().unwrap().replace('[', "").replace(']', "");
        result.push_str(&letter);
        // print!("{}", stacks[&x].last().unwrap().replace('[', "").replace(']', ""));
        x = x+1;
    }
    result
}

fn crate_stacks(top: &str) -> HashMap<usize, Vec<String>> {
    let mut map: HashMap<usize, Vec<String>> = HashMap::new();
    for line in top.lines() {
        let chunks = line.chars().chunks(4);
        for (index, chunk) in chunks.into_iter().enumerate() {
            let b = chunk.collect::<String>().trim().to_string();
            let stack = map.entry(index).or_insert(vec![]);
            if !b.is_empty() && b.len() > 1 {
                stack.insert(0, b);
            }
        }
    }
    map
}

fn parse_moves(bottom: &str, stacks: &mut HashMap<usize, Vec<String>>, multiple: bool) {
    for line in bottom.lines() {
        let mut iter = line.split(' ');
        iter.next(); // move
        let quantity = iter.next().unwrap().parse::<usize>().unwrap();
        iter.next(); // from
        let origin = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        iter.next(); // to
        let destination = iter.next().unwrap().parse::<usize>().unwrap() - 1;

        let mut popped: Vec<String> = vec![];
        {
            let stack_origin = stacks.get_mut(&origin).unwrap();
            if multiple {
                popped = stack_origin.drain(stack_origin.len() - quantity..stack_origin.len()).collect::<Vec<String>>();
            } else {
                for _ in 0..quantity {
                    let pop = stack_origin.pop().unwrap();
                    popped.push(pop);
                }
            }
        }
        let stack_destination = stacks.get_mut(&destination).unwrap();

        for x in popped {
            stack_destination.push(x);
        }
    }
}

pub fn part2(contents: &str) -> String {
    let (top, bottom) = contents.split_once("\n\n").unwrap();
    let mut stacks = crate_stacks(top);
    parse_moves(bottom, &mut stacks, true);
    let mut x = 0;
    let mut result: String = "".to_string();
    while stacks.get(&x).is_some() {
        let letter = stacks[&x].last().unwrap().replace('[', "").replace(']', "");
        result.push_str(&letter);
        // print!("{}", stacks[&x].last().unwrap().replace('[', "").replace(']', ""));
        x = x+1;
    }
    result
}
