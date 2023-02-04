use std::fs;

fn main() {
    let file = fs::read_to_string("file.txt").unwrap();
    let p1 = part1(&file);
    dbg!(p1);
}

fn part1(file: &str) -> usize {
    let mut result = file
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let i_iter = result.clone();
    let mut i_iter = i_iter.iter();

    loop {
        let i = i_iter.next().unwrap();
        if *i == 1 {
            let a = *i_iter.next().unwrap();
            let b = *i_iter.next().unwrap();
            let c = *i_iter.next().unwrap();
            result[c] = result[a] + result[b];
            // dbg!(a, b, c);
        } else if *i == 2 {
            let a = *i_iter.next().unwrap();
            let b = *i_iter.next().unwrap();
            let c = *i_iter.next().unwrap();
            result[c] = result[a] * result[b];
        } else if *i == 99 {
            break;
        }
    }
    result[0]
}
