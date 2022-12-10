use std::collections::HashSet;

use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_09/input.txt") {
        part2(&contents)
        // println!("part 1: {}", part1(&contents));
        // println!("part 2: {}", part2(&contents));
    }
}

enum Direction {
    Up(i8),
    Right(i8),
    Down(i8),
    Left(i8),
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    visited: HashSet<(i32, i32)>,
}

impl Point {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            visited: HashSet::from([(0, 0)]),
        }
    }

    pub fn save(&mut self) {
        self.visited.insert((self.x, self.y));
    }

    pub fn touching(&self, rhs: &Point) -> bool {
        match (i32::abs(self.x - rhs.x), i32::abs(self.y - rhs.y)) {
            (0, 0) => true,
            (1, 0) => true,
            (0, 1) => true,
            (1, 1) => true,
            _ => false,
        }
    }

    pub fn follow(&mut self, rhs: &Point) {
        match (self.x - rhs.x, self.y - rhs.y) {
            (2, 0) => self.x -= 1,  // go right
            (-2, 0) => self.x += 1, // go left
            (0, 2) => self.y -= 1,  // go up
            (0, -2) => self.y += 1, // go down
            (2, 1) => {
                self.x -= 1;
                self.y -= 1;
            }
            (-2, 1) => {
                self.x += 1;
                self.y -= 1;
            }
            (2, -1) => {
                self.x -= 1;
                self.y += 1;
            }
            (-2, -1) => {
                self.x += 1;
                self.y += 1;
            }
            (1, 2) => {
                self.x -= 1;
                self.y -= 1;
            }
            (-1, 2) => {
                self.x += 1;
                self.y -= 1;
            }
            (1, -2) => {
                self.x -= 1;
                self.y += 1;
            }
            (-1, -2) => {
                self.x += 1;
                self.y += 1;
            }
            (2, 2) => {
                self.x -= 1;
                self.y -= 1;
            }
            (-2, 2) => {
                self.x += 1;
                self.y -= 1;
            }
            (2, -2) => {
                self.x -= 1;
                self.y += 1;
            }
            (-2, -2) => {
                self.x += 1;
                self.y += 1;
            }
            _ => (),
        }
        self.save();
    }
}

pub fn part1(contents: &str) {
    let directions: Vec<Direction> = contents
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            match (iter.next().unwrap(), iter.next().unwrap()) {
                ("U", t) => Direction::Up(t.parse::<i8>().unwrap()),
                ("R", t) => Direction::Right(t.parse::<i8>().unwrap()),
                ("D", t) => Direction::Down(t.parse::<i8>().unwrap()),
                ("L", t) => Direction::Left(t.parse::<i8>().unwrap()),
                _ => panic!("Could not parse"),
            }
        })
        .collect();

    let start: Point = Point::new();
    let mut head: Point = Point::new();
    let mut tail: Point = Point::new();
    for direction in directions {
        // move head
        match direction {
            Direction::Up(t) => {
                for _ in 0..t {
                    head.y += 1;
                    // if tail is not touching head move it
                    if !head.touching(&tail) {
                        tail.follow(&head);
                    }
                    println!("({},{}) - ({},{})", head.x, head.y, tail.x, tail.y);
                }
            }
            Direction::Right(t) => {
                for _ in 0..t {
                    head.x += 1;
                    // if tail is not touching head move it
                    if !head.touching(&tail) {
                        tail.follow(&head);
                    }
                    println!("({},{}) - ({},{})", head.x, head.y, tail.x, tail.y);
                }
            }
            Direction::Down(t) => {
                for _ in 0..t {
                    head.y -= 1;
                    // if tail is not touching head move it
                    if !head.touching(&tail) {
                        tail.follow(&head);
                    }
                    println!("({},{}) - ({},{})", head.x, head.y, tail.x, tail.y);
                }
            }
            Direction::Left(t) => {
                for _ in 0..t {
                    head.x -= 1;
                    // if tail is not touching head move it
                    if !head.touching(&tail) {
                        tail.follow(&head);
                    }
                    println!("({},{}) - ({},{})", head.x, head.y, tail.x, tail.y);
                }
            }
        };
    }
    println!("{:?}", tail.visited.len());
}

pub fn part2(contents: &str) {
    let directions: Vec<Direction> = contents
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            match (iter.next().unwrap(), iter.next().unwrap()) {
                ("U", t) => Direction::Up(t.parse::<i8>().unwrap()),
                ("R", t) => Direction::Right(t.parse::<i8>().unwrap()),
                ("D", t) => Direction::Down(t.parse::<i8>().unwrap()),
                ("L", t) => Direction::Left(t.parse::<i8>().unwrap()),
                _ => panic!("Could not parse"),
            }
        })
        .collect();

    let start: Point = Point::new();
    let mut head: Point = Point::new();
    let mut b1: Point = Point::new();
    let mut b2: Point = Point::new();
    let mut b3: Point = Point::new();
    let mut b4: Point = Point::new();
    let mut b5: Point = Point::new();
    let mut b6: Point = Point::new();
    let mut b7: Point = Point::new();
    let mut b8: Point = Point::new();
    let mut tail: Point = Point::new();
    for direction in directions {
        // move head
        match direction {
            Direction::Up(t) => {
                for _ in 0..t {
                    head.y += 1;
                    // if tail is not touching head move it
                    if !b1.touching(&head) {
                        b1.follow(&head);
                    }
                    if !b2.touching(&b1) {
                        b2.follow(&b1);
                    }
                    if !b3.touching(&b2) {
                        b3.follow(&b2);
                    }
                    if !b4.touching(&b3) {
                        b4.follow(&b3);
                    }
                    if !b5.touching(&b4) {
                        b5.follow(&b4);
                    }
                    if !b6.touching(&b5) {
                        b6.follow(&b5);
                    }
                    if !b7.touching(&b6) {
                        b7.follow(&b6);
                    }
                    if !b8.touching(&b7) {
                        b8.follow(&b7);
                    }
                    if !tail.touching(&b8) {
                        tail.follow(&b8);
                    }
                    
        println!(
            "({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{})",
            head.x, head.y, 
            b1.x, b1.y, 
            b2.x, b2.y,
            b3.x, b3.y,
            b4.x, b4.y,
            b5.x, b5.y,
            b6.x, b6.y,
            b7.x, b7.y,
            b8.x, b8.y,
            tail.x, tail.y,
        );
                }
            }
            Direction::Right(t) => {
                for _ in 0..t {
                    head.x += 1;
                    // if tail is not touching head move it
                    if !b1.touching(&head) {
                        b1.follow(&head);
                    }
                    if !b2.touching(&b1) {
                        b2.follow(&b1);
                    }
                    if !b3.touching(&b2) {
                        b3.follow(&b2);
                    }
                    if !b4.touching(&b3) {
                        b4.follow(&b3);
                    }
                    if !b5.touching(&b4) {
                        b5.follow(&b4);
                    }
                    if !b6.touching(&b5) {
                        b6.follow(&b5);
                    }
                    if !b7.touching(&b6) {
                        b7.follow(&b6);
                    }
                    if !b8.touching(&b7) {
                        b8.follow(&b7);
                    }
                    if !tail.touching(&b8) {
                        tail.follow(&b8);
                    }
                    println!(
                        "({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{})",
                        head.x, head.y, 
                        b1.x, b1.y, 
                        b2.x, b2.y,
                        b3.x, b3.y,
                        b4.x, b4.y,
                        b5.x, b5.y,
                        b6.x, b6.y,
                        b7.x, b7.y,
                        b8.x, b8.y,
                        tail.x, tail.y,
                    );
                }
            }
            Direction::Down(t) => {
                for _ in 0..t {
                    head.y -= 1;
                    // if tail is not touching head move it
                    if !b1.touching(&head) {
                        b1.follow(&head);
                    }
                    if !b2.touching(&b1) {
                        b2.follow(&b1);
                    }
                    if !b3.touching(&b2) {
                        b3.follow(&b2);
                    }
                    if !b4.touching(&b3) {
                        b4.follow(&b3);
                    }
                    if !b5.touching(&b4) {
                        b5.follow(&b4);
                    }
                    if !b6.touching(&b5) {
                        b6.follow(&b5);
                    }
                    if !b7.touching(&b6) {
                        b7.follow(&b6);
                    }
                    if !b8.touching(&b7) {
                        b8.follow(&b7);
                    }
                    if !tail.touching(&b8) {
                        tail.follow(&b8);
                    }
                    println!(
                        "({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{})",
                        head.x, head.y, 
                        b1.x, b1.y, 
                        b2.x, b2.y,
                        b3.x, b3.y,
                        b4.x, b4.y,
                        b5.x, b5.y,
                        b6.x, b6.y,
                        b7.x, b7.y,
                        b8.x, b8.y,
                        tail.x, tail.y,
                    );
                }
            }
            Direction::Left(t) => {
                for _ in 0..t {
                    head.x -= 1;
                    // if tail is not touching head move it
                    if !b1.touching(&head) {
                        b1.follow(&head);
                    }
                    if !b2.touching(&b1) {
                        b2.follow(&b1);
                    }
                    if !b3.touching(&b2) {
                        b3.follow(&b2);
                    }
                    if !b4.touching(&b3) {
                        b4.follow(&b3);
                    }
                    if !b5.touching(&b4) {
                        b5.follow(&b4);
                    }
                    if !b6.touching(&b5) {
                        b6.follow(&b5);
                    }
                    if !b7.touching(&b6) {
                        b7.follow(&b6);
                    }
                    if !b8.touching(&b7) {
                        b8.follow(&b7);
                    }
                    if !tail.touching(&b8) {
                        tail.follow(&b8);
                    }
                    println!(
                        "({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{}) - ({},{})",
                        head.x, head.y, 
                        b1.x, b1.y, 
                        b2.x, b2.y,
                        b3.x, b3.y,
                        b4.x, b4.y,
                        b5.x, b5.y,
                        b6.x, b6.y,
                        b7.x, b7.y,
                        b8.x, b8.y,
                        tail.x, tail.y,
                    );
                }
            }
        };
    }
    println!("{:?}", tail.visited.len());
}

#[cfg(test)]
mod tests {
    #[test]
    fn check() {
        // assert_eq!(_, _);
    }
}
