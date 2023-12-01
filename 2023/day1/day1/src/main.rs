use std::fs;

fn main() {
    let a = fs::read_to_string("file.txt").unwrap();
    let p1 = part1(&a);
    dbg!(p1);
}

fn part1(file: &str) -> u32 {
    file.lines()
        .map(|line| {
            let digits: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
            let num = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
            num.parse::<u32>().unwrap_or_default()
        })
        .sum()
}
