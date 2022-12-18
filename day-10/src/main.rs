use std::{fs, io};

fn main() -> io::Result<()> {
    const FILE_PATH: &str = "input.txt";
    let input = fs::read_to_string(FILE_PATH)?;
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

    const INIT_INDEX: usize = 20;
    const FINAL_INDEX: usize = 220;
    const STEP: usize = 40;
    let mut signal_strength_sum = 0;
    for idx in (INIT_INDEX..=FINAL_INDEX).step_by(STEP) {
        let cycle_value = cycle_log[idx - 1];
        let signal_strength = idx as i32 * cycle_value;
        signal_strength_sum += signal_strength;
    }
    println!("day-10/part-1 = {signal_strength_sum}");
    Ok(())
}
