use anyhow::{Ok, Result};

use crate::runner::{Parse, Run};

pub struct Day1 {}

impl Parse<Vec<Vec<i32>>> for Day1 {
    fn parse_input(input: &str) -> Result<Vec<Vec<i32>>> {
        let mut parsed: Vec<Vec<i32>> = Vec::new();
        let mut elf: Vec<i32> = Vec::new();
        for line in input.lines() {
            if line.is_empty() {
                parsed.push(elf);
                elf = Vec::new();
                continue;
            }
            let cal = line.parse::<i32>().unwrap();
            elf.push(cal);
        }
        parsed.push(elf);
        Ok(parsed)
    }
}
impl Run<Vec<Vec<i32>>, i32> for Day1 {
    fn part_one(input: &Vec<Vec<i32>>) -> Result<i32> {
        Ok(max_calories(input))
    }

    fn part_two(input: &Vec<Vec<i32>>) -> Result<i32> {
        Ok(sum_max_calories(input))
    }
}

fn max_calories(input: &Vec<Vec<i32>>) -> i32 {
    let elf_counts: Vec<i32> = input.into_iter().map(|v| v.into_iter().sum()).collect();
    elf_counts.into_iter().max().unwrap()
}

fn sum_max_calories(input: &Vec<Vec<i32>>) -> i32 {
    let mut elf_counts: Vec<i32> = input.into_iter().map(|v| v.into_iter().sum()).collect();
    elf_counts.sort();
    elf_counts.reverse();
    elf_counts[0..3].into_iter().sum()
}

#[cfg(test)]
mod tests_day1 {
    use super::*;
    use crate::runner::Executor;
    use anyhow::Result;

    #[test]
    fn test_run() -> Result<()> {
        let (r1, r2) = Day1::run("inputs/day1.test")?;
        assert_eq!(r1, 24000);
        assert_eq!(r2, 45000);
        Ok(())
    }

    const INPUT: &str = include_str!("../inputs/day1.test");

    #[test]
    fn test_sum_max_calories() -> Result<()> {
        let input = Day1::parse_input(INPUT)?;
        let count = sum_max_calories(&input);
        assert_eq!(count, 45000);
        Ok(())
    }

    #[test]
    fn test_max_calories() -> Result<()> {
        let input = Day1::parse_input(INPUT)?;
        let count = max_calories(&input);
        assert_eq!(count, 24000);
        Ok(())
    }
}
