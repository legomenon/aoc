use std::fs;

fn main() {
    let file = fs::read_to_string("file.txt").unwrap();
    let calories = parse(&file);

    let p1 = part1(&calories);
    let p2 = part2(&calories);
    dbg!(p1, p2);
}

fn part1(v: &Vec<u32>) -> u32 {
    *v.into_iter().max().unwrap()
}

fn part2(v: &Vec<u32>) -> u32 {
    let mut part2_v = v.clone();
    part2_v.sort_by(|a, b| b.cmp(a));
    part2_v.iter().take(3).sum::<u32>()
}

fn parse(file: &str) -> Vec<u32> {
    file.split("\n\n")
        .map(|elf_chunks| {
            elf_chunks
                .split('\n')
                .map(|cal| cal.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect()
}
