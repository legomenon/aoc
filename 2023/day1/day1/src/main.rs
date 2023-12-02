use std::fs;

fn main() {
    let data = fs::read_to_string("file.txt").unwrap();
    let p1 = part1(&data);
    let p2 = part2(&data);
    dbg!(p1, p2);
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

fn part2(file: &str) -> u32 {
    let v = file.lines().map(|l| decode(l)).collect::<String>();
    let res = part1(&v);
    dbg!(res);
    res
}

fn decode(line: &str) -> String {
    let mut l = line.to_owned();
    l.push('\n');
    let nums: Vec<(&str, &str)> = vec![
        ("zero", "z0o"),
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    for (num, rep) in nums {
        if l.contains(num) {
            l = l.replace(num, rep);
        }
    }
    l
}
