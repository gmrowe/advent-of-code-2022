use std::{fs, io};

fn find_duplicates(input: &str, seg_len: usize) -> usize {
    let mut index_found = false;
    let mut i = 0;
    while i < input.len() - seg_len + 1 && !index_found {
        let segment = &input[i..i + seg_len];
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
    i + seg_len - 1
}

fn part_1(input: &str) -> usize {
    find_duplicates(input, 4)
}

fn part_2(input: &str) -> usize {
    find_duplicates(input, 14)
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let result_1 = part_1(&input);
    println!("day-06;part-1 = {}", result_1);

    let result_2 = part_2(&input);
    println!("day-06;part-2 = {}", result_2);
    Ok(())
}
