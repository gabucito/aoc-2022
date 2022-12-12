use std::{collections::{HashMap, BTreeMap}, rc::Rc, cell::{RefCell, RefMut}};

use itertools::Itertools;

use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_11/test.txt") {
        part1(&contents);
        // println!("part 1: {}", part1(&contents));
        // println!("{}", part2(&contents));
    }
}

pub fn part1(contents: &str) {
    let btree: BTreeMap<usize, Rc<RefCell<Monkey>>> = contents.lines().chunks(7).into_iter().fold(BTreeMap::new(), |mut btreemap, mut chunk| {
        let id = chunk.next().unwrap()[7..8].parse::<usize>().unwrap();

        let second_line = chunk.next().unwrap().trim().split(':');
        let items: Vec<u32> = second_line.skip(1).next().unwrap().trim().split(',').map(|i| i.trim().parse::<u32>().unwrap()).collect();

        let mut third_line = chunk.next().unwrap().trim().split(' ').rev().take(2);
        let operation = match (third_line.next().unwrap().parse::<u32>(), third_line.next().unwrap()) {
            (Ok(num), "*") => Operation::Multiply(num),
            (Ok(num), "+") => Operation::Add(num),
            (_, "*") => Operation::MultiplyOld,
            (_, "+") => Operation::AddOld,
            _ => panic!("operation unknown"),
        };

        
        let divisible = chunk.next().unwrap().trim().split(' ').rev().take(1).next().unwrap().parse::<u32>().unwrap();
        let t = chunk.next().unwrap().trim().split(' ').rev().take(1).next().unwrap().parse::<usize>().unwrap();
        let f = chunk.next().unwrap().trim().split(' ').rev().take(1).next().unwrap().parse::<usize>().unwrap();
        btreemap.insert(id, Rc::new(RefCell::new(Monkey {
            id,
            items,
            operation,
            divisible,
            t,
            f,
        })));
        btreemap
    });

    let mut monkey_tree = MonkeyTree{
        monkeys: btree
    };
    
    for idx in 0..monkey_tree.monkeys.len() {
        // let (idx2, value) = monkey.calculate();
        let mut monkey = monkey_tree.get_monkey_by_id(&idx);
        if let Some((idx2, value)) = monkey.calculate() {
            let monkey2 = monkey_tree.get_monkey_by_id(&idx2);
        }
        // while let Some((idx2, value)) = monkey.calculate() {
        //     let monkey2 = monkey_tree.get_monkey_by_id(&idx2);
        // }
    }
    // monkey_tree.calculate(monkey);
}

pub fn part2(contents: &str) {
}

#[derive(Debug)]
enum Operation {
    Add(u32),
    Multiply(u32),
    AddOld,
    MultiplyOld,
}

struct MonkeyTree {
    monkeys: BTreeMap<usize, Rc<RefCell<Monkey>>>
}
impl MonkeyTree {

    pub fn get_monkey_by_id(&mut self, id: &usize) -> RefMut<Monkey> {
        self.monkeys.get(id).unwrap().borrow_mut()
    }
}


#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<u32>,
    operation: Operation,
    divisible: u32,
    t: usize,
    f: usize,
}

impl Monkey {
    pub fn calculate(&mut self) -> Option<(usize, u32)> {
        // let &item = monkey.items[0];
        match self.items.get(0) {
            Some(&item) => {
                let new = match self.operation {
                    Operation::Add(num) => item + num,
                    Operation::Multiply(num) => item * num,
                    Operation::AddOld => item + item,
                    Operation::MultiplyOld => item * item,
                };
                let level = self.items[0] % 3;
                self.items[0] = new;
                if level % self.divisible == 0 { 
                    return Some((self.t, new))
                } else { 
                    return Some((self.f, new))
                }
            },
            None => None,
        }
    }
}

struct Move(usize, u32);

#[cfg(test)]
mod tests {
    // #[test]
    // fn check() {
    //     assert_eq!(_, _);
    // }
}
