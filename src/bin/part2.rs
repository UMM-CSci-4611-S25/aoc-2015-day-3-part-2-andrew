use std::{collections::HashSet, hash::Hash, ops::Add, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Pos {x: i32, y: i32}

impl Pos {
    pub const fn new(x: i32, y: i32) -> Self {
        Self {x,y}
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Add<Direction> for Pos {
    type Output = Self;

    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::North => Self::new(self.x, self.y + 1),
            Direction::South => Self::new(self.x, self.y - 1),
            Direction::East => Self::new(self.x + 1, self.y),
            Direction::West => Self::new(self.x - 1, self.y),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct IllegalChar(char);

impl TryFrom<char> for Direction {
    type Error = IllegalChar;

    fn try_from(c:char) -> Result<Self, Self::Error> {
        Ok(match c {
            '^' => Self::North,
            'v' => Self::South,
            '>' => Self::East,
            '<' => Self::West,
            _ => return Err(IllegalChar(c)),
        })
    }
}


pub struct Moves {
    moves: Vec<Direction>,
}

impl FromStr for Moves {
    type Err = IllegalChar;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s
            .trim()
            .chars()
            .filter(|&c| c != '\r')
            .map(Direction::try_from)
            .collect::<Result<Vec<Direction>, IllegalChar>>()?;
        Ok(Self { moves })
    }
}


pub struct Tracker {
    visited_houses: HashSet<Pos>,
    santa_position: Pos,
    robo_position: Pos,
    santa_move_turn: bool,
}

impl Tracker {
    #[must_use]
    pub fn new() -> Self {
        let initial_position = Pos::new(0,0);
        let mut visited_houses = HashSet::new();
        visited_houses.insert(initial_position);
        
        Self {
            visited_houses,
            santa_position: initial_position,
            robo_position: initial_position,
            santa_move_turn: true,
        }
    }

    #[must_use]
    pub fn num_visited_houses(&self) -> usize {
        self.visited_houses.len()
    }

    #[must_use]
    pub const fn current_pos(&self) -> (Pos, Pos) {
        (self.santa_position, self.robo_position)
    }

    pub fn perform_move(&mut self, direction: Direction) {
        if self.santa_move_turn {
            self.santa_position = self.santa_position + direction;
            self.visited_houses.insert(self.santa_position);
        } else {
            self.robo_position = self.robo_position + direction;
            self.visited_houses.insert(self.robo_position);
        }
        self.santa_move_turn = !self.santa_move_turn;
    }

    pub fn perform_moves(&mut self, moves: Moves) {
        for m in moves.moves {
            self.perform_move(m);
        }
    }

}

impl Default for Tracker {
    fn default() -> Self {
        Self::new()
    }
}


fn main() {
    let input_file_name = "input.txt";
    let file_contents = std::fs::read_to_string(input_file_name).expect("Failed to read the input file");
    let moves = Moves::from_str(file_contents.trim()).expect("Failed to parse the input file to list of moves");
    let mut santa_tracker = Tracker::new();
    santa_tracker.perform_moves(moves);
    println!(
        "Santa and his Robo had visited & delivered presents to {} houses",
        santa_tracker.num_visited_houses()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

     #[test]
    fn test_visited_houses_new() {
        let visited_houses = Tracker::new();
        assert_eq!(visited_houses.num_visited_houses(), 1);
        // What do you want to do about the current position?
        assert_eq!(visited_houses.current_pos(), (Pos::new(0, 0), Pos::new(0, 0)));
    }

    #[test]
    fn test_direction_try_from() {
        assert_eq!('^'.try_into(), Ok(Direction::North));
        assert_eq!('v'.try_into(), Ok(Direction::South));
        assert_eq!('<'.try_into(), Ok(Direction::West));
        assert_eq!('>'.try_into(), Ok(Direction::East));
        assert_eq!(Direction::try_from('x'), Err(IllegalChar('x')));
    }

    #[test]
    fn test_move_north_south() {
        let mut visited_houses = Tracker::new();
        visited_houses.perform_move(Direction::North);
        visited_houses.perform_move(Direction::South);
        assert_eq!(visited_houses.num_visited_houses(), 3);
        assert_eq!(visited_houses.current_pos(), (Pos::new(0, 1), Pos::new(0, -1)));
    }

    #[test]
    fn test_square_moves() {
        let mut visited_houses = Tracker::new();
        let moves = Moves::from_str("^>v<").unwrap();
        visited_houses.perform_moves(moves);
        assert_eq!(visited_houses.num_visited_houses(), 3);
        assert_eq!(visited_houses.current_pos(), (Pos::new(0, 0), Pos::new(0, 0)));
    }

    #[test]
    fn test_up_down_moves() {
        let mut visited_houses = Tracker::new();
        let moves = Moves::from_str("^v^v^v^v^v").unwrap();
        visited_houses.perform_moves(moves);
        assert_eq!(visited_houses.num_visited_houses(), 11);
        assert_eq!(visited_houses.current_pos(), (Pos::new(0, 5), Pos::new(0, -5)));
    }

    #[test]
    fn test_aoc_input() {
        let mut visited_houses = Tracker::new();
        let moves = Moves::from_str(include_str!("../../input.txt")).unwrap();
        visited_houses.perform_moves(moves);
        assert_eq!(visited_houses.num_visited_houses(), 2639);
    }
}