
//Run cycle returns program counter
pub fn run_cycle(command: &String, registers: &mut Vec<f64>, program_counter: &mut usize){
    println!("Run command");
    //Decode state 
    let command_parts = command.split_whitespace();
    let vec_command_parts = command_parts.collect::<Vec<&str>>();
    let opcode = vec_command_parts[0];
    let mut destination: &str = ""; let mut _reg_destination_int: usize = 0;
    let mut register_one: &str = ""; let mut _reg_one_int: usize = 0;
    let mut register_two: &str = ""; let mut _reg_two_int: usize = 0;

    println!("{}", vec_command_parts.len());

    if vec_command_parts.len() > 1 {
        destination = vec_command_parts[1];
        _reg_destination_int = destination.replace("r", "").parse::<usize>().expect("Err");
    }
    if vec_command_parts.len() > 2 {
        register_one = vec_command_parts[2];
        _reg_one_int = register_one.replace("r", "").parse::<usize>().expect("Err");
    }
    if vec_command_parts.len() > 3{
        register_two = vec_command_parts[3];
        _reg_two_int = register_two.replace("r", "").parse::<usize>().expect("Err");
    }

    println!("{}, {}, {}", destination, register_one, register_two);
    //Actual Commands 
    if opcode == "ADD" {
        if destination.contains("r") == false || register_one.contains("r") == false || register_two.contains("r") == false{
            panic!("ADD HAS BAD FORMATTING");
        }

        let ans = registers[_reg_one_int] + registers[_reg_two_int];

        (registers)[_reg_destination_int] = ans;
        *program_counter += 1;
    }
    else if opcode == "SUB"{
        if destination.contains("r") == false || register_one.contains("r") == false || register_two.contains("r") == false{
            panic!("SUB HAS BAD FORMATTING");
        }

        let ans = registers[_reg_one_int] - registers[_reg_two_int];

        (registers)[_reg_destination_int] = ans;
        *program_counter += 1;
    }
    else if opcode == "MUL"{
        if destination.contains("r") == false || register_one.contains("r") == false || register_two.contains("r") == false{
            panic!("MUL HAS BAD FORMATTING");
        }

        let ans = registers[_reg_one_int] * registers[_reg_two_int];

        (registers)[_reg_destination_int] = ans;
        *program_counter += 1;
    }
    else if opcode == "ADDI"{
        if destination.contains("r") == false || register_one.contains("r") == false || register_two.contains("r") == true{
            panic!("ADDI HAS BAD FORMATTING");
        }

        let ans = registers[_reg_one_int] + _reg_two_int as f64;

        (registers)[_reg_destination_int] = ans;
        *program_counter += 1;
    }
    else if opcode == "COMP"{

    }
    else if opcode == "LDC"{

    }
    else if opcode == "LD"{

    }
    else if opcode == "BEQ"{

    }
    else if opcode == "BNE"{

    }
    else if opcode == "STR"{

    }
    else if opcode == "JUMP"{

    }
    else if opcode == "NOP"{
        println!("Has break changed? this shoudln't be reached (opcode == nop)");
    }
    else{
        panic!("OPCODE NOT RECONGISED")
    }
}