use std::fs;

fn main() {
    let file = fs::read_to_string("file.txt").unwrap();

    dbg!(part1(&file));
    dbg!(part2(&file));
}

fn part1(file: &str) -> u32 {
    file.lines()
        .map(|x| x.parse::<u32>().unwrap())
        .map(|x| (x / 3) - 2)
        .sum::<u32>()
}

fn part2(file: &str) -> u32 {
    file.lines()
        .map(|x| x.parse::<i32>().unwrap())
        .map(|mut x| {
            let mut counter = 0;
            while x > 6 {
                x = (x / 3) - 2;
                counter += x;
            }
            counter
        })
        .sum::<i32>() as u32
}
