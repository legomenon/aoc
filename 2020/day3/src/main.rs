use std::fs;

fn main() {
    let file = fs::read_to_string("file.txt").unwrap();
    let forest = parse(&file);

    let p1 = part1(forest);
    dbg!(p1);
}

fn part1(forest: Vec<Vec<char>>) -> usize {
    let mut pos_x: usize = 0;
    forest
        .iter()
        .map(|tree_row| {
            let square = tree_row[pos_x];
            pos_x = (pos_x + 3) % tree_row.len();
            square
        })
        .filter(|&c| c == '#')
        .count()
}

fn parse(file: &str) -> Vec<Vec<char>> {
    file.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
