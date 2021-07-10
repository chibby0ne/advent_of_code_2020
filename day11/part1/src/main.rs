// Your plane lands with plenty of time to spare. The final leg of your journey is a ferry that
// goes directly to the tropical island where you can finally start your vacation. As you reach the
// waiting area to board the ferry, you realize you're so early, nobody else has even arrived yet!

// By modeling the process people use to choose (or abandon) their seat in the waiting area, you're
// pretty sure you can predict the best place to sit. You make a quick map of the seat layout (your
// puzzle input).

// The seat layout fits neatly on a grid. Each position is either floor (.), an empty seat (L), or
// an occupied seat (#). For example, the initial seat layout might look like this:

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

// Now, you just need to model the people who will be arriving shortly. Fortunately, people are
// entirely predictable and always follow a simple set of rules. All decisions are based on the
// number of occupied seats adjacent to a given seat (one of the eight positions immediately up,
// down, left, right, or diagonal from the seat). The following rules are applied to every seat
// simultaneously:

//     If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
//     If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
//     Otherwise, the seat's state does not change.

// Floor (.) never changes; seats don't move, and nobody sits on the floor.

// After one round of these rules, every seat in the example layout becomes occupied:

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

// After a second round, the seats with four or more occupied adjacent seats become empty again:

// #.LL.L#.##
// #LLLLLL.L#
// L.L.L..L..
// #LLL.LL.L#
// #.LL.LL.LL
// #.LLLL#.##
// ..L.L.....
// #LLLLLLLL#
// #.LLLLLL.L
// #.#LLLL.##

// This process continues for three more rounds:

// #.##.L#.##
// #L###LL.L#
// L.#.#..#..
// #L##.##.L#
// #.##.LL.LL
// #.###L#.##
// ..#.#.....
// #L######L#
// #.LL###L.L
// #.#L###.##

// #.#L.L#.##
// #LLL#LL.L#
// L.L.L..#..
// #LLL.##.L#
// #.LL.LL.LL
// #.LL#L#.##
// ..L.L.....
// #L#LLLL#L#
// #.LLLLLL.L
// #.#L#L#.##

// #.#L.L#.##
// #LLL#LL.L#
// L.#.L..#..
// #L##.##.L#
// #.#L.LL.LL
// #.#L#L#.##
// ..L.L.....
// #L#L##L#L#
// #.LLLLLL.L
// #.#L#L#.##

// At this point, something interesting happens: the chaos stabilizes and further applications of
// these rules cause no seats to change state! Once people stop moving around, you count 37
// occupied seats.

// Simulate your seating area by applying the seating rules repeatedly until no seats change state.
// How many seats end up occupied?

use std::io;
use std::io::BufRead;

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

fn is_seat_in_corner(coordinate_x: usize, coordinate_y: usize, max_x: usize, max_y: usize) -> bool {
    (coordinate_x == max_x - 1 || coordinate_x == 0)
        && (coordinate_y == max_y - 1 || coordinate_y == 0)
}

fn check_seat(
    input: &[Vec<SeatState>],
    coordinate_x: i64,
    coordinate_y: i64,
    state: SeatState,
) -> Option<bool> {
    if let Some(v) = input.get(coordinate_x as usize) {
        if let Some(&vv) = v.get(coordinate_y as usize) {
            if vv == state {
                return Some(true);
            } else {
                return Some(false);
            }
        }
    }
    None
}

fn get_seat_state(input: &[Vec<SeatState>], coordinate_x: usize, coordinate_y: usize) -> SeatState {
    input[coordinate_x][coordinate_y]
}

fn are_seats_unchanged(a: &[Vec<SeatState>], b: &[Vec<SeatState>]) -> bool {
    a == b
}

fn is_seat_occupied(
    input: &[Vec<SeatState>],
    coordinate_x: i64,
    coordinate_y: i64,
) -> Option<bool> {
    check_seat(input, coordinate_x, coordinate_y, SeatState::Occupied)
}

fn string_repr_of_vec_of_vec(vec: &[Vec<SeatState>]) -> String {
    let mut s = String::new();
    for sub_vec in vec {
        for &ch in sub_vec {
            s.push(ch.into())
        }
        s.push('\n');
    }
    s
}

fn are_at_least_four_surrounding_seats_occupied(
    input: &[Vec<SeatState>],
    coordinate_x: usize,
    coordinate_y: usize,
) -> bool {
    let mut count = 0;
    for i in -1..2_i64 {
        for j in -1..2_i64 {
            if i == 0 && j == 0 {
                continue;
            }
            let ans = is_seat_occupied(input, coordinate_x as i64 + i, coordinate_y as i64 + j);
            if let Some(val) = ans {
                if val {
                    count += 1;
                }
            }
        }
    }
    count >= 4
}

// If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes
// occupied.
fn are_no_occupied_seats_surrounding(
    input: &[Vec<SeatState>],
    coordinate_x: usize,
    coordinate_y: usize,
) -> bool {
    for i in -1..2_i64 {
        for j in -1..2_i64 {
            if i == 0 && j == 0 {
                continue;
            }
            if let Some(ans) =
                is_seat_occupied(input, coordinate_x as i64 + i, coordinate_y as i64 + j)
            {
                if ans {
                    return false;
                }
            }
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
    let max_x = input.len();
    let max_y = input.get(0).unwrap().len();

    for i in 0..max_x {
        let mut row: Vec<SeatState> = Vec::new();
        for j in 0..max_y {
            match get_seat_state(input, i, j) {
                SeatState::Empty => {
                    if are_no_occupied_seats_surrounding(input, i, j) {
                        row.push(SeatState::Occupied);
                    } else {
                        row.push(SeatState::Empty);
                    }
                }
                SeatState::Occupied => {
                    if !is_seat_in_corner(i, j, max_x, max_y)
                        && are_at_least_four_surrounding_seats_occupied(input, i, j)
                    {
                        row.push(SeatState::Empty);
                    } else {
                        row.push(SeatState::Occupied);
                    }
                }
                SeatState::Floor => {
                    row.push(SeatState::Floor);
                }
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
