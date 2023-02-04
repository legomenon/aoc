use std::fs;

enum Opcode {
    Add = 1,
    Mul = 2,
    Ext = 99,
}

fn main() {
    let file = fs::read_to_string("file.txt").unwrap();
    let p1 = part1(&file);
    dbg!(p1);
}

fn part1(file: &str) -> usize {
    let mut intcode_vec = file
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let i_iter = intcode_vec.clone();
    let mut i_iter = i_iter.iter();

    loop {
        let opcode = Opcode::try_from(*i_iter.next().unwrap()).unwrap();
        match opcode {
            Opcode::Add => {
                let a = i_iter.next().unwrap();
                let b = i_iter.next().unwrap();
                let c = i_iter.next().unwrap();
                intcode_vec[*c] = intcode_vec[*a] + intcode_vec[*b];
            }
            Opcode::Mul => {
                let a = i_iter.next().unwrap();
                let b = i_iter.next().unwrap();
                let c = i_iter.next().unwrap();
                intcode_vec[*c] = intcode_vec[*a] * intcode_vec[*b]
            }
            Opcode::Ext => break,
        }
    }
    intcode_vec[0]
}

impl TryFrom<usize> for Opcode {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            i if i == Opcode::Add as usize => Ok(Opcode::Add),
            i if i == Opcode::Mul as usize => Ok(Opcode::Mul),
            i if i == Opcode::Ext as usize => Ok(Opcode::Ext),
            _ => Err("invalid opcode value".to_owned()),
        }
    }
}
