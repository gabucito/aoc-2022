use crate::read::read_file;
use std::collections::HashSet;

pub fn run() {
    if let Ok(contents) = read_file("src/day_03/test.txt") {
        // part2(&contents)
        println!("part 1: {}", part1_using_intersection(&contents));
        println!("part 2: {}", part2(&contents));
    }
}

pub fn part1(contents: &str) -> i32 {
    contents.lines().fold(0, |total, line| {
        let half = line.len() / 2;
        let first = &line[0..half];
        let second = &line[half..];
        let c = get_repeated(first, second);
        if let Some(ch) = c {
            let digit = get_char_value(ch);
            return total + digit
        }
        total
    })
}


pub fn part1_using_intersection(contents: &str) -> i32 {
    contents.lines().fold(0, |total, line| {
        let half = line.len() / 2;
        let first: HashSet<char> = line[0..half].chars().collect();
        let second: HashSet<char> = line[half..].chars().collect();
        let intersection = first.intersection(&second);
        let test = &first & &second; // I had no idea you can do this
        println!("{:?}", test);
        if let Some(ch) = intersection.to_owned().next() {
            let digit = get_char_value(ch.to_owned());
            return total + digit
        }
        total
    })
}

pub fn part2(contents: &str) -> i32 {
    let lines: Vec<&str> = contents.lines().collect();
    lines.chunks(3).fold(0, |total, chunk| {
        let c = get_repeated2(chunk[0], chunk[1], chunk[2]);
        if let Some(ch) = c {
            let digit = get_char_value(ch);
            return total + digit
        }
        total
    })
}


// pub fn part2_using_intersection(contents: &str) -> i32 {
//     let lines: Vec<&str> = contents.lines().collect();
//     lines.chunks(3).fold(0, |total, chunk| {
//         let first: HashSet<char> = chunk[0].chars().collect();
//         let second: HashSet<char> = chunk[1].chars().collect();
//         let third: HashSet<char> = chunk[2].chars().collect();
//         let intersection = first.intersection(&second)
//         if let Some(ch) = c {
//             let digit = get_char_value(ch);
//             return total + digit
//         }
//         total
//     })
//     contents.lines().fold(0, |total, line| {
//         let half = line.len() / 2;
//         let first: HashSet<char> = line[0..half].chars().collect();
//         let second: HashSet<char> = line[half..].chars().collect();
//         let intersection = first.intersection(&second);
//         if let Some(ch) = intersection.to_owned().next() {
//             let digit = get_char_value(ch.to_owned());
//             return total + digit
//         }
//         total
//     })
// }

pub fn get_repeated(first: &str, second: &str) -> Option<char> {
    for f in first.chars() {
        for s in second.chars() {
            if f == s {
                return Some(s);
            }
        }
    }
    None
}

pub fn get_repeated2(first: &str, second: &str, third: &str) -> Option<char> {
    for f in first.chars() {
        for s in second.chars() {
            for t in third.chars() {
                if f == s && s == t {
                    return Some(s);
                }
            }
        }
    }
    None
}

pub fn get_char_value(c: char) -> i32 {
    if c.is_ascii_lowercase() {
        c as i32 - 96
    } else {
        c as i32 - 64 + 26
    }
}