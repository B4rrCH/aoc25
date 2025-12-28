use std::{
    cmp::{max, min},
    fs::OpenOptions,
    io::{BufRead, BufReader, Read, Result as IoResult},
};

pub fn day04(file_path: &str) -> IoResult<()> {
    let file = OpenOptions::new().read(true).open(file_path)?;
    let reader = BufReader::new(file);
    let result = part1_impl(reader);
    println!("Day 04, Part 1: {}", result);

    let file = OpenOptions::new().read(true).open(file_path)?;
    let reader = BufReader::new(file);
    let result = part2_impl(reader);
    println!("Day 04, Part 2: {}", result);

    Result::Ok(())
}

fn part1_impl<T>(reader: BufReader<T>) -> usize
where
    T: Read,
{
    let mut map = parse(reader);

    remove_accessible(&mut map)
}

fn part2_impl<T>(reader: BufReader<T>) -> usize
where
    T: Read,
{
    let mut map = parse(reader);
    let mut removed_total: usize = 0;

    loop {
        let removed = remove_accessible(&mut map);
        if removed > 0 {
            removed_total += removed;
        } else {
            return removed_total;
        }
    }
}

fn parse<T>(reader: BufReader<T>) -> Vec<Vec<bool>>
where
    T: Read,
{
    reader
        .lines()
        .flat_map(|line| line.ok())
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}

fn remove_accessible(map: &mut Vec<Vec<bool>>) -> usize {
    let mut accessible = vec![];
    for (i, line) in map.iter().enumerate() {
        for (j, &c) in line.iter().enumerate() {
            if !c {
                continue;
            }

            // We already subtract the roll at [i][j]
            let mut neighbours = -1;
            for x in max(i, 1) - 1..min(i + 2, map.len()) {
                for y in max(j, 1) - 1..min(j + 2, line.len()) {
                    if  map[x][y]{
                        neighbours += 1;
                    }
                }
            }

            if neighbours < 4 {
                accessible.push((i, j));
            }
        }
    }

    let res = accessible.len();

    for (i, j) in accessible {
        map[i][j] = false;
    }

    res
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::day04::part1_impl;
    use crate::day04::part2_impl;

    #[test]
    fn part1_example() {
        // Arrange
        let input = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ]
        .join("\n");

        // Act
        let result = part1_impl(BufReader::new(input.as_bytes()));

        // Assert
        assert_eq!(13, result);
    }
    #[test]
    fn part2_example() {
        // Arrange
        let input = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ]
        .join("\n");

        // Act
        let result = part2_impl(BufReader::new(input.as_bytes()));

        // Assert
        assert_eq!(43, result);
    }
}
