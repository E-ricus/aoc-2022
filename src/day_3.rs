use anyhow::Result;

use crate::runner::Parse;

pub struct Day3 {}

impl Parse<Vec<String>> for Day3 {
    fn parse_input(input: &str) -> Result<Vec<String>> {
        Ok(input.lines().map(|l| l.to_string()).collect())
    }
}
