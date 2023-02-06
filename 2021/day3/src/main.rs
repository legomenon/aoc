use std::{cmp, collections::HashMap, fs};

fn main() {
    let file = fs::read_to_string("file.txt").unwrap();
    let len = file.lines().take(1).map(|x| x.len()).sum::<usize>();

    let mut diag_vec = create_diagnostic_vec(len);
    get_frequency(&file, &mut diag_vec);

    let p1 = get_gamma_rate(&diag_vec);
    let p1_1 = get_epsilon_rate(&diag_vec);

    dbg!(&p1, &p1_1, get_power_consumption(&p1, &p1_1));
}

fn create_diagnostic_vec(len: usize) -> Vec<HashMap<char, u32>> {
    let mut diag_vec: Vec<HashMap<char, u32>> = Vec::with_capacity(len);
    for _i in 0..len {
        diag_vec.push(HashMap::new())
    }
    diag_vec
}

fn get_gamma_rate(diag_vec: &Vec<HashMap<char, u32>>) -> String {
    let mut gamma = String::new();
    for map in diag_vec {
        let v0 = map.get(&'0').unwrap();
        let v1 = map.get(&'1').unwrap();

        match v0.cmp(v1) {
            cmp::Ordering::Less | cmp::Ordering::Equal => gamma.push('1'),
            cmp::Ordering::Greater => gamma.push('0'),
        };
    }

    gamma
}

fn get_epsilon_rate(diag_vec: &Vec<HashMap<char, u32>>) -> String {
    let gamma = get_gamma_rate(diag_vec);
    gamma
        .chars()
        .map(|c| match c {
            '0' => '1',
            '1' => '0',
            _ => '0',
        })
        .collect::<String>()
}

fn get_power_consumption(gamma: &str, epsilon: &str) -> u32 {
    let gamma = u32::from_str_radix(gamma, 2).unwrap();
    let epsilon = u32::from_str_radix(epsilon, 2).unwrap();

    gamma * epsilon
}

fn get_frequency(file: &str, diag_vec: &mut Vec<HashMap<char, u32>>) {
    file.lines()
        .map(|x| {
            x.chars()
                .enumerate()
                .map(|(i, c)| {
                    diag_vec[i]
                        .entry(c)
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);
                })
                .count()
        })
        .count();
}
