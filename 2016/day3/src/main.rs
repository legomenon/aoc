use std::fs;

fn main() {
    let file = fs::read_to_string("file.txt").unwrap();
    let vec_input = parse(&file);
    let p1 = part1(&vec_input);
    let p2 = part2(&vec_input);

    dbg!(p1);
    dbg!(p2);
}

fn part1(vec_input: &Vec<Vec<u32>>) -> u32 {
    vec_input
        .iter()
        .filter_map(|x| {
            let mux_side = x.iter().max().unwrap() * 2;
            let sum = x.iter().sum::<u32>();

            match mux_side.cmp(&sum) {
                std::cmp::Ordering::Less => Some(1),
                std::cmp::Ordering::Equal => None,
                std::cmp::Ordering::Greater => None,
            }
        })
        .count() as u32
}

fn part2(vec_input: &Vec<Vec<u32>>) -> u32 {
    vec_input
        .chunks(3)
        .map(|x| {
            let cols = transpose(x.to_vec());
            part1(&cols)
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>()
}

fn parse(file: &str) -> Vec<Vec<u32>> {
    file.lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
