use std::{collections::HashMap, fs, io, str::FromStr};

#[derive(Debug)]
struct Claim {
    id: u32,
    x_y: (u32, u32),
    size: (u32, u32),
}

fn main() -> Result<(), io::Error> {
    let data = fs::read_to_string("data.txt")?;
    let data = parse(&data);

    let p1 = part1(&data);

    dbg!(p1);

    Ok(())
}

fn part1(v: &[Claim]) -> u32 {
    let mut fabric: HashMap<(u32, u32), (u32, u32)> = HashMap::new();

    for a in v {
        let x_min = a.x_y.0;
        let x_max = a.x_y.0 + a.size.0;

        let y_min = a.x_y.1;
        let y_max = a.x_y.1 + a.size.1;
        for i in x_min..x_max {
            for j in y_min..y_max {
                fabric
                    .entry((i, j))
                    .and_modify(|counter| counter.0 += 1)
                    .or_insert((0, a.id));
            }
        }
    }

    let res = fabric
        .iter()
        .filter_map(|(_, v)| if v.0 != 0 { Some(1) } else { None })
        .count();
    res as u32
}

fn parse(d: &str) -> Vec<Claim> {
    d.lines()
        .map(|c| c.parse::<Claim>().unwrap())
        .collect::<Vec<Claim>>()
}

impl FromStr for Claim {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let d = s.split_whitespace().collect::<Vec<&str>>();
        let id = d[0].trim_start_matches('#').parse::<u32>()?;

        let p = d[2].trim_end_matches(':').split(',').collect::<Vec<&str>>();
        let x1 = p[0].parse::<u32>()?;
        let y1 = p[1].parse::<u32>()?;

        let len = d[3].trim_end_matches(':').split('x').collect::<Vec<&str>>();
        let x2 = len[0].parse::<u32>()?;
        let y2 = len[1].parse::<u32>()?;

        Ok(Self {
            id,
            x_y: (x1, y1),
            size: (x2, y2),
        })
    }
}

impl Iterator for Claim {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
