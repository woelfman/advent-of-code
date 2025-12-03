use adv_code_2025::*;
use anyhow::{Result, anyhow};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let mut joltage = 0;

        for line in reader.lines() {
            let bank = line?;
            if bank.is_empty() {
                break;
            }
            // Find the largest character excluding the last character
            let (first_pos, left_battery) = bank
                .chars()
                .enumerate()
                .take(bank.len() - 1)
                .max_by(|a, b| {
                    let ord = a.1.cmp(&b.1);
                    match ord {
                        std::cmp::Ordering::Equal => std::cmp::Ordering::Greater, // Prefer first occurrence
                        _ => ord,
                    }
                })
                .ok_or_else(|| anyhow!("Empty line"))?;
            // Find the next largest character after the largest character
            let right_battery = bank
                .chars()
                .enumerate()
                .skip(first_pos + 1)
                .max_by_key(|&(_i, c)| c)
                .ok_or_else(|| anyhow!("No second largest"))?
                .1;
            joltage += (left_battery
                .to_digit(10)
                .ok_or_else(|| anyhow!("Invalid digit"))?
                * 10
                + right_battery
                    .to_digit(10)
                    .ok_or_else(|| anyhow!("Invalid digit"))?) as u64;
        }

        Ok(joltage)
    }

    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let mut joltage = 0;

        for line in reader.lines() {
            let bank = line?;
            if bank.is_empty() {
                break;
            }
            let mut banks: Vec<(usize, u8)> = vec![];
            for i in (0..12).rev() {
                // Find the largest character excluding the last character
                let (first_pos, left_battery) = bank
                    .chars()
                    .enumerate()
                    .take(bank.len() - i)
                    .skip(banks.last().map_or(0, |b| b.0 + 1))
                    .max_by(|a, b| {
                        let ord = a.1.cmp(&b.1);
                        match ord {
                            std::cmp::Ordering::Equal => std::cmp::Ordering::Greater, // Prefer first occurrence
                            _ => ord,
                        }
                    })
                    .ok_or_else(|| anyhow!("Empty line"))?;
                banks.push((
                    first_pos,
                    left_battery
                        .to_digit(10)
                        .ok_or_else(|| anyhow!("Invalid digit"))? as u8,
                ));
            }
            // Calculate the total for this bank by summing the values shifted by their position
            let bank_joltage = banks
                .iter()
                .rev()
                .enumerate()
                .map(|(i, &(_pos, val))| val as u64 * 10u64.pow(i as u32))
                .sum::<u64>();
            joltage += bank_joltage;
        }

        Ok(joltage)
    }

    assert_eq!(3121910778619, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
