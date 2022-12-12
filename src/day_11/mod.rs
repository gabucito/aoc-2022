use std::{
    cell::RefCell,
    collections::{BTreeMap, VecDeque},
};

use itertools::Itertools;

use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_11/input.txt") {
        // part1(&contents);
        println!("part 1: {}", part1(&contents));
        println!("part 2: {}", part2(&contents));
    }
}

pub fn part1(contents: &str) -> u64 {
    let monkey_tree = generate_tree(contents);
    for _ in 0..20 {
        round(&monkey_tree, true);
    }
    monkey_tree.monkey_inspect_result()
}

pub fn part2(contents: &str) -> u64 {
    let monkey_tree = generate_tree(contents);
    for _ in 0..10000 {
        round(&monkey_tree, false);
    }
    monkey_tree.monkey_inspect_result()
}

pub fn generate_tree(contents: &str) -> MonkeyTree {
    let btree: BTreeMap<usize, Monkey> =
        contents
            .lines()
            .chunks(7)
            .into_iter()
            .fold(BTreeMap::new(), |mut btreemap, mut chunk| {
                let id = chunk.next().unwrap()[7..8].parse::<usize>().unwrap();

                let second_line = chunk.next().unwrap().trim().split(':');
                let items: VecDeque<u64> = second_line
                    .skip(1)
                    .next()
                    .unwrap()
                    .trim()
                    .split(',')
                    .map(|i| i.trim().parse::<u64>().unwrap())
                    .collect();

                let mut third_line = chunk.next().unwrap().trim().split(' ').rev().take(2);
                let operation = match (
                    third_line.next().unwrap().parse::<u64>(),
                    third_line.next().unwrap(),
                ) {
                    (Ok(num), "*") => Operation::Multiply(num),
                    (Ok(num), "+") => Operation::Add(num),
                    (_, "*") => Operation::MultiplyOld,
                    (_, "+") => Operation::AddOld,
                    _ => panic!("operation unknown"),
                };

                let divisible = chunk
                    .next()
                    .unwrap()
                    .trim()
                    .split(' ')
                    .rev()
                    .take(1)
                    .next()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                let t = chunk
                    .next()
                    .unwrap()
                    .trim()
                    .split(' ')
                    .rev()
                    .take(1)
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let f = chunk
                    .next()
                    .unwrap()
                    .trim()
                    .split(' ')
                    .rev()
                    .take(1)
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                btreemap.insert(
                    id,
                    Monkey {
                        id,
                        items: RefCell::new(items),
                        operation,
                        divisible,
                        t,
                        f,
                        inspect: RefCell::new(0),
                    },
                );
                btreemap
            });

    MonkeyTree { monkeys: btree }
}

pub fn round(monkey_tree: &MonkeyTree, divide_by_3: bool) {
    let period = monkey_tree.get_period();
    for idx in 0..monkey_tree.monkeys.len() {
        // let (idx2, value) = monkey.calculate();
        let monkey = monkey_tree.get_monkey_by_id(&idx);
        while let Some(idx2) = monkey.calculate(divide_by_3, period) {
            let value = monkey.items.borrow_mut().pop_front().unwrap();
            let monkey2 = monkey_tree.get_monkey_by_id(&idx2);
            monkey2.items.borrow_mut().push_back(value);
        }
        // while let Some((idx2, value)) = monkey.calculate() {
        //     let monkey2 = monkey_tree.get_monkey_by_id(&idx2);
        // }
    }
    // monkey_tree.calculate(monkey);
    //  monkey_tree.monkey_inspect_result()
}

#[derive(Debug)]
enum Operation {
    Add(u64),
    Multiply(u64),
    AddOld,
    MultiplyOld,
}

pub struct MonkeyTree {
    pub monkeys: BTreeMap<usize, Monkey>,
}
impl MonkeyTree {
    pub fn get_monkey_by_id(&self, id: &usize) -> &Monkey {
        self.monkeys.get(id).unwrap()
    }

    pub fn monkey_inspect_result(&self) -> u64 {
        let mut inspects: Vec<u64> = self
            .monkeys
            .iter()
            .map(|(_, monkey)| monkey.inspect.borrow().clone())
            .collect();
        inspects.sort();
        inspects.reverse();
        inspects
            .into_iter()
            .take(2)
            .reduce(|total, i| total * i)
            .unwrap()
    }

    pub fn get_period(&self) -> u64 {
        self.monkeys
            .iter()
            .map(|(_, monkey)| monkey.divisible)
            .reduce(|total, i| total * i)
            .unwrap()
    }
}

#[derive(Debug)]
pub struct Monkey {
    id: usize,
    items: RefCell<VecDeque<u64>>,
    operation: Operation,
    divisible: u64,
    t: usize,
    f: usize,
    inspect: RefCell<u64>,
}

impl Monkey {
    pub fn calculate(&self, divide_by_3: bool, period: u64) -> Option<usize> {
        match self.items.borrow_mut().get_mut(0) {
            Some(item) => {
                *item = *item % period;
                *self.inspect.borrow_mut() += 1;
                *item = match self.operation {
                    Operation::Add(num) => *item + num,
                    Operation::Multiply(num) => *item * num,
                    Operation::AddOld => *item + *item,
                    Operation::MultiplyOld => *item * *item,
                };
                if divide_by_3 {
                    *item = *item / 3;
                }
                if *item % self.divisible == 0 {
                    return Some(self.t);
                } else {
                    return Some(self.f);
                }
            }
            None => None,
        }
    }
}

struct Move(usize, u64);

#[cfg(test)]
mod tests {
    // #[test]
    // fn check() {
    //     assert_eq!(_, _);
    // }
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
