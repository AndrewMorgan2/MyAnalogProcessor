
//Run cycle returns program counter
pub fn run_cycle(command: &String, registers: &mut Vec<f64>, program_counter: &mut usize){
    println!("Run command");
    //Decode state 
    let command_parts = command.split_whitespace();
    let vec_command_parts = command_parts.collect::<Vec<&str>>();
    let opcode = vec_command_parts[0];
    let destination = vec_command_parts[1];
    let register_one = vec_command_parts[2];
    let register_two = vec_command_parts[3];
    println!("{}, {}, {}", destination, register_one, register_two);

    //Actual Commands 
    if opcode == "ADD" {
        if destination.contains("r") == false || register_one.contains("r") == false || register_two.contains("r") == false{
            panic!("ADD HAS BAD FORMATTING");
        }

        println!("{}", register_one.replace("r", ""));

        let reg_one_int = register_one.parse::<usize>().expect("Err");
        let reg_two_int = register_two.parse::<usize>().expect("Err");

        let ans = registers[reg_one_int] + registers[reg_two_int];
        registers[destination.parse::<usize>().expect("Err")] = ans;
        *program_counter += 1;
    }
    else if opcode == "SUB"{

    }
    else if opcode == "MUL"{

    }
    else if opcode == "ADDI"{

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

    }
    else{
        panic!("OPCODE NOT RECONGISED")
    }
}