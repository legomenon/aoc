use std::{collections::HashSet, fs};

const UPPER_OFFSET: u32 = 38;
const LOWER_OFFSET: u32 = 96;
fn main() {
    let file = fs::read_to_string("file.txt").unwrap();
    println!("part 1: {}", part1(&file));
}

fn part1(file: &str) -> u32 {
    let data = file
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| {
            (
                a.chars().collect::<HashSet<_>>(),
                b.chars().collect::<HashSet<_>>(),
            )
        })
        .collect::<Vec<_>>();

    data.iter()
        .map(|(a, b)| {
            a.intersection(b)
                .map(|x| {
                    if x.is_ascii_lowercase() {
                        return *x as u32 - LOWER_OFFSET;
                    }
                    *x as u32 - UPPER_OFFSET
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<u32>>>()
        .iter()
        .flatten()
        .sum::<u32>()
}
