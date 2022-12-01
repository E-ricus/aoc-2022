mod day_1;
mod runner;

use anyhow::Result;
use day_1::Day1;
use runner::Executor;

fn main() -> Result<()> {
    Day1::run("inputs/day1.input")?;
    Ok(())
}
