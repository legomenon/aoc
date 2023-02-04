use std::{fs, str::FromStr};
#[derive(Debug)]
struct Password {
    pass: String,
    rng: (usize, usize),
    ch: char,
}

fn main() {
    let file = fs::read_to_string("file.txt").unwrap();
    let p1 = part1(&file);
    let p2 = part2(&file);

    dbg!(p1, p2);
}

fn part1(file: &str) -> usize {
    file.lines()
        .map(|x| x.parse::<Password>().unwrap())
        .filter(|pas| pas.is_correct_part1())
        .count()
}

fn part2(file: &str) -> usize {
    file.lines()
        .map(|x| x.parse::<Password>().unwrap())
        .filter(|pas| pas.is_correct_part2())
        .count()
}

impl Password {
    fn is_correct_part1(&self) -> bool {
        let counter = self.pass.chars().filter(|c| *c == self.ch).count();
        counter >= self.rng.0 && counter <= self.rng.1
    }

    fn is_correct_part2(&self) -> bool {
        let temp = self.pass.chars().collect::<Vec<char>>();

        if self.rng.0 > self.pass.len() || self.rng.1 > self.pass.len() {
            return false;
        } else if temp[self.rng.0 - 1] == self.ch && temp[self.rng.1 - 1] != self.ch
            || temp[self.rng.1 - 1] == self.ch && temp[self.rng.0 - 1] != self.ch
        {
            return true;
        }
        false
    }
}

impl FromStr for Password {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s.split(' ').collect::<Vec<&str>>();
        if line.len() != 3 {
            return Err("Invalid password config".to_owned());
        }
        let (rng_l, rng_r) = line[0].split_once('-').unwrap();
        let ch = line[1].replace(":", "").chars().nth(0).unwrap();
        let pass = line[2];
        let rng_l = rng_l.parse::<usize>().unwrap();
        let rng_r = rng_r.parse::<usize>().unwrap();

        Ok(Self {
            pass: pass.to_owned(),
            rng: (rng_l, rng_r),
            ch,
        })
    }
}
