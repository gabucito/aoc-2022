use std::collections::HashSet;

use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_06/input.txt") {
        // part1(&contents)
        println!("part 1: {:?}", part1(&contents));
        println!("part 2: {}", part2(&contents));
    }
}

pub fn part1(contents: &str) -> usize {
    first_unique_chars(contents, 4)
}

pub fn part2(contents: &str) -> usize {
    first_unique_chars(contents, 14)
}

fn first_unique_chars(contents: &str, limit: usize) -> usize {
    let mut iter = contents.as_bytes().windows(limit);
    let mut index = limit;
    while let Some(window) = iter.next() {
        let set: HashSet<&u8> = HashSet::from_iter(window);
        if set.len() == limit {
            return index;
        }
        index += 1;
    }
    index
}

#[cfg(test)]
mod tests {
    use crate::day_06::first_unique_chars;

    #[test]
    fn check() {
        assert_eq!(first_unique_chars(&"bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(first_unique_chars(&"nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(
            first_unique_chars(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            10
        );
        assert_eq!(
            first_unique_chars(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            11
        );

        
        assert_eq!(
            first_unique_chars(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14),
            19
        );
        assert_eq!(
            first_unique_chars(&"bvwbjplbgvbhsrlpgdmjqwftvncz", 14),
            23
        );
        assert_eq!(
            first_unique_chars(&"nppdvjthqldpwncqszvftbrmjlhg", 14),
            23
        );
        assert_eq!(
            first_unique_chars(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            29
        );
        assert_eq!(
            first_unique_chars(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            26
        );
    }
}
