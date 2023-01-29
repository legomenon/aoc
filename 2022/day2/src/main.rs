use anyhow::Result;
use std::{cmp::Ordering, fs, str::FromStr};

fn main() -> Result<()> {
    let file = fs::read_to_string("file.txt")?;
    let p1 = part1(&file);
    let p2 = part2(&file);

    println!("{:#?}", p1);
    println!("{:#?}", p2);

    Ok(())
}
#[derive(Debug, Copy, Clone, PartialEq)]
enum Game {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Game::Rock),
            "B" | "Y" => Ok(Game::Paper),
            "C" | "Z" => Ok(Game::Scissors),
            _ => Err("Invalid game state".to_string()),
        }
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if *self == Game::Scissors && *other == Game::Rock {
            return Some(Ordering::Less);
        } else if *self == Game::Rock && *other == Game::Scissors {
            return Some(Ordering::Greater);
        }
        return Some((*self as u8).cmp(&(*other as u8)));
    }
}

fn part1(file: &str) -> Option<u32> {
    let result = file
        .lines()
        .map(|x| x.split(" "))
        .map(|x| x.collect::<Vec<&str>>())
        .map(|x| (x[0].parse::<Game>().unwrap(), x[1].parse::<Game>().unwrap()))
        .map(
            |(opponent, player)| match player.partial_cmp(&opponent).unwrap() {
                Ordering::Greater => player as u32 + 6 as u32,
                Ordering::Equal => player as u32 + 3 as u32,
                Ordering::Less => player as u32 + 0 as u32,
            },
        )
        .sum::<u32>();

    Some(result)
}
#[derive(Debug)]
enum RoundEnd {
    Win,
    Draw,
    Lose,
}

impl FromStr for RoundEnd {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "X" => Ok(RoundEnd::Lose),
            "Y" => Ok(RoundEnd::Draw),
            "Z" => Ok(RoundEnd::Win),
            _ => Err("Invalid game state".to_string()),
        }
    }
}

fn part2(file: &str) -> Option<u32> {
    let result = file
        .lines()
        .map(|x| x.split(" "))
        .map(|x| x.collect::<Vec<&str>>())
        .map(|x| {
            (
                x[0].parse::<Game>().unwrap(),
                x[1].parse::<RoundEnd>().unwrap(),
            )
        })
        .map(|(opponent, round_end)| match round_end {
            RoundEnd::Win => win(opponent) as u32 + 6,
            RoundEnd::Draw => opponent as u32 + 3,
            RoundEnd::Lose => lose(opponent) as u32,
        })
        .sum::<u32>();

    Some(result)
}

fn win(g: Game) -> Game {
    match g {
        Game::Rock => Game::Paper,
        Game::Paper => Game::Scissors,
        Game::Scissors => Game::Rock,
    }
}

fn lose(g: Game) -> Game {
    match g {
        Game::Rock => Game::Scissors,
        Game::Paper => Game::Rock,
        Game::Scissors => Game::Paper,
    }
}
