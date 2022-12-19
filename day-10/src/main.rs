use std::{fs, io};

fn build_cycle_log(input: &str) -> Vec<i32> {
    let mut register = 1;
    let mut cycle_log = Vec::new();
    cycle_log.push(0);
    for line in input.lines() {
        cycle_log.push(register);
        if line == "noop" {
            continue;
        } else if line.starts_with("addx") {
            let (_instr, value_tok) = line
                .split_once(char::is_whitespace)
                .unwrap_or_else(|| panic!("Malformed input line: {line}"));
            let value = value_tok
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("{value_tok} cannot be converted to i32"));
            register += value;
            cycle_log.push(register);
        } else {
            unreachable!("Malformed input line: {line}");
        }
    }
    cycle_log
}

#[allow(dead_code)]
fn part_1(cycle_log: &[i32]) -> i32 {
    const INIT_INDEX: usize = 20;
    const FINAL_INDEX: usize = 220;
    const STEP: usize = 40;
    let mut signal_strength_sum = 0;
    for idx in (INIT_INDEX..=FINAL_INDEX).step_by(STEP) {
        let cycle_value = cycle_log[idx - 1];
        let signal_strength = idx as i32 * cycle_value;
        signal_strength_sum += signal_strength;
    }
    signal_strength_sum
}

fn part_2(cycle_log: &[i32]) -> String {
    const LINE_WIDTH: usize = 40;
    let mut result = String::new();
    for screen_line in cycle_log.chunks(LINE_WIDTH) {
        for (pixel_num, cycle) in screen_line.iter().enumerate() {
            let pixel_range = pixel_num as i32 - 1..=pixel_num as i32 + 1;
            if pixel_range.contains(cycle) {
                result.push('#');
            } else {
                result.push(' ');
            }
        }
        result.push('\n');
    }
    result
}

fn main() -> io::Result<()> {
    const FILE_PATH: &str = "input.txt";
    let input = fs::read_to_string(FILE_PATH)?;
    let cycle_log = build_cycle_log(&input);

    let result_1 = part_1(&cycle_log);
    println!("day-10/part-1 = {result_1}");

    let result_2 = part_2(&cycle_log);
    println!("day-10/part-2=\n{result_2}");
    Ok(())
}
