#![allow(dead_code)]
#![allow(unused_must_use)]
use anyhow::Result;

use crate::runner::{Parse, Run};
use std::collections::{HashMap, HashSet};

pub struct Day3 {}

impl Parse<Vec<String>> for Day3 {
    fn parse_input(input: &str) -> Result<Vec<String>> {
        Ok(input.lines().map(|l| l.to_string()).collect())
    }
}

impl Run<Vec<String>, usize> for Day3 {
    fn part_one(input: &Vec<String>) -> Result<usize> {
        Ok(rucksacks(input))
    }

    fn part_two(input: &Vec<String>) -> Result<usize> {
        Ok(badges(input))
    }
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn get_value_of_char(c: &char) -> usize {
    let base = match c.is_uppercase() {
        true => 27,
        false => 1,
    };
    let ind = ALPHABET.find(&c.to_lowercase().to_string()).unwrap_or(0);
    base + ind
}

fn rucksacks(input: &[String]) -> usize {
    input.iter().map(|r| {
        let half: usize = r.len() / 2;
        let mut items: HashMap<char, bool> = HashMap::with_capacity(half);
        let (first, second) = r.split_at(half);
        first.chars().for_each(|c| {
            items.insert(c, true);
        });
        second.chars().fold(0, |acc, c| {
            match items.get(&c){
                Some(_) => get_value_of_char(&c),
                None => acc,
            }
        })
    }).sum()
}

fn char_in_items(i: usize, c: char, items: &mut HashMap<char, HashSet<usize>>) {
    match items.get_mut(&c){
        Some(it) => {
            it.insert(i);
        }
        None => {
            let it = HashSet::from([i]);
            items.insert(c, it);
        }
    }
}

fn badges(input: &[String]) -> usize {
    input.chunks(3).map(|g| {
        let mut items: HashMap<char, HashSet<usize>> = HashMap::new();
        g.iter().enumerate().for_each(|(i, s)| {
            s.chars().for_each(|c| {
                char_in_items(i, c, &mut items);
            })
        });
        items.iter().fold(0, |acc, (c, i)| {
            match i.len() {
                3 => get_value_of_char(c),
                _ => acc,
            }
        })

    }).sum()
}

#[cfg(test)]
mod tests_day3 {
    use crate::runner::Executor;
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_run() -> Result<()> {
        let (r1, r2) = Day3::run("inputs/day3.test")?;
        assert_eq!(r1, 157);
        assert_eq!(r2, 70);
        Ok(())
    }

    #[test]
    fn test_rucksack() {
        let input = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string()
        ];
        let value = rucksacks(&input);
        assert_eq!(157, value)
    }                           

    #[test]
    fn test_badges() {
        let input = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string()
        ];
        let value = badges(&input);
        assert_eq!(70, value)
    }

    #[test]
    fn test_char_in_items() {
        let mut items: HashMap<char, HashSet<usize>> = HashMap::new();
        // Inserts 'a' with 0 once
        char_in_items(0, 'a', &mut items);
        assert_eq!(items.len(), 1);
        assert_eq!(items.get(&'a').unwrap().len(), 1 );
        // Shoudn't change
        char_in_items(0, 'a', &mut items);
        assert_eq!(items.len(), 1);
        assert_eq!(items.get(&'a').unwrap().len(), 1 );
        // Inserts 'b' with 0
        char_in_items(0, 'b', &mut items);
        assert_eq!(items.len(), 2);
        assert_eq!(items.get(&'a').unwrap().len(), 1 );
        assert_eq!(items.get(&'b').unwrap().len(), 1 );
        // Inserts 'a' with 1
        char_in_items(1, 'a', &mut items);
        assert_eq!(items.len(), 2);
        assert_eq!(items.get(&'a').unwrap().len(), 2 );
        assert_eq!(items.get(&'b').unwrap().len(), 1 );
    }

}
