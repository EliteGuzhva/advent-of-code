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
    y: i32
}

pub fn solve(data: &mut String) {
    let commands: Vec<Command> = data.split('\n').into_iter()
        .filter_map(|v| Command::from_str(v).ok())
        .collect();

    let mut position = Position{x: 0, y: 0};
    for command in commands {
        match command.dir {
            Direction::FORWARD => position.x += command.value,
            Direction::DOWN => position.y += command.value,
            Direction::UP => position.y -= command.value
        }
    }

    let answer = position.x * position.y;
    println!("Answer: {:?}, {}", position, answer);
}
