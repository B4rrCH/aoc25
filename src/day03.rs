use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Read, Result as IoResult},
};

pub fn day03(file_path: &str) -> IoResult<()> {
    let file = OpenOptions::new().read(true).open(file_path)?;
    let reader = BufReader::new(file);
    let result = part1_impl(reader);
    println!("Day 03, Part 1: {}", result);

    let file = OpenOptions::new().read(true).open(file_path)?;
    let reader = BufReader::new(file);
    let result = part2_impl(reader);
    println!("Day 03, Part 2: {}", result);

    Result::Ok(())
}

fn part1_impl<T>(reader: BufReader<T>) -> u64
where
    T: Read,
{
    parse(reader).filter_map(|x| largest_joltage(&x, 2)).sum()
}

fn part2_impl<T>(reader: BufReader<T>) -> u64
where
    T: Read,
{
    parse(reader).filter_map(|x| largest_joltage(&x, 12)).sum()
}

fn parse<T>(
    reader: BufReader<T>,
) -> std::iter::Map<
    std::iter::FilterMap<
        std::io::Lines<BufReader<T>>,
        impl FnMut(Result<String, std::io::Error>) -> Option<String>,
    >,
    fn(String) -> Vec<u32>,
>
where
    T: Read,
{
    reader.lines().filter_map(IoResult::ok).map(parse_line)
}

fn parse_line(line: String) -> Vec<u32> {
    line.chars().filter_map(|c| char::to_digit(c, 10)).collect()
}

fn largest_joltage(line: &[u32], digits: usize) -> Option<u64> {
    if digits > line.len() {
        return None;
    }

    if digits == 1 {
        return line.iter().map(|x| (*x).into()).max();
    }

    if digits <= 0 {
        return Some(0);
    }

    let digits = digits - 1;

    let (index, first_digit): (usize, u64) = line[..(line.len() - digits)]
        .iter()
        .map(|x| (*x).into())
        .enumerate()
        // Find the max value, with the smallest index
        // Would we not explicitly specify `line.len() - index`, the last occurence would be picked
        // We are using that tuples are ordered lexicographically
        .max_by_key(|(index, value)| (*value, line.len() - *index))?;

    let head: u64 = first_digit * 10_u64.pow(digits.try_into().unwrap());
    let rest = largest_joltage(&line[(index + 1)..], digits)?;
    Some(head + rest)
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::day03::largest_joltage;
    use crate::day03::part1_impl;
    use crate::day03::part2_impl;

    #[test]
    fn part1_example() {
        // Arrange
        let test_input = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ]
        .join("\n");

        let reader = BufReader::new(test_input.as_bytes());

        // Act
        let result = part1_impl(reader);

        // Assert
        assert_eq!(357, result);
    }

    #[test]
    fn part2_example() {
        // Arrange
        let test_input = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ]
        .join("\n");

        let reader = BufReader::new(test_input.as_bytes());

        // Act
        let result = part2_impl(reader);

        // Assert
        assert_eq!(3121910778619, result);
    }
    #[test]
    fn largest_joltage_one_digit() {
        // Act
        let result = largest_joltage(&[1_u32], 1);

        // Assert
        assert_eq!(Some(1_u64), result);
    }

    #[test]
    fn largest_joltage_two_digits_same_digit() {
        // Act
        let result = largest_joltage(&[1_u32, 1_u32], 2);

        // Assert
        assert_eq!(Some(11_u64), result);
    }

    #[test]
    fn largest_joltage_two_digits_no_choice() {
        // Act
        let result = largest_joltage(&[1_u32, 9_u32], 2);

        // Assert
        assert_eq!(Some(19_u64), result);
    }

    #[test]
    fn largest_joltage_two_digits_choice() {
        // Act
        let result = largest_joltage(&[9_u32, 1_u32, 9_u32], 2);

        // Assert
        assert_eq!(Some(99_u64), result);
    }

    #[test]
    fn largest_joltage_part2_example_separated() {
        assert_eq!(
            Some(987654321111_u64),
            largest_joltage(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12)
        );

        assert_eq!(
            Some(811111111119_u64),
            largest_joltage(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 12)
        );

        assert_eq!(
            Some(434234234278_u64),
            largest_joltage(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 12)
        );

        assert_eq!(
            Some(888911112111_u64),
            largest_joltage(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 12)
        );
    }
}
