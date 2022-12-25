use std::{
    fmt::Error,
    fmt::{Display, Write},
    fs, io,
};

#[allow(unused)]
fn dump<T: Display>(height_map: &[Vec<T>]) -> Result<String, Error> {
    let mut output = String::new();
    for line in height_map.iter() {
        for byte in line.iter() {
            write!(&mut output, "{byte:4}")?;
        }
        writeln!(&mut output)?;
    }
    Ok(output)
}

fn border<T: Clone>(slice_2d: &[Vec<T>], border_val: T) -> Vec<Vec<T>> {
    assert!(!slice_2d.is_empty());
    let border_len = slice_2d[0].len() + 2;
    let mut output = Vec::new();
    output.push(vec![border_val.clone(); border_len]);
    for v in slice_2d.iter() {
        let mut row = Vec::new();
        row.push(border_val.clone());
        row.extend(v.iter().cloned());
        row.push(border_val.clone());
        output.push(row);
    }
    output.push(vec![border_val; border_len]);
    output
}

fn find_element(height_map: &[Vec<u8>], element: u8) -> (usize, usize) {
    for (i, row) in height_map.iter().enumerate() {
        if let Some(j) = row.iter().position(|&b| b == element) {
            return (i, j);
        }
    }
    panic!("Element not found in map: {element}")
}

fn find_next_index(nodes: &[Vec<DijkstraNode>]) -> (usize, usize) {
    let mut next_index = None;
    let mut min_distance = u32::MAX;
    for (r, vec) in nodes.iter().enumerate() {
        for (c, node) in vec.iter().enumerate() {
            if !node.visited && node.distance < min_distance {
                min_distance = node.distance;
                next_index = Some((r, c));
            }
        }
    }
    next_index.expect("There should be at least one unvisited index")
}

#[derive(Debug, Clone, Copy)]
struct DijkstraNode {
    distance: u32,
    prev: Option<(usize, usize)>,
    visited: bool,
}

impl Default for DijkstraNode {
    fn default() -> DijkstraNode {
        DijkstraNode {
            distance: u32::MAX,
            prev: None,
            visited: false,
        }
    }
}

// Shortest path implemented using Dijkstra's algorithm [https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm]
fn shortest_path(height_map: &[Vec<u8>], start: (usize, usize), target: (usize, usize)) -> u32 {
    let mut nodes = vec![vec![DijkstraNode::default(); height_map[0].len()]; height_map.len()];
    let mut target_found = false;
    nodes[start.0][start.1].distance = 0;

    while !target_found {
        let next_index @ (row, col) = find_next_index(&nodes);
        let mut curr_node = &mut nodes[row][col];
        curr_node.visited = true;
        if next_index == target {
            target_found = true;
        } else {
            let current_height = height_map[row][col];
            let current_distance = curr_node.distance;
            let neighbors = [
                (row + 1, col),
                (row - 1, col),
                (row, col + 1),
                (row, col - 1),
            ];
            for &(r, c) in neighbors.iter() {
                let mut neighbor_node = &mut nodes[r][c];
                if !neighbor_node.visited && height_map[r][c] <= current_height + 1 {
                    let alt = current_distance + 1;
                    if alt < neighbor_node.distance {
                        neighbor_node.distance = alt;
                        neighbor_node.prev = Some((row, col));
                    }
                }
            }
        }
    }
    nodes[target.0][target.1].distance
}

fn build_height_map(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.as_bytes().to_owned())
        .collect()
}

fn part_1(input: &str) -> u32 {
    let height_map = build_height_map(input);

    // Surround map with u8::MAX to simplify edge detection
    let mut bordered_map = border(&height_map, u8::MAX);
    let start @ (ri, ci) = find_element(&bordered_map, b'S');
    let target @ (rf, cf) = find_element(&bordered_map, b'E');
    bordered_map[ri][ci] = b'a';
    bordered_map[rf][cf] = b'z';
    shortest_path(&bordered_map, start, target)
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let result_1 = part_1(&input);
    println!("day-12/part-1: {result_1}");
    Ok(())
}
