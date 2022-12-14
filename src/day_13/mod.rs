use std::cmp::Ordering;

use crate::read::read_file;
use nom::character::complete::digit1;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{char, one_of},
    combinator::{map, map_res},
    error::ErrorKind,
    multi::{many0, separated_list0},
    sequence::{delimited, terminated},
    IResult, Parser,
};

pub fn run() {
    if let Ok(contents) = read_file("src/day_13/input.txt") {
        // println!("part 1: {}", part1(&contents));
        println!("part 2: {}", part2(&contents));
    }
}

pub fn part1(contents: &str) -> usize {
    contents
        .split("\n\n")
        .enumerate()
        .fold(0, |total, (i, s)| {
            let idx = i + 1;
            let mut iter = s.lines();
            let first_line = iter.next().unwrap();
            let second_line = iter.next().unwrap();
            let pair = Pair::new(first_line, second_line);
            if pair.compare() {
                return total + idx
            }
            total
        })
}

pub fn part2(contents: &str) -> usize {
    let mut packets = contents
        .lines()
        .filter(|&l| l != "")
        .map(|line| {
            let (_, packet) = parse_packet(line).unwrap();
            packet
        })
        .collect::<Vec<Packet>>();
    
    let divider_packets = vec![Packet::List(vec![Packet::Number(2)]), Packet::List(vec![Packet::Number(6)])];
    packets.push(divider_packets[0].clone());
    packets.push(divider_packets[1].clone());
    packets.sort();
    packets.iter().enumerate().fold(1, |total, (i, packet)| {
        if  divider_packets.contains(packet) {
            total * (i+1)
        } else {
            total
        }
    })
}

#[derive(Debug)]
pub struct Pair {
    left: Packet,
    right: Packet,
}

impl Pair {
    pub fn new(first: &str, second: &str) -> Self {
        // println!("{first}");
        let (_, left) = parse_packet(first).unwrap();
        let (_, right) = parse_packet(second).unwrap();

        Pair {
            left,
            right,
        }
    }

    pub fn compare(&self) -> bool {
        self.left < self.right
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Number(a), Packet::Number(b)) => {
                println!("Compare {a} vs {b}");
                a.cmp(b)
            }
            (Packet::Number(a), Packet::List(_)) => {
                println!("Mixed types; convert left to [{}] and retry comparison", a);
                let x = Packet::List(vec![Packet::Number(*a)]);
                x.cmp(other)
            }
            (Packet::List(_), Packet::Number(b)) => {
                println!("Mixed types; convert right to [{}] and retry comparison", b);
                let y = Packet::List(vec![Packet::Number(*b)]);
                self.cmp(&y)
            }
            (Packet::List(x), Packet::List(y)) => {
                let mut ordering = Ordering::Equal;
                let mut left_iter = x.iter();
                let mut right_iter = y.iter();
                while ordering == Ordering::Equal {
                    ordering = match (left_iter.next(), right_iter.next()) {
                        (Some(left), Some(right)) => left.cmp(right),
                        (Some(_), None) => {
                            println!("RIGHT side ran out of items");
                            return Ordering::Greater
                        }
                        (None, Some(_)) => {
                            println!("LEFT side ran out of items");
                            return Ordering::Less
                        }
                        (None, None) => return ordering,
                    };
                }
                ordering
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn parse_packet(line: &str) -> IResult<&str, Packet> {
    alt((
        delimited(char('['), separated_list0(tag(","), parse_packet), char(']'))
            .map(|v| Packet::List(v)),
        nom::character::complete::u32.map(|n| Packet::Number(n)),
    ))(line)
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn check() {
//         assert_eq!(_, _);
//     }
// }
