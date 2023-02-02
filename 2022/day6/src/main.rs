use std::{collections::HashSet, fs};

fn main() {
    let file = fs::read_to_string("file.txt").unwrap();

    let p1 = check_string_windows(&file, 4);
    let p2 = check_string_windows(&file, 14);

    dbg!(p1, p2);
}

fn check_string_windows(file: &str, win_size: usize) -> u32 {
    file.lines()
        .map(|x| (x.trim(), x.trim().chars().collect::<Vec<char>>()))
        .map(|(data, vec_chars)| {
            vec_chars
                .windows(win_size)
                .filter_map(|chars| check_unique_substring(data, chars, win_size))
                .collect::<Vec<usize>>()
        })
        .filter_map(|x| x.into_iter().min())
        .collect::<Vec<_>>()
        .into_iter()
        .sum::<usize>() as u32
}

fn check_unique_substring(data: &str, sub: &[char], win_size: usize) -> Option<usize> {
    let unique = sub.iter().collect::<HashSet<&char>>();
    if unique.len() == win_size {
        let sub_s: String = sub.into_iter().collect();
        match data.rfind(&sub_s) {
            Some(i) => {
                return Some(i + win_size);
            }
            None => return None,
        }
    } else {
        None
    }
}
