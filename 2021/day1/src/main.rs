use std::cmp::Ordering;
use std::fs;

fn main() {
    let file = fs::read_to_string("file.txt").unwrap();
    let p1 = part1(&file);
    dbg!(p1);
    let p2 = part2(&file);
    dbg!(p2);
}

fn part1(file: &str) -> u32 {
    let (current, _other) = file.split_once('\n').unwrap();
    let mut current = current.parse::<u32>().unwrap();
    file.split('\n')
        .map(|x| x.parse::<u32>().unwrap())
        .map(|x| {
            if let Ordering::Greater = x.cmp(&current) {
                current = x;
                x
            } else {
                current = x;
                0
            }
        })
        .filter(|x| *x != 0)
        .count() as u32
}

fn part2(file: &str) -> u32 {
    let res = file
        .split('\n')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .windows(3)
        .map(|x| x.iter().sum::<u32>())
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    part1(&res)
}
