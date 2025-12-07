use adv_code_2025::*;
use anyhow::Result;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
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
    // Get the input
    let mut tachyon_manifold: Vec<Vec<u8>> = vec![];
    for line in reader.lines() {
        let line = line?;
        tachyon_manifold.push(line.into_bytes());
    }

    // Extend the tachyon beam from 'S'
    let i = tachyon_manifold[0].iter().position(|&c| c == b'S').unwrap();
    tachyon_manifold[1][i] = b'|';

    // Populate the tachyon manifold
    for i in 1..tachyon_manifold.len() - 1 {
        let mut beams = vec![];
        for (j, &c) in tachyon_manifold[i].iter().enumerate() {
            if c == b'|' {
                beams.push(j);
            }
        }
        for j in beams {
            if j == 0 || j == tachyon_manifold[i].len() - 1 {
                if tachyon_manifold[i][j] == b'|' {
                    tachyon_manifold[i + 1][j] = b'|';
                }
                continue;
            }
            if tachyon_manifold[i + 1][j] == b'^' {
                if tachyon_manifold[i + 1][j - 1] != b'|' {
                    tachyon_manifold[i + 1][j - 1] = b'|';
                }
                if tachyon_manifold[i + 1][j + 1] != b'|' {
                    tachyon_manifold[i + 1][j + 1] = b'|';
                }
            } else {
                tachyon_manifold[i + 1][j] = b'|';
            }
        }
    }

    // Count the splits
    let mut total_splits = 0;
    for i in (1..tachyon_manifold.len() - 1).step_by(2) {
        for (&beam, &splitter) in tachyon_manifold[i]
            .iter()
            .zip(tachyon_manifold[i + 1].iter())
        {
            if beam == b'|' && splitter == b'^' {
                total_splits += 1;
            }
        }
    }

    Ok(total_splits)
}

fn part2<R: BufRead>(reader: R) -> Result<u64> {
    // Get the input
    let mut line_iter = reader.lines().enumerate();
    let i = line_iter.next().unwrap().1?.chars().position(|c| c == 'S').unwrap();

    // Create timelines
    let map = line_iter.fold(
        {
            let mut m = HashMap::<usize, u64>::new();
            m.insert(i, 1);
            m
        },
        |positions, (_index, line)| {
            let mut new_positions = HashMap::<usize, u64>::new();

            for (index, count) in positions {
                if line.as_ref().unwrap().chars().nth(index).unwrap() == '^' {
                    new_positions
                        .entry(index - 1)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                    new_positions
                        .entry(index + 1)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                } else {
                    new_positions
                        .entry(index)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                }
            }

            new_positions
        },
    );
    Ok(map.values().sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(21, part1(BufReader::new(TEST.as_bytes()))?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(40, part2(BufReader::new(TEST.as_bytes()))?);
        Ok(())
    }
}
