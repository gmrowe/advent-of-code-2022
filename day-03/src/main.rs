use std::{collections::HashSet, fs, io};

fn main() -> io::Result<()> {
    const INPUT_FILE_PATH: &str = "input.txt";
    const LOWER_CASE_OFFSET: u8 = 96;
    const UPPER_CASE_OFFSET: u8 = 38;

    let input = fs::read_to_string(INPUT_FILE_PATH)?;

    let mut total = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();
        let (front, back) = bytes.split_at(bytes.len() / 2);
        let front_set = HashSet::<u8>::from_iter(front.iter().copied());
        let back_set = HashSet::<u8>::from_iter(back.iter().copied());
        let common = front_set
            .intersection(&back_set)
            .next()
            .expect("At least one character in common");
        let score = if (*common as char).is_ascii_lowercase() {
            common - LOWER_CASE_OFFSET
        } else {
            common - UPPER_CASE_OFFSET
        };
        total += score as u32;
    }
    println!("day-03/part-1 = {}", total);
    Ok(())
}
