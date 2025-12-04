use adv_code_2025::*;
use anyhow::Result;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    // region Part 2
    println!("\n=== Part 2 ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}

fn part1<R: BufRead>(reader: R) -> Result<u64> {
    let mut at_positions = vec![];

    // Read the grid from input
    // Find all the "@" positions
    for (y, line) in reader.lines().enumerate() {
        let row = line?;
        if row.is_empty() {
            break;
        }

        for (x, ch) in row.chars().enumerate() {
            if ch == '@' {
                at_positions.push((x as isize, y as isize));
            }
        }
    }

    // Count the number of "@" positions that have fewer than 4 adjacent "@" positions
    let mut rolls = 0;
    for &(x, y) in &at_positions {
        let mut adjacent_count = 0;
        for &(dx, dy) in &[
            (-1, 1),
            (0, 1),
            (1, 1),
            (-1, 0),
            (1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ] {
            if at_positions.contains(&(x + dx, y + dy)) {
                adjacent_count += 1;
            }
        }

        if adjacent_count < 4 {
            rolls += 1;
        }
    }

    Ok(rolls)
}

fn part2<R: BufRead>(reader: R) -> Result<u64> {
    let mut at_positions = vec![];

    // Read the grid from input
    // Find all the "@" positions
    for (y, line) in reader.lines().enumerate() {
        let row = line?;
        if row.is_empty() {
            break;
        }

        for (x, ch) in row.chars().enumerate() {
            if ch == '@' {
                at_positions.push((x as isize, y as isize));
            }
        }
    }

    // Counter the number of "@" positions that have fewer than 4 adjacent "@" positions. If the position is counted then remove it from the list. Iterate over the list until no more positions can be removed.
    let mut rolls = 0;
    loop {
        let mut to_remove = vec![];
        for &(x, y) in &at_positions {
            let mut adjacent_count = 0;
            for &(dx, dy) in &[
                (-1, 1),
                (0, 1),
                (1, 1),
                (-1, 0),
                (1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ] {
                if at_positions.contains(&(x + dx, y + dy)) {
                    adjacent_count += 1;
                }
            }

            if adjacent_count < 4 {
                to_remove.push((x, y));
            }
        }

        if to_remove.is_empty() {
            break;
        }

        rolls += to_remove.len() as u64;
        at_positions.retain(|pos| !to_remove.contains(pos));
    }

    Ok(rolls)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);
        Ok(())
    }
}
