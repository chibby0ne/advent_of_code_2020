// --- Day 15: Rambunctious Recitation ---
//
// You catch the airport shuttle and try to book a new flight to your vacation island. Due to the storm, all direct flights have been cancelled, but a route is available to get around the storm. You take it.
//
// While you wait for your flight, you decide to check in with the Elves back at the North Pole. They're playing a memory game and are ever so excited to explain the rules!
//
// In this game, the players take turns saying numbers. They begin by taking turns reading from a list of starting numbers (your puzzle input). Then, each turn consists of considering the most recently spoken number:
//
//     If that was the first time the number has been spoken, the current player says 0.
//     Otherwise, the number had been spoken before; the current player announces how many turns apart the number is from when it was previously spoken.
//
// So, after the starting numbers, each turn results in that player speaking aloud either 0 (if the last number is new) or an age (if the last number is a repeat).
//
// For example, suppose the starting numbers are 0,3,6:
//
//     Turn 1: The 1st number spoken is a starting number, 0.
//     Turn 2: The 2nd number spoken is a starting number, 3.
//     Turn 3: The 3rd number spoken is a starting number, 6.
//     Turn 4: Now, consider the last number spoken, 6. Since that was the first time the number had been spoken, the 4th number spoken is 0.
//     Turn 5: Next, again consider the last number spoken, 0. Since it had been spoken before, the next number to speak is the difference between the turn number when it was last spoken (the previous turn, 4) and the turn number of the time it was most recently spoken before then (turn 1). Thus, the 5th number spoken is 4 - 1, 3.
//     Turn 6: The last number spoken, 3 had also been spoken before, most recently on turns 5 and 2. So, the 6th number spoken is 5 - 2, 3.
//     Turn 7: Since 3 was just spoken twice in a row, and the last two turns are 1 turn apart, the 7th number spoken is 1.
//     Turn 8: Since 1 is new, the 8th number spoken is 0.
//     Turn 9: 0 was last spoken on turns 8 and 4, so the 9th number spoken is the difference between them, 4.
//     Turn 10: 4 is new, so the 10th number spoken is 0.
//
// (The game ends when the Elves get sick of playing or dinner is ready, whichever comes first.)
//
// Their question for you is: what will be the 2020th number spoken? In the example above, the 2020th number spoken will be 436.
//
// Here are a few more examples:
//
//     Given the starting numbers 1,3,2, the 2020th number spoken is 1.
//     Given the starting numbers 2,1,3, the 2020th number spoken is 10.
//     Given the starting numbers 1,2,3, the 2020th number spoken is 27.
//     Given the starting numbers 2,3,1, the 2020th number spoken is 78.
//     Given the starting numbers 3,2,1, the 2020th number spoken is 438.
//     Given the starting numbers 3,1,2, the 2020th number spoken is 1836.
//
// Given your starting numbers, what will be the 2020th number spoken?
//
// Your puzzle input is 8,11,0,19,1,2.
//
// Answer:
// You can also [Shareon Bluesky Twitter Mastodon] this puzzle.

use std::{
    collections::HashMap,
    io::{self, Read, Result},
};

#[derive(Debug)]
enum Seen {
    FirstTime(i64),
    MoreThanOnce((), i64),
}

#[derive(Default, Debug)]
struct GameState {
    state: HashMap<i64, Seen>,
}

impl GameState {
    fn new(initial_values: &[i64]) -> Self {
        let mut s = Self {
            state: HashMap::new(),
        };
        for (i, number) in initial_values.iter().enumerate() {
            s.take_turn(i as i64 + 1, *number);
        }
        s
    }

    fn take_turn(&mut self, turn: i64, number: i64) -> i64 {
        if self.state.contains_key(&number) {
            self.update(turn, number)
        } else {
            self.add(turn, number)
        }
    }

    fn update(&mut self, turn: i64, number: i64) -> i64 {
        match self.state.get(&number) {
            Some(&Seen::FirstTime(most_recent_time)) => {
                self.state.insert(number, Seen::MoreThanOnce((), turn));
                turn - most_recent_time
            }
            Some(&Seen::MoreThanOnce(_, most_recent_time)) => {
                self.state.insert(number, Seen::MoreThanOnce((), turn));
                turn - most_recent_time
            }
            None => panic!("impossible!"),
        }
    }

    fn add(&mut self, turn: i64, number: i64) -> i64 {
        self.state.insert(number, Seen::FirstTime(turn));
        0
    }
}

fn play_turns(starting_numbers: &[i64], turns_to_play: usize) -> Vec<i64> {
    let mut game_state = GameState::new(&starting_numbers[..starting_numbers.len() - 1]);
    let mut result: Vec<i64> = starting_numbers.to_vec();
    let mut last_seen = *starting_numbers.last().unwrap();
    for i in starting_numbers.len()..turns_to_play {
        let new_last_seen = game_state.take_turn(i as i64, last_seen);
        last_seen = new_last_seen;
        result.push(last_seen);
    }
    result
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;
    let initial_numbers: Vec<i64> = buffer
        .trim_end()
        .split(',')
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();

    let turns_to_play = 2020;
    let result = play_turns(&initial_numbers, turns_to_play);
    println!("Result is: {}", result.last().unwrap());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let input = vec![0, 3, 6];
        let turns = 10;
        let expected = vec![0, 3, 6, 0, 3, 3, 1, 0, 4, 0];
        let actual = play_turns(&input, turns);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let input = vec![1, 3, 2];
        let turns = 2020;
        let actual = play_turns(&input, turns);
        assert_eq!(actual.last(), Some(&1));
    }

    #[test]
    fn test_3() {
        let input = vec![2, 1, 3];
        let turns = 2020;
        let actual = play_turns(&input, turns);
        assert_eq!(actual.last(), Some(&10));
    }

    #[test]
    fn test_4() {
        let input = vec![1, 2, 3];
        let turns = 2020;
        let actual = play_turns(&input, turns);
        assert_eq!(actual.last(), Some(&27));
    }

    #[test]
    fn test_5() {
        let input = vec![2, 3, 1];
        let turns = 2020;
        let actual = play_turns(&input, turns);
        assert_eq!(actual.last(), Some(&78));
    }

    #[test]
    fn test_6() {
        let input = vec![3, 2, 1];
        let turns = 2020;
        let actual = play_turns(&input, turns);
        assert_eq!(actual.last(), Some(&438));
    }

    #[test]
    fn test_7() {
        let input = vec![3, 1, 2];
        let turns = 2020;
        let actual = play_turns(&input, turns);
        assert_eq!(actual.last(), Some(&1836));
    }

    #[test]
    fn test_8() {
        let input = vec![0, 3, 6];
        let turns = 2020;
        let actual = play_turns(&input, turns);
        assert_eq!(actual.last(), Some(&436));
    }

    #[test]
    fn test_9() {
        let input = vec![8, 11, 0, 19, 1, 2];
        let turns = 2020;
        let actual = play_turns(&input, turns);
        assert_eq!(actual.last(), Some(&447));
    }
}
