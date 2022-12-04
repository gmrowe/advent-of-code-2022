use std::{collections::HashSet, fs, io};

fn score_byte(b: u8) -> u32 {
    const LOWER_CASE_OFFSET: u8 = 96;
    const UPPER_CASE_OFFSET: u8 = 38;

    if (b as char).is_ascii_lowercase() {
        (b - LOWER_CASE_OFFSET).into()
    } else {
        (b - UPPER_CASE_OFFSET).into()
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
        let mut byte_sets = group
            .iter()
            .map(|s| s.as_bytes())
            .map(|b| HashSet::<u8>::from_iter(b.iter().copied()));
        let mut set = byte_sets.next().expect("bytes has GROUP_SIZE elements");
        byte_sets.for_each(|b_set| set.retain(|e| b_set.contains(e)));
        let &common = set
            .iter()
            .next()
            .expect("At least one character in common per group");
        part_2_total += score_byte(common);
    }
    println!("day-03/part-2 = {}", part_2_total);
    Ok(())
}
