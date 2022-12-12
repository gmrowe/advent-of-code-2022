use std::{collections::HashMap, fs, io};

#[derive(Debug, Default)]
struct Dir {
    size: u32,
    children: Vec<String>,
    parent: Option<String>,
}

impl Dir {
    fn with_parent(parent: &str) -> Dir {
        Self {
            parent: Some(parent.to_owned()),
            ..Default::default()
        }
    }
}

fn total_dir_size(directory: &str, file_system: &HashMap<String, Dir>) -> u32 {
    let dir = &file_system[directory];
    let contained: u32 = dir
        .children
        .iter()
        .map(|d| total_dir_size(d, file_system))
        .sum();
    dir.size + contained
}

fn is_cd_instruction(line: &str) -> bool {
    line.starts_with("$ cd")
}

fn is_ls_instruction(line: &str) -> bool {
    line.starts_with("$ ls")
}

fn is_dir_description(line: &str) -> bool {
    line.starts_with("dir")
}

fn change_dir(current_dir_name: &str, line: &str, file_system: &HashMap<String, Dir>) -> String {
    let new_segment = line
        .split_whitespace()
        .rev()
        .next()
        .expect("cd must be followed by dir name");

    match new_segment {
        "/" => new_segment.to_owned(),
        ".." => {
            let dir = file_system
                .get(current_dir_name)
                .unwrap_or_else(|| panic!("{current_dir_name} is not in file_system"));
            dir.parent
                .as_ref()
                .map(String::from)
                .unwrap_or_else(|| panic!("{current_dir_name} has no parent"))
        }
        _ => {
            let mut new_dir = String::from(current_dir_name);
            new_dir.push_str(new_segment);
            new_dir.push('/');
            new_dir
        }
    }
}

fn add_new_dir(current_dir_name: &str, line: &str, file_system: &mut HashMap<String, Dir>) {
    let dir_name = line
        .split_whitespace()
        .rev()
        .next()
        .expect("Dir format is 'dir dir_name'");
    let mut full_child_path = String::from(current_dir_name);
    full_child_path.push_str(dir_name);
    full_child_path.push('/');
    let current_dir = file_system
        .get_mut(current_dir_name)
        .unwrap_or_else(|| panic!("{current_dir_name} is not in file_system"));
    current_dir.children.push(String::from(&full_child_path));
    file_system.insert(full_child_path, Dir::with_parent(current_dir_name));
}

fn update_dir_size(current_dir_name: &str, line: &str, file_system: &mut HashMap<String, Dir>) {
    // Line must represent a filename
    let (filesize, _filename) = line
        .split_once(char::is_whitespace)
        .expect("Filename format is 'filesize filename");
    let size = filesize.parse::<u32>().expect("Filesize is a valid u32");
    let mut dir = file_system
        .get_mut(current_dir_name)
        .expect("Current dir is in filesystem");
    dir.size += size;
}

fn parse_file_system_from_transctipt(transcript: &str) -> HashMap<String, Dir> {
    let mut file_system = HashMap::<String, Dir>::new();
    file_system.insert("/".to_owned(), Dir::default());
    let mut current_dir_name = "".to_owned();

    for line in transcript.lines() {
        if is_cd_instruction(line) {
            current_dir_name = change_dir(&current_dir_name, line, &file_system);
        } else if is_ls_instruction(line) {
            continue;
        } else if is_dir_description(line) {
            add_new_dir(&current_dir_name, line, &mut file_system);
        } else {
            update_dir_size(&current_dir_name, line, &mut file_system);
        }
    }
    file_system
}

fn part_1(file_system: &HashMap<String, Dir>) -> u32 {
    const SIZE_LIMIT: u32 = 100000;
    file_system
        .keys()
        .map(|name| total_dir_size(name, file_system))
        .filter(|&size| size <= SIZE_LIMIT)
        .sum()
}

fn part_2(file_system: &HashMap<String, Dir>) -> u32 {
    const TOTAL_DISK_SPACE: u32 = 70000000;
    const REQUIRED_SPACE: u32 = 30000000;
    let used_space = total_dir_size("/", file_system);
    let free_space = TOTAL_DISK_SPACE - used_space;
    let need_to_free = REQUIRED_SPACE - free_space;
    file_system
        .keys()
        .map(|name| total_dir_size(name, file_system))
        .filter(|&size| size >= need_to_free)
        .min()
        .expect("At least one file exists that can be deleted")
}

#[allow(unused)]
fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let file_system = parse_file_system_from_transctipt(&input);

    let part_1_result = part_1(&file_system);
    println!("day-07;part-1 = {part_1_result}");
    let part_2_result = part_2(&file_system);
    println!("day-07;part-2 = {part_2_result}");

    Ok(())
}
