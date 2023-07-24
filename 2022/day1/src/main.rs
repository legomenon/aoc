use color_eyre::Result;
use std::fs;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut calories = parse("file.txt")?;

    let p1 = part1(&calories)?;
    let p2 = part2(&mut calories);
    dbg!(p1, p2);

    Ok(())
}

fn part1(v: &[u32]) -> Result<u32> {
    v.iter()
        .max()
        .cloned()
        .ok_or_else(|| color_eyre::eyre::eyre!("Slice is empty"))
}

fn part2(v: &mut [u32]) -> u32 {
    v.sort_by(|a, b| b.cmp(a));
    v.iter().take(3).sum::<u32>()
}

fn parse(p: &str) -> Result<Vec<u32>> {
    let file = fs::read_to_string(p)?;
    let data = file
        .split("\n\n")
        .map(|elf_chunks| {
            elf_chunks
                .lines()
                .map(|cal| cal.parse::<u32>())
                .sum::<Result<u32, _>>()
        })
        .collect::<Result<Vec<u32>, _>>()?;
    Ok(data)
}
