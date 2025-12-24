mod day01;
mod day02;

use crate::day01::day01;
use crate::day02::day02;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day01("input/day01.txt")?;
    day02("input/day02.txt")?;
    Ok(())
}
