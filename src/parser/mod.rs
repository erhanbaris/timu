
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, alphanumeric1, char};
use nom::combinator::{cut, map, opt, recognize};
use nom::sequence::{pair, preceded, terminated};
use nom::Err;
use nom::branch::alt;
use nom::error::context;
use nom::multi::{many0, many0_count};
use nom::{IResult, Parser};
use nom_language::error::{VerboseError, VerboseErrorKind};

use crate::ast::{ClassDefinitionAst, ExtendDefinitionAst, FileAst, FunctionDefinitionAst, InterfaceDefinitionAst, UseAst};
use crate::nom_tools::{Span, State, cleanup};

mod body;
mod class;
mod expression;
mod extend;
mod field;
mod file;
mod function_definition;
mod function_call;
mod if_condition;
mod interface;
mod module_use;
mod primitive;
mod ref_info;
mod type_info;
mod variable;

pub type TimuParserError<'a> = VerboseError<Span<'a>>;

pub fn parse(state: State<'_>) -> IResult<Span<'_>, FileAst<'_>, TimuParserError> {
    let file = state.file.clone();

    let input = Span::new_extra(state.file.code(), state);
    let (remaining, statements) =
        many0(alt((
            cleanup(UseAst::parse_for_file),
            cleanup(ClassDefinitionAst::parse),
            cleanup(FunctionDefinitionAst::parse_for_file),
            cleanup(InterfaceDefinitionAst::parse),
            cleanup(ExtendDefinitionAst::parse),
        )))
        .parse(input)?;

    if remaining.len() > 0 {
        let error = VerboseError {
            errors: vec![(remaining, VerboseErrorKind::Context("Unknown syntax"))],
        };
        return Err(Err::Failure(error));
    }

    Ok((
        remaining,
        FileAst {
            file,
            statements,
        },
    ))
}


#[allow(warnings)]
pub fn comment<'a>(input: Span<'a>) -> IResult<Span<'a>, Span<'a>, TimuParserError<'a>> {
    preceded(char('/'), alt((preceded(char('*'), cut(terminated(take_until("*/"), tag("*/")))),))).parse(input)
}

pub fn is_public(input: Span<'_>) -> IResult<Span<'_>, Option<Span<'_>>, TimuParserError<'_>> {
    cleanup(opt(tag("pub"))).parse(input)
}

pub fn is_nullable(input: Span<'_>) -> IResult<Span<'_>, bool, TimuParserError<'_>> {
    cleanup(map(opt(char('?')), |item| item.is_some())).parse(input)
}

pub fn expected_ident<'a>(message: &'static str, input: Span<'a>) -> IResult<Span<'a>, Span<'a>, TimuParserError<'a>> {
    context(message, cut(ident())).parse(input)
}

pub fn ident<'a>() -> impl Parser<Span<'a>, Output = Span<'a>, Error = TimuParserError<'a>> {
    cleanup(recognize(pair(alt((alpha1, tag("_"))), many0_count(alt((alphanumeric1, tag("_")))))))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::{rc::Rc, vec};

    use rstest::rstest;

    use crate::{
        ast::{PrimitiveType, TypeNameAst},
        file::SourceFile, nom_tools::State, parser::primitive::{number, string},
    };

    use super::Span;

    #[rstest]
    #[case(r#""hello""#, PrimitiveType::String("hello".to_string()))]
    #[case(r#""hello\nworld""#, PrimitiveType::String("hello\nworld".to_string()))]
    #[case(r#""hello\tworld""#, PrimitiveType::String("hello\tworld".to_string()))]
    #[case(r#""hello\\world""#, PrimitiveType::String("hello\\world".to_string()))]
    #[case(r#""hello\"world""#, PrimitiveType::String("hello\"world".to_string()))]
    #[case(r#""hello/world""#, PrimitiveType::String("hello/world".to_string()))]
    fn string_test<'a>(#[case] code: &'a str, #[case] expected: PrimitiveType) {
        let source_file = Rc::new(SourceFile::new("<memory>", "<memory>".into(), code));

        let state = State {
            file: source_file.clone(),
        };

        let input = Span::new_extra(code, state);
        let (_, string) = string(input).unwrap();

        assert_eq!(string, expected, "Parsed string does not match expected");
    }

    #[rstest]
    #[case("true", PrimitiveType::Bool(true))]
    #[case("false", PrimitiveType::Bool(false))]
    fn boolean_test<'a>(#[case] code: &'a str, #[case] expected: PrimitiveType) {
        let source_file = Rc::new(SourceFile::new("<memory>", "<memory>".into(), code));

        let state = State {
            file: source_file.clone(),
        };

        let input = Span::new_extra(code, state);
        let (_, boolean) = PrimitiveType::parse(input).unwrap();

        assert_eq!(boolean, expected, "Parsed boolean does not match expected");
    }

    #[rstest]
    #[case("123", PrimitiveType::I8(123))]
    #[case("-123", PrimitiveType::I8(-123))]
    #[case("255", PrimitiveType::U8(255))]
    #[case("32767", PrimitiveType::I16(32767))]
    #[case("65535", PrimitiveType::U16(65535))]
    #[case("2147483647", PrimitiveType::I32(2147483647))]
    #[case("4294967295", PrimitiveType::U32(4294967295))]
    #[case("9223372036854775807", PrimitiveType::I64(9223372036854775807))]
    #[case("18446744073709551615", PrimitiveType::U64(18446744073709551615))]
    fn integer_test<'a>(#[case] code: &'a str, #[case] expected: PrimitiveType) {
        let source_file = Rc::new(SourceFile::new("<memory>", "<memory>".into(), code));

        let state = State {
            file: source_file.clone(),
        };

        let input = Span::new_extra(code, state);
        let (_, number) = number(input).unwrap();

        assert_eq!(number, expected, "Parsed integer does not match expected");
    }

    #[rstest]
    #[case("string", false, vec!["string"])]
    #[case(" string ", false, vec!["string"])]
    #[case("string.base", false, vec!["string", "base"])]
    #[case("string.base . test", false, vec!["string", "base", "test"])]
    #[case(" string   .        base        . test", false, vec!["string", "base", "test"])]
    #[case(" ? string   .        base        . test", true, vec!["string", "base", "test"])]
    #[case("?string", true, vec!["string"])]
    fn parse_type_name_test<'a>(#[case] code: &'a str, #[case] nullable: bool, #[case] expected: Vec<&str>) {
        let source_file = Rc::new(SourceFile::new("<memory>", "<memory>".into(), code));

        let state = State {
            file: source_file.clone(),
        };

        let input = Span::new_extra(code, state);
        let result = TypeNameAst::parse(input);
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
        let source_file = Rc::new(SourceFile::new("<memory>", "<memory>".into(), code));

        let state = State {
            file: source_file.clone(),
        };

        let input = Span::new_extra(code, state);
        let (_, number) = number(input).unwrap();

        assert_eq!(number, PrimitiveType::Float(expected, dot_place), "Parsed type name does not match expected");
    }

    #[rstest]
    #[case("1.7976931348623157E+300", 1797693134862315647938267463293564874600617718166104931943772918675666340832537361829116717802808644459281636809871223917508254623303542508952824391223228755068260245991425339269180741930617451225745000201898803634683406373476746438518757597828943183163861984879702567874510145974570799930947550576640.0000000000000000, 16)]
    fn double_test<'a>(#[case] code: &'a str, #[case] expected: f64, #[case] dot_place: u8) {
        let source_file = Rc::new(SourceFile::new("<memory>", "<memory>".into(), code));

        let state = State {
            file: source_file.clone(),
        };

        let input = Span::new_extra(code, state);
        let (_, number) = number(input).unwrap();

        assert_eq!(number, PrimitiveType::Double(expected, dot_place), "Parsed type name does not match expected");
    }
}
