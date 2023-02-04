use std::{fs, str::FromStr};
#[derive(Debug)]
struct Password {
    pass: String,
    rng: (u32, u32),
    ch: char,
}

fn main() {
    let file = fs::read_to_string("file.txt").unwrap();
    let p1 = part1(&file);

    dbg!(p1);
}

fn part1(file: &str) -> u32 {
    file.lines()
        .map(|x| x.parse::<Password>().unwrap())
        .filter(|pas| pas.is_correct())
        .count() as u32
}

impl Password {
    fn is_correct(&self) -> bool {
        let counter = self.pass.chars().filter(|c| *c == self.ch).count() as u32;
        counter >= self.rng.0 && counter <= self.rng.1
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
        let rng_l = rng_l.parse::<u32>().unwrap();
        let rng_r = rng_r.parse::<u32>().unwrap();

        Ok(Self {
            pass: pass.to_owned(),
            rng: (rng_l, rng_r),
            ch,
        })
    }
}
