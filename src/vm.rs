use std::num::ParseIntError;

use thiserror::Error;


pub const NUM_REGS: u8 = 4;

pub const NUM_MEMS: u32 = 65536;

pub const INERTIA_ADD: u8 = 0x0; // Addition
pub const INERTIA_DIV: u8 = 0x1; // Division
pub const INERTIA_MUL: u8 = 0x2; // Multiplication

pub const INERTIA_LTN: u8 = 0x3; // Less Than
pub const INERTIA_EQL: u8 = 0x4; // Equal To
pub const INERTIA_AND: u8 = 0x5; // Bitwise AND
pub const INERTIA_NOT: u8 = 0x6; // Bitwise NOT
pub const INERTIA_OR: u8 = 0x7; //  Bitwise OR
pub const INERTIA_INC: u8 = 0x8; //Increase by 1
pub const INERTIA_DEC: u8 = 0x9; //Decrease by 1

pub const INERTIA_PRINT: u8 = 0xA; // Print to stdout
pub const INERTIA_LOAD: u8 = 0xB; // Load value
pub const INERTIA_GOTO: u8 = 0xC; //goto
pub const INERTIA_IF: u8 = 0xD; //if par1 == false, skip to par2
pub const INERTIA_RETURN: u8 = 0xE; //return
pub const INERTIA_CALL: u8 = 0xF; // call function

// https://en.wikibooks.org/wiki/Creating_a_Virtual_Machine/Register_VM_in_C
// https://github.com/Conceptual-Inertia/Inertia/blob/master/Testcodes/Fibonacci.txt

#[derive(Debug)]
pub struct Instruction(u32);

#[derive(Debug)]
pub enum Number {
    Register(u8),
    Immediate(u8),
    Memory(u8)
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("ExpectedNumber")]
    ExpectedNumber
}


impl From<ParseIntError> for ParserError {
    fn from(value: ParseIntError) -> Self {
        ParserError::ExpectedNumber
    }
}

#[derive(Debug)]
pub struct ParserContext {
    opcodes: Vec<OpCode> 
}

#[derive(Debug)]
pub enum OpCode {
    Add {
        left: Number,
        right: Number,
        target: Number
    },

    Sub {
        left: Number,
        right: Number,
        target: Number
    },

    Mul {
        left: Number,
        right: Number,
        target: Number
    },

    Div {
        left: Number,
        right: Number,
        target: Number
    },

    Load {
        target: Number,
        value: Number
    }
}

#[derive(Debug)]
pub struct Parser;
impl Parser {
    fn eat_whitespaces(content: &str) -> &str {
        for (index, ch) in content.chars().enumerate() {
            if ch != ' ' && ch != '\r' && ch != '\n' && ch != '\t' {
                return &content[index..]
            }
        }

        content
    }

    fn eat_until_space(content: &str) -> &str {
        for (index, ch) in content.chars().enumerate() {
            if ch == ' ' {
                return &content[index..]
            }
        }

        content
    }

    fn eat_text(content: &str) -> (&str, &str) {
        for (index, ch) in content.chars().enumerate() {
            if ch == ' ' {
                return (&content[..index], &content[index..])
            }
        }

        (content, content)
    }

    fn eat_number(content: &str) -> Result<(Number, &str), ParserError> {
        let (number, content) = Self::eat_text(content);
        if number.starts_with('r') {
            Ok((Number::Register(number[1..].parse::<u8>()?), content))
        }
        else if number.starts_with('#') {
            Ok((Number::Immediate(number[1..].parse::<u8>()?), content))
        }
        else if number.starts_with('@') {
            Ok((Number::Memory(number[1..].parse::<u8>()?), content))
        } else {
            Err(ParserError::ExpectedNumber)
        }
    }

    fn parser_comment(opcodes: &mut Vec<OpCode>, line: &str) -> Result<bool, ParserError>  {
        Ok(line.starts_with('#'))
    }

    fn parser_add(opcodes: &mut Vec<OpCode>, line: &str) -> Result<bool, ParserError>  {
        if !line.starts_with("add") {
            return Ok(false);
        }

        let (add, line) = Self::eat_text(line);
        debug_assert_eq!(add, "add");

        let (left, line) = Self::eat_number(Self::eat_whitespaces(line))?;
        let (right, line) = Self::eat_number(Self::eat_whitespaces(line))?;
        let (target, _) = Self::eat_number(Self::eat_whitespaces(line))?;

        opcodes.push(OpCode::Add { left, right, target });
        Ok(true)
    }

    fn parser_sub(opcodes: &mut Vec<OpCode>, line: &str) -> Result<bool, ParserError>  {
        if !line.starts_with("sub") {
            return Ok(false);
        }

        let (sub, line) = Self::eat_text(line);
        debug_assert_eq!(sub, "sub");

        let (left, line) = Self::eat_number(Self::eat_whitespaces(line))?;
        let (right, line) = Self::eat_number(Self::eat_whitespaces(line))?;
        let (target, _) = Self::eat_number(Self::eat_whitespaces(line))?;

        opcodes.push(OpCode::Sub { left, right, target });
        Ok(true)
    }

    fn parser_div(opcodes: &mut Vec<OpCode>, line: &str) -> Result<bool, ParserError>  {
        if !line.starts_with("div") {
            return Ok(false);
        }

        let (div, line) = Self::eat_text(line);
        debug_assert_eq!(div, "div");

        let (left, line) = Self::eat_number(Self::eat_whitespaces(line))?;
        let (right, line) = Self::eat_number(Self::eat_whitespaces(line))?;
        let (target, _) = Self::eat_number(Self::eat_whitespaces(line))?;

        opcodes.push(OpCode::Div { left, right, target });
        Ok(true)
    }

    fn parser_mul(opcodes: &mut Vec<OpCode>, line: &str) -> Result<bool, ParserError>  {
        if !line.starts_with("mul") {
            return Ok(false);
        }

        let (mul, line) = Self::eat_text(line);
        debug_assert_eq!(mul, "mul");

        let (left, line) = Self::eat_number(Self::eat_whitespaces(line))?;
        let (right, line) = Self::eat_number(Self::eat_whitespaces(line))?;
        let (target, _) = Self::eat_number(Self::eat_whitespaces(line))?;

        opcodes.push(OpCode::Mul { left, right, target });
        Ok(true)
    }

    fn parser_ltn(opcodes: &mut Vec<OpCode>, line: &str) -> Result<bool, ParserError>  {
        Ok(line.starts_with("ltn"))
    }

    fn parser_eql(opcodes: &mut Vec<OpCode>, line: &str) -> Result<bool, ParserError>  {
        Ok(line.starts_with("eql"))
    }

    fn parser_and(opcodes: &mut Vec<OpCode>, line: &str) -> Result<bool, ParserError>  {
        Ok(line.starts_with("and"))
    }

    fn parser_not(opcodes: &mut Vec<OpCode>, line: &str) -> Result<bool, ParserError>  {
        Ok(line.starts_with("not"))
    }

    fn parser_or(opcodes: &mut Vec<OpCode>, line: &str) -> Result<bool, ParserError>  {
        Ok(line.starts_with("or"))
    }

    fn parser_inc(opcodes: &mut Vec<OpCode>, line: &str) -> Result<bool, ParserError>  {
        Ok(line.starts_with("inc"))
    }

    fn parser_dec(opcodes: &mut Vec<OpCode>, line: &str) -> Result<bool, ParserError>  {
        Ok(line.starts_with("dec"))
    }

    fn parser_print(opcodes: &mut Vec<OpCode>, line: &str) -> Result<bool, ParserError>  {
        Ok(line.starts_with("print"))
    }

    fn parser_load(opcodes: &mut Vec<OpCode>, line: &str) -> Result<bool, ParserError>  {
        if !line.starts_with("load") {
            return Ok(false);
        }

        let (load, line) = Self::eat_text(line);
        debug_assert_eq!(load, "load");

        let (target, line) = Self::eat_number(Self::eat_whitespaces(line))?;
        let (value, _) = Self::eat_number(Self::eat_whitespaces(line))?;

        opcodes.push(OpCode::Load { target, value });
        Ok(true)
    }

    fn parser_goto(opcodes: &mut Vec<OpCode>, line: &str) -> Result<bool, ParserError>  {
        Ok(line.starts_with("goto"))
    }

    fn parser_if(opcodes: &mut Vec<OpCode>, line: &str) -> Result<bool, ParserError>  {
        Ok(line.starts_with("if"))
    }

    fn parser_return(opcodes: &mut Vec<OpCode>, line: &str) -> Result<bool, ParserError>  {
        Ok(line.starts_with("ret"))
    }

    fn parser_call(opcodes: &mut Vec<OpCode>, line: &str) -> Result<bool, ParserError>  {
        Ok(line.starts_with("call"))
    }

    pub fn compile(context: &str) -> Result<ParserContext, ParserError> {
        let lines = context.lines();
        let mut opcodes = Vec::new();

        let parsers: Vec<(&str, fn(&mut Vec<OpCode>, &str) -> Result<bool, ParserError>)> = vec![
            ("COMMENT", Self::parser_comment),
            ("ADD", Self::parser_add),
            ("SUB", Self::parser_sub),
            ("DIV", Self::parser_div),
            ("MUL", Self::parser_mul),
            ("LTN", Self::parser_ltn),
            ("EQL", Self::parser_eql),
            ("AND", Self::parser_and),
            ("NOT", Self::parser_not),
            ("OR", Self::parser_or),
            ("INC", Self::parser_inc),
            ("DEC", Self::parser_dec),
            ("LOAD", Self::parser_load),
            ("PRINT", Self::parser_print),
            ("GOTO", Self::parser_goto),
            ("IF", Self::parser_if),
            ("RET", Self::parser_return),
            ("CALL", Self::parser_call),
        ];
        
        for line in lines.into_iter() {
            let line = Self::eat_whitespaces(line);
            let mut line_parsed = false;

            for (name, parser) in parsers.iter() {
                if parser(&mut opcodes, line)? {
                    println!("{} [{}]", name, line);
                    line_parsed = true;
                    break;
                }
            }

            if !line_parsed {
                println!("NOT PARSED [{}]", line);
            }
        }

        Ok(ParserContext { opcodes })
    }
}

#[cfg(test)]
mod test {
    use super::{Parser, ParserError};

    #[test]
        fn test_1() -> Result<(), ParserError> {
            let parser = Parser::compile("# Hello
# world
load r1 #100
add r1 r1 r1
sub r1 r1 r1")?;

            Ok(())
        }
}