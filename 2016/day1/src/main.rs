use std::{collections::HashSet, fs, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
enum Instruction {
    Left(i32),
    Right(i32),
}

#[derive(Debug)]
struct ElfPosition {
    pos: (i32, i32),
    direction: Direction,
    visited_pos: HashSet<(i32, i32)>,
    visited_twice: Option<i32>,
}
fn main() {
    let file = fs::read_to_string("file.txt").unwrap();
    let inst_vec = parse(&file);

    let mut p1 = ElfPosition::new();
    p1.execute_instructions(&inst_vec);
    let p2 = p1.visited_twice.unwrap();

    dbg!(p1.position(), p1.direction);
    dbg!(p2);
}

impl ElfPosition {
    fn new() -> Self {
        Self {
            pos: (0, 0),
            direction: Direction::North,
            visited_pos: HashSet::new(),
            visited_twice: None,
        }
    }

    fn position(&self) -> i32 {
        self.pos.0.abs() + self.pos.1.abs()
    }

    fn execute_instructions(&mut self, instructions: &Vec<Instruction>) {
        for inst in instructions {
            self.change_direction(inst);
            self.walk(inst);
        }
    }
    fn walk(&mut self, instruction: &Instruction) {
        let step = match instruction {
            Instruction::Left(i) => *i,
            Instruction::Right(i) => *i,
        };

        for _ in 1..=step {
            dbg!(self.pos);
            match self.direction {
                Direction::North => self.pos.1 -= 1,
                Direction::East => self.pos.0 += 1,
                Direction::South => self.pos.1 += 1,
                Direction::West => self.pos.0 -= 1,
            }

            if self.visited_twice.is_none() {
                self.is_visited();
            }
        }
    }

    fn change_direction(&mut self, instruction: &Instruction) {
        let direction = match instruction {
            Instruction::Left(_) => (self.direction as i32 - 1) % 4,
            Instruction::Right(_) => ((self.direction as i32) + 1) % 4,
        };

        self.direction = Direction::try_from(direction).unwrap();
    }

    fn is_visited(&mut self) {
        let pos = (self.pos.0, self.pos.1);

        match self.visited_pos.contains(&pos) {
            true => self.visited_twice = Some(self.pos.0.abs() + self.pos.1.abs()),

            false => {
                self.visited_pos.insert(pos);
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().unwrap() {
            'L' => match s.replace('L', "").parse::<i32>() {
                Ok(i) => Ok(Instruction::Left(i)),
                Err(_) => Err("Invalid instruction".to_owned()),
            },
            'R' => match s.replace('R', "").parse::<i32>() {
                Ok(i) => Ok(Instruction::Right(i)),
                Err(_) => Err("Invalid instruction".to_owned()),
            },
            e_dir => Err(format!("Invalid instruction: {:?}", e_dir)),
        }
    }
}

impl TryFrom<i32> for Direction {
    type Error = String;

    fn try_from(i: i32) -> Result<Self, Self::Error> {
        match i % 4 {
            n if n == Direction::South as i32 => Ok(Direction::South),
            n if n == Direction::East as i32 => Ok(Direction::East),
            n if n == Direction::North as i32 => Ok(Direction::North),
            n if n == Direction::West as i32 || n == -1 => Ok(Direction::West),
            e_dir => Err(format!("Invalid direction: {:?}", e_dir)),
        }
    }
}

fn parse(file: &str) -> Vec<Instruction> {
    file.trim()
        .split(',')
        .map(str::trim)
        .map(|x| x.parse::<Instruction>().unwrap())
        .collect::<Vec<Instruction>>()
}
