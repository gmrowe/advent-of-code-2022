use std::{cmp::Reverse, fs, io};

fn part_1(input: &str) -> u64 {
    let mut monkeys = Monkeys::default();
    input
        .split("\n\n")
        .for_each(|text| monkeys.update_from_text(text));

    const ROUNDS: usize = 20;
    for _ in 0..ROUNDS {
        monkeys.execute_round_part_1();
    }
    let mut counts = monkeys.counts.clone();
    counts.sort_by_key(|&k| Reverse(k));
    counts[0] * counts[1]
}

fn update_worry_level_oart_1(old_worry: u64, op: &Op) -> u64 {
    let get_op = |o: &Operand| match o {
        Operand::Const(n) => *n,
        Operand::Old => old_worry,
    };

    match op {
        Op::Add(o) => (old_worry + get_op(o)) / 3,
        Op::Mul(o) => (old_worry * get_op(o)) / 3,
    }
}

fn part_2(input: &str) -> u64 {
    let mut monkeys = Monkeys::default();
    input
        .split("\n\n")
        .for_each(|text| monkeys.update_from_text(text));

    const ROUNDS: usize = 10000;
    for _ in 0..ROUNDS {
        monkeys.execute_round_part_2();
    }
    let mut counts = monkeys.counts.clone();
    counts.sort_by_key(|&k| Reverse(k));
    counts[0] * counts[1]
}

fn update_worry_level_oart_2(old_worry: u64, op: &Op, modulo: u64) -> u64 {
    let get_op = |o: &Operand| match o {
        Operand::Const(n) => *n,
        Operand::Old => old_worry,
    };

    match op {
        Op::Add(o) => (old_worry + get_op(o)) % modulo,
        Op::Mul(o) => (old_worry * get_op(o)) % modulo,
    }
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let result_1 = part_1(&input);
    println!("day-11;part-1 = {}", result_1);

    let result_2 = part_2(&input);
    println!("day-11;part-2 = {}", result_2);
    Ok(())
}

#[derive(Debug, Default)]
struct Monkeys {
    ids: Vec<usize>,
    items: Vec<Vec<u64>>,
    operations: Vec<Op>,
    div_tests: Vec<u64>,
    if_trues: Vec<usize>,
    if_falses: Vec<usize>,
    counts: Vec<u64>,
}

#[derive(Debug, Eq, PartialEq)]
enum Operand {
    Const(u64),
    Old,
}

#[derive(Debug, Eq, PartialEq)]
enum Op {
    Mul(Operand),
    Add(Operand),
}

impl Monkeys {
    fn update_from_text(&mut self, text: &str) {
        let mut text_lines = text.lines();

        // Parse id
        let id_line = text_lines.next().unwrap();
        let mut id_line_tokens = id_line.split_whitespace();
        let id_token = id_line_tokens.nth(1).unwrap();
        let id = id_token.trim_end_matches(':').parse::<usize>().unwrap();

        // Parse starting items
        let item_line = text_lines.next().unwrap();
        let items = item_line
            .split_whitespace()
            .skip(2)
            .map(|s| s.trim_end_matches(','))
            .map(|s| s.trim_start())
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        // Parse operation
        let operation_line = text_lines.next().unwrap();
        let mut operation_tokens = operation_line.split_whitespace().skip(4);
        let operator = operation_tokens.next().unwrap();
        let value = operation_tokens
            .next()
            .and_then(|s| s.parse::<u64>().ok())
            .map(Operand::Const)
            .unwrap_or(Operand::Old);
        let operation = match operator {
            "*" => Op::Mul(value),
            "+" => Op::Add(value),
            _ => todo!(),
        };

        // Parse divisivility test
        let div_test_line = text_lines.next().unwrap();
        let div_test = div_test_line
            .split_whitespace()
            .nth(3)
            .map(|s| s.parse::<u64>().unwrap())
            .unwrap();

        // Parse if_true
        let if_true_line = text_lines.next().unwrap();
        let if_true = if_true_line
            .split_whitespace()
            .nth(5)
            .map(|s| s.parse::<usize>().unwrap())
            .unwrap();

        // Parse if_false
        let if_false_line = text_lines.next().unwrap();
        let if_false = if_false_line
            .split_whitespace()
            .nth(5)
            .map(|s| s.parse::<usize>().unwrap())
            .unwrap();

        self.ids.push(id);
        self.items.push(items);
        self.operations.push(operation);
        self.div_tests.push(div_test);
        self.if_trues.push(if_true);
        self.if_falses.push(if_false);
        self.counts.push(0);
    }

    fn execute_round_part_1(&mut self) {
        for i in 0..self.ids.len() {
            for j in 0..self.items[i].len() {
                let new_worry_level =
                    update_worry_level_oart_1(self.items[i][j], &self.operations[i]);
                let new_index = if new_worry_level % self.div_tests[i] == 0 {
                    self.if_trues[i]
                } else {
                    self.if_falses[i]
                };
                self.items[new_index].push(new_worry_level);
                self.counts[i] += 1;
            }
            self.items[i].clear();
        }
    }

    fn execute_round_part_2(&mut self) {
        for i in 0..self.ids.len() {
            for j in 0..self.items[i].len() {
                let modulo = self.div_tests.iter().product();
                let new_worry_level =
                    update_worry_level_oart_2(self.items[i][j], &self.operations[i], modulo);
                let new_index = if new_worry_level % self.div_tests[i] == 0 {
                    self.if_trues[i]
                } else {
                    self.if_falses[i]
                };
                self.items[new_index].push(new_worry_level);
                self.counts[i] += 1;
            }
            self.items[i].clear();
        }
    }
}
