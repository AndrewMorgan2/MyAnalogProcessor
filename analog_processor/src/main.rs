//import standard
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
//Other files
mod ioport;

fn main() {
    //Fetching program to run from user
    let mut line = String::new();
    println!("Enter program to run:");
    std::io::stdin().read_line(&mut line).unwrap();

    let test_number: usize;
    let file: String;
    //Run tests when program input is nothing
    if line.len() < 2 {
        file = "../tests/testCase1.txt".to_string();
        test_number = 1;
    } else {
    test_number = 0;
    //Remove the new line character
    let len = line.len();
    line.truncate(len - 1);

    //Formatting the fetch file
    let director_string: &str = "../code/";
    let file_type_string: &str = ".txt";
    file = format!("{}{}{}",director_string, line, file_type_string);
    }
    //Get file 
    println!("Starting program {}", file);
    let commands = lines_from_file(file).expect("Could not load commands");

    let result = ioport::recieve_commands(commands, test_number);

    if result == true {
        println!("Successful exuction");
    } else {
        println!("Something when wrong");
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

pub fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}