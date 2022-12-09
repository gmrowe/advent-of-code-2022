use std::{fs, io};

fn part_1(input: &str) -> usize {
    const SEG_LEN: usize = 4;
    let mut index_found = false;
    let mut i = 0;
    while i < input.len() - SEG_LEN + 1 && !index_found {
        let segment = &input[i..i + SEG_LEN];
        let mut duplicate_found = false;
        let mut j = 0;
        while j < segment.len() && !duplicate_found {
            let curr = segment
                .chars()
                .nth(j)
                .expect("index can never be out of range");
            let mut inner_iter = segment[j + 1..].chars();
            while let (Some(c), true) = (inner_iter.next(), !duplicate_found) {
                duplicate_found = curr == c;
            }
            j += 1;
        }
        index_found = !duplicate_found;
        i += 1
    }
    i - SEG_LEN - 1
}

fn main() -> io::Result<()> {
    const FILE_PATH: &str = "input.txt";
    let input = fs::read_to_string(FILE_PATH)?;

    let result_1 = part_1(&input);
    println!("day-06;part-1 = {}", result_1);
    Ok(())
}
