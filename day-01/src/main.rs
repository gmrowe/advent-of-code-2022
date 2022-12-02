use std::{cmp::Reverse, fs, io};

fn calorie_counts(input: &str) -> Vec<u32> {
    let mut calorie_counts = Vec::new();
    for token in input.split("\n\n").collect::<Vec<_>>() {
        let mut calories = 0;
        for line in token.lines() {
            let meal = line.parse::<u32>().expect("Each line is a valid u32");
            calories += meal;
        }
        calorie_counts.push(calories);
    }
    calorie_counts
}

fn part_1(counts: &[u32]) -> u32 {
    *counts.iter().max().unwrap_or(&0)
}

fn part_2(counts: &mut [u32]) -> u32 {
    counts.sort_by_key(|&k| Reverse(k));
    counts.iter().take(3).sum()
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let mut calorie_counts = calorie_counts(&input);

    println!("day-01/part-1 = {}", part_1(&calorie_counts));
    println!("day-02/part-2 = {}", part_2(&mut calorie_counts));
    Ok(())
}
