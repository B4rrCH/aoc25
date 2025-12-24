use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Read, Result as IoResult},
};

pub fn day01(file_path: &str) -> IoResult<()> {
    let file = OpenOptions::new().read(true).open(file_path)?;
    let reader = BufReader::new(file);
    let result = part1_impl(reader);
    println!("Day 01, Part 1: {}", result);

    let file = OpenOptions::new().read(true).open(file_path)?;
    let reader = BufReader::new(file);
    let result = part2_impl(reader);
    println!("Day 01, Part 2: {}", result);

    Result::Ok(())
}

fn part1_impl<T>(reader: BufReader<T>) -> i32
where
    T: Read,
{
    parse_input(reader).fold((0, 50), part1_folder).0
}

fn part2_impl<T>(reader: BufReader<T>) -> i32
where
    T: Read,
{
    parse_input(reader).fold((0, 50), part2_folder).0
}

fn part1_folder(acc: (i32, i32), direction: i32) -> (i32, i32) {
    let (count, position) = acc;
    let new_position = (position + direction).rem_euclid(100);
    (
        count + (if new_position == 0 { 1 } else { 0 }),
        new_position,
    )
}

fn part2_folder(acc: (i32, i32), direction: i32) -> (i32, i32) {
    let (count, position) = acc;
    let new_position = position + direction;
    let new_position_rem = new_position.rem_euclid(100);
    let count_diff = (new_position.div_euclid(100) - position.div_euclid(100)).abs()
        + match (direction < 0, new_position_rem, position) {
            // Go away and land on 0 => difference of divisions works
            (true, 0, 0) => 0,
            // Land on zero exactly from the left => difference of division undercounts
            (true, 0, _) => 1,
            // Go away from zero to the left => difference of division overcounts
            (true, _, 0) => -1,
            _ => 0,
        };
    (count + count_diff, new_position_rem)
}

fn parse_input<T>(
    reader: BufReader<T>,
) -> std::iter::FilterMap<
    std::io::Lines<BufReader<T>>,
    fn(Result<String, std::io::Error>) -> Option<i32>,
>
where
    T: Read,
{
    fn parse_line(line: Result<String, std::io::Error>) -> Option<i32> {
        match line {
            Ok(left) if left.starts_with("L") => left[1..].parse::<i32>().map(|x| -x).ok(),
            Ok(right) if right.starts_with("R") => right[1..].parse::<i32>().ok(),
            _ => Option::None,
        }
    }
    reader.lines().filter_map(parse_line)
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::day01::part1_impl;
    use crate::day01::part2_impl;

    #[test]
    fn part1_example() {
        // Arrange
        let example_data = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ]
        .join("\n");
        let reader = BufReader::new(example_data.as_bytes());

        // Act
        let result = part1_impl(reader);

        // Assert
        assert_eq!(3, result);
    }

    #[test]
    fn part2_example() {
        // Arrange
        let example_data = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ]
        .join("\n");
        let reader = BufReader::new(example_data.as_bytes());

        // Act
        let result = part2_impl(reader);

        // Assert
        assert_eq!(6, result);
    }

    #[test]
    fn part2_lands_on_0() {
        // Arrange
        let example_data = vec!["L100", "L150", "R1"].join("\n");
        let reader = BufReader::new(example_data.as_bytes());

        // Act
        let result = part2_impl(reader);

        // Assert
        assert_eq!(3, result);
    }

    #[test]
    fn part2_lands_on_0_again() {
        // Arrange
        let example_data = vec!["L100", "L150", "R1", "L1"].join("\n");
        let reader = BufReader::new(example_data.as_bytes());

        // Act
        let result = part2_impl(reader);

        // Assert
        assert_eq!(4, result);
    }

    #[test]
    fn part2_lands_on_0_again_then_leaves() {
        // Arrange
        let example_data = vec!["L100", "L150", "R1", "L1", "L1"].join("\n");
        let reader = BufReader::new(example_data.as_bytes());

        // Act
        let result = part2_impl(reader);

        // Assert
        assert_eq!(4, result);
    }
}
