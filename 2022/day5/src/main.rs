use std::{fs, str::FromStr};
#[derive(Debug)]
struct Command {
    quantity: usize,
    from: usize,
    to: usize,
}

fn main() {
    let a = fs::read_to_string("file.txt").unwrap();
    let commands = parse_commands(&a);
    let cargo = parse_crates(&a);
    // dbg!(execute_commands(commands, cargo));
    dbg!(execute_commands_at_once(commands, cargo));
}

fn parse_crates(file: &str) -> Vec<Vec<String>> {
    let (cargo, _other) = file.split_once("\n\n").unwrap();
    let mut temp_vec = cargo
        .lines()
        .map(|x| x.replace("    ", " [_] "))
        .collect::<Vec<String>>()
        .iter()
        .map(|x| x.trim())
        .map(|x| x.replace("  ", " "))
        .collect::<Vec<String>>();

    let _p = temp_vec.pop();

    let result_vec = temp_vec
        .iter()
        .map(|x| x.split(' ').collect::<Vec<_>>())
        .map(|x| {
            x.iter()
                .map(|x| x.replace("[", "").replace("]", ""))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    let result_vec = transpose_rev(result_vec);

    println!("");

    for i in result_vec.iter() {
        println!("{:?}", i)
    }
    result_vec
}

fn transpose_rev<T: std::cmp::PartialEq<str>>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .map(|n| n.next().unwrap())
                .filter(|n| n != "_")
                .collect::<Vec<T>>()
        })
        .collect()
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let temp_com = s.split(' ').collect::<Vec<&str>>();

        if temp_com.len() < 6 {
            return Err("invalid command".to_owned());
        }
        let quantity = temp_com[1].parse::<usize>().unwrap();
        let from = temp_com[3].parse::<usize>().unwrap() - 1;
        let to = temp_com[5].parse::<usize>().unwrap() - 1;

        Ok(Self { quantity, from, to })
    }
}
fn parse_commands(file: &str) -> Vec<Command> {
    let (_other, file_commands) = file.split_once("\n\n").unwrap();
    let commands = file_commands
        .split("\n")
        .map(|x| x.parse::<Command>().unwrap())
        .collect::<Vec<Command>>();
    // dbg!(&commands);
    commands
}

fn execute_commands(commands: Vec<Command>, cargo: Vec<Vec<String>>) -> String {
    let mut cargo = cargo;
    for command in commands.iter() {
        for _i in 0..command.quantity {
            let crg = cargo[command.from].pop().unwrap();
            cargo[command.to].push(crg)
        }
    }
    // dbg!(&cargo);
    let result = cargo
        .into_iter()
        .map(move |x| x.last().unwrap().clone())
        .collect::<Vec<String>>()
        .join("");

    result
}

fn execute_commands_at_once(commands: Vec<Command>, cargo: Vec<Vec<String>>) -> String {
    let mut cargo = cargo;
    for command in commands.iter() {
        let mut result_vec: Vec<String> = Vec::new();
        for _i in 0..command.quantity {
            let crg = cargo[command.from].pop().unwrap();
            result_vec.push(crg);
            // cargo[command.to].push(crg)
        }
        let mut result_vec = result_vec.into_iter().rev().collect::<Vec<String>>();
        cargo[command.to].append(&mut result_vec);
    }
    // dbg!(&cargo);
    let result = cargo
        .into_iter()
        .map(move |x| x.last().unwrap().clone())
        .collect::<Vec<String>>()
        .join("");

    result
}
