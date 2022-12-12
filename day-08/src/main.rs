use std::{fs, io};

fn build_tree_map(input: &str) -> Vec<Vec<u32>> {
    const RADIX: u32 = 10;
    let mut tree_map = Vec::new();
    for line in input.lines() {
        let tree_line = line
            .chars()
            .map(|d| d.to_digit(RADIX).expect("Every digit is a valid u32"))
            .collect::<Vec<_>>();
        tree_map.push(tree_line);
    }
    tree_map
}

fn initalize_result_map<T, U: Copy>(tree_map: &[Vec<T>], u: U) -> Vec<Vec<U>> {
    let mut vis_map = Vec::new();
    for row in tree_map.iter() {
        vis_map.push(vec![u; row.len()]);
    }
    vis_map
}

fn part_1(tree_map: &[Vec<u32>]) -> u32 {
    let mut vis_map = initalize_result_map(tree_map, false);

    // Check L to R
    for r in 0..tree_map.len() {
        let mut max_seen = None;
        for c in 0..tree_map[r].len() {
            let curr = Some(tree_map[r][c]);
            if curr > max_seen {
                vis_map[r][c] = true;
                max_seen = curr;
            }
        }
    }

    // Check R to L
    for r in 0..tree_map.len() {
        let mut max_seen = None;
        for c in (0..tree_map[r].len()).rev() {
            let curr = Some(tree_map[r][c]);
            if curr > max_seen {
                vis_map[r][c] = true;
                max_seen = curr;
            }
        }
    }

    // Check T to B
    for c in 0..tree_map[0].len() {
        let mut max_seen = None;
        for r in 0..tree_map.len() {
            let curr = Some(tree_map[r][c]);
            if curr > max_seen {
                vis_map[r][c] = true;
                max_seen = curr;
            }
        }
    }

    // Check B to T
    for c in 0..tree_map[0].len() {
        let mut max_seen = None;
        for r in (0..tree_map.len()).rev() {
            let curr = Some(tree_map[r][c]);
            if curr > max_seen {
                vis_map[r][c] = true;
                max_seen = curr;
            }
        }
    }

    // Count visible
    let mut count = 0;
    for r in vis_map.iter() {
        for c in r.iter() {
            if *c {
                count += 1;
            }
        }
    }
    count
}

fn part_2(tree_map: &[Vec<u32>]) -> u32 {
    let mut score_map = initalize_result_map(tree_map, 0);
    for r in 0..tree_map.len() {
        for c in 0..tree_map[r].len() {
            let tree_height = tree_map[r][c];

            let right_score = {
                let mut score = 0;
                for c_right in c + 1..tree_map[r].len() {
                    score += 1;
                    if tree_map[r][c_right] >= tree_height {
                        break;
                    }
                }
                score
            };

            let left_score = {
                let mut score = 0;
                for c_left in (0..c).rev() {
                    score += 1;
                    if tree_map[r][c_left] >= tree_height {
                        break;
                    }
                }
                score
            };

            let down_score = {
                let mut score = 0;
                for r_down in tree_map.iter().skip(r + 1) {
                    score += 1;
                    if r_down[c] >= tree_height {
                        break;
                    }
                }
                score
            };

            let up_score = {
                let mut score = 0;
                for r_up in (0..r).rev() {
                    score += 1;
                    if tree_map[r_up][c] >= tree_height {
                        break;
                    }
                }
                score
            };
            score_map[r][c] = right_score * left_score * down_score * up_score;
        }
    }

    // Find best score
    let mut best_score = 0;
    for r in score_map.iter() {
        for c in r.iter() {
            if *c > best_score {
                best_score = *c;
            }
        }
    }
    best_score
}

fn main() -> io::Result<()> {
    const FILE_PATH: &str = "input.txt";
    let input = fs::read_to_string(FILE_PATH)?;

    let tree_map = build_tree_map(&input);

    let result_1 = part_1(&tree_map);
    println!("day-08/part-1 = {result_1}");

    let result_2 = part_2(&tree_map);
    println!("day-08/part-2 = {result_2}");

    Ok(())
}
