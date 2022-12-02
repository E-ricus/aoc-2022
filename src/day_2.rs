#![allow(dead_code)]
#![allow(unused_variables)]
use std::str::FromStr;

use anyhow::{Result, Error};

use crate::runner::{Parse, RunMut};

pub struct Day2 {}

impl Parse<Vec<Game>> for Day2 {
    fn parse_input(input: &str) -> Result<Vec<Game>> {
        Ok(input.lines().filter_map(|s| Game::from_str(s).ok()).collect())
    }
}

impl RunMut<Vec<Game>, usize> for Day2 {
    fn part_one(input: &mut Vec<Game>) -> Result<usize> {
        let mut acc = 0;
        for game in input {
            game.outcome_by_play();
            acc += game.outcome;
        }
        Ok(acc)
    }

    fn part_two(input: &mut Vec<Game>) -> Result<usize> {
        let mut acc = 0;
        for game in input {
            game.outcome_by_desition();
            acc += game.outcome;
        }
        Ok(acc)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Game {
    elf: Play,
    player: Play,
    desition: Desition,
    outcome: usize,
}

impl Game {
    fn new(elf: Play, player: Play, desition: Desition) -> Self {
        Self { elf, player, desition, outcome: 0}
    }

    fn outcome_by_play(&mut self) {
        match (&self.elf, &self.player) {
            (Play::Rock, Play::Rock) => self.outcome = 4,
            (Play::Rock, Play::Paper) => self.outcome = 8,
            (Play::Rock, Play::Scissors) => self.outcome = 3,
            (Play::Paper, Play::Rock) => self.outcome = 1,
            (Play::Paper, Play::Paper) => self.outcome = 5,
            (Play::Paper, Play::Scissors) => self.outcome = 9,
            (Play::Scissors, Play::Rock) => self.outcome = 7,
            (Play::Scissors, Play::Paper) => self.outcome = 2,
            (Play::Scissors, Play::Scissors) => self.outcome = 6,
        }
    }
    
    fn outcome_by_desition(&mut self) {
        match (&self.elf, &self.desition){
            (Play::Rock, Desition::Win) => self.outcome = 8,
            (Play::Rock, Desition::Draw) => self.outcome = 4,
            (Play::Rock, Desition::Lose) => self.outcome = 3,
            (Play::Paper, Desition::Win) => self.outcome = 9,
            (Play::Paper, Desition::Draw) => self.outcome = 5,
            (Play::Paper, Desition::Lose) => self.outcome = 1,
            (Play::Scissors, Desition::Win) => self.outcome = 7,
            (Play::Scissors, Desition::Draw) => self.outcome = 6,
            (Play::Scissors, Desition::Lose) => self.outcome = 2,
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

#[derive(Debug, Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum Desition {
    Win,
    Draw,
    Lose,
}


#[cfg(test)]
mod tests_day1 {
    use crate::runner::MutExecutor;
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
