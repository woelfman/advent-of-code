use adv_code_2025::*;
use anyhow::Result;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

const DAY: &str = "05";
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
    let mut fresh_ids: HashSet<Range<u64>> = HashSet::new();
    let mut iter = reader.lines();

    for line in iter.by_ref() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let parts: Vec<&str> = line.split('-').collect();
        let start: u64 = parts[0].parse()?;
        let end: u64 = parts[1].parse()?;
        fresh_ids.insert(start..end + 1);
    }

    let mut available_fresh_ids = 0;

    for line in iter {
        let id: u64 = line?.parse()?;

        for range in &fresh_ids {
            if range.contains(&id) {
                available_fresh_ids += 1;
                break;
            }
        }
    }

    Ok(available_fresh_ids)
}

fn part2<R: BufRead>(reader: R) -> Result<u64> {
    let mut fresh_ids: HashSet<Range<u64>> = HashSet::new();
    let mut iter = reader.lines();

    for line in iter.by_ref() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let parts: Vec<&str> = line.split('-').collect();
        let start: u64 = parts[0].parse()?;
        let end: u64 = parts[1].parse()?;
        fresh_ids.insert(start..end + 1);
    }

    // Merge overlapping ranges, such that 16-20 and 12-18 become 12-20
    let mut merged_ranges: Vec<Range<u64>> = Vec::new();
    let mut sorted_ranges: Vec<Range<u64>> = fresh_ids.into_iter().collect();
    sorted_ranges.sort_by_key(|r| r.start);

    for range in sorted_ranges {
        if let Some(last) = merged_ranges.last_mut()
            && range.start <= last.end
        {
            last.end = last.end.max(range.end);
            continue;
        }
        merged_ranges.push(range);
    }

    // Count the number of available fresh IDs
    let mut available_fresh_ids = 0;
    for range in merged_ranges {
        available_fresh_ids += range.end - range.start;
    }

    Ok(available_fresh_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(14, part2(BufReader::new(TEST.as_bytes()))?);
        Ok(())
    }
}
