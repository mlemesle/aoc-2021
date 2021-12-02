use lib;
use std::{ops::Add, str::FromStr};

#[derive(Debug)]
enum SubMarineDirection {
    UP,
    DOWN,
    FORWARD,
}

struct SubMarineCommand {
    direction: SubMarineDirection,
    distance: isize,
}

impl FromStr for SubMarineDirection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(SubMarineDirection::FORWARD),
            "up" => Ok(SubMarineDirection::UP),
            "down" => Ok(SubMarineDirection::DOWN),
            _ => Err(format!(
                "Couldn't deserialize {} into SubMarineDirection",
                s
            )),
        }
    }
}

impl FromStr for SubMarineCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        Ok(SubMarineCommand {
            direction: parts
                .next()
                .expect("")
                .parse::<SubMarineDirection>()
                .expect(""),
            distance: parts.next().expect("").parse::<isize>().expect(""),
        })
    }
}

struct SubMarine {
    horizontal_pos: isize,
    depth: isize,
    aim: isize,
}

impl SubMarine {
    fn new() -> Self {
        SubMarine {
            horizontal_pos: 0,
            depth: 0,
            aim: 0,
        }
    }
}

impl Add<SubMarineCommand> for SubMarine {
    type Output = Self;

    fn add(self, smc: SubMarineCommand) -> Self::Output {
        match smc.direction {
            SubMarineDirection::FORWARD => SubMarine {
                horizontal_pos: self.horizontal_pos + smc.distance,
                depth: self.depth + self.aim * smc.distance,
                aim: self.aim,
            },
            SubMarineDirection::DOWN => SubMarine {
                horizontal_pos: self.horizontal_pos,
                depth: self.depth,
                aim: self.aim + smc.distance,
            },
            SubMarineDirection::UP => SubMarine {
                horizontal_pos: self.horizontal_pos,
                depth: self.depth,
                aim: self.aim - smc.distance,
            },
        }
    }
}

fn parts() {
    let directions = lib::input::<SubMarineCommand>("input.txt");
    let submarine = directions.fold(SubMarine::new(), |submarine, next_command| {
        submarine + next_command
    });

    println!(
        "Depth * position = {}",
        submarine.horizontal_pos * submarine.aim
    );
    println!(
        "Depth * position = {}",
        submarine.horizontal_pos * submarine.depth
    );
}

fn main() {
    parts();
}
