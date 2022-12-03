mod day_1;
mod day_2;
mod day_3;
mod runner;

use anyhow::Result;
use day_1::Day1;
use day_2::Day2;
use day_3::Day3;
use runner::Executor;

fn main() -> Result<()> {
    Day1::run("inputs/day1.input")?;
    Day2::run("inputs/day2.input")?;
    Day3::run("inputs/day3.input")?;
    Ok(())
}
