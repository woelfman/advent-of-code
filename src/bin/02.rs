use adv_code_2025::*;
use anyhow::{Result, anyhow};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<u64> {
        let mut answer = 0;
        let mut buf = vec![];

        loop {
            buf.clear();

            let _n = match reader.read_until(b',', &mut buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    break;
                }
            };

            buf.pop_if(|x| *x == b',');

            let mut parts = buf.split(|&b| b == b'-');

            let start = parts.next().ok_or_else(|| anyhow!("Missing first part"))?;
            let end = parts.next().ok_or_else(|| anyhow!("Missing second part"))?;

            // Find invalid IDs in the range
            let start_id = String::from_utf8_lossy(start).parse::<u64>()?;
            let end_id = String::from_utf8_lossy(end).parse::<u64>()?;
            for i in start_id..=end_id {
                let s = i.to_string();
                let half = s.len() / 2;

                if s[..half] == s[half..] {
                    answer += i;
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<u64> {
        let mut answer = 0;
        let mut buf = vec![];

        loop {
            buf.clear();

            let _n = match reader.read_until(b',', &mut buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    break;
                }
            };

            buf.pop_if(|x| *x == b',');

            let mut parts = buf.split(|&b| b == b'-');

            let start = parts.next().ok_or_else(|| anyhow!("Missing first part"))?;
            let end = parts.next().ok_or_else(|| anyhow!("Missing second part"))?;

            // Find invalid IDs in the range
            let start_id = String::from_utf8_lossy(start).parse::<u64>()?;
            let end_id = String::from_utf8_lossy(end).parse::<u64>()?;
            for i in start_id..=end_id {
                let s = i.to_string();
                let half = s.len() / 2;

                for n in 1..=half {
                    if s.len() % n != 0 {
                        continue;
                    }

                    let mut invalid = true;
                    for chunk in s.as_bytes().chunks(n) {
                        if *chunk != s.as_bytes()[0..n] {
                            invalid = false;
                            break;
                        }
                    }
                    if invalid {
                        answer += i;
                        break;
                    }
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(4174379265, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
