use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::BufRead;

enum Command {
    Cd,
    Ls,
    Output,
}

impl Command {
    fn is_command(&self) -> bool {
        match self {
            Command::Output => false,
            _ => true,
        }
    }

    fn is_output(&self) -> bool {
        !self.is_command()
    }
}

#[derive(Debug)]
struct Directory {
    path: String, //put in whole path here, makes things easier, ie. /d/ (end with /)
    sub_directories: Vec<String>, //only name of subdir
    files: Vec<usize>, //just input the size of the files for now
}

impl Directory {
    fn sub_directory_paths(&self) -> Vec<String> {
        self.sub_directories
            .iter()
            .map(|d| format!("{}{}/", self.path, d))
            .collect()
    }

    fn files_size(&self) -> usize {
        self.files.iter().sum()
    }

    fn size(&self, directory_paths: &HashMap<String, Directory>) -> usize {
        let files = self.files_size();
        let sub_dirs: usize = self
            .sub_directory_paths()
            .iter()
            .map(|dir| directory_paths[dir].size(&directory_paths))
            .sum();
        files + sub_dirs
    }
}

fn main() -> Result<()> {
    let result = run("input.txt")?;
    println!("{}", result);
    Ok(())
}

fn run(input: &str) -> Result<usize> {
    // Do in a way, so it can also be tested!!
    let input = read_input(input)?;
    let mut current_dir_path: Vec<String> = vec!["".to_owned()]; // "" is for "/"
    let mut directories: HashMap<String, Directory> = HashMap::new();
    let mut input = input.into_iter().skip(1);
    while let Some(line) = input.next() {
        match command_type(&line) {
            Command::Cd => cd(&line, &mut current_dir_path),
            Command::Ls => {
                let current_path = path_to_string(&current_dir_path);
                let output: Vec<String> = input
                    .take_while_ref(|x| command_type(x).is_output())
                    .collect();
                directories.insert(current_path.clone(), ls(&output, current_path));
            }
            Command::Output => panic!("An 'Output' enum should never be passed here"),
        }
    }
    // Ok(at_most_100000(directories))
    Ok(free_space_30_000_000(directories))
}

fn at_most_100000(directories: HashMap<String, Directory>) -> usize {
    directories
        .iter()
        .map(|(_, dir)| dir.size(&directories))
        .filter(|&x| x <= 100_000)
        .sum()
}

fn free_space_30_000_000(directories: HashMap<String, Directory>) -> usize {
    let used_space = directories["/"].size(&directories);
    let free_space = 70_000_000 - used_space;
    directories
        .iter()
        .map(|(_, dir)| dir.size(&directories))
        .filter(|x| free_space + x >= 30_000_000)
        .min()
        .expect("Removing root (/) will always give enough space, so should never be empty")

}

fn read_input(input: &str) -> Result<Vec<String>> {
    let file = std::fs::File::open(input)?;
    let buf = std::io::BufReader::new(file);
    buf.lines().map(|l| Ok(l?)).collect()
}

fn path_to_string(dir_path: &Vec<String>) -> String {
    if dir_path.len() == 1 {
        return String::from("/");
    }
    format!("/{}/", dir_path[1..].join("/"))
    // dir_path.join("")
}

fn command_type(line: &str) -> Command {
    let mut line = line.split_whitespace();
    match line.next() {
        Some(x) if x == "$" => match line.next() {
            Some(x) if x == "cd" => Command::Cd,
            Some(x) if x == "ls" => Command::Ls,
            _ => panic!("Not a valid command"),
        },
        Some(_) => Command::Output,
        _ => panic!("Not a valid input"),
    }
}

fn cd(line: &str, current_dir_path: &mut Vec<String>) {
    let mut line = line.split_whitespace();
    match line.nth(2) {
        Some(x) if x == ".." => {
            current_dir_path.pop();
        }
        Some(x) => current_dir_path.push(x.to_owned()),
        None => panic!("Line should not be empty"),
    }
}

fn ls(lines: &[String], path: String) -> Directory {
    let mut files = Vec::new();
    let mut sub_directories = Vec::new();

    for line in lines.iter() {
        let mut line = line.split_whitespace();
        match line.next() {
            Some(x) if x == "dir" => sub_directories.push(line.next().unwrap().to_owned()),
            Some(x) if x.parse::<usize>().is_ok() => files.push(x.parse::<usize>().unwrap()),
            _ => (),
        };
    }

    Directory {
        path,
        files,
        sub_directories,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_sample() -> Result<()> {
        let result = run("test.txt")?;
        // Ok(assert_eq!(result, 95437))
        Ok(assert_eq!(result, 24933642))
    }

    #[test]
    fn random_stuff() {
        let x = vec!["hej".to_owned(), "med".to_owned(), "dig".to_owned()];
        let mut x = x.iter();
        x.next();
        let vecc: Vec<&String> = x.collect();
        println!("{:?}", vecc);
        panic!()
    }
}
