pub mod token;
use self::token::Token;
use colored::Colorize;

pub fn tokenize(content: &str, v: &str, loud: bool) -> Vec<Token> {
    super::louden("INTERPRETER".on_green(), "Tokenizing...", loud);
    let mut tokens: Vec<Token> = vec![];
    for command in content.split("\n").collect::<Vec<&str>>() {
        let mut split_cmd: Vec<&str> = command.split("").collect();
        split_cmd.retain(|&x| !(x == "" || x == "\r"));
        split_cmd.push("EOL");
        let mut instr: String = String::new();
        let mut instr_temp: String = String::new();
        let mut current_value: String = String::new();
        let mut value: Vec<String> = vec![];
        let mut is_label: bool = false;
        let mut label_temp: String = String::new();
        let mut label: String = String::new();
        let mut is_string: bool = false;
        for i in 0..split_cmd.len() {
            let _char: &str = split_cmd[i];
            if _char == "\"" {
                is_string = !is_string;
                continue;
            }
            if is_string {
                value.push((_char.chars().next().expect("") as u32).to_string());
                continue;
            }
            if is_label {
                if _char == ":" {
                    label = label_temp;
                    label_temp = String::new();
                } else {
                    label_temp += _char;
                    continue;
                }
            }
            if _char == "EOL" || _char == ";" {
                if instr == "" {
                    instr = instr_temp;
                    instr_temp = String::new();
                } else if current_value != "" {
                    value.push(current_value);
                }
                break;
            }
            if _char == " " {
                if current_value != "" {
                    // We have reached a new value
                    value.push(current_value);
                    current_value = String::new();
                }
                if instr_temp == "" {
                    // we have not yet received any instruction, so it's an indentation whitespace
                    continue;
                } else if current_value == "" {
                    // we haven't reached any values yet
                    instr = instr_temp;
                    instr_temp = String::new();
                }
            } else if _char != " " {
                if i == 0 {
                    // :: !whitespace & first char => label
                    is_label = true;
                    label_temp += _char;
                    continue;
                }
                if instr == "" {
                    // we still don't have an instruction
                    instr_temp += _char;
                } else {
                    // we have an instruction
                    current_value += _char;
                }
            }
        }
        if label != "" {
            tokens.push(Token::from("label", &label));
        }
        if instr != "" {
            tokens.push(Token::from("instr", &instr));
            tokens.push(Token::from("value", &value.join(";")));
        }
    }
    return tokens;
}