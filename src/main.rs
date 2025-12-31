mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

use std::time::Instant;

use crate::day01::day01;
use crate::day02::day02;
use crate::day03::day03;
use crate::day04::day04;
use crate::day05::day05;

fn main() -> std::io::Result<()> {
    time(day01, "input/day01.txt")?;
    time(day02, "input/day02.txt")?;
    time(day03, "input/day03.txt")?;
    time(day04, "input/day04.txt")?;
    time(day05, "input/day05.txt")?;
    Ok(())
}

fn time<F>(f: F, input: &str) -> std::io::Result<()>
where
    F: Fn(&str) -> std::io::Result<()>,
{
    let start = Instant::now();

    f(input)?;

    let end = Instant::now();

    let passed = end - start;

    println!("\tin {:?}", passed);

    Ok(())
}
