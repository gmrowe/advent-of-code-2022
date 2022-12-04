use std::{collections::HashSet, fs, io};

fn score_byte(byte: u8) -> u32 {
    const LOWER_CASE_OFFSET: u8 = b'a';
    const UPPER_CASE_OFFSET: u8 = b'A';

    if (byte as char).is_ascii_lowercase() {
        u32::from(byte - LOWER_CASE_OFFSET) + 1
    } else {
        u32::from(byte - UPPER_CASE_OFFSET) + 27
    }
}

fn main() -> io::Result<()> {
    const INPUT_FILE_PATH: &str = "input.txt";

    let input = fs::read_to_string(INPUT_FILE_PATH)?;

    let mut part_1_total = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();
        let (front, back) = bytes.split_at(bytes.len() / 2);
        let front_set = HashSet::<u8>::from_iter(front.iter().copied());
        let back_set = HashSet::<u8>::from_iter(back.iter().copied());
        let &common = front_set
            .intersection(&back_set)
            .next()
            .expect("At least one character in common");
        part_1_total += score_byte(common);
    }
    println!("day-03/part-1 = {}", part_1_total);

    const GROUP_SIZE: usize = 3;
    let mut part_2_total = 0;
    let lines = input.lines().collect::<Vec<_>>();
    for group in lines.chunks(GROUP_SIZE) {
        let mut byte_sets = group.iter().map(|s| HashSet::<u8>::from_iter(s.bytes()));
        let expect_message = format!("There are {} byte sets", GROUP_SIZE);
        let mut first = byte_sets.next().expect(&expect_message);
        let second = byte_sets.next().expect(&expect_message);
        let third = byte_sets.next().expect(&expect_message);
        first.retain(|b| second.contains(b) && third.contains(b));
        let &common = first
            .iter()
            .next()
            .expect("At least one character in common per group");
        part_2_total += score_byte(common);
    }
    println!("day-03/part-2 = {}", part_2_total);
    Ok(())
}
