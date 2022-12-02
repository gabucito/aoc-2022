use std::str::FromStr;

use crate::read::read_file;

enum GameResult {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl FromStr for GameResult {
    type Err = ();

    fn from_str(input: &str) -> Result<GameResult, Self::Err> {
        match input {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Ok(GameResult::Lose),
        }
    }
}

impl GameResult {
    pub fn results(&self, left: &RockPaperScissor) -> RockPaperScissor {
        match (self, left) {
            (GameResult::Win, RockPaperScissor::Rock) => RockPaperScissor::Paper,
            (GameResult::Win, RockPaperScissor::Paper) => RockPaperScissor::Scissor,
            (GameResult::Win, RockPaperScissor::Scissor) => RockPaperScissor::Rock,
            (GameResult::Draw, RockPaperScissor::Rock) => RockPaperScissor::Rock,
            (GameResult::Draw, RockPaperScissor::Paper) => RockPaperScissor::Paper,
            (GameResult::Draw, RockPaperScissor::Scissor) => RockPaperScissor::Scissor,
            (GameResult::Lose, RockPaperScissor::Rock) => RockPaperScissor::Scissor,
            (GameResult::Lose, RockPaperScissor::Paper) => RockPaperScissor::Rock,
            (GameResult::Lose, RockPaperScissor::Scissor) => RockPaperScissor::Paper,
        }
    }
}

enum RockPaperScissor {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl RockPaperScissor {
    pub fn results(left: &RockPaperScissor, right: &RockPaperScissor) -> GameResult {
        match (right, left) {
            (RockPaperScissor::Rock, RockPaperScissor::Rock) => GameResult::Draw,
            (RockPaperScissor::Rock, RockPaperScissor::Paper) => GameResult::Lose,
            (RockPaperScissor::Rock, RockPaperScissor::Scissor) => GameResult::Win,
            (RockPaperScissor::Paper, RockPaperScissor::Rock) => GameResult::Win,
            (RockPaperScissor::Paper, RockPaperScissor::Paper) => GameResult::Draw,
            (RockPaperScissor::Paper, RockPaperScissor::Scissor) => GameResult::Lose,
            (RockPaperScissor::Scissor, RockPaperScissor::Rock) => GameResult::Lose,
            (RockPaperScissor::Scissor, RockPaperScissor::Paper) => GameResult::Win,
            (RockPaperScissor::Scissor, RockPaperScissor::Scissor) => GameResult::Draw,
        }
    }

    pub fn calculate(left: RockPaperScissor, right: RockPaperScissor) -> i32 {
        let result = RockPaperScissor::results(&left, &right);
        result as i32 + right as i32
    }

    pub fn calculate2(left: RockPaperScissor, result: GameResult) -> i32 {
        let right = result.results(&left);
        result as i32 + right as i32
    }
}

impl FromStr for RockPaperScissor {
    type Err = ();

    fn from_str(input: &str) -> Result<RockPaperScissor, Self::Err> {
        match input {
            "A" => Ok(RockPaperScissor::Rock),
            "B" => Ok(RockPaperScissor::Paper),
            "C" => Ok(RockPaperScissor::Scissor),
            "X" => Ok(RockPaperScissor::Rock),
            "Y" => Ok(RockPaperScissor::Paper),
            "Z" => Ok(RockPaperScissor::Scissor),
            _ => Ok(RockPaperScissor::Rock),
        }
    }
}

pub fn run() {
    if let Ok(contents) = read_file("src/day_02/input.txt") {
        println!("part 1: {}", part1(&contents));
        println!("part 2: {}", part2(&contents));
    }
}

//
pub fn part1(contents: &str) -> i32 {
    contents.lines().fold(0, |total, line| {
        let mut iter = line.split_whitespace();
        let leftstr = iter.next().unwrap();
        let rightstr = iter.next().unwrap();
        let left = RockPaperScissor::from_str(leftstr).unwrap();
        let right = RockPaperScissor::from_str(rightstr).unwrap();
        let match_point = RockPaperScissor::calculate(left, right);
        total + match_point
    })
}

//
pub fn part2(contents: &str) -> i32 {
    contents.lines().fold(0, |total, line| {
        let mut iter = line.split_whitespace();
        let leftstr = iter.next().unwrap();
        let rightstr = iter.next().unwrap();
        let left = RockPaperScissor::from_str(leftstr).unwrap();
        let right = GameResult::from_str(rightstr).unwrap();
        let match_point = RockPaperScissor::calculate2(left, right);
        total + match_point
    })
}
