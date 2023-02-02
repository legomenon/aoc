use std::fs;

fn main() {
    let file = fs::read_to_string("file.txt").unwrap();
    let p1 = part1(&file);

    dbg!(p1);
}

fn part1(file: &str) -> u32 {
    file.lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter_map(|x| {
            let mux_side = x.iter().max().unwrap() * 2;
            let sum = x.iter().sum::<u32>();
            // dbg!(mux_side, sum);
            if sum > mux_side {
                Some(1)
            } else {
                None
            }
        })
        .count() as u32
}
