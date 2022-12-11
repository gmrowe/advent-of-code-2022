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

#[allow(unused)]
fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    // file system maps directory name to (size, children names, optional parent names)
    let mut file_system = HashMap::<String, Dir>::new();
    file_system.insert("/".to_owned(), Dir::default());
    let mut current_dir_name = "".to_owned();

    for line in input.lines() {
        if line.starts_with("$ cd") {
            let dir_name = line
                .split_whitespace()
                .rev()
                .next()
                .expect("cd must be followed by dir name");
            if dir_name == "/" {
                current_dir_name = "/".to_owned();
            } else if dir_name == ".." {
                let dir = file_system
                    .get(&current_dir_name)
                    .unwrap_or_else(|| panic!("{current_dir_name} is not in file_system"));
                let parent = dir
                    .parent
                    .as_ref()
                    .unwrap_or_else(|| panic!("{current_dir_name} has no parent"));
                current_dir_name = String::from(parent);
            } else {
                current_dir_name.push_str(dir_name);
                current_dir_name.push('/');
            }
        } else if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("dir") {
            let dir_name = line
                .split_whitespace()
                .rev()
                .next()
                .expect("Dir format is 'dir dir_name'");
            let mut full_child_path = String::from(&current_dir_name);
            full_child_path.push_str(dir_name);
            full_child_path.push('/');
            let mut current_dir = file_system
                .get_mut(&current_dir_name)
                .unwrap_or_else(|| panic!("{current_dir_name} is not in file_system"));
            current_dir.children.push(String::from(&full_child_path));
            file_system.insert(full_child_path, Dir::with_parent(&current_dir_name));
        } else {
            // Line must represent a filename
            let (filesize, _filename) = line
                .split_once(char::is_whitespace)
                .expect("Filename format is 'filesize filename");
            let size = filesize.parse::<u32>().expect("Filesize is a valid u32");
            let mut dir = file_system
                .get_mut(&current_dir_name)
                .expect("Current dir is in filesystem");
            dir.size += size;
        }
    }

    const SIZE_LIMIT: u32 = 100000;
    let total_size: u32 = file_system
        .keys()
        .map(|name| total_dir_size(name, &file_system))
        .filter(|&size| size <= SIZE_LIMIT)
        .sum();

    println!("day-07;part-1 = {total_size}");

    Ok(())
}
