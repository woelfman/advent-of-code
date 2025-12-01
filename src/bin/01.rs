use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut dial: i32 = 50;
        let mut answer = 0;
        for line in reader.lines().map_while(Result::ok) {
            let mut iter = line.chars();
            let direction = iter.next().unwrap();
            let distance: i32 = iter.collect::<String>().parse()?;
            if direction == 'L' {
                dial -= distance;
            } else {
                dial += distance;
            }
            dial = dial.rem_euclid(100);
            answer += (dial == 0) as usize;
        }
        Ok(answer)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut dial: i32 = 50;
        let mut answer = 0;

        for line in reader.lines().map_while(Result::ok) {
            let mut iter = line.chars();
            let direction = iter.next().unwrap();
            let mut distance: i32 = iter.collect::<String>().parse()?;

            if direction == 'L' {
                distance *= -1;
            }

            let full_wraps = distance.abs() / 100;
            let remainder = distance.abs() % 100;

            let partial_cross = if distance > 0 {
                let end = (dial + remainder).rem_euclid(100);
                if end < dial && dial != 0 { 1 } else { 0 }
            } else if distance < 0 {
                let end = (dial - remainder).rem_euclid(100);
                if end > dial && dial != 0 { 1 } else { 0 }
            } else {
                0
            };

            answer += full_wraps as usize + partial_cross;

            dial = (dial + distance).rem_euclid(100);

            if partial_cross == 0 && dial == 0 {
                // Special case: stopping at 0 and not wrapping still counts as crossing
                answer += 1;
            }
        }
        Ok(answer)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
