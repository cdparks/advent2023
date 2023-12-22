use std::{
    fs::{File, OpenOptions},
    io::Write,
    process,
};

use crate::Day;

const MODULE_TEMPLATE: &str = r#"advent_of_code::solution!(DAY_NUMBER);

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(1, "examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(2, "examples", DAY));
        assert_eq!(result, None);
    }
}
"#;

fn safe_create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

fn create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create(true).open(path)
}

pub fn handle(day: Day) {
    let input_path = format!("data/inputs/{day}.txt");
    let example_path_1 = format!("data/examples/{day}-1.txt");
    let example_path_2 = format!("data/examples/{day}-2.txt");
    let module_path = format!("src/bin/{day}.rs");

    let mut file = match safe_create_file(&module_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create module file: {e}");
            process::exit(1);
        }
    };

    match file.write_all(
        MODULE_TEMPLATE
            .replace("DAY_NUMBER", &day.into_inner().to_string())
            .as_bytes(),
    ) {
        Ok(()) => {
            println!("Created module file \"{}\"", &module_path);
        }
        Err(e) => {
            eprintln!("Failed to write module contents: {e}");
            process::exit(1);
        }
    }

    match create_file(&input_path) {
        Ok(_) => {
            println!("Created empty input file \"{}\"", &input_path);
        }
        Err(e) => {
            eprintln!("Failed to create input file: {e}");
            process::exit(1);
        }
    }

    match create_file(&example_path_1) {
        Ok(_) => {
            println!("Created empty example file for part 1 \"{}\"", &example_path_1);
        }
        Err(e) => {
            eprintln!("Failed to create example file: {e}");
            process::exit(1);
        }
    }

    match create_file(&example_path_2) {
        Ok(_) => {
            println!("Created empty example file for part 2 \"{}\"", &example_path_2);
        }
        Err(e) => {
            eprintln!("Failed to create example file: {e}");
            process::exit(2);
        }
    }

    println!("---");
    println!("🎄 Type `cargo solve {}` to run your solution.", day);
}
