use std::{fs, str::FromStr};

fn main() {
    let file = fs::read_to_string("file.txt").unwrap();

    let c = part1(&file);
    let d = part2(&file);

    dbg!(c);
    dbg!(d);
}

fn part1(file: &str) -> u32 {
    file.lines()
        .map(|x| x.parse::<Assignment>().unwrap())
        .filter(|x| x.task_left.contains(&x.task_right))
        .count() as u32
}

fn part2(file: &str) -> u32 {
    file.lines()
        .map(|x| x.parse::<Assignment>().unwrap())
        .filter(|x| x.task_left.partially_contains(&x.task_right))
        .count() as u32
}

#[derive(Debug)]
struct Assignment {
    task_left: Task,
    task_right: Task,
}

#[derive(Debug)]
struct Task {
    task: (u32, u32),
}

impl Task {
    fn contains(&self, other: &Task) -> bool {
        (self.task.0..=self.task.1).contains(&other.task.0)
            && (self.task.0..=self.task.1).contains(&other.task.1)
            || (other.task.0..=other.task.1).contains(&self.task.0)
                && (other.task.0..=other.task.1).contains(&self.task.1)
    }

    fn partially_contains(&self, other: &Task) -> bool {
        (self.task.0..=self.task.1).contains(&other.task.0)
            || (self.task.0..=self.task.1).contains(&other.task.1)
            || (other.task.0..=other.task.1).contains(&self.task.0)
            || (other.task.0..=other.task.1).contains(&self.task.1)
    }
}

impl FromStr for Assignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(',').unwrap();
        let left = left.parse::<Task>().unwrap();
        let right = right.parse::<Task>().unwrap();
        Ok(Self {
            task_left: left,
            task_right: right,
        })
    }
}

impl FromStr for Task {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.trim().split_once('-').unwrap();
        let left = left.parse::<u32>().unwrap();
        let right = right.parse::<u32>().unwrap();

        Ok(Self {
            task: (left, right),
        })
    }
}
