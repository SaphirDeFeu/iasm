use super::lexer;
use super::lexer::token::Token;
use super::lexer::token::TokenType;
use colored::Colorize;
use std::collections::HashMap;
use std::io::Write;

#[allow(unused_mut)]
pub fn interpret(content: &str, v: &str, loud: bool) -> Result<(), std::io::Error> {
    let tokens: Vec<Token> = lexer::tokenize(content, v, loud);
    let mut labels: HashMap<&str, usize> = HashMap::new();
    let mut executing: bool = false;
    let mut started: bool = false;
    let mut current_instruction: &str = "";
    let mut param_amount: usize = 0;
    let mut register_a: u32 = 0;
    let mut data: [u32; 0xFFFF] = [0u32; 0xFFFF];
    let mut stack: [u32; 0xFF] = [0u32; 0xFF];
    let mut subroutine_stack: [usize; 0xFF] = [0usize; 0xFF];
    let mut stack_ptr: u8 = 0;
    let mut sr_stack_ptr: u8 = 0;
    let mut f_zero: bool = false;
    let mut f_less: bool = false;
    let mut f_more: bool = false;
    super::louden("INTERPRETER".on_green(), "Finding labels...", loud);
    // Identify labels
    for i in 0..tokens.len() {
        let token = &tokens[i];
        if token.token_type == TokenType::LABEL {
            labels.insert(&token.value, i);
        }
        if i == tokens.len() - 1 {
            executing = true;
        }
    }
    // Check if we have a main label
    if !labels.contains_key("main") {
        super::throw(
            "ERR_LABEL_NOT_FOUND",
            "Couldn't find `main` label",
            0x5,
            file!(),
            v,
            line!(),
            true,
        );
    }
    super::louden("INTERPRETER".on_green(), "Interpreting...", loud);
    let mut i = 0;
    while i < tokens.len() {
        let token: &Token = &tokens[i];
        let mut imvalue: bool = false;
        if started == false {
            // First execution
            started = true;
            i = match labels.get("main") {
                Some(value) => value.to_owned(),
                None => {
                    super::throw(
                        "ERR_LABEL_NOT_FOUND",
                        "Couldn't find `main` label",
                        0x300,
                        file!(),
                        v,
                        line!(),
                        true,
                    );
                    0usize
                }
            };
            continue;
        }
        // Start interpreting
        match token.token_type {
            TokenType::INSTRUCTION => {
                current_instruction = &token.value;
            }
            TokenType::VALUES => {
                let str_values = token.value.split(";").collect::<Vec<&str>>();
                let mut values: Vec<u32> = vec![];
                let mut is_value_label: bool = false;
                let mut mem_used: bool = false;
                // some checks for immediate values and hexadecimal and binary
                for value in &str_values {
                    let mut list_of_chars = value.split("").collect::<Vec<&str>>();
                    list_of_chars.retain(|&x| x != "");
                    let mut newvalue: u32 = 0;
                    let mut hex: bool = false;
                    let mut bin: bool = false;
                    let mut from_memory: bool = false;
                    for j in 0..list_of_chars.len() {
                        if list_of_chars[j] == "#" {
                            if j == 0 {
                                imvalue = true;
                            } else {
                                super::throw(
                                    "ERR_INVALID_VALUE",
                                    "Immediate value specifier must stay at the first character of value",
                                    0x302,
                                    file!(),
                                    v,
                                    line!(),
                                    true
                                );
                            }
                        } else if list_of_chars[j] == ":" {
                            if j == 0 {
                                from_memory = true;
                                if current_instruction == "lda" {
                                    mem_used = true;
                                }
                            } else {
                                super::throw(
                                    "ERR_INVALID_VALUE",
                                    "Memory value specifier must stay at the first character of value",
                                    0x302,
                                    file!(),
                                    v,
                                    line!(),
                                    true
                                );
                            }
                        } else if list_of_chars[j] == "$" && (j == 0 || j == 1) {
                            // hex
                            hex = true;
                            newvalue =
                                match u32::from_str_radix(&list_of_chars[(j + 1)..].join(""), 16) {
                                    Ok(value) => value,
                                    Err(e) => {
                                        super::throw(
                                            "ERR_INT_PARSE_ERROR",
                                            &format!("{}", e),
                                            0x001,
                                            file!(),
                                            v,
                                            line!(),
                                            true,
                                        );
                                        0u32
                                    }
                                };
                            if from_memory && !mem_used {
                                let memory_value = data[newvalue as usize];
                                values.push(memory_value);
                                newvalue = 0;
                            }
                            break;
                        } else if list_of_chars[j] == "%" && (j == 0 || j == 1) {
                            // bin
                            bin = true;
                            newvalue =
                                match u32::from_str_radix(&list_of_chars[(j + 1)..].join(""), 2) {
                                    Ok(value) => value,
                                    Err(e) => {
                                        super::throw(
                                            "ERR_INT_PARSE_ERROR",
                                            &format!("{}", e),
                                            0x001,
                                            file!(),
                                            v,
                                            line!(),
                                            true,
                                        );
                                        0u32
                                    }
                                };
                            if from_memory && !mem_used {
                                let memory_value = data[newvalue as usize];
                                values.push(memory_value);
                                newvalue = 0;
                            }
                            break;
                        } else if !(hex || bin) {
                            newvalue =
                                match to_u32(list_of_chars[j..].join("").to_string(), v, false) {
                                    Some(value) => value,
                                    None => {
                                        is_value_label = true;
                                        0u32
                                    }
                                };
                            if from_memory && !mem_used {
                                let memory_value = data[newvalue as usize];
                                values.push(memory_value);
                                newvalue = 0;
                            }
                            break;
                        }
                    }
                    if !from_memory || mem_used {
                        values.push(newvalue);
                    }
                }
                match current_instruction {
                    "ret" => {
                        // Stops execution of the program
                        let ret_code = values[0];
                        std::process::exit(ret_code as i32);
                    }
                    "lda" => {
                        // Loads a value into register A
                        if imvalue {
                            register_a = values[0];
                        } else if mem_used {
                            register_a = data[data[values[0] as usize] as usize];
                        } else {
                            register_a = data[values[0] as usize];
                        }
                    }
                    "sta" => {
                        // Stores the value of register_a into the "RAM"
                        data[values[0] as usize] = register_a;
                    }
                    "pha" => {
                        // Push register A's value into the stack
                        stack[stack_ptr as usize] = register_a;
                        if stack_ptr == 255 {
                            stack_ptr = 0
                        } else {
                            stack_ptr += 1;
                        }
                    }
                    "pla" => {
                        // Pull from the stack and loads the pulled value into register A
                        if stack_ptr == 0 {
                            stack_ptr = 255
                        } else {
                            stack_ptr -= 1;
                        }
                        register_a = stack[stack_ptr as usize];
                    }
                    "add" => {
                        // Adds a number to register A
                        if imvalue {
                            register_a += values[0];
                        } else {
                            register_a += data[values[0] as usize];
                        }
                    }
                    "sub" => {
                        // Subtracts a number from register A
                        if imvalue {
                            register_a -= values[0];
                        } else {
                            register_a -= data[values[0] as usize];
                        }
                    }
                    "cmp" => {
                        // Compares two values at the specified data values
                        // If they're equal, then the zero flag is set to true
                        f_zero = data[values[0] as usize] == data[values[1] as usize];
                        f_less = data[values[0] as usize] < data[values[1] as usize];
                        f_more = data[values[0] as usize] > data[values[1] as usize];
                    }
                    "jmp" => {
                        // Jumps to a specified location
                        let _label = str_values[0];
                        if labels.contains_key(_label) {
                            i = match labels.get(_label) {
                                Some(value) => *value,
                                None => i,
                            };
                        } else {
                            i = values[0] as usize;
                        }
                    }
                    "beq" => {
                        // Jumps to a specified location if the zero flag is true
                        if f_zero {
                            let _label = str_values[0];
                            if labels.contains_key(_label) {
                                i = match labels.get(_label) {
                                    Some(value) => *value,
                                    None => i,
                                };
                            } else {
                                i = values[0] as usize;
                            }
                        }
                    }
                    "bne" => {
                        // Jumps to a specified location if the zero flag is true
                        if !f_zero {
                            let _label = str_values[0];
                            if labels.contains_key(_label) {
                                i = match labels.get(_label) {
                                    Some(value) => *value,
                                    None => i,
                                };
                            } else {
                                i = values[0] as usize;
                            }
                        }
                    }
                    "blt" => {
                        if f_less {
                            let _label = str_values[0];
                            if labels.contains_key(_label) {
                                i = match labels.get(_label) {
                                    Some(value) => *value,
                                    None => i,
                                };
                            } else {
                                i = values[0] as usize;
                            }
                        }
                    }
                    "bgt" => {
                        if f_more {
                            let _label = str_values[0];
                            if labels.contains_key(_label) {
                                i = match labels.get(_label) {
                                    Some(value) => *value,
                                    None => i,
                                };
                            } else {
                                i = values[0] as usize;
                            }
                        }
                    }
                    "jsr" => {
                        subroutine_stack[sr_stack_ptr as usize] = i;
                        if sr_stack_ptr == 0xFE {
                            sr_stack_ptr = 0x00;
                        } else {
                            sr_stack_ptr += 1;
                        }
                        let _label: &str = str_values[0];
                        if labels.contains_key(_label) {
                            i = match labels.get(_label) {
                                Some(value) => *value,
                                None => i,
                            };
                        } else {
                            i = values[0] as usize;
                        }
                    }
                    "rsr" => {
                        if sr_stack_ptr == 0 {
                            sr_stack_ptr = 0xFE;
                        } else {
                            sr_stack_ptr -= 1;
                        }
                        i = subroutine_stack[sr_stack_ptr as usize];
                    }
                    "and" => {
                        if imvalue {
                            register_a = register_a & values[0];
                        } else {
                            register_a = register_a & data[values[0] as usize];
                        }
                    }
                    "not" => {
                        if register_a == 1 {
                            register_a = 0;
                        } else if register_a == 0 {
                            register_a = 1;
                        } else {
                            super::throw(
                                "ERR_BIT_MANIPULATION_IMPOSSIBLE",
                                "Accumulator must be either 1 or 0 for `not` operation to function! To invert all bits, use `xor $1111111111111111`",
                                0x304,
                                file!(),
                                v,
                                line!(),
                                true
                            );
                        }
                    }
                    "xor" => {
                        if imvalue {
                            register_a = register_a ^ values[0];
                        } else {
                            register_a = register_a ^ data[values[0] as usize];
                        }
                    }
                    "or" => {
                        if imvalue {
                            register_a = register_a | values[0];
                        } else {
                            register_a = register_a | data[values[0] as usize];
                        }
                    }
                    "shl" => {
                        if imvalue {
                            register_a = register_a << values[0];
                        } else {
                            register_a = register_a << data[values[0] as usize];
                        }
                    }
                    "shr" => {
                        if imvalue {
                            register_a = register_a >> values[0];
                        } else {
                            register_a = register_a >> data[values[0] as usize];
                        }
                    }
                    "rol" => {
                        if imvalue {
                            register_a = ((register_a << values[0]) | (register_a >> (32 - values[0]))) & 0xFFFFFFFF;
                        } else {
                            register_a = ((register_a << data[values[0] as usize]) | (register_a >> (32 - data[values[0] as usize]))) & 0xFFFFFFFF;
                        }
                    }
                    "ror" => {
                        if imvalue {
                            register_a = ((register_a >> values[0]) | (register_a << (32 - values[0]))) & 0xFFFFFFFF;
                        } else {
                            register_a = ((register_a >> data[values[0] as usize]) | (register_a << (32 - data[values[0] as usize]))) & 0xFFFFFFFF;
                        }
                    }
                    "nop" => {
                        // No-operation for a certain time
                        if values.len() != 0 {
                            std::thread::sleep(std::time::Duration::from_millis(values[0] as u64));
                        }
                    }
                    "mov" => {
                        // Moves a value into a certain register
                        let dir = str_values[0];
                        match dir {
                            "cout" => {
                                for i in &values[1..] {
                                    if let Some(char_) = std::char::from_u32(*i) {
                                        print!("{}", char_);
                                    }
                                }
                                std::io::stdout().flush().unwrap();
                            }
                            "cin" => {
                                let mut x: String = String::new();
                                match std::io::stdin().read_line(&mut x) {
                                    Ok(n) => {
                                        let split: Vec<char> = x.chars().collect::<Vec<char>>().iter().filter(|&&c| c != '\r' && c != '\n').cloned().collect();
                                        let mut j: usize = 0;
                                        let mut limit: usize = split.len() + (values[1] as usize) - 1;
                                        if values.len() > 2 {
                                            limit = min(values[2] as usize, split.len()) + (values[1] as usize) - 1;
                                        }
                                        for i in (values[1] as usize)..=limit {
                                            data[i] = split[j] as u32;
                                            j += 1;
                                        }
                                        data[n + (values[1] as usize) - 2] = 0u32;
                                        // null byte to indicate end of string
                                    }
                                    Err(e) => {
                                        super::throw(
                                            "ERR_GENERAL",
                                            &format!("{}", e),
                                            0x001,
                                            file!(),
                                            v,
                                            line!(),
                                            true,
                                        );
                                    }
                                };
                            }
                            _ => {
                                super::throw(
                                    "ERR_INVALID_REGISTER",
                                    &format!("Unknown register: {dir:?}"),
                                    0x303,
                                    file!(),
                                    v,
                                    line!(),
                                    true,
                                );
                            }
                        }
                    }
                    _ => {
                        super::throw(
                            "ERR_INVALID_INSTRUCTION",
                            &format!("Unknown instruction: {current_instruction:?}"),
                            0x301,
                            file!(),
                            v,
                            line!(),
                            true,
                        );
                    }
                }
            }
            _ => {}
        };
        i += 1;
    }
    Ok(())
}

fn to_u32(string: String, v: &str, exit: bool) -> Option<u32> {
    let option = string.parse::<u32>();
    match option {
        Ok(value) => return Some(value),
        Err(e) => {
            if exit {
                super::throw(
                    "ERR_INT_PARSE_ERROR",
                    &format!("{}", e),
                    0x001,
                    file!(),
                    v,
                    line!(),
                    true,
                );
            }
            return None;
        }
    };
}

fn min(a: usize, b: usize) -> usize {
    if a < b {
        return a;
    } else {
        return b;
    }
}