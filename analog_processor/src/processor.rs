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

    //Remove the new line character
    let len = line.len();
    line.truncate(len - 1);

    //Formatting the fetch file
    let director_string: &str = "../code/";
    let file_type_string: &str = ".txt";
    let file = format!("{}{}{}",director_string, line, file_type_string);

    //Get file 
    println!("Starting program {}", file);
    let commands = lines_from_file(file).expect("Could not load commands");
    let all_opcode = lines_from_file("../all_opcodes.txt").expect("Could not load opcodes");
    let command_opcode_length = 4;

    for command in commands.clone() {
        //Check to see if the command is valid 
        
        //Loop though and see if the opcode is in all_opcode
        let command_opcode_check = truncate(command.as_str(), command_opcode_length).to_string();
        let mut opcode_seen = false;
        for opcode in &all_opcode{
            if opcode == &command_opcode_check {
                opcode_seen = true;
            }
        }
        if opcode_seen == false {
            println!("{}", truncate(command.as_str(), command_opcode_length));
            panic!("File had unknown opcodes");
        }
    }

    ioport::recieve_commands(commands);
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}