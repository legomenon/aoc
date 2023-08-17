use std::{collections::HashMap, fs};

fn main() {
    let data = fs::read_to_string("data.txt").expect("Failed to read file");
    let data = parse(&data);
    let p1 = part1(&data);
    let p2 = part2(&data);

    dbg!(p1, p2);
}

fn part1(v: &[String]) -> i32 {
    let mut double_count = 0;
    let mut triple_count = 0;

    v.iter().for_each(|s| {
        let mut result: HashMap<char, i32> = HashMap::new();

        s.chars().for_each(|c| {
            result.entry(c).and_modify(|i| *i += 1).or_insert(1);
        });

        let mut is_two: bool = false;
        let mut is_three: bool = false;

        result.iter().for_each(|(_, v)| match *v {
            val if val == 2 && !is_two => {
                double_count += 1;
                is_two = true
            }
            val if val == 3 && !is_three => {
                triple_count += 1;
                is_three = true
            }
            _ => (),
        });
    });

    double_count * triple_count
}

fn part2(v: &[String]) -> String {
    for i in v.iter() {
        for j in v {
            let res = str_diff(i, j);
            if !res.is_empty() {
                return res;
            }
        }
    }

    unreachable!()
}

fn parse(s: &str) -> Vec<String> {
    s.lines().map(|s| s.to_owned()).collect()
}

fn str_diff(l: &str, r: &str) -> String {
    let mut diff = 0;
    let mut diff_index = 0;

    (0..l.len()).for_each(|i| {
        if l.chars().nth(i) != r.chars().nth(i) {
            diff += 1;
            diff_index = i;
        }
    });
    if diff == 1 {
        let mut res = l.clone().to_owned();
        res.remove(diff_index);
        res
    } else {
        "".to_owned()
    }
}
