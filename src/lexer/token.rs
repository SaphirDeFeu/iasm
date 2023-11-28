#[derive(Debug, PartialEq)]
pub enum TokenType {
    INSTRUCTION,
    VALUES,
    LABEL,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn from(_type: &str, _value: &str) -> Token {
        let type_found: TokenType = match _type {
            "label" | "LABEL" => TokenType::LABEL,
            "values" | "value" | "VALUES" | "VALUE" => TokenType::VALUES,
            "instruction" | "instr" | "INSTRUCTION" | "INSTR" | _ => TokenType::INSTRUCTION,
        };
        Token {
            token_type: type_found,
            value: _value.to_owned(),
        }
    }
}