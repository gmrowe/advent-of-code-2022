use std::{fs, io, str::FromStr};

fn main() -> io::Result<()> {
    const FILE_PATH: &str = "input.txt";
    let input = fs::read_to_string(FILE_PATH)?;

    let mut total_overlap_count = 0;
    let mut any_overlap_count = 0;
    for line in input.lines() {
        let (range_1, range_2) = line.split_once(',').expect("Each line has a comma");
        let (str_min_1, str_max_1) = range_1.split_once('-').expect("Each range has a dash");
        let (str_min_2, str_max_2) = range_2.split_once('-').expect("Each range has a dash");
        let min_1 = u32::from_str(str_min_1).expect("Token is a valid u32");
        let max_1 = u32::from_str(str_max_1).expect("Token is a valid u32");
        let min_2 = u32::from_str(str_min_2).expect("Token is a valid u32");
        let max_2 = u32::from_str(str_max_2).expect("Token is a valid u32");

        if (min_1 <= min_2 && max_1 >= max_2) || (min_2 <= min_1 && max_2 >= max_1) {
            total_overlap_count += 1;
        }

        if (min_1 <= min_2 && max_1 >= min_2) || (min_2 <= min_1 && max_2 >= min_1) {
            any_overlap_count += 1;
        }
    }

    println!("day-04;part-1 = {}", total_overlap_count);
    println!("day-04;part-2 = {}", any_overlap_count);
    Ok(())
}
