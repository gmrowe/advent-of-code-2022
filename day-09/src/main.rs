#![allow(dead_code)]

use std::{fs, io};

fn part_1(input: &str) -> usize {
    let mut rope = Rope::new();
    for line in input.lines() {
        let instr = Instr::from_str(line);
        rope.move_by_instr(&instr);
    }
    let mut tail_positions = rope.tail_positions.clone();
    tail_positions.sort();
    tail_positions.dedup();
    tail_positions.len()
}

fn part_2(input: &str) -> usize {
    const ROPE_SIZE: usize = 10;
    let mut rope_10 = RopeN::from_size(ROPE_SIZE);
    for line in input.lines() {
        let instr = Instr::from_str(line);
        rope_10.move_by_instr(&instr);
    }
    let mut tail_positions = rope_10.tail_positions.clone();
    tail_positions.sort();
    tail_positions.dedup();
    tail_positions.len()
}

fn main() -> io::Result<()> {
    const FILE_PATH: &str = "input.txt";
    let input = fs::read_to_string(FILE_PATH)?;

    let result_1 = part_1(&input);
    println!("day-09;part-1 = {result_1}");

    let result_2 = part_2(&input);
    println!("day-09;part-2 = {result_2}");

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

    fn set_head(&mut self, head_pos: Position) {
        self.head_pos = head_pos;
        self.update_tail();
    }

    fn move_left(&mut self) {
        self.head_pos.x -= 1;
        self.update_tail();
        self.tail_positions.push(self.tail_pos);
    }

    fn move_right(&mut self) {
        self.head_pos.x += 1;
        self.update_tail();
        self.tail_positions.push(self.tail_pos);
    }

    fn move_up(&mut self) {
        self.head_pos.y += 1;
        self.update_tail();
        self.tail_positions.push(self.tail_pos);
    }

    fn move_down(&mut self) {
        self.head_pos.y -= 1;
        self.update_tail();
        self.tail_positions.push(self.tail_pos);
    }

    fn move_downn(&mut self, n: u32) {
        self.move_n_generic(Self::move_down, n)
    }

    fn move_upn(&mut self, n: u32) {
        self.move_n_generic(Self::move_up, n)
    }

    fn move_leftn(&mut self, n: u32) {
        self.move_n_generic(Self::move_left, n)
    }

    fn move_rightn(&mut self, n: u32) {
        self.move_n_generic(Self::move_right, n)
    }

    fn move_n_generic(&mut self, f: fn(&mut Self) -> (), n: u32) {
        for _ in 0..n {
            f(self);
        }
    }

    fn update_tail(&mut self) {
        let mut delta_x = self.head_pos.x - self.tail_pos.x;
        let mut delta_y = self.head_pos.y - self.tail_pos.y;
        let direction_x = delta_x.signum();
        let direction_y = delta_y.signum();

        if delta_x == 0 {
            self.tail_pos.y += direction_y * (delta_y.abs() - 1);
        } else if delta_y == 0 {
            self.tail_pos.x += direction_x * (delta_x.abs() - 1);
        } else {
            while delta_x.abs() > 1 || delta_y.abs() > 1 {
                self.tail_pos.x += direction_x;
                self.tail_pos.y += direction_y;
                delta_x = self.head_pos.x - self.tail_pos.x;
                delta_y = self.head_pos.y - self.tail_pos.y;
            }
        }
    }

    fn move_by_instr(&mut self, instr: &Instr) {
        match instr.dir {
            Dir::Right => self.move_rightn(instr.distance),
            Dir::Left => self.move_leftn(instr.distance),
            Dir::Up => self.move_upn(instr.distance),
            Dir::Down => self.move_downn(instr.distance),
        }
    }
}

struct RopeN {
    segments: Vec<Rope>,
    tail_positions: Vec<Position>,
}

impl RopeN {
    fn from_size(size: usize) -> RopeN {
        assert!(size > 0);
        RopeN {
            segments: vec![Rope::new(); size],
            tail_positions: vec![Position::new(0, 0)],
        }
    }

    fn move_generic(&mut self, f: fn(&mut Rope) -> ()) {
        let (head, rest) = self.segments.split_at_mut(1);
        let head_segment = &mut head[0];
        f(head_segment);
        let mut curr_tail_pos = head_segment.tail_pos;
        for segment in rest.iter_mut() {
            segment.set_head(curr_tail_pos);
            curr_tail_pos = segment.tail_pos;
        }

        let last_segment = self
            .segments
            .iter()
            .last()
            .expect("Rope has at lease one segement");
        self.tail_positions.push(last_segment.head_pos);
    }

    fn move_right(&mut self) {
        self.move_generic(Rope::move_right);
    }

    fn move_left(&mut self) {
        self.move_generic(Rope::move_left);
    }

    fn move_up(&mut self) {
        self.move_generic(Rope::move_up);
    }

    fn move_down(&mut self) {
        self.move_generic(Rope::move_down);
    }

    fn move_rightn(&mut self, n: u32) {
        self.move_n_generic(Self::move_right, n);
    }

    fn move_leftn(&mut self, n: u32) {
        self.move_n_generic(Self::move_left, n);
    }

    fn move_upn(&mut self, n: u32) {
        self.move_n_generic(Self::move_up, n);
    }

    fn move_downn(&mut self, n: u32) {
        self.move_n_generic(Self::move_down, n);
    }

    fn move_n_generic(&mut self, f: fn(&mut Self) -> (), n: u32) {
        for _ in 0..n {
            f(self);
        }
    }

    fn move_by_instr(&mut self, instr: &Instr) {
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
            rope.move_by_instr(&instr);
            assert_eq!(rope.head_pos, Position::new(4, 0));
            assert_eq!(rope.tail_pos, Position::new(3, 0));
        }

        #[test]
        fn move_left_via_instruction() {
            let mut rope = Rope::new();
            let instr = Instr::from_str("L 16");
            rope.move_by_instr(&instr);
            assert_eq!(rope.head_pos, Position::new(-16, 0));
            assert_eq!(rope.tail_pos, Position::new(-15, 0));
        }
    }
}
