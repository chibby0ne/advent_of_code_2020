// Your ferry made decent progress toward the island, but the storm came in faster than anyone
// expected. The ferry needs to take evasive actions!

// Unfortunately, the ship's navigation computer seems to be malfunctioning; rather than giving a
// route directly to safety, it produced extremely circuitous instructions. When the captain uses
// the PA system to ask if anyone can help, you quickly volunteer.

// The navigation instructions (your puzzle input) consists of a sequence of single-character
// actions paired with integer input values. After staring at them for a few minutes, you work out
// what they probably mean:

//     Action N means to move north by the given value.
//     Action S means to move south by the given value.
//     Action E means to move east by the given value.
//     Action W means to move west by the given value.
//     Action L means to turn left the given number of degrees.
//     Action R means to turn right the given number of degrees.
//     Action F means to move forward by the given value in the direction the ship is currently facing.

// The ship starts by facing east. Only the L and R actions change the direction the ship is
// facing. (That is, if the ship is facing east and the next instruction is N10, the ship would
// move north 10 units, but would still move east if the following action were F.)

// For example:

// F10
// N3
// F7
// R90
// F11

// These instructions would be handled as follows:

//     F10 would move the ship 10 units east (because the ship starts by facing east) to east 10, north 0.
//     N3 would move the ship 3 units north to east 10, north 3.
//     F7 would move the ship another 7 units east (because the ship is still facing east) to east 17, north 3.
//     R90 would cause the ship to turn right by 90 degrees and face south; it remains at east 17, north 3.
//     F11 would move the ship 11 units south to east 17, south 8.

// At the end of these instructions, the ship's Manhattan distance (sum of the absolute values of
// its east/west position and its north/south position) from its starting position is 17 + 8 = 25.

// Figure out where the navigation instructions lead. What is the Manhattan distance between that
// location and the ship's starting position?

use std::io;
use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
enum Rotation {
    Left(i64),
    Right(i64),
}

#[derive(Debug, Clone, Copy)]
enum Action {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Forward(i64),
    Rotation(Rotation),
}

impl From<String> for Action {
    fn from(s: String) -> Action {
        if s.len() < 2 {
            panic!("All actions should have one letter and a number, so at least 2 chars")
        }
        let direction = s.chars().next();
        let amount = s[1..].parse::<u64>().expect("Not a positive number") as i64;
        match direction {
            Some('N') => Action::North(amount),
            Some('S') => Action::South(amount),
            Some('E') => Action::East(amount),
            Some('W') => Action::West(amount),
            Some('L') => Action::Rotation(Rotation::Left(amount)),
            Some('R') => Action::Rotation(Rotation::Right(amount)),
            Some('F') => Action::Forward(amount),
            _ => panic!("Not a recognized action"),
        }
    }
}

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Position { x, y }
    }

    fn move_east(&mut self, y: i64) {
        self.y += y
    }

    fn move_west(&mut self, y: i64) {
        self.y -= y
    }

    fn move_north(&mut self, x: i64) {
        self.x += x
    }

    fn move_south(&mut self, x: i64) {
        self.x -= x
    }

    fn manhattan_distance(self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug)]
enum Orientation {
    North,
    South,
    East,
    West,
}

fn rotate(orientation: Orientation, rotation: Rotation) -> Orientation {
    match orientation {
        Orientation::East => match rotation {
            Rotation::Left(90) | Rotation::Right(270) => Orientation::North,
            Rotation::Left(180) | Rotation::Right(180) => Orientation::West,
            Rotation::Left(270) | Rotation::Right(90) => Orientation::South,
            _ => panic!("unhandled rotation angle"),
        },
        Orientation::West => match rotation {
            Rotation::Left(90) | Rotation::Right(270) => Orientation::South,
            Rotation::Left(180) | Rotation::Right(180) => Orientation::East,
            Rotation::Left(270) | Rotation::Right(90) => Orientation::North,
            _ => panic!("unhandled rotation angle"),
        },
        Orientation::North => match rotation {
            Rotation::Left(90) | Rotation::Right(270) => Orientation::West,
            Rotation::Left(180) | Rotation::Right(180) => Orientation::South,
            Rotation::Left(270) | Rotation::Right(90) => Orientation::East,
            _ => panic!("unhandled rotation angle"),
        },
        Orientation::South => match rotation {
            Rotation::Left(90) | Rotation::Right(270) => Orientation::East,
            Rotation::Left(180) | Rotation::Right(180) => Orientation::North,
            Rotation::Left(270) | Rotation::Right(90) => Orientation::West,
            _ => panic!("unhandled rotation angle"),
        },
    }
}

fn navigate(actions: Vec<Action>) -> (Position, Orientation) {
    let mut position = Position::new(0, 0); // Initial position in cartesian coordinates (0, 0)
    let mut orientation = Orientation::East; // Initial orientation is East
    for action in actions {
        match action {
            Action::East(val) => position.move_east(val),
            Action::West(val) => position.move_west(val),
            Action::North(val) => position.move_north(val),
            Action::South(val) => position.move_south(val),
            Action::Rotation(rotation) => orientation = rotate(orientation, rotation),
            Action::Forward(val) => match orientation {
                Orientation::East => position.move_east(val),
                Orientation::West => position.move_west(val),
                Orientation::North => position.move_north(val),
                Orientation::South => position.move_south(val),
            },
        }
    }
    (position, orientation)
}

fn main() {
    let stdin = io::stdin();
    let actions: Vec<Action> = stdin
        .lock()
        .lines()
        .filter_map(|x| x.ok())
        .map(Action::from)
        .collect();
    let (position, _) = dbg!(navigate(actions));
    println!("{}", position.manhattan_distance());
}
