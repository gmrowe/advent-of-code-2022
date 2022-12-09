use std::{fs, io};

fn parse_stack_data(stack_data: &str) -> Option<Vec<Vec<char>>> {
    const PACKAGE_WIDTH: usize = 3;
    const PAD_WIDTH: usize = 1;

    let mut stack_iter = stack_data.lines().rev();
    let stack_labels = stack_iter.next()?;

    let (_, last_label) = stack_labels.trim_end().rsplit_once(char::is_whitespace)?;
    let stack_count = last_label.parse::<usize>().ok()?;
    let mut stacks = vec![Vec::new(); stack_count];
    for line in stack_iter {
        let mut remaining = line;
        for stack in stacks.iter_mut() {
            let (package, rest) = remaining.split_at(PACKAGE_WIDTH);
            let package_label = package.chars().nth(1)?;
            if package_label != ' ' {
                stack.push(package_label);
            }
            if !rest.is_empty() {
                remaining = &rest[PAD_WIDTH..];
            }
        }
    }
    Some(stacks)
}

fn parse_move_op(input: &str) -> Option<(usize, usize, usize)> {
    let tokens = input.split_whitespace();
    // Skip 'move'
    let mut tokens = tokens.skip(1);
    let n = tokens.next().and_then(|s| s.parse::<usize>().ok())?;
    // Skip 'from'
    let mut tokens = tokens.skip(1);
    let from = tokens.next().and_then(|s| s.parse::<usize>().ok())?;
    // Skip 'to'
    let mut tokens = tokens.skip(1);
    let to = tokens.next().and_then(|s| s.parse::<usize>().ok())?;
    Some((n, from, to))
}

fn part_1(stack_data: &str, instructions: &str) -> String {
    let mut stacks = parse_stack_data(stack_data).expect("Stack data is parseable");
    for op in instructions.lines() {
        let (n, from, to) = parse_move_op(op).expect("All moves in file are valid");
        let split_index = stacks[from - 1].len() - n;
        let last_n = stacks[from - 1].split_off(split_index);
        stacks[to - 1].extend(last_n.iter().rev());
    }

    stacks.iter().map(|s| s.last().unwrap_or(&'?')).collect()
}

fn part_2(stack_data: &str, instructions: &str) -> String {
    let mut stacks = parse_stack_data(stack_data).expect("Stack data is parseable");
    for op in instructions.lines() {
        let (n, from, to) = parse_move_op(op).expect("All moves in file are valid");
        let split_index = stacks[from - 1].len() - n;
        let mut last_n = stacks[from - 1].split_off(split_index);
        stacks[to - 1].append(&mut last_n);
    }
    stacks.iter().map(|s| s.last().unwrap_or(&'?')).collect()
}

fn main() -> io::Result<()> {
    const FILE_PATH: &str = "aoc_2022_day05_large_input.txt";
    let input = fs::read_to_string(FILE_PATH)?;

    let (stack_data, instructions) = input
        .split_once("\n\n")
        .expect("Input has a stack data and instructions");
    let result_1 = part_1(stack_data, instructions);
    println!("day-05;part-1 = {}", result_1);

    let result_2 = part_2(stack_data, instructions);
    println!("day-05;part-2 = {}", result_2);
    Ok(())
}
