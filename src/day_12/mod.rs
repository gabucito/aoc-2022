use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use crate::read::read_file;
type Step = usize;
type Idx = usize;

pub fn run() {
    if let Ok(contents) = read_file("src/day_12/input.txt") {
        // part1(&contents)
        println!("part 1: {}", part1(&contents));
        println!("part 2: {}", part2(&contents));
    }
}

pub fn part1(contents: &str) -> usize {
    let width = contents.lines().next().unwrap().len();
    let mut start = 0;
    let mut end = 0;
    let chars: Vec<char> = contents
        .chars()
        .filter(|&c| c != '\n')
        .enumerate()
        .map(|(idx, c)| match c {
            'S' => {
                start = idx;
                'a'
            }
            'E' => {
                end = idx;
                'z'
            }
            _ => c,
        })
        .collect::<Vec<char>>();
    println!("start: {}, end: {}", start, end);
    let one_matrix = OneDimentionalMatrix::new(width, chars);
    one_matrix.shortest_path(start, end)
}

pub fn part2(contents: &str) -> usize {
    let width = contents.lines().next().unwrap().len();
    let mut starts = vec![];
    let mut end = 0;
    let chars: Vec<char> = contents
        .chars()
        .filter(|&c| c != '\n')
        .enumerate()
        .map(|(idx, c)| match c {
            'S' => {
                starts.push(idx);
                'a'
            }
            'E' => {
                end = idx;
                'z'
            }
            'a' => {
                starts.push(idx);
                c
            }
            _ => c,
        })
        .collect::<Vec<char>>();

    let one_matrix = OneDimentionalMatrix::new(width, chars);
    starts
        .iter()
        .map(|&start| {
            one_matrix.shortest_path(start, end)
        })
        .reduce(
            |selected, next| {
                if next < selected {
                    next
                } else {
                    selected
                }
            },
        ).unwrap()
}

pub struct OneDimentionalMatrix {
    width: usize,
    height: usize,
    array: Vec<char>,
}

impl OneDimentionalMatrix {
    pub fn new(width: usize, array: Vec<char>) -> Self {
        let height = array.len() / width;
        Self {
            width,
            height,
            array,
        }
    }

    pub fn set_value(&mut self, x: usize, y: usize, val: char) {
        if x > self.width {
            panic!("point x is greater than width height");
        }
        if y > self.height {
            panic!("point y is greater than matrix height");
        }
        let idx = (y * x) + x;
        self.array[idx] = val;
    }

    pub fn get_value(&self, x: usize, y: usize) -> char {
        if x >= self.width {
            return '0';
        }
        if y >= self.height {
            return '0';
        }
        let y_add = self.height * y;
        let idx = y_add + x;
        self.array[idx]
    }

    pub fn get_coordinate_from_index(&self, idx: usize) -> (usize, usize) {
        let y = idx / self.height;
        let minus_y = self.height * y;
        let x = idx - minus_y;

        (x, y)
    }

    pub fn get_value_from_index(&self, idx: usize) -> char {
        if idx >= (self.width * self.height) {
            panic!("idx: {}, Index out of range", idx);
        }
        self.array[idx]
    }

    pub fn get_top_index(&self, idx: usize) -> Option<usize> {
        if idx >= self.width {
            Some(idx - self.width)
        } else {
            None
        }
    }

    pub fn get_bottom_index(&self, idx: usize) -> Option<usize> {
        if idx + self.width < self.array.len() {
            Some(idx + self.width)
        } else {
            None
        }
    }

    pub fn get_right_index(&self, idx: usize) -> Option<usize> {
        // If the index is at the right edge return None
        if idx % self.width == self.width - 1 {
            return None;
        }
        Some(idx + 1)
    }

    pub fn get_left_index(&self, idx: usize) -> Option<usize> {
        // If the index is at the left edge return None
        if idx % self.width == 0 {
            return None;
        }
        Some(idx - 1)
    }

    pub fn get_neighbors(&self, current_idx: usize) -> Vec<usize> {
        let mut neighbors = vec![];
        let current_value = self.array[current_idx];

        for idx_option in [
            self.get_left_index(current_idx),
            self.get_right_index(current_idx),
            self.get_top_index(current_idx),
            self.get_bottom_index(current_idx),
        ] {
            if let Some(idx2) = idx_option {
                let value = self.get_value_from_index(idx2);
                match value as i8 - current_value as i8 {
                    (i8::MIN..=1) => neighbors.push(idx2),
                    _ => (),
                }
            }
        }
        neighbors
    }

    pub fn shortest_path(&self, start: usize, end: usize) -> Step {
        let length = self.array.len();
        let mut distances = vec![None; length];
        let mut nodes: VecDeque<Idx> = VecDeque::new();
        nodes.push_back(start);
        distances[start] = Some(0);

        while nodes.len() > 0 {
            let current_index = nodes.pop_front().unwrap();
            let current_value = distances[current_index].unwrap();
            for neighbor in self.get_neighbors(current_index) {
                if distances[neighbor].is_none() {
                    nodes.push_back(neighbor);
                    distances[neighbor] = Some(current_value + 1)
                }
            }
        }
        distances[end].unwrap_or(Step::MAX)
    }
}
