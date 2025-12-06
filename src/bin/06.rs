use adv_code_2025::*;
use anyhow::Result;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
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
    let mut problems: Vec<Vec<String>> = vec![];

    for line in reader.lines() {
        let line = line?;
        for (i, field) in line.split_whitespace().enumerate() {
            if problems.len() <= i {
                problems.push(vec![]);
            }
            problems[i].push(field.to_string());
        }
    }

    let mut sum = 0;
    for problem in &mut problems {
        let operation = problem.pop().unwrap();
        let numbers: Vec<u64> = problem.iter().map(|s| s.parse().unwrap()).collect();
        let result: u64 = match operation.as_str() {
            "*" => numbers.iter().product(),
            "+" => numbers.iter().sum(),
            _ => panic!("Unknown operation: {}", operation),
        };
        sum += result;
    }

    Ok(sum)
}

fn part2<R: BufRead>(reader: R) -> Result<u64> {
    // Find the column width by reading the last line and getting the space between the characters
    let rows: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let start_index: Vec<usize> = rows
        .iter()
        .last()
        .unwrap()
        .char_indices()
        .filter(|(_i, c)| *c != ' ')
        .map(|(i, _c)| i)
        .collect();
    let last = rows.last().unwrap().len();
    let end_index: Vec<usize> = start_index
        .iter()
        .skip(1)
        .map(|a| a - 1)
        .chain(std::iter::once(last))
        .collect();
    let column_ranges = start_index
        .iter()
        .zip(end_index.iter())
        .map(|(start, end)| *start..*end)
        .collect::<Vec<std::ops::Range<usize>>>();

    // Extract columns into problems
    let mut problems = column_ranges
        .iter()
        .map(|range| {
            rows.iter()
                .map(|line| line[range.clone()].to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    // Extract the operation and numbers, compute result
    let mut sum = 0;
    for problem in &mut problems {
        let operation = problem.pop().unwrap().trim().chars().next().unwrap();
        let mut numbers: Vec<String> = vec![];
        for num in problem.iter() {
            for (i, c) in num.chars().rev().enumerate() {
                if numbers.len() <= i {
                    numbers.push(String::new());
                }
                if c != ' ' {
                    numbers[i].push(c);
                }
            }
        }
        let numbers: Vec<u64> = numbers.into_iter().map(|s| s.parse().unwrap()).collect();
        let result: u64 = match operation {
            '*' => numbers.iter().product(),
            '+' => numbers.iter().sum(),
            _ => panic!("Unknown operation: {}", operation),
        };
        sum += result;
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(4277556, part1(BufReader::new(TEST.as_bytes()))?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(3263827, part2(BufReader::new(TEST.as_bytes()))?);
        Ok(())
    }
}
