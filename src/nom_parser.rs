use std::fmt::{Display, Formatter};

use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::character::complete::{alpha1, alphanumeric1, char, none_of, one_of};
use nom::combinator::{cut, map, opt, peek, recognize, value};
use nom::error::{context, ErrorKind};
use nom::multi::{fold, many0, many0_count, many1, separated_list0};
use nom::sequence::{pair, preceded, terminated};
use nom::Err;
use nom::{IResult, Parser, error::ParseError, sequence::delimited};
use nom::bytes::complete::tag;
use nom_language::error::VerboseError;

use crate::ast::{
    BodyAst, BodyStatementAst, ClassDefinitionAst, ClassDefinitionFieldAst, ExpressionAst, FieldAst, FileAst, FileStatementAst, FunctionArgumentAst, FunctionDefinitionAst, PrimitiveType, TypeNameAst, VariableDefinitionAst, VariableType
};
use crate::nom_tools::{Span, State, cleanup};

static I8_RANGE: std::ops::Range<i128> = (i8::MIN as i128)..(i8::MAX as i128);
static U8_RANGE: std::ops::Range<i128> = (u8::MIN as i128)..(u8::MAX as i128);

static I16_RANGE: std::ops::Range<i128> = (i16::MIN as i128)..(i16::MAX as i128);
static U16_RANGE: std::ops::Range<i128> = (u16::MIN as i128)..(u16::MAX as i128);

static I32_RANGE: std::ops::Range<i128> = (i32::MIN as i128)..(i32::MAX as i128);
static U32_RANGE: std::ops::Range<i128> = (u32::MIN as i128)..(u32::MAX as i128);

static I64_RANGE: std::ops::Range<i128> = (i64::MIN as i128)..(i64::MAX as i128);
static U64_RANGE: std::ops::Range<i128> = (u64::MIN as i128)..(u64::MAX as i128);

static FLOAT_RANGE: std::ops::Range<f64> = (f32::MIN as f64)..(f32::MAX as f64);

#[allow(warnings)]
pub fn comment<'a, E: std::fmt::Debug + ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {
    preceded(char('/'), alt((preceded(char('*'), cut(terminated(take_until("*/"), tag("*/")))),))).parse(input)
}

pub fn is_public<'a, E: std::fmt::Debug + ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, bool, E> {    
    cleanup(map(opt(tag("pub")), |item| item.is_some())).parse(input)
}

pub fn is_nullable<'a, E: std::fmt::Debug + ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, bool, E> {    
    cleanup(map(opt(char('?')), |item| item.is_some())).parse(input)
}

pub fn expected_ident<'a, E: std::fmt::Debug + ParseError<Span<'a>> + nom::error::ContextError<Span<'a>>>(message: &'static str, input: Span<'a>) -> IResult<Span<'a>, Span<'a>, E> {    
    context(message, cut(ident())).parse(input)
}

pub fn ident<'a, E: std::fmt::Debug + ParseError<Span<'a>>>() -> impl Parser<Span<'a>, Output = Span<'a>, Error = E> {    
    cleanup(recognize(pair
        (alt(
            (alpha1, tag("_"))), 
            many0_count(alt((alphanumeric1, tag("_")))))
        )
    )
}

pub fn parse(state: State<'_>) -> IResult<Span<'_>, FileAst<'_>, VerboseError<Span<'_>>> {
    let input = Span::new_extra(state.file.code(), state);
    let (remaining, statements) = many0(alt((cleanup(ClassDefinitionAst::parse), cleanup(FunctionDefinitionAst::parse_file_function)))).parse(input)?;

    if remaining.len() > 0 {
        return Err(Err::Failure(VerboseError::from_error_kind(remaining, ErrorKind::Eof)));
    }

    Ok((
        remaining,
        FileAst {
            statements,
        },
    ))
}

impl Display for FileAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for statement in self.statements.iter() {
            write!(f, "{}", statement)?;
        }
        Ok(())
    }
}

impl Display for FileStatementAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileStatementAst::ClassDefinition(class) => write!(f, "{}", class),
            FileStatementAst::FunctionDefinition(function) => write!(f, "{}", function),
        }
    }
}

impl ClassDefinitionAst<'_> {
    pub fn parse<'a, E: std::fmt::Debug + ParseError<Span<'a>> + nom::error::ContextError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, FileStatementAst<'a>, E> {
        let (input, _) = cleanup(tag("class")).parse(input)?;
        let (input, name) = expected_ident("Missing class name", input)?;
        let (input, _) = context("Missing class '{'", cut(peek(cleanup(char('{'))))).parse(input)?;
        let (input, fields) = delimited(
                char('{'),
                cleanup(many0(alt((FieldAst::parse_class_field, FunctionDefinitionAst::parse_class_function)))),
                context("Missing class '}'", cut(char('}'))),
            )
        .parse(input)?;

        Ok((
            input,
            FileStatementAst::ClassDefinition(ClassDefinitionAst {
                name,
                fields,
            }),
        ))
    }
}

impl Display for ClassDefinitionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "class {} {{", self.name.fragment())?;
        for field in self.fields.iter() {
            match field {
                ClassDefinitionFieldAst::ClassField(field) => {
                    write!(f, "{}", field)?;
                }
                ClassDefinitionFieldAst::ClassFunction(function) => {
                    write!(f, "{}", function)?;
                }
            }
        }
        write!(f, "}}")
    }
}

impl TypeNameAst<'_> {
    pub fn parse<'a, E: std::fmt::Debug + ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, TypeNameAst<'a>, E> {
        let (input, nullable) = is_nullable(input)?;
        let (input, names) = map(separated_list0(char('.'), ident()), |items| items).parse(input)?;
        Ok((
            input,
            TypeNameAst {
                nullable,
                names,
            },
        ))
    }
}

impl Display for TypeNameAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.nullable {
            write!(f, "?")?;
        }

        for (i, name) in self.names.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            write!(f, "{}", name.fragment())?;
        }
        Ok(())
    }
}

impl FunctionArgumentAst<'_> {
    pub fn parse<'a, E: std::fmt::Debug + ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, FunctionArgumentAst<'a>, E> {
        let (input, (name, field_type)) = (cleanup(terminated(ident(), cleanup(char(':')))), cleanup(TypeNameAst::parse)).parse(input)?;
        Ok((
            input,
            FunctionArgumentAst {
                name,
                field_type,
            },
        ))
    }
}

impl Display for FunctionArgumentAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name.fragment(), self.field_type)
    }
}

impl Display for BodyStatementAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BodyStatementAst::Variable(var) => write!(f, "{}", var),
        }
    }
}

impl BodyAst<'_> {
    pub fn parse<'a, E: std::fmt::Debug + ParseError<Span<'a>> + nom::error::ContextError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, BodyAst<'a>, E> {
        let (input, _) = context("Missing '{'", cut(cleanup(char('{')))).parse(input)?;
        let (input, statements) = many0(VariableDefinitionAst::parse_body_statement::<E>).parse(input)?;
        let (input, _) = context("Missing '}'", cut(cleanup(char('}')))).parse(input)?;

        Ok((
            input,
            BodyAst {
                statements,
            },
        ))
    }
}

impl Display for BodyAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (index, statement) in self.statements.iter().enumerate() {
            write!(f, "{}", statement)?;
            if index < self.statements.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")
    }
}

impl VariableDefinitionAst<'_> {
    pub fn parse_body_statement<'a, E: std::fmt::Debug + ParseError<Span<'a>> + nom::error::ContextError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, BodyStatementAst<'a>, E> {
        let (input, variable) = Self::parse(input)?;
        Ok((input, BodyStatementAst::Variable(variable)))
    }

    pub fn parse<'a, E: std::fmt::Debug + ParseError<Span<'a>> + nom::error::ContextError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, VariableDefinitionAst<'a>, E> {
        let (input, variable_type) = cleanup(alt((map(tag("var"), |_| VariableType::Var), map(tag("const"), |_| VariableType::Const)))).parse(input)?;
        let (input, name) = expected_ident("Missing variable name", input)?;
        let (input, _) = context("Missing '='", cleanup(char('='))).parse(input)?;
        let (input, expression) = context("Invalid expression", cut(ExpressionAst::parse)).parse(input)?;
        let (input, _) = context("Missing ';'", cleanup(char(';'))).parse(input)?;

        Ok((
            input,
            VariableDefinitionAst {
                variable_type,
                name,
                expression,
            },
        ))
    }
}

impl Display for VariableDefinitionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} = {};", self.variable_type, self.name.fragment(), self.expression)
    }
}

impl Display for VariableType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableType::Var => write!(f, "var"),
            VariableType::Const => write!(f, "const"),
        }
    }
}

impl ExpressionAst {
    pub fn parse<'a, E: std::fmt::Debug + ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, ExpressionAst, E> {
        let (input, expression) = alt((cleanup(PrimitiveType::parse),)).parse(input)?;

        Ok((input, ExpressionAst::Primitive(expression)))
    }
}

impl Display for ExpressionAst {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionAst::Primitive(primitive) => write!(f, "{}", primitive),
        }
    }
}

fn character<'a, E: std::fmt::Debug + ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, char, E> {
    let (input, c) = none_of("\"")(input)?;
    if c == '\\' {
        alt((
            value('\x08', tag("\\b")),
            value('\x0C', tag("\\f")),
            value('\n', tag("\\n")),
            value('\r', tag("\\r")),
            value('\t', tag("\\t")),
            value('\\', char('\\')),
            value('"', char('"')),
            value('/', char('/')),
        ))
        .parse(input)
    } else {
        Ok((input, c))
    }
}

pub fn string<'a, E: std::fmt::Debug + ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, PrimitiveType, E> {
    let (input, string) = delimited(
        char('"'),
        fold(0.., character, String::new, |mut string, c| {
            string.push(c);
            string
        }),
        char('"'),
    )
    .parse(input)?;

    Ok((input, PrimitiveType::String(string)))
}

pub fn number<'a, E: std::fmt::Debug + ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, PrimitiveType, E> {
    let (input, (representing, (number, floating))) = (
        opt(one_of("+-")), 
        (recognize::<Span<'a>, E, _>(many1(terminated(one_of("0123456789"), many0(char('_'))))), 
            opt(preceded(
                char('.'),
                (
                    recognize::<Span<'a>, E, _>(many1(terminated(one_of("0123456789"), many0(char('_'))))),
                    opt(preceded(
                        one_of("Ee"),
                        (
                            opt(alt((
                                value(true, char('-')),
                                value(false, char('+'))
                            ))),
                            recognize::<Span<'a>, E, _>(many1(terminated(one_of("0123456789"), many0(char('_')))))
                        ),
                    ))
                )
            ))
        )
    ).parse(input)?;

    let number = number.replace("_", "");

    let number = if let Some((floating, e_info)) = floating {        
        let dot_place = floating.len();
        let floating = floating.replace("_", "");

        let number = if let Some((is_minus, exponent)) = e_info {
            let mut exponent = exponent.replace("_", "").parse::<i32>().unwrap_or(0);
            if let Some(true) = is_minus {
                exponent = -exponent
            };

            let number: f64 = minimal_lexical::parse_float(number.as_bytes().iter(), floating.as_bytes().iter(), exponent);
            number
        } else {
            minimal_lexical::parse_float(number.as_bytes().iter(), floating.as_bytes().iter(), 0)
        };

        let number = match representing {
            Some('-') => -number,
            _ => number,
        };

        if FLOAT_RANGE.contains(&number) {
            PrimitiveType::Float(number, dot_place as u8)
        } else {
            PrimitiveType::Double(number, dot_place as u8)
        }
    } else {
        let number = match number.replace("_", "").parse::<i128>() {
            Ok(number) => number,
            Err(_) => {
                return Err(Err::Error(E::from_error_kind(input, ErrorKind::Digit)));
            }
        };

        let number = match representing {
            Some('-') => -number,
            _ => number,
        };

        if I8_RANGE.contains(&number) {
            PrimitiveType::I8(number as i8)
        } else if U8_RANGE.contains(&number) {
            PrimitiveType::U8(number as u8)
        } else if I16_RANGE.contains(&number) {
            PrimitiveType::I16(number as i16)
        } else if U16_RANGE.contains(&number) {
            PrimitiveType::U16(number as u16)
        } else if I32_RANGE.contains(&number) {
            PrimitiveType::I32(number as i32)
        } else if U32_RANGE.contains(&number) {
            PrimitiveType::U32(number as u32)
        } else if I64_RANGE.contains(&number) {
            PrimitiveType::I64(number as i64)
        } else if U64_RANGE.contains(&number) {
            PrimitiveType::U64(number as u64)
        } else {
            return Err(Err::Error(E::from_error_kind(input, ErrorKind::Digit)));
        }
    };

    Ok((input, number))
}


impl PrimitiveType {
    pub fn parse<'a, E: std::fmt::Debug + ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, PrimitiveType, E> {
        let (input, value) = cleanup(alt((
            number,
            string,
            value(PrimitiveType::Bool(true), tag("true")),
            value(PrimitiveType::Bool(false), tag("false")),
        ))).parse(input)?;

        Ok((input, value))
    }
}

impl Display for PrimitiveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveType::String(value) => write!(f, "{}", value),
            PrimitiveType::Bool(value) => write!(f, "{}", value),
            PrimitiveType::I8(value) => write!(f, "{}", value),
            PrimitiveType::U8(value) => write!(f, "{}", value),
            PrimitiveType::I16(value) => write!(f, "{}", value),
            PrimitiveType::U16(value) => write!(f, "{}", value),
            PrimitiveType::I32(value) => write!(f, "{}", value),
            PrimitiveType::U32(value) => write!(f, "{}", value),
            PrimitiveType::I64(value) => write!(f, "{}", value),
            PrimitiveType::U64(value) => write!(f, "{}", value),
            PrimitiveType::Float(value, len) => write!(f, "{:.*}", *len as usize, value),
            PrimitiveType::Double(value, len) => write!(f, "{:.*}", *len as usize, value),
        }
    }
}

impl FunctionDefinitionAst<'_> {
    pub fn parse_file_function<'a, E: std::fmt::Debug + ParseError<Span<'a>> + nom::error::ContextError<Span<'a>>>(
        input: Span<'a>,
    ) -> IResult<Span<'a>, FileStatementAst<'a>, E> {
        let (input, function) = Self::parse(input)?;

        Ok((input, FileStatementAst::FunctionDefinition(function)))
    }

    pub fn parse_class_function<'a, E: std::fmt::Debug + ParseError<Span<'a>> + nom::error::ContextError<Span<'a>>>(
        input: Span<'a>,
    ) -> IResult<Span<'a>, ClassDefinitionFieldAst<'a>, E> {
        let (input, function) = Self::parse(input)?;
        Ok((input, ClassDefinitionFieldAst::ClassFunction(function)))
    }

    pub fn parse<'a, E: std::fmt::Debug + ParseError<Span<'a>> + nom::error::ContextError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, FunctionDefinitionAst<'a>, E> {
        let (input, is_public) = is_public(input)?;
        let (input, _) = cleanup(tag("func")).parse(input)?;
        let (input, name) = expected_ident("Missing function name", input)?;
        let (input, _) = context("Missing '('", cut(peek(cleanup(char('('))))).parse(input)?;
        let (input, arguments) = map(delimited(char('('), cleanup(separated_list0(char(','), FunctionArgumentAst::parse::<E>)), context("Missing ')'", cut(char(')')))), |items| items) .parse(input)?;

        let (input, _) = context("Missing ':'", cleanup(opt(char(':')))).parse(input)?;
        let (input, return_type) = context("Missing function return type", cut(cleanup(cleanup(TypeNameAst::parse)))).parse(input)?;

        let (input, body) = BodyAst::parse::<E>.parse(input)?;

        Ok((
            input,
            FunctionDefinitionAst {
                is_public,
                name,
                arguments,
                body,
                return_type,
            },
        ))
    }
}

impl Display for FunctionDefinitionAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}func {}(", if self.is_public { "pub " } else { "" }, self.name.fragment())?;
        for (index, arg) in self.arguments.iter().enumerate() {
            write!(f, "{}", arg)?;
            if index < self.arguments.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "): {} {}", self.return_type, self.body)
    }
}

impl Display for ClassDefinitionFieldAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassDefinitionFieldAst::ClassField(field) => write!(f, "{}", field),
            ClassDefinitionFieldAst::ClassFunction(function) => write!(f, "{}", function),
        }
    }
}

impl FieldAst<'_> {
    pub fn parse_field<'a, E: std::fmt::Debug + ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, FieldAst<'a>, E> {
        let (input, (is_public, name, field_type, _)) =
            (is_public, cleanup(terminated(ident(), cleanup(char(':')))), cleanup(TypeNameAst::parse), cleanup(char(';'))).parse(input)?;

        Ok((
            input,
            FieldAst {
                is_public,
                name,
                field_type,
            },
        ))
    }

    pub fn parse_class_field<'a, E: std::fmt::Debug + ParseError<Span<'a>>>(
        input: Span<'a>,
    ) -> IResult<Span<'a>, ClassDefinitionFieldAst<'a>, E> {
        let (input, field) = Self::parse_field(input)?;
        Ok((input, ClassDefinitionFieldAst::ClassField(field)))
    }
}

impl Display for FieldAst<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}: {};",
            match self.is_public {
                true => "pub ",
                false => "",
            },
            self.name.fragment(),
            self.field_type
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{rc::Rc, vec};
    use pretty_assertions::assert_eq;

    use nom_language::error::VerboseError;
    use rstest::rstest;

    use crate::{
        ast::PrimitiveType,
        file::SourceFile,
        nom_parser::{number, State},
    };

    use super::{Span, TypeNameAst};

    #[rstest]
    #[case("string", false, vec!["string"])]
    #[case(" string ", false, vec!["string"])]
    #[case("string.base", false, vec!["string", "base"])]
    #[case("string.base . test", false, vec!["string", "base", "test"])]
    #[case(" string   .        base        . test", false, vec!["string", "base", "test"])]
    #[case(" ? string   .        base        . test", true, vec!["string", "base", "test"])]
    #[case("?string", true, vec!["string"])]
    fn parse_type_name_test<'a>(#[case] code: &'a str, #[case] nullable: bool, #[case] expected: Vec<&str>) {
        let source_file = Rc::new(SourceFile::new("<memory>".into(), code));

        let state = State {
            file: source_file.clone(),
        };

        let input = Span::new_extra(code, state);
        let result = TypeNameAst::parse::<VerboseError<Span>>(input);
        assert!(result.is_ok(), "Failed to parse type name: {:?}", result.err());
        let (_, parsed) = result.unwrap();

        assert_eq!(parsed.nullable, nullable, "nullable info does not match expected");

        let parsed: Vec<_> = parsed.names.into_iter().map(|s| s.fragment().to_string()).collect();
        assert_eq!(parsed, expected, "Parsed type name does not match expected");
    }
 
    #[rstest]
    #[case("1.2", 1.2, 1)]
    #[case("2.2", 2.2, 1)]
    #[case("2.20000000000000", 2.2, 14)]
    #[case("1.23", 1.23, 2)]
    #[case("1024.0", 1024.0, 1)]
    #[case("-1024.0", -1024.0, 1)]
    #[case("1.0e-7", 1.0e-7, 1)]
    #[case("123456789.0e+7", 1234567890000000.0, 1)]
    fn float_test<'a>(#[case] code: &'a str, #[case] expected: f64, #[case] dot_place: u8) {
        let source_file = Rc::new(SourceFile::new("<memory>".into(), code));

        let state = State {
            file: source_file.clone(),
        };

        let input = Span::new_extra(code, state);
        let (_, number) = number::<VerboseError<Span>>(input).unwrap();

        assert_eq!(number, PrimitiveType::Float(expected, dot_place), "Parsed type name does not match expected");
    }

    #[rstest]
    #[case("1.7976931348623157E+300", 1797693134862315647938267463293564874600617718166104931943772918675666340832537361829116717802808644459281636809871223917508254623303542508952824391223228755068260245991425339269180741930617451225745000201898803634683406373476746438518757597828943183163861984879702567874510145974570799930947550576640.0000000000000000, 16)]
    fn double_test<'a>(#[case] code: &'a str, #[case] expected: f64, #[case] dot_place: u8) {
        let source_file = Rc::new(SourceFile::new("<memory>".into(), code));

        let state = State {
            file: source_file.clone(),
        };

        let input = Span::new_extra(code, state);
        let (_, number) = number::<VerboseError<Span>>(input).unwrap();

        assert_eq!(number, PrimitiveType::Double(expected, dot_place), "Parsed type name does not match expected");
    }
}
