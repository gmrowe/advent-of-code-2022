use std::{fs, io};

fn main() -> io::Result<()> {
    const INPUT_FILE_PATH: &str = "input.txt";
    let input = fs::read_to_string(INPUT_FILE_PATH)?;

    let mut total = 0;
    for line in input.lines() {
        let outcome_score = match line {
            // A, X = rock
            // B, Y = paper
            // C, Z = scissors

            // I lose
            "A Z" | "B X" | "C Y" => 0,
            // Tie
            "A X" | "B Y" | "C Z" => 3,
            // I win
            "A Y" | "B Z" | "C X" => 6,
            _ => unreachable!(),
        };

        let selection = line
            .split_whitespace()
            .nth(1)
            .expect("A line will always have 2 tokens");
        let selection_score = match selection {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => unreachable!(),
        };
        total += outcome_score + selection_score;
    }
    println!("day-02/part-1 = {}", total);
    Ok(())
}
