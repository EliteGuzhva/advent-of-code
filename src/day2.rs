use std::str::FromStr;


#[derive(Debug)]
enum Direction {
    FORWARD,
    DOWN,
    UP
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::FORWARD),
            "down" => Ok(Direction::DOWN),
            "up" => Ok(Direction::UP),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
struct Command {
    dir: Direction,
    value: i32
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(());
        }

        let data: Vec<&str> = s.split(' ').collect();
        let dir = Direction::from_str(data[0]).unwrap();
        let value = data[1].parse::<i32>().unwrap();

        Ok(Command{dir, value})
    }
}

#[derive(Debug)]
struct Position {
    x: i32,
    depth: i32,
    aim: i32
}

pub fn solve(data: &mut String) {
    let commands: Vec<Command> = data.split('\n').into_iter()
        .filter_map(|v| Command::from_str(v).ok())
        .collect();

    // Part 1
    let mut position = Position{x: 0, depth: 0, aim: 0};
    for command in &commands {
        match command.dir {
            Direction::FORWARD => position.x += command.value,
            Direction::DOWN => position.depth += command.value,
            Direction::UP => position.depth -= command.value
        }
    }

    let answer = position.x * position.depth;
    println!("1. Answer: {:?}, {}", position, answer);

    // Part 2
    let mut position = Position{x: 0, depth: 0, aim: 0};
    for command in &commands {
        match command.dir {
            Direction::FORWARD => {
                position.x += command.value;
                position.depth += command.value * position.aim;
            },
            Direction::DOWN => position.aim += command.value,
            Direction::UP => position.aim -= command.value
        }
    }

    let answer = position.x * position.depth;
    println!("2. Answer: {:?}, {}", position, answer);
}
