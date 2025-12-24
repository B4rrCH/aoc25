use std::{
    collections::HashSet,
    fs::OpenOptions,
    io::{BufRead, BufReader, Read, Result as IoResult},
};

pub fn day02(file_path: &str) -> IoResult<()> {
    let file = OpenOptions::new().read(true).open(file_path)?;
    let reader = BufReader::new(file);
    let result = part1_impl(reader);
    println!("Day 02, Part 1: {}", result);

    let file = OpenOptions::new().read(true).open(file_path)?;
    let reader = BufReader::new(file);
    let result = part2_impl(reader);
    println!("Day 02, Part 2: {}", result);

    Ok(())
}

fn part1_impl<T>(reader: BufReader<T>) -> u64
where
    T: Read,
{
    let ranges = parse(reader);
    let max = ranges.iter().map(|r| r.end).max().unwrap_or_default();
    let silly_ids = (1u32..)
        .map(concat_digits)
        .take_while(|x| *x <= max)
        .filter(|n| ranges.iter().any(|r| r.contains(*n)));

    silly_ids.sum()
}

fn part2_impl<T>(reader: BufReader<T>) -> u64
where
    T: Read,
{
    let ranges = parse(reader);
    let max = ranges.iter().map(|r| r.end).max().unwrap_or_default();

    let silly_ids: HashSet<u64> = (1u32..)
        .take_while(|i| concat_digits(*i) <= max)
        .flat_map(|i| concat_digits_many(i).take_while(|x| *x <= max))
        .take_while(|x| *x <= max)
        .filter(|n| ranges.iter().any(|r| r.contains(*n)))
        .collect();

    silly_ids.iter().sum()
}

fn concat_digits(i: u32) -> u64 {
    let number_of_digits = i.ilog10() + 1;
    let i: u64 = i.into();
    10u64.pow(number_of_digits) * i + i
}

fn concat_digits_many(
    i: u32,
) -> std::iter::Scan<std::ops::RangeFrom<i32>, u64, impl FnMut(&mut u64, i32) -> Option<u64>> {
    let number_of_digits = i.ilog10() + 1;
    let i: u64 = i.into();
    (0..).scan(i, move |x, _| {
        *x = 10u64.pow(number_of_digits) * (*x) + i;
        Some(*x)
    })
}

fn parse<T>(mut reader: BufReader<T>) -> Vec<IdRange>
where
    T: Read,
{
    let mut res = vec![];

    loop {
        let range = parse_next(&mut reader);
        if range.is_none() {
            return res;
        }
        res.push(range.unwrap());
    }
}

fn parse_next<T>(reader: &mut BufReader<T>) -> Option<IdRange>
where
    T: Read,
{
    let mut buf = vec![];

    let start_length = reader.read_until_before(b'-', &mut buf).ok()?;
    let start = str::from_utf8(&buf[..start_length])
        .ok()?
        .parse::<u64>()
        .ok()?;

    buf.clear();

    let end_length = reader.read_until_before(b',', &mut buf).ok()?;
    let end = str::from_utf8(&buf[..end_length])
        .ok()?
        .parse::<u64>()
        .ok()?;

    Some(IdRange { start, end })
}

trait ReadUntilBefore: BufRead {
    fn read_until_before(&mut self, byte: u8, buf: &mut Vec<u8>) -> IoResult<usize>;
}

impl<T> ReadUntilBefore for T
where
    T: BufRead,
{
    fn read_until_before(&mut self, byte: u8, buf: &mut Vec<u8>) -> IoResult<usize> {
        let read = self.read_until(byte, buf)?;

        Ok(if read > 0 && buf[read - 1] == byte {
            read - 1
        } else {
            read
        })
    }
}

struct IdRange {
    start: u64,
    end: u64,
}

impl IdRange {
    fn contains(&self, number: u64) -> bool {
        self.start <= number && number <= self.end
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::day02::part1_impl;
    use crate::day02::part2_impl;

    #[test]
    fn part1_example() {
        // Arrange
        let input = concat!(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,",
            "1698522-1698528,446443-446449,38593856-38593862,565653-565659,",
            "824824821-824824827,2121212118-2121212124"
        );
        let reader = BufReader::new(input.as_bytes());

        // Act
        let result = part1_impl(reader);

        // Assert
        assert_eq!(1227775554, result);
    }

    #[test]
    fn part2_example() {
        // Arrange
        let input = concat!(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,",
            "1698522-1698528,446443-446449,38593856-38593862,565653-565659,",
            "824824821-824824827,2121212118-2121212124"
        );
        let reader = BufReader::new(input.as_bytes());

        // Act
        let result = part2_impl(reader);

        // Assert
        assert_eq!(4174379265, result);
    }
}
