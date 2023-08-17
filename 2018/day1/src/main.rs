use std::{collections::HashSet, fs};

fn main() {
    let data = fs::read_to_string("data.txt").expect("Failed to read a file");
    let data = parse(&data);
    let part_1 = pt_1(&data);
    let part_2 = pt_2(&data);
    
    dbg!(part_1);
    dbg!(part_2);
}

fn pt_1(v: &[i32]) -> i32 {
    v.iter().sum()
}

fn pt_2(v: &[i32]) -> i32 {
    let mut freq: HashSet<i32> = HashSet::new();
    freq.insert(0);
    let mut acc = 0;
    for i in v.iter().cycle() {
        acc += i;
        if freq.contains(&acc) {
            return acc;
        } else {
            freq.insert(acc);
        }
    }

    unreachable!()
}

fn parse(data: &str) -> Vec<i32> {
    data.lines()
        .map(|i| i.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
