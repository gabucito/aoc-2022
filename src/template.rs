use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_01/input.txt") {
        // part1(&contents)
        println!("part 1: {}", part1(&contents));
        println!("part 2: {}", part2(&contents));
    }
}

pub fn part1(contents: &str) -> i32 {
}

pub fn part2(contents: &str) -> i32 {
}
