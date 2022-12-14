use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_14/test.txt") {
        // part1(&contents)
        println!("part 1: {}", part1(&contents));
        println!("part 2: {}", part2(&contents));
    }
}

pub fn part1(contents: &str) -> usize {
    let mut matrix = Grid::new();
    let mut lowest_point = 0;
    let coordinates: Vec<Vec<Coordinate>> = contents
        .lines()
        .map(|line| line.split(" -> "))
        .map(|mut coordinates| {
            let mut result = vec![];
            while let Some(coordinate) = coordinates.next() {
                let (left, right) = coordinate.split_once(",").unwrap();
                let x = left.parse::<usize>().unwrap();
                let y = right.parse::<usize>().unwrap();
                if y > lowest_point { lowest_point = y }
                result.push(Coordinate {
                    x,
                    y,
                });
            }
            return result;
        })
        .map(|coordinates| {
            let mut prev_coordinate: Option<Coordinate> = None;
            for coordinate in coordinates {
                match prev_coordinate {
                    Some(prev) => {
                        let dir_x = coordinate.x.cmp(&prev.x);
                        let dir_y = coordinate.y.cmp(&prev.y);
                        let direction = match (dir_x, dir_y) {
                            (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => Direction::Left,
                            (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => Direction::Right,
                            (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => Direction::Up,
                            (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => Direction::Down,
                            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => unreachable!("Diagonal Up Left"),
                            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => unreachable!("Diagonal Down Left"),
                            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => unreachable!("Diagonal Up Right"),
                            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => unreachable!("Diagonal Down Right"),
                            (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => unreachable!("Same coordinate!!!"),
                        };

                        match direction {
                            Direction::Up => {
                                let diff = coordinate.y.abs_diff(prev.y);
                                for y in 0..=diff {
                                    matrix.0[prev.y-y][prev.x] = '#';
                                }
                            },
                            Direction::Left => {
                                let diff = coordinate.x.abs_diff(prev.x);
                                for x in 0..=diff {
                                    matrix.0[prev.y][prev.x-x] = '#';
                                }
                            },
                            Direction::Right => {
                                let diff = coordinate.x.abs_diff(prev.x);
                                for x in 0..=diff {
                                    matrix.0[prev.y][prev.x+x] = '#';
                                }
                            },
                            Direction::Down => {
                                let diff = coordinate.y.abs_diff(prev.y);
                                for y in 0..=diff {
                                    matrix.0[prev.y+y][prev.x] = '#';
                                }
                            },
                            _ => ()
                        }
                    },
                    None => (),
                };
                
                prev_coordinate = Some(coordinate)
            }
            vec![]
        })
        .collect();

        matrix.get_result(lowest_point)
        // matrix.print();
}

pub fn part2(contents: &str) -> usize {
    let mut matrix = Grid::new();
    let mut lowest_point = 0;
    let coordinates: Vec<Vec<Coordinate>> = contents
        .lines()
        .map(|line| line.split(" -> "))
        .map(|mut coordinates| {
            let mut result = vec![];
            while let Some(coordinate) = coordinates.next() {
                let (left, right) = coordinate.split_once(",").unwrap();
                let x = left.parse::<usize>().unwrap();
                let y = right.parse::<usize>().unwrap();
                if y > lowest_point { lowest_point = y }
                result.push(Coordinate {
                    x,
                    y,
                });
            }
            return result;
        })
        .map(|coordinates| {
            let mut prev_coordinate: Option<Coordinate> = None;
            for coordinate in coordinates {
                match prev_coordinate {
                    Some(prev) => {
                        let dir_x = coordinate.x.cmp(&prev.x);
                        let dir_y = coordinate.y.cmp(&prev.y);
                        let direction = match (dir_x, dir_y) {
                            (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => Direction::Left,
                            (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => Direction::Right,
                            (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => Direction::Up,
                            (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => Direction::Down,
                            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => unreachable!("Diagonal Up Left"),
                            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => unreachable!("Diagonal Down Left"),
                            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => unreachable!("Diagonal Up Right"),
                            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => unreachable!("Diagonal Down Right"),
                            (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => unreachable!("Same coordinate!!!"),
                        };

                        match direction {
                            Direction::Up => {
                                let diff = coordinate.y.abs_diff(prev.y);
                                for y in 0..=diff {
                                    matrix.0[prev.y-y][prev.x] = '#';
                                }
                            },
                            Direction::Left => {
                                let diff = coordinate.x.abs_diff(prev.x);
                                for x in 0..=diff {
                                    matrix.0[prev.y][prev.x-x] = '#';
                                }
                            },
                            Direction::Right => {
                                let diff = coordinate.x.abs_diff(prev.x);
                                for x in 0..=diff {
                                    matrix.0[prev.y][prev.x+x] = '#';
                                }
                            },
                            Direction::Down => {
                                let diff = coordinate.y.abs_diff(prev.y);
                                for y in 0..=diff {
                                    matrix.0[prev.y+y][prev.x] = '#';
                                }
                            },
                            _ => ()
                        }
                    },
                    None => (),
                };
                
                prev_coordinate = Some(coordinate)
            }
            vec![]
        })
        .collect();

        for x in 0..800 {
            matrix.0[lowest_point+2][x] = '#';
        }
        matrix.get_result(lowest_point+2)
}

#[derive(Debug)]
pub struct Grid([[char; 800]; 800]);

impl Grid {
    pub fn new() -> Self {
        Self([['.'; 800];800])
    }

    pub fn get_value(&self, x: usize, y: usize) -> char {
        self.0[y][x]
    }

    pub fn print(&self) {
        for y in 0..30{
            for x in 490..520 {
                print!("{}", self.0[y][x]);
            }
            println!("");
        }        
    }

    pub fn drop_sand(&mut self, lowest_point: usize) -> Option<bool> {
        let mut coor = Coordinate {
            x: 500,
            y: 0
        };
        let mut sand = Sand::Down;
        let mut count = 0;
        while sand != Sand::Rest && count < lowest_point {
            let down = self.get_value(coor.x, coor.y+1);
            let down_left = self.get_value(coor.x-1, coor.y+1);
            let down_right = self.get_value(coor.x+1, coor.y+1);

            match (down, down_left, down_right) {
                ('.', _, _) => {
                    coor.y+=1;
                },
                ('#' | 'o', '.', _) => {
                    coor.y+=1;
                    coor.x-=1;
                },
                ('#' | 'o', '#' | 'o', '.') => {
                    coor.y+=1;
                    coor.x+=1;
                },
                ('#' | 'o', '#' | 'o', '#' | 'o') => {
                    self.0[coor.y][coor.x] = 'o';
                    sand = Sand::Rest;
                },
                _ => unreachable!("Should not be reached")
            }
            count+=1;
        }
        Some(true)
    }

    pub fn get_result(&mut self, lowest_point: usize) ->usize {
        let mut count = 0;
        let mut prev_state = self.0;
        while let Some(_) = self.drop_sand(lowest_point) {
            count += 1;
            // self.print();
            if prev_state == self.0 {
                return count-1;
            };
            prev_state = self.0;
        }
        count
    }
}

#[derive(Debug)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
    Level,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Sand {
    Down,
    DownLeft,
    DownRight,
    Rest,
}

#[cfg(test)]
mod tests {
    #[test]
    fn check() {
        let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
    }
}
