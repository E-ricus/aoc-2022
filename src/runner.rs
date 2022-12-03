use std::fmt::Display;

use anyhow::Result;

// Parsers to parse the input into generic types
pub trait Parse<I> {
    fn parse_input(input: &str) -> Result<I>;
}
pub trait ParseWithLifeTime<'a, I> {
    fn parse_input(input: &'a str) -> Result<I>;
}

pub trait Parser<I> {
    fn parse_input_part_one(input: &str) -> Result<I>;
    fn parse_input_part_two(input: &str) -> Result<I>;
}

impl<I,T> Parser<I> for T 
where T: Parse<I> {
    fn parse_input_part_one(input: &str) -> Result<I> {
        <T as Parse<I>>::parse_input(input)
    }

    fn parse_input_part_two(input: &str) -> Result<I> {
        <T as Parse<I>>::parse_input(input)
    }
}

pub trait Run<I, R> {
    fn part_one(input: &I) -> Result<R>;
    fn part_two(input: &I) -> Result<R>;
}
pub trait RunMut<I, R> {
    fn part_one(input: &mut I) -> Result<R>;
    fn part_two(input: &mut I) -> Result<R>;
}

// There's gotta be better way to implement this, than to have a trait per implementation

pub trait Executor<I, R, T>
where
    T: Run<I, R> + Parser<I>,
    R: Display,
{
    fn run(path: &str) -> Result<(R, R)>;
}

pub trait MutExecutor<I, R, T>
where
    T: RunMut<I, R> + Parser<I>,
    R: Display,
{
    fn run(path: &str) -> Result<(R, R)>;
}

impl<I, R, T> Executor<I, R, T> for T
where
    T: Run<I, R> + Parser<I>,
    R: Display,
{
    fn run(path: &str) -> Result<(R, R)> {
        let input = std::fs::read_to_string(path)?;
        let input_one = <T as Parser<I>>::parse_input_part_one(input.as_str())?;
        let input_two = <T as Parser<I>>::parse_input_part_two(input.as_str())?;
        let r1 = <T as Run<I, R>>::part_one(&input_one)?;
        println!("{} part 1: {}", path, r1);
        let r2 = <T as Run<I, R>>::part_two(&input_two)?;
        println!("{} part 2: {}", path, r2);
        Ok((r1, r2))
    }
}

impl<I, R, T> MutExecutor<I, R, T> for T
where
    T: RunMut<I, R> + Parse<I>,
    R: Display,
{
    fn run(path: &str) -> Result<(R, R)> {
        let input = std::fs::read_to_string(path)?;
        let mut input_one = <T as Parser<I>>::parse_input_part_one(input.as_str())?;
        let mut input_two = <T as Parser<I>>::parse_input_part_two(input.as_str())?;
        let r1 = <T as RunMut<I, R>>::part_one(&mut input_one)?;
        println!("{} part 1: {}", path, r1);
        let r2 = <T as RunMut<I, R>>::part_two(&mut input_two)?;
        println!("{} part 2: {}", path, r2);
        Ok((r1, r2))
    }
}
