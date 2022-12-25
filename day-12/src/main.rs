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

fn shortest_path(height_map: &[Vec<u8>], start: (usize, usize), target: (usize, usize)) -> u32 {
    let mut distances = vec![vec![u32::MAX; height_map[0].len()]; height_map.len()];
    let mut prev = vec![vec![None; height_map[0].len()]; height_map.len()];
    let mut visited = vec![vec![false; height_map[0].len()]; height_map.len()];
    let mut target_found = false;
    distances[start.0][start.1] = 0;

    while visited.iter().flatten().any(|b| !b) && !target_found {
        let mut min_distance = u32::MAX;

        let mut next_index = (0, 0);
        for r in 0..distances.len() {
            for c in 0..distances[r].len() {
                let distance = distances[r][c];
                if !visited[r][c] && distance < min_distance {
                    min_distance = distance;
                    next_index = (r, c);
                }
            }
        }
        let (row, col) = next_index;
        visited[row][col] = true;
        if next_index == target {
            target_found = true;
        } else {
            let current_height = height_map[row][col];
            let current_distance = distances[row][col];
            let neighbors = [
                (row + 1, col),
                (row - 1, col),
                (row, col + 1),
                (row, col - 1),
            ];
            for &(r, c) in neighbors.iter() {
                if !visited[r][c] && height_map[r][c] <= current_height + 1 {
                    let alt = current_distance + 1;
                    if alt < distances[r][c] {
                        distances[r][c] = alt;
                        prev[r][c] = Some((row, col));
                    }
                }
            }
        }
    }
    distances[target.0][target.1]
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let height_map = input
        .lines()
        .map(|line| line.as_bytes().to_owned())
        .collect::<Vec<_>>();

    let mut bordered_map = border(&height_map, u8::MAX);
    let start @ (ri, ci) = find_element(&bordered_map, b'S');
    let target @ (rf, cf) = find_element(&bordered_map, b'E');
    bordered_map[ri][ci] = b'a';
    bordered_map[rf][cf] = b'z';
    let result_1 = shortest_path(&bordered_map, start, target);
    println!("day-12/part-1: {result_1}");
    Ok(())
}
