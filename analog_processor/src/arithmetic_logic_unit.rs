
//Run cycle returns program counter
pub fn run_cycle(command: &String, registers: &mut Vec<f64>, program_counter: &mut usize){
    
    //println!("Run command {}", command);
    //Decode state 
    let command_parts = command.split_whitespace();
    let vec_command_parts = command_parts.collect::<Vec<&str>>();
    let opcode = vec_command_parts[0];
    let mut destination: &str = ""; let mut _reg_destination_int: usize = 0;
    let mut register_one: &str = ""; let mut _reg_one_int: usize = 0;
    let mut register_two: &str = ""; let mut _reg_two_int: usize = 0;

    //println!("{}", vec_command_parts.len());

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

    //println!("{}, {}, {}", destination, register_one, register_two);

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
    else if opcode == "DIV"{
        if destination.contains("r") == false || register_one.contains("r") == false || register_two.contains("r") == false{
            panic!("DIV HAS BAD FORMATTING");
        }

        let ans = registers[_reg_one_int] / registers[_reg_two_int];

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
        if destination.contains("r") == false || register_one.contains("r") == false{
            panic!("COMP HAS BAD FORMATTING");
        }
        let compare_int:f64;
        if register_two.contains("r") == true {
            compare_int = registers[_reg_two_int];
        } 
        else {
            compare_int = _reg_two_int as f64;
        }

        let ans:f64;
        //Cases for compare 
        if registers[_reg_one_int] < compare_int{
            ans = -1.0;
        }
        else if registers[_reg_one_int] > compare_int{
            ans = 1.0;
        }
        else {
            ans = 0.0;
        }
        //println!("COMP result {}", ans);
        (registers)[_reg_destination_int] = ans;
        *program_counter += 1;
    }
    else if opcode == "LDC"{
        if destination.contains("r") == false || register_one.contains("r") == true{
            panic!("LDC HAS BAD FORMATTING");
        }
        (registers)[_reg_destination_int] = _reg_one_int as f64;
        *program_counter += 1;
    }
    else if opcode == "LD"{
        if destination.contains("r") == false || register_one.contains("r") == false{
            panic!("LD HAS BAD FORMATTING");
        }
        registers[_reg_destination_int] = registers[_reg_one_int];
        *program_counter += 1;
    }
    else if opcode == "BEQ"{
        if destination.contains("r") == true || register_one.contains("r") == false || register_two.contains("r") == true{
            panic!("BEQ HAS BAD FORMATTING");
        }

        if registers[_reg_one_int] == _reg_two_int as f64{
            *program_counter = _reg_destination_int;
            //println!("BEQ caused jump to {}", _reg_destination_int);
        } else {
            *program_counter += 1;
        }
    }
    else if opcode == "BNE"{
        if destination.contains("r") == true || register_one.contains("r") == false || register_two.contains("r") == false{
            panic!("BNE HAS BAD FORMATTING");
        }

        if registers[_reg_one_int] != registers[_reg_two_int]{
            *program_counter = _reg_destination_int;
            //println!("BNE caused jump to {}", _reg_destination_int);
        } else {
            *program_counter += 1;
        }
    }
    else if opcode == "STR"{
        if destination.contains("r") == false || register_one.contains("r") == false{
            panic!("STR HAS BAD FORMATTING");
        }
        let index_register: usize = registers[_reg_destination_int] as usize;
        
        registers[index_register] = registers[_reg_one_int];
        *program_counter += 1;
    }
    else if opcode == "JUMP"{
        if destination.contains("r") == true {
            panic!("JUMP HAS BAD FORMATTING");
        }
        *program_counter = _reg_destination_int;
    }
    else if opcode == "NOP"{
        println!("Has break changed? this shoudln't be reached (opcode == nop)");
    }
    else{
        panic!("OPCODE NOT RECONGISED")
    }
}