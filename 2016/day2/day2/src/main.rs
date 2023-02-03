use std::{
    cmp::{max, min},
    fs,
    str::FromStr,
};

enum Command {
    Up,
    Left,
    Right,
    Down,
}

struct Keypad {
    fields: Vec<Vec<char>>,
    pos: (usize, usize),
}

fn main() {
    let file = fs::read_to_string("file.txt").unwrap();
    let result = file
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_string().parse::<Command>().unwrap())
                .collect::<Vec<Command>>()
        })
        .collect::<Vec<Vec<Command>>>();

    let mut keypad = Keypad::new_part1();
    let p1 = keypad.execute_commands(&result);

    let mut keypad2 = Keypad::new_part2();
    let p2 = keypad2.execute_commands_part2(&result);
    dbg!(p1, p2);
}

impl Keypad {
    fn new_part1() -> Self {
        Self {
            fields: vec![
                vec!['1', '2', '3'],
                vec!['4', '5', '6'],
                vec!['7', '8', '9'],
            ],
            pos: (1, 1),
        }
    }

    fn new_part2() -> Self {
        Self {
            fields: vec![
                vec!['_', '_', '1', '_', '_'],
                vec!['_', '2', '3', '4', '_'],
                vec!['5', '6', '7', '8', '8'],
                vec!['_', 'A', 'B', 'C', '_'],
                vec!['_', '_', 'D', '_', '_'],
            ],
            pos: (2, 0),
        }
    }

    fn execute_commands(&mut self, commands: &Vec<Vec<Command>>) -> String {
        commands
            .iter()
            .map(|c| self.press_part1(c))
            .collect::<String>()
    }

    fn execute_commands_part2(&mut self, commands: &Vec<Vec<Command>>) -> String {
        commands
            .iter()
            .map(|c| self.press_part2(c))
            .collect::<String>()
    }

    fn press_part1(&mut self, commands: &Vec<Command>) -> char {
        for c in commands {
            let pos_x = self.pos.0 as i32;
            let pos_y = self.pos.1 as i32;

            match c {
                Command::Up => self.pos.0 = max(0, pos_x - 1) as usize,
                Command::Left => self.pos.1 = max(0, pos_y - 1) as usize,
                Command::Right => self.pos.1 = min(2, pos_y + 1) as usize,
                Command::Down => self.pos.0 = min(2, pos_x + 1) as usize,
            }
        }
        self.fields[self.pos.0][self.pos.1]
    }

    fn press_part2(&mut self, commands: &Vec<Command>) -> char {
        for c in commands {
            let pos_x = self.pos.0 as i32;
            let pos_y = self.pos.1 as i32;
            let mut temp_x = pos_x;
            let mut temp_y = pos_y;
            match c {
                Command::Up => temp_x = max(0, pos_x - 1),
                Command::Left => temp_y = max(0, pos_y - 1),
                Command::Right => temp_y = min(4, pos_y + 1),
                Command::Down => temp_x = min(4, pos_x + 1),
            }
            if self.fields[temp_x as usize][temp_y as usize] == '_' {
                continue;
            } else {
                self.pos = (temp_x as usize, temp_y as usize);
            }
        }

        self.fields[self.pos.0][self.pos.1]
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Command::Up),
            "L" => Ok(Command::Left),
            "R" => Ok(Command::Right),
            "D" => Ok(Command::Down),
            _ => Err("Invalid command: {}".to_owned()),
        }
    }
}
