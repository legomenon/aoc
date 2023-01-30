#![feature(iter_array_chunks)]
use std::{collections::HashSet, fs};

const UPPER_OFFSET: u32 = 'A' as u32 - 1;
const LOWER_OFFSET: u32 = 'a' as u32 - 1;

fn main() {
    let file = fs::read_to_string("file.txt").unwrap();
    println!("part 1: {}", part1(&file));
    println!("part 1: {}", part2(&file));
}

fn part1(file: &str) -> u32 {
    file.lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| {
            (
                a.chars().collect::<HashSet<_>>(),
                b.chars().collect::<HashSet<_>>(),
            )
        })
        .map(|(a, b)| {
            a.intersection(&b)
                .map(|x| CharScore::from(*x).score)
                .collect::<Vec<_>>()
        })
        .flatten()
        .sum::<u32>()
}

fn part2(file: &str) -> u32 {
    file.lines()
        .array_chunks::<3>()
        .map(|data| {
            (
                data[0].chars().collect::<HashSet<char>>(),
                data[1].chars().collect::<HashSet<char>>(),
                data[2].chars().collect::<HashSet<char>>(),
            )
        })
        .map(|(a, b, c)| {
            let intersect = a.intersection(&c).map(|x| *x).collect::<HashSet<char>>();
            b.intersection(&intersect)
                .map(|x| CharScore::from(*x).score)
                .sum::<u32>()
        })
        .sum::<u32>()
}

struct CharScore {
    score: u32,
}

impl From<char> for CharScore {
    fn from(value: char) -> Self {
        if value.is_ascii_lowercase() {
            return Self {
                score: value as u32 - LOWER_OFFSET,
            };
        }
        return Self {
            score: value as u32 - UPPER_OFFSET + 26,
        };
    }
}
