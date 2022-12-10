use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_08/input.txt") {
        part2(&contents)
        // println!("part 1: {}", part1(&contents));
        // println!("part 2: {}", part2(&contents));
    }
}

pub fn part1(contents: &str) {
    let lines: Vec<&str> = contents.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let array = contents
        .replace('\n', "")
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let matrix = OneDimentionalMatrix {
        width,
        height,
        array,
    };

    let visibles: Vec<_> = matrix
        .array
        .iter()
        .enumerate()
        .filter(|(idx, _)| matrix.is_visible(*idx))
        .collect();

    println!("{}", visibles.len());
    // for visible in &visibles {
    //     println!("{:?}", visible)
    // }

    // println!("{}", matrix.get_value_from_index(9801));
    // matrix.test()
}

pub fn part2(contents: &str) {
    let lines: Vec<&str> = contents.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let array = contents
        .replace('\n', "")
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let matrix = OneDimentionalMatrix {
        width,
        height,
        array,
    };

    let score = matrix
        .array
        .iter()
        .enumerate()
        .fold(0, |total, (idx, _)| {
            let score = matrix.get_score(idx);
            if score > total {
                score
            } else {
                total
            }
        });

    println!("{}", score);
}

pub struct OneDimentionalMatrix {
    width: usize,
    height: usize,
    array: Vec<u8>,
}

impl OneDimentionalMatrix {
    pub fn new(width: usize, height: usize, array: Vec<u8>) -> Self {
        Self {
            width,
            height,
            array,
        }
    }

    pub fn set_value(&mut self, x: usize, y: usize, val: u8) {
        if x > self.width {
            panic!("point x is greater than width height");
        }
        if y > self.height {
            panic!("point y is greater than matrix height");
        }
        let idx = (y * x) + x;
        self.array[idx] = val;
    }

    pub fn get_value(&self, x: usize, y: usize) -> u8 {
        if x >= self.width {
            return 0;
        }
        if y >= self.height {
            return 0;
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

    pub fn get_value_from_index(&self, idx: usize) -> u8 {
        if idx >= (self.width * self.height) {
            panic!("idx: {}, Index out of range", idx);
        }
        self.array[idx]
    }

    pub fn is_visible(&self, idx: usize) -> bool {
        let mut top_index_option = self.get_top_index(idx);
        let mut right_index_option = self.get_right_index(idx);
        let mut bottom_index_option = self.get_bottom_index(idx);
        let mut left_index_option = self.get_left_index(idx);
        let origin_value = self.get_value_from_index(idx);

        let mut visible_from_left = true;
        let mut visible_from_right = true;
        let mut visible_from_top = true;
        let mut visible_from_bottom = true;

        while let Some(index) = top_index_option {
            let value = self.get_value_from_index(index);
            if value >= origin_value {
                visible_from_top = false;
                break;
            }
            top_index_option = self.get_top_index(index);
        }

        while let Some(index) = right_index_option {
            let value = self.get_value_from_index(index);
            if value >= origin_value {
                visible_from_right = false;
                break;
            }
            right_index_option = self.get_right_index(index);
        }

        while let Some(index) = bottom_index_option {
            let value = self.get_value_from_index(index);
            if value >= origin_value {
                visible_from_bottom = false;
                break;
            }
            bottom_index_option = self.get_bottom_index(index);
        }

        while let Some(index) = left_index_option {
            let value = self.get_value_from_index(index);
            if value >= origin_value {
                visible_from_left = false;
                break;
            }
            left_index_option = self.get_left_index(index);
        }

        visible_from_left || visible_from_right || visible_from_top || visible_from_bottom
    }

    pub fn get_score(&self, idx: usize) -> u32 {
        let mut top_index_option = self.get_top_index(idx);
        let mut right_index_option = self.get_right_index(idx);
        let mut bottom_index_option = self.get_bottom_index(idx);
        let mut left_index_option = self.get_left_index(idx);
        let origin_value = self.get_value_from_index(idx);

        let mut score_from_left = 0;
        let mut score_from_right = 0;
        let mut score_from_top = 0;
        let mut score_from_bottom = 0;

        while let Some(index) = top_index_option {
            let value = self.get_value_from_index(index);
            score_from_top += 1;
            if value >= origin_value {
                break;
            }
            top_index_option = self.get_top_index(index);
        }

        while let Some(index) = right_index_option {
            let value = self.get_value_from_index(index);
            score_from_right += 1;
            if value >= origin_value {
                break;
            }
            right_index_option = self.get_right_index(index);
        }

        while let Some(index) = bottom_index_option {
            let value = self.get_value_from_index(index);
            score_from_bottom += 1;
            if value >= origin_value {
                break;
            }
            bottom_index_option = self.get_bottom_index(index);
        }

        while let Some(index) = left_index_option {
            let value = self.get_value_from_index(index);
            score_from_left += 1;
            if value >= origin_value {
                break;
            }
            left_index_option = self.get_left_index(index);
        }

        score_from_top * score_from_right * score_from_bottom * score_from_left
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
}
