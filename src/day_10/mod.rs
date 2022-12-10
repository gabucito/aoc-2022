use std::collections::HashMap;

use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_10/input.txt") {
        println!("part 1: {}", part1(&contents));
        println!("{}", part2(&contents));
    }
}

pub fn part1(contents: &str) -> i32 {
    let signals = signal(&contents);
    const CYCLES: &[i32] = &[20, 60, 100, 140, 180, 220];
    CYCLES.iter().fold(0, |total, cycle| {
        total + (cycle * signals.get(cycle).unwrap().0)
    })
}

pub fn part2(contents: &str) -> String {
    let signals = signal(&contents);
    // let mut crt: Vec<char> = vec![];
    let mut crt: Vec<String> = vec![];
    for l in 0..6 {
        let mut line: Vec<char> = vec![];
        for pixel in 1..=40 {
            let cycle = l * 40 + pixel;
            let register = signals.get(&cycle).unwrap();
            line.push(register.pixel_has_sprite(cycle));
        }
        let as_string: String = line.into_iter().collect();
        crt.push(as_string)
    }

    crt.join("\n")
}

enum Instruction {
    Addx(i32),
    Noop,
}

impl Instruction {
    fn cycles(&self) -> u8 {
        match self {
            Instruction::Addx(_) => 2,
            Instruction::Noop => 1,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Sprite(i32);

impl Sprite {
    pub fn pixel_has_sprite(&self, cycle: i32) -> char {
        let mut cursor = cycle % 40;
        if cursor == 0 {
            cursor = 40;
        }
        if cursor >= self.0 && cursor <= self.0 + 2 {
            return '#';
        }
        '.'
    }
}

pub fn signal(contents: &str) -> HashMap<i32, Sprite> {
    let mut register: Sprite = Sprite(1);
    let mut cycle = 1;
    let mut hash = HashMap::new();
    for line in contents.lines() {
        let mut iter = line.split_whitespace();
        let op = iter.next().unwrap();
        let instruction = match op {
            "addx" => Instruction::Addx(iter.next().unwrap().parse::<i32>().unwrap()),
            _ => Instruction::Noop,
        };
        for _ in 0..instruction.cycles() {
            hash.insert(cycle, register);
            cycle += 1;
        }
        if let Instruction::Addx(n) = instruction {
            register.0 += n;
        }
    }
    hash
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn check() {
    //     assert_eq!(_, _);
    // }
}
