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

fn initalize_visibility_map<T>(tree_map: &[Vec<T>]) -> Vec<Vec<bool>> {
    let mut vis_map = Vec::new();
    for row in tree_map.iter() {
        vis_map.push(vec![false; row.len()]);
    }
    vis_map
}

fn main() -> io::Result<()> {
    const FILE_PATH: &str = "input.txt";
    let input = fs::read_to_string(FILE_PATH)?;

    let tree_map = build_tree_map(&input);
    let mut vis_map = initalize_visibility_map(&tree_map);

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

    println!("day-08/part-1 = {count}");
    Ok(())
}
