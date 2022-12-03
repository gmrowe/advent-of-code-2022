use std::{fs, io};

fn second_token(line: &str) -> &str {
    line.split_whitespace()
        .nth(1)
        .expect("A line will always have 2 tokens")
}

fn part_1(input: &str) -> u32 {
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

        let selection = second_token(line);
        let selection_score = match selection {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => unreachable!(),
        };
        total += outcome_score + selection_score;
    }
    total
}

fn part_2(input: &str) -> u32 {
    // A = rock
    // B = paper
    // C = scissors

    // X = lose
    // Y = tie
    // Z = win
    let mut total = 0;
    for line in input.lines() {
        let outcome = second_token(line);
        let outcome_score = match outcome {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => unreachable!(),
        };

        let selection_score = match line {
            // I choose scissors when:
            // they choose rock, and I must lose
            // they choose scidssors, and I must tie
            // they choose paper, and I must win
            "A X" | "C Y" | "B Z" => 3,
            // I choose paper when:
            // they choose scissors and I must lose
            // they choose paper, and I must tie
            // they choose rock, and I must win
            "C X" | "B Y" | "A Z" => 2,
            // I choose rock when:
            // they choose paper, and I must lose
            // they choose rock, and I must tie
            // they choose scissors, and I must win
            "B X" | "A Y" | "C Z" => 1,
            _ => unreachable!(),
        };
        total += outcome_score + selection_score;
    }
    total
}

fn main() -> io::Result<()> {
    const INPUT_FILE_PATH: &str = "input.txt";
    let input = fs::read_to_string(INPUT_FILE_PATH)?;
    let part_1_total = part_1(&input);
    println!("day-02/part-1 = {}", part_1_total);

    let part_2_total = part_2(&input);
    println!("day-02/part-2 = {}", part_2_total);
    Ok(())
}
