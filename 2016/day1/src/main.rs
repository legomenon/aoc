use std::{fs, str::FromStr};

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
struct Position {
    pos: (i32, i32),
    direction: Direction,
}
fn main() {
    let file = fs::read_to_string("file.txt").unwrap();
    let inst_vec = parse(&file);

    let mut p1 = Position::new();
    p1.execute_instructions(&inst_vec);

    dbg!(p1.position());
}

impl Position {
    fn new() -> Self {
        Self {
            pos: (0, 0),
            direction: Direction::North,
        }
    }

    fn position(&self) -> i32 {
        self.pos.0.abs() + self.pos.1.abs()
    }

    fn execute_instructions(&mut self, instructions: &Vec<Instruction>) {
        for inst in instructions {
            self.change_direction(&inst);
            self.walk(&inst);
        }
    }
    fn walk(&mut self, instruction: &Instruction) {
        let step = match instruction {
            Instruction::Left(i) => i,
            Instruction::Right(i) => i,
        };
        match self.direction {
            Direction::North => self.pos.1 -= step,
            Direction::East => self.pos.0 += step,
            Direction::South => self.pos.1 += step,
            Direction::West => self.pos.0 -= step,
        }
    }

    fn change_direction(&mut self, instruction: &Instruction) {
        let direction = match instruction {
            Instruction::Left(_) => (self.direction as i32 - 1) % 4,
            Instruction::Right(_) => (self.direction as i32) + 1 % 4,
        };

        self.direction = Direction::try_from(direction).unwrap();
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0).unwrap() {
            'L' => match s.replace("L", "").parse::<i32>() {
                Ok(i) => Ok(Instruction::Left(i)),
                Err(_) => Err("Invalid instruction".to_owned()),
            },
            'R' => match s.replace("R", "").parse::<i32>() {
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
