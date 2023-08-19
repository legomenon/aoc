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

    v.iter().map(ClaimPoints::new).for_each(|claim| {
        claim.into_iter().for_each(|point| {
            fabric
                .entry(point)
                .and_modify(|counter| counter.0 += 1)
                .or_insert((0, claim.claim.id));
        })
    });

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
            size: (x2 - 1, y2 - 1),
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct ClaimPoints<'a> {
    claim: &'a Claim,
    current_x: u32,
    current_y: u32,
}

impl<'a> ClaimPoints<'a> {
    fn new(claim: &'a Claim) -> Self {
        ClaimPoints {
            claim,
            current_x: claim.x_y.0,
            current_y: claim.x_y.1,
        }
    }
}

impl<'a> Iterator for ClaimPoints<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_y <= self.claim.x_y.1 + self.claim.size.1 {
            let result = (self.current_x, self.current_y);
            if self.current_x < self.claim.x_y.0 + self.claim.size.0 {
                self.current_x += 1;
            } else {
                self.current_x = self.claim.x_y.0;
                self.current_y += 1;
            }
            Some(result)
        } else {
            None
        }
    }
}
