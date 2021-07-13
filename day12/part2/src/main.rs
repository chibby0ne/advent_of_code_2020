// Before you can give the destination to the captain, you realize that the actual action meanings
// were printed on the back of the instructions the whole time.

// Almost all of the actions indicate how to move a waypoint which is relative to the ship's
// position:

//     Action N means to move the waypoint north by the given value.
//     Action S means to move the waypoint south by the given value.
//     Action E means to move the waypoint east by the given value.
//     Action W means to move the waypoint west by the given value.
//     Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
//     Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
//     Action F means to move forward to the waypoint a number of times equal to the given value.

// The waypoint starts 10 units east and 1 unit north relative to the ship. The waypoint is
// relative to the ship; that is, if the ship moves, the waypoint moves with it.

// For example, using the same instructions as above:

//     F10 moves the ship to the waypoint 10 times (a total of 100 units east and 10 units north), leaving the ship at east 100, north 10. The waypoint stays 10 units east and 1 unit north of the ship.
//     N3 moves the waypoint 3 units north to 10 units east and 4 units north of the ship. The ship remains at east 100, north 10.
//     F7 moves the ship to the waypoint 7 times (a total of 70 units east and 28 units north), leaving the ship at east 170, north 38. The waypoint stays 10 units east and 4 units north of the ship.
//     R90 rotates the waypoint around the ship clockwise 90 degrees, moving it to 4 units east and 10 units south of the ship. The ship remains at east 170, north 38.
//     F11 moves the ship to the waypoint 11 times (a total of 44 units east and 110 units south), leaving the ship at east 214, south 72. The waypoint stays 4 units east and 10 units south of the ship.

// After these operations, the ship's Manhattan distance from its starting position is 214 + 72 =
// 286.

// Figure out where the navigation instructions actually lead. What is the Manhattan distance
// between that location and the ship's starting position?

use std::io;
use std::io::BufRead;
use std::ops;

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

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Position { x, y }
    }

    fn move_east(&mut self, x: i64) {
        self.x += x
    }

    fn move_west(&mut self, x: i64) {
        self.x -= x
    }

    fn move_north(&mut self, y: i64) {
        self.y += y
    }

    fn move_south(&mut self, y: i64) {
        self.y -= y
    }

    fn manhattan_distance(self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    fn move_in_waypoint_direction(&mut self, waypoint: &Position, val: i64) {
        self.x += waypoint.x * val;
        self.y += waypoint.y * val;
    }
}

impl ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Position {
        Position::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

fn rotate(relative_waypoint_position: &Position, rotation: Rotation) -> Position {
    match rotation {
        Rotation::Left(90) | Rotation::Right(270) => {
            Position::new(-relative_waypoint_position.y, relative_waypoint_position.x)
        }
        Rotation::Left(180) | Rotation::Right(180) => {
            Position::new(-relative_waypoint_position.x, -relative_waypoint_position.y)
        }
        Rotation::Left(270) | Rotation::Right(90) => {
            Position::new(relative_waypoint_position.y, -relative_waypoint_position.x)
        }
        _ => panic!("unhandled rotation angle"),
    }
}

fn navigate(actions: Vec<Action>) -> (Position, Position) {
    let mut ship_position = Position::new(0, 0); // Initial position in cartesian coordinates (0, 0)
    let mut relative_position = Position::new(10, 1);
    let mut waypoint_position = ship_position + relative_position;
    for action in actions {
        match action {
            Action::East(val) => relative_position.move_east(val),
            Action::West(val) => relative_position.move_west(val),
            Action::North(val) => relative_position.move_north(val),
            Action::South(val) => relative_position.move_south(val),
            Action::Rotation(rotation) => relative_position = rotate(&relative_position, rotation),
            Action::Forward(val) => {
                ship_position.move_in_waypoint_direction(&relative_position, val)
            }
        }
        waypoint_position = relative_position + ship_position;
    }
    (ship_position, waypoint_position)
}

fn main() {
    let stdin = io::stdin();
    let actions: Vec<Action> = stdin
        .lock()
        .lines()
        .filter_map(|x| x.ok())
        .map(Action::from)
        .collect();
    let (position, _) = navigate(actions);
    println!("{}", position.manhattan_distance());
}
