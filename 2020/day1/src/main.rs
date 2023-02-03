use std::{collections::HashSet, fs};

fn main() {
    let file = fs::read_to_string("file.txt").unwrap();
    let p1 = find_two(&file, 2020);
    let p2 = find_three(&file, 2020);

    dbg!(p1, p2);
}

fn find_two(file: &str, searched_num: u32) -> u32 {
    let mut rep_set: HashSet<u32> = HashSet::new();
    file.lines()
        .filter_map(|x| {
            let n = x.parse::<u32>().unwrap();
            let searched = searched_num - n;
            match rep_set.get(&searched) {
                Some(i) => Some(n * i),
                None => {
                    rep_set.insert(n);
                    None
                }
            }
        })
        .take(1)
        .sum::<u32>()
}

fn find_three(file: &str, searched_num: u32) -> Option<u32> {
    let mut rep_set: HashSet<u32> = HashSet::new();
    let num_vec = file
        .lines()
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    for i in num_vec.iter() {
        for j in num_vec.iter() {
            let searched = match searched_num.overflowing_sub(i + j) {
                (_, b) if b => continue,
                (i, _) => i,
            };
            match rep_set.get(&searched) {
                Some(num) => return Some(i * j * num),
                None => {
                    rep_set.insert(*j);
                    continue;
                }
            };
        }
    }
    None
}
