#![allow(dead_code)]

use std::{fs, io};

fn main() -> io::Result<()> {
    const FILE_PATH: &str = "input.txt";
    let input = fs::read_to_string(FILE_PATH)?;

    let mut rope = Rope::new();
    for line in input.lines() {
        let instr = Instr::from_str(line);
        rope.move_instr(&instr);
    }
    let mut tail_positions = rope.tail_positions.clone();
    tail_positions.sort();
    tail_positions.dedup();
    let result_1 = tail_positions.len();

    println!("day-09;part-1 = {result_1}");
    Ok(())
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Eq, PartialEq)]
struct Instr {
    dir: Dir,
    distance: u32,
}

impl Instr {
    fn from_str(s: &str) -> Instr {
        let (dir, dist) = s
            .split_once(char::is_whitespace)
            .unwrap_or_else(|| panic!("Malformed instruction string: '{s}'"));

        let direction = match dir {
            "R" => Dir::Right,
            "L" => Dir::Left,
            "U" => Dir::Up,
            "D" => Dir::Down,
            _ => panic!("Unknown direction: '{dir}'"),
        };

        let distance = dist
            .parse::<u32>()
            .unwrap_or_else(|_| panic!("Malformed instruction string: '{s}'"));

        Instr::new(direction, distance)
    }

    fn new(dir: Dir, distance: u32) -> Instr {
        Instr { dir, distance }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Rope {
    head_pos: Position,
    tail_pos: Position,
    tail_positions: Vec<Position>,
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head_pos: Position::new(0, 0),
            tail_pos: Position::new(0, 0),
            tail_positions: vec![Position::new(0, 0)],
        }
    }

    fn move_left(&mut self) {
        let head_x = self.head_pos.x;
        let tail_x = self.tail_pos.x;
        let head_y = self.head_pos.y;
        let tail_y = self.tail_pos.y;

        self.head_pos.x -= 1;
        if head_x - tail_x < 0 {
            self.tail_pos.x -= 1;

            if head_y.abs_diff(tail_y) > 0 {
                self.tail_pos.y = self.head_pos.y;
            }
        }
        self.tail_positions.push(self.tail_pos);
    }

    fn move_leftn(&mut self, n: u32) {
        for _ in 0..n {
            self.move_left();
        }
    }

    fn move_right(&mut self) {
        let head_x = self.head_pos.x;
        let tail_x = self.tail_pos.x;
        let head_y = self.head_pos.y;
        let tail_y = self.tail_pos.y;

        self.head_pos.x += 1;
        if head_x - tail_x > 0 {
            self.tail_pos.x += 1;

            if head_y.abs_diff(tail_y) > 0 {
                self.tail_pos.y = self.head_pos.y;
            }
        }
        self.tail_positions.push(self.tail_pos);
    }

    fn move_rightn(&mut self, n: u32) {
        for _ in 0..n {
            self.move_right();
        }
    }

    fn move_up(&mut self) {
        let head_x = self.head_pos.x;
        let tail_x = self.tail_pos.x;
        let head_y = self.head_pos.y;
        let tail_y = self.tail_pos.y;

        self.head_pos.y += 1;
        if head_y - tail_y > 0 {
            self.tail_pos.y += 1;

            if head_x.abs_diff(tail_x) > 0 {
                self.tail_pos.x = self.head_pos.x;
            }
        }
        self.tail_positions.push(self.tail_pos);
    }

    fn move_upn(&mut self, n: u32) {
        for _ in 0..n {
            self.move_up();
        }
    }

    fn move_down(&mut self) {
        let head_x = self.head_pos.x;
        let tail_x = self.tail_pos.x;
        let head_y = self.head_pos.y;
        let tail_y = self.tail_pos.y;

        self.head_pos.y -= 1;
        if head_y - tail_y < 0 {
            self.tail_pos.y -= 1;
            if head_x.abs_diff(tail_x) > 0 {
                self.tail_pos.x = self.head_pos.x;
            }
        }
        self.tail_positions.push(self.tail_pos);
    }

    fn move_downn(&mut self, n: u32) {
        for _ in 0..n {
            self.move_down();
        }
    }

    fn move_instr(&mut self, instr: &Instr) {
        match instr.dir {
            Dir::Right => self.move_rightn(instr.distance),
            Dir::Left => self.move_leftn(instr.distance),
            Dir::Up => self.move_upn(instr.distance),
            Dir::Down => self.move_downn(instr.distance),
        }
    }
}

#[cfg(test)]
mod day_09_tests {

    mod test_from_str {
        use crate::Dir;
        use crate::Instr;

        fn assert_correct_parse(input: &str, direction: Dir, distance: u32) {
            let instruction = Instr::from_str(input);
            assert_eq!(instruction, Instr::new(direction, distance));
        }

        #[test]
        fn parse_right_instruction() {
            assert_correct_parse("R 4", Dir::Right, 4);
        }

        #[test]
        fn parse_left_instruction() {
            assert_correct_parse("L 3", Dir::Left, 3);
        }

        #[test]
        fn parse_up_instruction() {
            assert_correct_parse("U 42", Dir::Up, 42);
        }

        #[test]
        fn parse_down_instruction() {
            assert_correct_parse("D 6969", Dir::Down, 6969);
        }
    }

    mod test_create_rope {
        use crate::{Position, Rope};

        #[test]
        fn head_and_tail_of_a_new_rope_are_overlapping() {
            let rope = Rope::new();
            assert_eq!(rope.head_pos, rope.tail_pos);
        }

        #[test]
        fn new_rope_starts_out_at_0_0() {
            let rope = Rope::new();
            assert_eq!(rope.head_pos, Position::new(0, 0));
        }
    }

    mod test_move {
        use crate::{Instr, Position, Rope};

        #[test]
        fn move_left_from_overlapping_moves_head() {
            let mut rope = Rope::new();
            rope.move_left();
            assert_eq!(rope.head_pos, Position::new(-1, 0));
        }

        #[test]
        fn move_left_from_overlapping_moves_leaves_tail_in_place() {
            let mut rope = Rope::new();
            rope.move_left();
            assert_eq!(rope.tail_pos, Position::new(0, 0));
        }

        #[test]
        fn move_left_when_head_is_left_moves_head_and_tail() {
            let mut rope = Rope::new();
            // Move head to left of tail => head = (-1, 0), tail = (0, 0);
            rope.move_left();
            // Move head left again => head = (-2, 0), tail = (-1, 0);
            rope.move_left();
            assert_eq!(rope.head_pos, Position::new(-2, 0));
            assert_eq!(rope.tail_pos, Position::new(-1, 0));
        }

        #[test]
        fn move_right_from_overlapping() {
            let mut rope = Rope::new();
            rope.move_right();
            assert_eq!(rope.head_pos, Position::new(1, 0));
            assert_eq!(rope.tail_pos, Position::new(0, 0));
        }

        #[test]
        fn move_right_when_head_is_right() {
            let mut rope = Rope::new();
            rope.move_right();
            rope.move_right();
            assert_eq!(rope.head_pos, Position::new(2, 0));
            assert_eq!(rope.tail_pos, Position::new(1, 0));
        }

        #[test]
        fn move_left_when_head_is_right() {
            let mut rope = Rope::new();
            rope.move_right();
            rope.move_left();
            assert_eq!(rope.head_pos, Position::new(0, 0));
            assert_eq!(rope.tail_pos, Position::new(0, 0));
        }

        #[test]
        fn move_right_when_head_is_left() {
            let mut rope = Rope::new();
            rope.move_left();
            rope.move_right();
            assert_eq!(rope.head_pos, Position::new(0, 0));
            assert_eq!(rope.tail_pos, Position::new(0, 0));
        }

        #[test]
        fn move_up_when_overlapping() {
            let mut rope = Rope::new();
            rope.move_up();
            assert_eq!(rope.head_pos, Position::new(0, 1));
            assert_eq!(rope.tail_pos, Position::new(0, 0));
        }

        #[test]
        fn move_up_when_head_is_above() {
            let mut rope = Rope::new();
            rope.move_up();
            rope.move_up();
            assert_eq!(rope.head_pos, Position::new(0, 2));
            assert_eq!(rope.tail_pos, Position::new(0, 1));
        }

        #[test]
        fn move_down_when_overlapping() {
            let mut rope = Rope::new();
            rope.move_down();
            assert_eq!(rope.head_pos, Position::new(0, -1));
            assert_eq!(rope.tail_pos, Position::new(0, 0));
        }

        #[test]
        fn move_down_when_head_is_below() {
            let mut rope = Rope::new();
            rope.move_down();
            rope.move_down();
            assert_eq!(rope.head_pos, Position::new(0, -2));
            assert_eq!(rope.tail_pos, Position::new(0, -1));
        }

        #[test]
        fn move_head_diagonally_up_right_from_overlapping() {
            let mut rope = Rope::new();
            rope.move_up();
            rope.move_right();
            assert_eq!(rope.head_pos, Position::new(1, 1));
            assert_eq!(rope.tail_pos, Position::new(0, 0));
        }

        #[test]
        fn move_head_up_from_diagonally_up_right() {
            let mut rope = Rope::new();
            rope.move_up();
            rope.move_right();
            rope.move_up();
            assert_eq!(rope.head_pos, Position::new(1, 2));
            assert_eq!(rope.tail_pos, Position::new(1, 1));
        }

        #[test]
        fn move_head_right_from_diagonally_up_right() {
            let mut rope = Rope::new();
            rope.move_up();
            rope.move_right();
            rope.move_right();
            assert_eq!(rope.head_pos, Position::new(2, 1));
            assert_eq!(rope.tail_pos, Position::new(1, 1));
        }

        #[test]
        fn move_head_left_from_diagonally_down_left() {
            let mut rope = Rope::new();
            rope.move_down();
            rope.move_left();
            rope.move_left();
            assert_eq!(rope.head_pos, Position::new(-2, -1));
            assert_eq!(rope.tail_pos, Position::new(-1, -1));
        }

        #[test]
        fn move_head_down_from_diagonally_down_left() {
            let mut rope = Rope::new();
            rope.move_down();
            rope.move_left();
            rope.move_down();
            assert_eq!(rope.head_pos, Position::new(-1, -2));
            assert_eq!(rope.tail_pos, Position::new(-1, -1));
        }

        #[test]
        fn move_rightn_from_overlapping() {
            let mut rope = Rope::new();
            rope.move_rightn(4);
            assert_eq!(rope.head_pos, Position::new(4, 0));
            assert_eq!(rope.tail_pos, Position::new(3, 0));
        }

        #[test]
        fn move_right_via_instruction() {
            let mut rope = Rope::new();
            let instr = Instr::from_str("R 4");
            rope.move_instr(&instr);
            assert_eq!(rope.head_pos, Position::new(4, 0));
            assert_eq!(rope.tail_pos, Position::new(3, 0));
        }

        #[test]
        fn move_left_via_instruction() {
            let mut rope = Rope::new();
            let instr = Instr::from_str("L 16");
            rope.move_instr(&instr);
            assert_eq!(rope.head_pos, Position::new(-16, 0));
            assert_eq!(rope.tail_pos, Position::new(-15, 0));
        }
    }
}
