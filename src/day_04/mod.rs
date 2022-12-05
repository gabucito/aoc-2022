use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_04/input.txt") {
        // part1(&contents)
        println!("part 1: {}", part1(&contents));
        println!("part 2: {}", part2(&contents));
    }
}

#[derive(Debug)]
pub struct Assignment {
    start: usize,
    end: usize,
}

impl Assignment {
    pub fn new(start: usize, end: usize) -> Self {
        Assignment { start, end }
    }

    pub fn contains(&self, rhs: &Assignment) -> bool {
        self.start <= rhs.start && self.end >= rhs.end
    }

    pub fn overlap(&self, rhs: &Assignment) -> bool {
        (self.start >= rhs.start && self.start <= rhs.end) || (self.end >= rhs.start && self.end <= rhs.end)
    }
}

pub fn part1(contents: &str) -> usize {
    let lines: Vec<&str> = contents
        .lines()
        .filter_map(|line| {
            let (left, right) = line.split_once(',').unwrap();
            let left_assignment = id_to_num(left);
            let right_assignment = id_to_num(right);
            if left_assignment.contains(&right_assignment) {
                return Some(line);
            }
            if right_assignment.contains(&left_assignment) {
                return Some(line);
            }
            None
        })
        .collect();

    lines.len()
}

pub fn id_to_num(assignment: &str) -> Assignment {
    let (start, end) = assignment.split_once('-').unwrap();
    Assignment::new(
        start.parse::<usize>().unwrap(),
        end.parse::<usize>().unwrap(),
    )
}

pub fn part2(contents: &str) -> usize {
    let lines: Vec<&str> = contents
        .lines()
        .filter_map(|line| {
            let (left, right) = line.split_once(',').unwrap();
            let left_assignment = id_to_num(left);
            let right_assignment = id_to_num(right);
            if left_assignment.overlap(&right_assignment) {
                return Some(line);
            }
            if right_assignment.overlap(&left_assignment) {
                return Some(line);
            }
            None
        })
        .collect();

    lines.len()
}
