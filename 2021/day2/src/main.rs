use std::{fs, str::FromStr};

#[derive(Debug, Default)]
struct SubPosition {
    x: u32,
    y: u32,
}
#[derive(Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn main() {
    let file = fs::read_to_string("file.txt").unwrap();
    let result = file
        .lines()
        .map(|x| x.parse::<Command>().unwrap())
        .collect::<Vec<Command>>();

    let mut sub = SubPosition::default();
    sub.execute_commands(result);
    dbg!(sub.result_position());
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, pos) = s.split_once(' ').unwrap();
        let pos = pos.parse::<u32>().unwrap();
        match command.trim() {
            "forward" => Ok(Command::Forward(pos)),
            "down" => Ok(Command::Down(pos)),
            "up" => Ok(Command::Up(pos)),
            _ => Err("Invalid command".to_owned()),
        }
    }
}

impl SubPosition {
    fn execute_commands(&mut self, commands: Vec<Command>) {
        for command in commands {
            match command {
                Command::Forward(pos) => self.x += pos,
                Command::Down(pos) => self.y += pos,
                Command::Up(pos) => self.y -= pos,
            }
        }
    }

    fn result_position(self) -> u32 {
        self.x * self.y
    }
}
