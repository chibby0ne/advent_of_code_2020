// As soon as people start to arrive, you realize your mistake. People don't just care about adjacent seats - they care about the first seat they can see in each of those eight directions!

// Now, instead of considering just the eight immediately adjacent seats, consider the first seat in each of those eight directions. For example, the empty seat below would see eight occupied seats:

// .......#.
// ...#.....
// .#.......
// .........
// ..#L....#
// ....#....
// .........
// #........
// ...#.....

// The leftmost empty seat below would only see one empty seat, but cannot see any of the occupied ones:

// .............
// .L.L.#.#.#.#.
// .............

// The empty seat below would see no occupied seats:

// .##.##.
// #.#.#.#
// ##...##
// ...L...
// ##...##
// #.#.#.#
// .##.##.

// Also, people seem to be more tolerant than you expected: it now takes five or more visible occupied seats for an occupied seat to become empty (rather than four or more from the previous rules). The other rules still apply: empty seats that see no occupied seats become occupied, seats matching no rule don't change, and floor never changes.

// Given the same starting layout as above, these new rules cause the seating area to shift around as follows:

// L.LL.LL.LL
// LLLLLLL.LL
// L.L.L..L..
// LLLL.LL.LL
// L.LL.LL.LL
// L.LLLLL.LL
// ..L.L.....
// LLLLLLLLLL
// L.LLLLLL.L
// L.LLLLL.LL

// #.##.##.##
// #######.##
// #.#.#..#..
// ####.##.##
// #.##.##.##
// #.#####.##
// ..#.#.....
// ##########
// #.######.#
// #.#####.##

// #.LL.LL.L#
// #LLLLLL.LL
// L.L.L..L..
// LLLL.LL.LL
// L.LL.LL.LL
// L.LLLLL.LL
// ..L.L.....
// LLLLLLLLL#
// #.LLLLLL.L
// #.LLLLL.L#

// #.L#.##.L#
// #L#####.LL
// L.#.#..#..
// ##L#.##.##
// #.##.#L.##
// #.#####.#L
// ..#.#.....
// LLL####LL#
// #.L#####.L
// #.L####.L#

// #.L#.L#.L#
// #LLLLLL.LL
// L.L.L..#..
// ##LL.LL.L#
// L.LL.LL.L#
// #.LLLLL.LL
// ..L.L.....
// LLLLLLLLL#
// #.LLLLL#.L
// #.L#LL#.L#

// #.L#.L#.L#
// #LLLLLL.LL
// L.L.L..#..
// ##L#.#L.L#
// L.L#.#L.L#
// #.L####.LL
// ..#.#.....
// LLL###LLL#
// #.LLLLL#.L
// #.L#LL#.L#

// #.L#.L#.L#
// #LLLLLL.LL
// L.L.L..#..
// ##L#.#L.L#
// L.L#.LL.L#
// #.LLLL#.LL
// ..#.L.....
// LLL###LLL#
// #.LLLLL#.L
// #.L#LL#.L#

// Again, at this point, people stop shifting around and the seating area reaches equilibrium. Once this occurs, you count 26 occupied seats.

// Given the new visibility method and the rule change for occupied seats becoming empty, once equilibrium is reached, how many seats end up occupied?

use std::io;
use std::io::BufRead;

const THRESHOLD: usize = 5;

#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(u8)]
enum SeatState {
    Empty = b'L',
    Occupied = b'#',
    Floor = b'.',
}

impl Into<char> for SeatState {
    fn into(self) -> char {
        self as u8 as char
    }
}

impl SeatState {
    pub fn from(c: char) -> Result<SeatState, ()> {
        match c {
            'L' => Ok(SeatState::Empty),
            '#' => Ok(SeatState::Occupied),
            '.' => Ok(SeatState::Floor),
            _ => Err(()),
        }
    }
}

fn is_seat_in_corner(coordinate_x: i64, coordinate_y: i64, max_x: i64, max_y: i64) -> bool {
    (coordinate_x == max_x - 1 || coordinate_x == 0)
        && (coordinate_y == max_y - 1 || coordinate_y == 0)
}

fn get_seat_state(
    input: &[Vec<SeatState>],
    coordinate_x: i64,
    coordinate_y: i64,
) -> Option<SeatState> {
    if let Some(row) = input.get(coordinate_x as usize) {
        if let Some(&state) = row.get(coordinate_y as usize) {
            return Some(state);
        }
    }
    None
}

fn are_seats_unchanged(a: &[Vec<SeatState>], b: &[Vec<SeatState>]) -> bool {
    a == b
}

fn are_at_least_four_surrounding_seats_occupied(
    input: &[Vec<SeatState>],
    coordinate_x: i64,
    coordinate_y: i64,
) -> bool {
    let mut count = 0;
    // Up
    for i in 1.. {
        match get_seat_state(input, coordinate_x - i, coordinate_y) {
            Some(SeatState::Empty) => break,
            Some(SeatState::Occupied) => {
                count += 1;
                break;
            }
            Some(SeatState::Floor) => continue,
            None => break,
        }
    }
    // Down
    for i in 1.. {
        match get_seat_state(input, coordinate_x + i, coordinate_y) {
            Some(SeatState::Empty) => break,
            Some(SeatState::Occupied) => {
                count += 1;
                break;
            }
            Some(SeatState::Floor) => continue,
            None => break,
        }
    }
    // Left
    for i in 1.. {
        match get_seat_state(input, coordinate_x, coordinate_y - i) {
            Some(SeatState::Empty) => break,
            Some(SeatState::Occupied) => {
                count += 1;
                break;
            }
            Some(SeatState::Floor) => continue,
            None => break,
        }
    }
    // Right
    for i in 1.. {
        match get_seat_state(input, coordinate_x, coordinate_y + i) {
            Some(SeatState::Empty) => break,
            Some(SeatState::Occupied) => {
                count += 1;
                break;
            }
            Some(SeatState::Floor) => continue,
            None => break,
        }
    }
    // Up left
    for i in 1.. {
        match get_seat_state(input, coordinate_x - i, coordinate_y - i) {
            Some(SeatState::Empty) => break,
            Some(SeatState::Occupied) => {
                count += 1;
                break;
            }
            Some(SeatState::Floor) => continue,
            None => break,
        }
    }
    // Up right
    for i in 1.. {
        match get_seat_state(input, coordinate_x - i, coordinate_y + i) {
            Some(SeatState::Empty) => break,
            Some(SeatState::Occupied) => {
                count += 1;
                break;
            }
            Some(SeatState::Floor) => continue,
            None => break,
        }
    }
    // Down left
    for i in 1.. {
        match get_seat_state(input, coordinate_x + i, coordinate_y - i) {
            Some(SeatState::Empty) => break,
            Some(SeatState::Occupied) => {
                count += 1;
                break;
            }
            Some(SeatState::Floor) => continue,
            None => break,
        }
    }
    // Down right
    for i in 1.. {
        match get_seat_state(input, coordinate_x + i, coordinate_y + i) {
            Some(SeatState::Empty) => break,
            Some(SeatState::Occupied) => {
                count += 1;
                break;
            }
            Some(SeatState::Floor) => continue,
            None => break,
        }
    }
    count >= THRESHOLD
}

// If a seat is empty (L) and there are no occupied seats in any direction to it, the seat becomes
// occupied.
fn are_no_occupied_seats_surrounding(
    input: &[Vec<SeatState>],
    coordinate_x: i64,
    coordinate_y: i64,
) -> bool {
    // Up
    for i in 1.. {
        match get_seat_state(input, coordinate_x - i, coordinate_y) {
            Some(SeatState::Empty) => break,
            Some(SeatState::Occupied) => return false,
            Some(SeatState::Floor) => continue,
            None => break,
        }
    }
    // Down
    for i in 1.. {
        match get_seat_state(input, coordinate_x + i, coordinate_y) {
            Some(SeatState::Empty) => break,
            Some(SeatState::Occupied) => return false,
            Some(SeatState::Floor) => continue,
            None => break,
        }
    }
    // Left
    for i in 1.. {
        match get_seat_state(input, coordinate_x, coordinate_y - i) {
            Some(SeatState::Empty) => break,
            Some(SeatState::Occupied) => return false,
            Some(SeatState::Floor) => continue,
            None => break,
        }
    }
    // Right
    for i in 1.. {
        match get_seat_state(input, coordinate_x, coordinate_y + i) {
            Some(SeatState::Empty) => break,
            Some(SeatState::Occupied) => return false,
            Some(SeatState::Floor) => continue,
            None => break,
        }
    }
    // Up left
    for i in 1.. {
        match get_seat_state(input, coordinate_x - i, coordinate_y - i) {
            Some(SeatState::Empty) => break,
            Some(SeatState::Occupied) => return false,
            Some(SeatState::Floor) => continue,
            None => break,
        }
    }
    // Up right
    for i in 1.. {
        match get_seat_state(input, coordinate_x - i, coordinate_y + i) {
            Some(SeatState::Empty) => break,
            Some(SeatState::Occupied) => return false,
            Some(SeatState::Floor) => continue,
            None => break,
        }
    }
    // Down left
    for i in 1.. {
        match get_seat_state(input, coordinate_x + i, coordinate_y - i) {
            Some(SeatState::Empty) => break,
            Some(SeatState::Occupied) => return false,
            Some(SeatState::Floor) => continue,
            None => break,
        }
    }
    // Down right
    for i in 1.. {
        match get_seat_state(input, coordinate_x + i, coordinate_y + i) {
            Some(SeatState::Empty) => break,
            Some(SeatState::Occupied) => return false,
            Some(SeatState::Floor) => continue,
            None => break,
        }
    }
    true
}

// Apply the rules mutating:
// * the occupied seats -> empty if are_at_least_four_surrounding_seats_occupied
// * the empty seats -> occupied if are_all_surrounding_seats_empty
// * leave floor as floor
fn apply_round(input: &[Vec<SeatState>]) -> Vec<Vec<SeatState>> {
    let mut resulting_seats: Vec<Vec<SeatState>> = Vec::new();
    let max_x = input.len() as i64;
    let max_y = input.get(0).unwrap().len() as i64;

    for i in 0..max_x {
        let mut row: Vec<SeatState> = Vec::new();
        for j in 0..max_y {
            match get_seat_state(input, i, j) {
                Some(SeatState::Empty) => {
                    if are_no_occupied_seats_surrounding(input, i, j) {
                        row.push(SeatState::Occupied);
                    } else {
                        row.push(SeatState::Empty);
                    }
                }
                Some(SeatState::Occupied) => {
                    if !is_seat_in_corner(i, j, max_x, max_y)
                        && are_at_least_four_surrounding_seats_occupied(input, i, j)
                    {
                        row.push(SeatState::Empty);
                    } else {
                        row.push(SeatState::Occupied);
                    }
                }
                Some(SeatState::Floor) => {
                    row.push(SeatState::Floor);
                }
                None => panic!(format!(
                    "This seat is beyond the bounds of the seating plan: x: {}, y: {}",
                    i, j
                )),
            };
        }
        resulting_seats.push(row);
    }
    resulting_seats
}

fn count_number_of_occupied_seats(vec: &[Vec<SeatState>]) -> usize {
    vec.iter().fold(0, |mut acc, x| {
        acc += x.iter().filter(|&&y| y == SeatState::Occupied).count();
        acc
    })
}

fn main() {
    let stdin = io::stdin();
    let mut new_input;
    let mut input: Vec<Vec<SeatState>> = stdin
        .lock()
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| x.chars().filter_map(|y| SeatState::from(y).ok()).collect())
        .collect();
    new_input = apply_round(&input);
    while !are_seats_unchanged(&input, &new_input) {
        input = new_input.clone();
        new_input = apply_round(&input);
    }
    println!("{}", count_number_of_occupied_seats(&new_input));
}
