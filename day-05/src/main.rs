use std::{fs, io};

fn parse_stack_data(stack_data: &str) -> Option<Vec<Vec<char>>> {
    const PACKAGE_WIDTH: usize = 3;
    const PAD_WIDTH: usize = 1;

    let mut stack_iter = stack_data.lines().rev();
    let stack_labels = stack_iter.next()?;

    let (_butlast_label, last_label) = stack_labels.trim_end().rsplit_once(char::is_whitespace)?;
    let stack_count = last_label
        .parse::<usize>()
        .expect("Every label is a valid usize");
    let mut stacks = vec![Vec::<char>::new(); stack_count];
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
        for _ in 0..n {
            if let Some(c) = stacks[from - 1].pop() {
                stacks[to - 1].push(c);
            }
        }
    }

    let mut result = String::new();
    for stack in stacks.iter() {
        let last = stack.last().unwrap_or(&'?');
        result.push(*last);
    }
    result
}

fn main() -> io::Result<()> {
    const FILE_PATH: &str = "input.txt";
    let input = fs::read_to_string(FILE_PATH)?;

    let (stack_data, instructions) = input
        .split_once("\n\n")
        .expect("Input has a stack data and instriuctions");
    let result = part_1(stack_data, instructions);
    println!("day-05;part-1 = {}", result);

    Ok(())
}
