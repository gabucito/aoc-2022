use std::str::Split;

use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_01/input.txt") {
        // part1(&contents)
        println!("part 1: {}", part1(&contents));
        println!("part 2: {}", part2(&contents));
    }
}

// Return the total calories of the elf with most calories
pub fn part1(contents: &str) -> i32 {
    elves_total_calories(contents, 1)
}

// Return the total calories of the 3 elves with most calories
pub fn part2(contents: &str) -> i32 {
    elves_total_calories(contents, 3)
}

// pub fn elf_carrying_moset_calories(contents: &str) -> i32 {
//     let mut max = 0;
//     for paragraph in split_elfs(contents) {
//         let total = paragraph
//             .lines()
//             .fold(0, |total, line| total + line.parse::<i32>().unwrap());
//         if total > max {
//             max = total
//         }
//     }
//     max
// }

pub fn elves_total_calories(contents: &str, limit: usize) -> i32 {
    let mut vector_most_calories: Vec<i32> = split_elfs(contents)
        .map(|paragraph| {
            paragraph
                .lines()
                .fold(0, |total, line| total + line.parse::<i32>().unwrap())
        })
        .collect();
    vector_most_calories.sort();
    vector_most_calories.into_iter().rev().take(limit).sum()
}

pub fn split_elfs(contents: &str) -> Split<&str> {
    contents.split("\n\n")
}