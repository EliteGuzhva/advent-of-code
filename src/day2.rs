use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    FORWARD,
    DOWN,
    UP,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::FORWARD),
            "down" => Ok(Direction::DOWN),
            "up" => Ok(Direction::UP),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Command {
    dir: Direction,
    value: i32,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(());
        }

        let data: Vec<&str> = s.split(' ').collect();
        let dir = data[0].parse::<Direction>().unwrap();
        let value = data[1].parse::<i32>().unwrap();

        Ok(Command { dir, value })
    }
}

#[derive(Debug)]
struct Position {
    x: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn new() -> Position {
        Position {
            x: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn answer(self) -> i32 {
        self.x * self.depth
    }
}

pub fn solve(data: &mut String) {
    let commands = aoc::input_to_vec::<Command>(data);

    let res1 = commands
        .iter()
        .fold(Position::new(), |pos, c| match c.dir {
            Direction::FORWARD => Position {
                x: pos.x + c.value,
                ..pos
            },
            Direction::DOWN => Position {
                depth: pos.depth + c.value,
                ..pos
            },
            Direction::UP => Position {
                depth: pos.depth - c.value,
                ..pos
            },
        })
        .answer();
    println!("Part 1: {}", res1);

    let res2 = commands
        .iter()
        .fold(Position::new(), |pos, c| match c.dir {
            Direction::FORWARD => Position {
                x: pos.x + c.value,
                depth: pos.depth + c.value * pos.aim,
                ..pos
            },
            Direction::DOWN => Position {
                aim: pos.aim + c.value,
                ..pos
            },
            Direction::UP => Position {
                aim: pos.aim - c.value,
                ..pos
            },
        })
        .answer();
    println!("Part 2: {}", res2);
}
