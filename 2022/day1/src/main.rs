use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let file = fs::read_to_string("file.txt")?;
    let p1 = part1(&file);
    let p2 = part2(&file);

    println!("{:#?}", p1);

    println!("{:#?}", p2);

    Ok(())
}

fn part1(file: &str) -> Option<u32> {
    let num = file
        .split("\n\n")
        .map(|elf_chunks| {
            elf_chunks
                .split('\n')
                .map(|cal| cal.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    Some(num)
}

fn part2(file: &str) -> Option<u32> {
    let mut num: Vec<u32> = file
        .split("\n\n")
        .map(|elf_chunks| {
            elf_chunks
                .split('\n')
                .map(|cal| cal.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    num.sort_by(|a, b| b.cmp(a));
    let result = num.iter().take(3).sum::<u32>();

    Some(result)
}
