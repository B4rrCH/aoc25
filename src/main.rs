mod day01;
mod day02;
mod day03;

use crate::day01::day01;
use crate::day02::day02;
use crate::day03::day03;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day01("input/day01.txt")?;
    day02("input/day02.txt")?;
    day03("input/day03.txt")?;
    Ok(())
}
