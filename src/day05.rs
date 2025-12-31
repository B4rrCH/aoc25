use std::{
    cmp::max,
    fs::OpenOptions,
    io::{BufRead, BufReader, Read, Result as IoResult},
};

pub fn day05(file_path: &str) -> IoResult<()> {
    let file = OpenOptions::new().read(true).open(file_path)?;
    let reader = BufReader::new(file);
    let result = part1_impl(reader);
    println!("Day 05, Part 1: {}", result);

    let file = OpenOptions::new().read(true).open(file_path)?;
    let reader = BufReader::new(file);
    let result = part2_impl(reader);
    println!("Day 05, Part 2: {}", result);

    Result::Ok(())
}

fn part1_impl<T>(mut reader: BufReader<T>) -> usize
where
    T: Read,
{
    let ranges = parse_ranges(&mut reader);
    parse_ingredients(reader)
        .filter(|ingredient| ranges.contains(*ingredient))
        .count()
}

fn part2_impl<T>(mut reader: BufReader<T>) -> usize
where
    T: Read,
{
    let ranges = parse_ranges(&mut reader);
    ranges.iter().map(Range::size).sum()
}

fn parse_ingredients<T>(
    reader: BufReader<T>,
) -> std::iter::FilterMap<
    std::io::Lines<BufReader<T>>,
    fn(Result<String, std::io::Error>) -> Option<usize>,
>
where
    T: Read,
{
    fn parse_usize(s: IoResult<String>) -> Option<usize> {
        s.ok()?.parse().ok()
    }
    reader.lines().filter_map(parse_usize)
}

fn parse_ranges<T>(reader: &mut BufReader<T>) -> Vec<Range>
where
    T: Read,
{
    let ranges: Vec<Range> = reader
        .lines()
        .filter_map(IoResult::ok)
        .take_while(|line| !line.is_empty())
        .filter_map(parse_range)
        .collect();

    merge_ranges(ranges)
}

fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    if ranges.len() <= 0 {
        return ranges;
    }

    ranges.sort_unstable();

    ranges.iter().fold(vec![], |mut merged_ranges, range| {
        // The ranges are sorted, so we only need to look at the last one
        match merged_ranges.last_mut() {
            Some(current) if current.end + 1 >= range.start => {
                // if the last range itersects or is adjacent, we can merge them
                current.end = max(current.end, range.end)
            }
            // otherwise, make a new range
            _ => merged_ranges.push(*range),
        };
        merged_ranges
    })
}

fn parse_range(s: String) -> Option<Range> {
    let dash = s.find('-')?;

    Some(Range {
        start: s[..dash].parse().ok()?,
        end: s[(dash + 1)..].parse().ok()?,
    })
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn contains(&self, ingredient: usize) -> bool {
        self.start <= ingredient && ingredient <= self.end
    }

    fn size(&self) -> usize {
        self.end - self.start + 1
    }
}

trait Ranges {
    fn contains(&self, ingredient: usize) -> bool;
}

impl Ranges for Vec<Range> {
    fn contains(&self, ingredient: usize) -> bool {
        let insertion_index = self.binary_search_by_key(&ingredient, |r| r.start);
        match insertion_index {
            Ok(i) if self[i].contains(ingredient) => true,
            Ok(i) if 0 < i && self[i - 1].contains(ingredient) => true,
            Err(i) if i < self.len() && self[i].contains(ingredient) => true,
            Err(i) if i > 0 && self[i - 1].contains(ingredient) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::day05::Range;
    use crate::day05::parse_range;
    use crate::day05::part1_impl;
    use crate::day05::part2_impl;

    use crate::day05::Ranges;

    #[test]
    fn part1_example() {
        // Arrange
        let input = vec![
            "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
        ]
        .join("\n");

        // Act
        let res = part1_impl(BufReader::new(input.as_bytes()));

        // Assert
        assert_eq!(3, res)
    }

    #[test]
    fn part2_example() {
        // Arrange
        let input = vec![
            "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
        ]
        .join("\n");

        // Act
        let res = part2_impl(BufReader::new(input.as_bytes()));

        // Assert
        assert_eq!(14, res)
    }

    #[test]
    fn parse_range_example() {
        let result = parse_range("123-456".to_string());

        assert_eq!(
            Some(Range {
                start: 123,
                end: 456
            }),
            result
        )
    }

    #[test]
    fn range_contains() {
        // Arrange
        let range = Range { start: 1, end: 2 };

        // Act / Assert
        assert!(!range.contains(0));
        assert!(range.contains(1));
        assert!(range.contains(2));
        assert!(!range.contains(3));
    }

    #[test]
    fn vec_ranges_contains_single() {
        // Arrange
        let ranges = vec![Range { start: 1, end: 2 }];

        // Act / Assert
        assert!(!ranges.contains(0));
        assert!(ranges.contains(1));
        assert!(ranges.contains(2));
        assert!(!ranges.contains(3));
    }

    #[test]
    fn vec_ranges_contains_two() {
        // Arrange
        let ranges = vec![Range { start: 1, end: 2 }, Range { start: 4, end: 5 }];

        // Act / Assert
        assert!(!ranges.contains(0));
        assert!(ranges.contains(1));
        assert!(ranges.contains(2));
        assert!(!ranges.contains(3));
        assert!(ranges.contains(4));
        assert!(ranges.contains(5));
        assert!(!ranges.contains(6));
    }
}
