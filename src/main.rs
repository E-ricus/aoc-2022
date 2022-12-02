mod day_1;
mod day_2;
mod runner;

use anyhow::Result;
use day_1::Day1;
use day_2::Day2;
use runner::{Executor, MutExecutor};

fn main() -> Result<()> {
    Day1::run("inputs/day1.input")?;
    Day2::run("inputs/day2.input")?;
    Ok(())
}
