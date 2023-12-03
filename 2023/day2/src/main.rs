use std::{collections::HashSet, fs, str::FromStr};

fn main() {
    let data = fs::read_to_string("file.txt").unwrap();

    let p1 = Game::part1(&data);
    dbg!(p1);

    let p2 = Game::part2(&data);
    dbg!(p2);
}
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum CubeColor {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<HashSet<CubeColor>>,
}

impl Game {
    fn part1(data: &str) -> u32 {
        let red_load = CubeColor::Red(12);
        let green_load = CubeColor::Green(13);
        let blue_load = CubeColor::Blue(14);

        let game_vec = Game::parse(data);
        let game_sum: HashSet<u32> = (1..=game_vec.len() as u32).collect();

        let h = game_vec
            .iter()
            .flat_map(|g| {
                g.sets.iter().map(|s| {
                    s.iter().map(|c: &CubeColor| match c {
                        CubeColor::Red(_) if c > &red_load => g.id,
                        CubeColor::Green(_) if c > &green_load => g.id,
                        CubeColor::Blue(_) if c > &blue_load => g.id,
                        _ => 0,
                    })
                })
            })
            .flatten()
            .collect::<HashSet<u32>>();

        game_sum.difference(&h).sum::<u32>()
    }

    fn part2(data: &str) -> u32 {
        let game_vec = Game::parse(data);

        game_vec
            .iter()
            .map(|g| {
                let mut red_set: HashSet<u32> = HashSet::new();
                let mut green_set: HashSet<u32> = HashSet::new();
                let mut blue_set: HashSet<u32> = HashSet::new();
                g.sets.iter().for_each(|s| {
                    s.iter().for_each(|c| match c {
                        CubeColor::Red(v) => {
                            red_set.insert(*v);
                        }
                        CubeColor::Green(v) => {
                            green_set.insert(*v);
                        }
                        CubeColor::Blue(v) => {
                            blue_set.insert(*v);
                        }
                    });
                });

                let red_max = red_set.iter().max().unwrap_or(&1);
                let green_max = green_set.iter().max().unwrap_or(&1);
                let blue_max = blue_set.iter().max().unwrap_or(&1);

                red_max * green_max * blue_max
            })
            .sum()
    }

    fn parse(data: &str) -> Vec<Game> {
        data.lines()
            .map(|l| l.parse::<Game>().unwrap())
            .collect::<Vec<_>>()
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, sets) = s.split_once(':').unwrap();

        let id = id.replace("Game ", "");
        let id = id.parse::<u32>().unwrap();

        let sets = sets.trim_start().split(';').collect::<Vec<_>>();
        let sets = sets
            .iter()
            .map(|l| {
                l.trim()
                    .split(", ")
                    .map(|x| x.parse::<CubeColor>().unwrap())
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Game { id, sets })
    }
}

impl FromStr for CubeColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (n, color) = s.split_once(' ').unwrap();

        let n = n.parse::<u32>().unwrap();

        match color {
            "red" => Ok(CubeColor::Red(n)),
            "green" => Ok(CubeColor::Green(n)),
            "blue" => Ok(CubeColor::Blue(n)),
            _ => Err("Invalid cube color. Make sure the data set is correct".to_owned()),
        }
    }
}
