use std::fs;

#[derive(Debug)]
enum Opcode {
    Add = 1,
    Mul = 2,
    Ext = 99,
}

fn main() {
    let file = fs::read_to_string("file.txt").unwrap();
    let p1 = part1(&file);
    let p2 = part2(&file, 19690720);
    dbg!(p1, p2);
}

fn part1(file: &str) -> usize {
    let mut intcode_vec = parse(file);

    intcode_vec[1] = 12;
    intcode_vec[2] = 2;

    let i_iter = intcode_vec.clone();
    let mut i_iter = i_iter.into_iter();

    loop {
        let opcode = Opcode::try_from(i_iter.next().unwrap()).expect("invalid opcode value");
        match opcode {
            Opcode::Add => {
                let a = i_iter.next().unwrap();
                let b = i_iter.next().unwrap();
                let c = i_iter.next().unwrap();
                intcode_vec[c] = intcode_vec[a] + intcode_vec[b];
            }
            Opcode::Mul => {
                let a = i_iter.next().unwrap();
                let b = i_iter.next().unwrap();
                let c = i_iter.next().unwrap();
                intcode_vec[c] = intcode_vec[a] * intcode_vec[b]
            }
            Opcode::Ext => break,
        }
    }
    intcode_vec[0]
}

fn part2(file: &str, num: usize) -> usize {
    for i in 0..=99 {
        for j in 0..=99 {
            let mut intcode_vec = parse(&file);
            let i_iter = intcode_vec.clone();
            let mut i_iter = i_iter.into_iter();

            intcode_vec[1] = i;
            intcode_vec[2] = j;

            loop {
                let opcode = match Opcode::try_from(i_iter.next().unwrap()) {
                    Ok(opcode) => opcode,
                    Err(_) => continue,
                };

                match opcode {
                    Opcode::Add => {
                        let a = i_iter.next().unwrap();
                        let b = i_iter.next().unwrap();
                        let c = i_iter.next().unwrap();
                        intcode_vec[c] = intcode_vec[a] + intcode_vec[b];
                    }
                    Opcode::Mul => {
                        let a = i_iter.next().unwrap();
                        let b = i_iter.next().unwrap();
                        let c = i_iter.next().unwrap();
                        intcode_vec[c] = intcode_vec[a] * intcode_vec[b]
                    }
                    Opcode::Ext => {
                        if intcode_vec[0] == num {
                            return 100 * intcode_vec[1] + intcode_vec[2];
                        }
                        break;
                    }
                }
            }
        }
    }
    0
}

impl TryFrom<usize> for Opcode {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            i if i == Opcode::Add as usize => Ok(Opcode::Add),
            i if i == Opcode::Mul as usize => Ok(Opcode::Mul),
            i if i == Opcode::Ext as usize => Ok(Opcode::Ext),
            _ => Err("ivalid opcode".to_owned()),
        }
    }
}

fn parse(file: &str) -> Vec<usize> {
    file.split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}
