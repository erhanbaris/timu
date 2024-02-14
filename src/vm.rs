
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
pub enum ParserError {

}

#[derive(Debug)]
pub struct Context;

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

    fn eat_text(content: &str) -> &str {
        for (index, ch) in content.chars().enumerate() {
            if ch == ' ' {
                return &content[index..]
            }
        }

        content
    }

    fn parser_comment(line: &str) -> bool {
        line.starts_with('#')
    }

    fn parser_add(line: &str) -> bool {
        line.starts_with("add")
    }

    fn parser_sub(line: &str) -> bool {
        line.starts_with("sub")
    }

    fn parser_div(line: &str) -> bool {
        line.starts_with("div")
    }

    fn parser_mul(line: &str) -> bool {
        line.starts_with("mul")
    }

    fn parser_ltn(line: &str) -> bool {
        line.starts_with("ltn")
    }

    fn parser_eql(line: &str) -> bool {
        line.starts_with("eql")
    }

    fn parser_and(line: &str) -> bool {
        line.starts_with("and")
    }

    fn parser_not(line: &str) -> bool {
        line.starts_with("not")
    }

    fn parser_or(line: &str) -> bool {
        line.starts_with("or")
    }

    fn parser_inc(line: &str) -> bool {
        line.starts_with("inc")
    }

    fn parser_dec(line: &str) -> bool {
        line.starts_with("dec")
    }

    fn parser_print(line: &str) -> bool {
        line.starts_with("print")
    }

    fn parser_load(line: &str) -> bool {
        if !line.starts_with("load") {
            return false;
        }

        Self::eat_whitespaces(Self::eat_until_space(line));

        true
    }

    fn parser_goto(line: &str) -> bool {
        line.starts_with("goto")
    }

    fn parser_if(line: &str) -> bool {
        line.starts_with("if")
    }

    fn parser_return(line: &str) -> bool {
        line.starts_with("ret")
    }

    fn parser_call(line: &str) -> bool {
        line.starts_with("call")
    }

    pub fn compile(context: &str) -> Result<Context, ParserError> {
        let lines = context.lines();
        let parsers: Vec<(&str, fn(&str) -> bool)> = vec![
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
                if parser(line) {
                    println!("{} [{}]", name, line);
                    line_parsed = true;
                    break;
                }
            }

            if !line_parsed {
                println!("NOT PARSED [{}]", line);
            }
        }

        Ok(Context)
    }
}

#[cfg(test)]
mod test {
    use super::{Parser, ParserError};

    #[test]
        fn test_1() -> Result<(), ParserError> {
            let parser = Parser::compile("# Hello
# world
add
 sub")?;

            Ok(())
        }
}