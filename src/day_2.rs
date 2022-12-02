use std::str::FromStr;

use anyhow::{Result, Error};

use crate::runner::{Parse, Run};

pub struct Day2 {}

impl Parse<Vec<Game>> for Day2 {
    fn parse_input(input: &str) -> Result<Vec<Game>> {
        Ok(input.lines().filter_map(|s| Game::from_str(s).ok()).collect())
    }
}

impl Run<Vec<Game>, usize> for Day2 {
    fn part_one(input: &Vec<Game>) -> Result<usize> {
        Ok(input.iter().map(|g| g.outcome_by_play()).sum())
    }

    fn part_two(input: &Vec<Game>) -> Result<usize> {
        Ok(input.iter().map(|g| g.outcome_by_desition()).sum())
    }
}

#[derive(Debug)]
pub struct Game {
    elf: Play,
    player: Play,
    desition: Desition,
}

impl Game {
    fn new(elf: Play, player: Play, desition: Desition) -> Self {
        Self { elf, player, desition}
    }

    fn outcome_by_play(&self) -> usize {
        match (&self.elf, &self.player) {
            (Play::Rock, Play::Rock) =>  4,
            (Play::Rock, Play::Paper) =>  8,
            (Play::Rock, Play::Scissors) => 3,
            (Play::Paper, Play::Rock) => 1,
            (Play::Paper, Play::Paper) => 5,
            (Play::Paper, Play::Scissors) => 9,
            (Play::Scissors, Play::Rock) => 7,
            (Play::Scissors, Play::Paper) => 2,
            (Play::Scissors, Play::Scissors) => 6,
        }
    }
    
    fn outcome_by_desition(&self) -> usize {
        match (&self.elf, &self.desition){
            (Play::Rock, Desition::Win) => 8,
            (Play::Rock, Desition::Draw) => 4,
            (Play::Rock, Desition::Lose) => 3,
            (Play::Paper, Desition::Win) => 9,
            (Play::Paper, Desition::Draw) => 5,
            (Play::Paper, Desition::Lose) => 1,
            (Play::Scissors, Desition::Win) => 7,
            (Play::Scissors, Desition::Draw) => 6,
            (Play::Scissors, Desition::Lose) => 2,
        }
    }
}

impl FromStr for Game{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split_once(' ') {
            Some(("A", "X")) => Ok(Game::new(Play::Rock, Play::Rock, Desition::Lose)),
            Some(("A", "Y")) => Ok(Game::new(Play::Rock, Play::Paper, Desition::Draw)),
            Some(("A", "Z")) => Ok(Game::new(Play::Rock, Play::Scissors, Desition::Win)),
            Some(("B", "X")) => Ok(Game::new(Play::Paper, Play::Rock, Desition::Lose)),
            Some(("B", "Y")) => Ok(Game::new(Play::Paper, Play::Paper, Desition::Draw)),
            Some(("B", "Z")) => Ok(Game::new(Play::Paper, Play::Scissors, Desition::Win)),
            Some(("C", "X")) => Ok(Game::new(Play::Scissors, Play::Rock, Desition::Lose)),
            Some(("C", "Y")) => Ok(Game::new(Play::Scissors, Play::Paper, Desition::Draw)),
            Some(("C", "Z")) => Ok(Game::new(Play::Scissors, Play::Scissors, Desition::Win)),
            _ => Err(anyhow::format_err!("Invalid input")),
        }
    }
}

#[derive(Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Desition {
    Win,
    Draw,
    Lose,
}


#[cfg(test)]
mod tests_day1 {
    use crate::runner::Executor;
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_run() -> Result<()> {
        let (r1, r2) = Day2::run("inputs/day2.test")?;
        assert_eq!(r1, 15);
        assert_eq!(r2, 12);
        Ok(())
    }

}
