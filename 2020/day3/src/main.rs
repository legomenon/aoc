use std::fs;

fn main() {
    let file = fs::read_to_string("file.txt").unwrap();
    let forest = parse(&file);

    let p1 = part1(&forest, 3);
    let p2 = part2(&forest, 7);

    dbg!(p1, p2);
}

fn part1(forest: &Vec<Vec<char>>, step: usize) -> usize {
    let mut pos_x: usize = 0;
    forest
        .iter()
        .map(|tree_row| {
            let square = tree_row[pos_x];
            pos_x = (pos_x + step) % tree_row.len();
            square
        })
        .filter(|&c| c == '#')
        .count()
}

fn part2(forest: &Vec<Vec<char>>, max_step: usize) -> usize {
    let mut result: Vec<usize> = Vec::new();

    let steps: Vec<usize> = (1..=max_step).filter(|i| i % 2 != 0).collect();
    steps.iter().map(|&i| result.push(part1(forest, i))).count();

    let down_2 = forest
        .iter()
        .cloned()
        .enumerate()
        .filter_map(move |(i, row)| {
            if i % 2 == 0 {
                return Some(row);
            }
            None
        })
        .collect::<Vec<_>>();

    result.push(part1(&down_2, 1));

    result.iter().fold(1, |acc, x| acc * x)
}

fn parse(file: &str) -> Vec<Vec<char>> {
    file.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
