use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{ tag, take_while1 },
    character::complete::{ multispace0, satisfy },
    combinator::{ map_res, opt, recognize },
    error::{ FromExternalError, ParseError },
    sequence::{ delimited, preceded },
    AsChar,
    Compare,
    Input,
    Offset,
    Parser,
};

/// Recognizes a keyword in the input stream.
/// The keyword is defined as a sequence of alphanumeric characters, underscores, or hyphens.
/// It is optionally preceded and followed by whitespace.
pub fn keyword<T, I, E: ParseError<I>>(word: T) -> impl Parser<I, Output = I, Error = E>
    where I: Input + Compare<T>, T: Input + Clone, <I as Input>::Item: AsChar
{
    delimited(multispace0, tag(word.clone()), multispace0)
}

pub fn recognize_with_valid_chars<I: Clone + Offset + Input, E: ParseError<I>, F>(
    valid_char: F
) -> impl Parser<I, Output = I, Error = E>
    where F: Fn(<I as Input>::Item) -> bool
{
    recognize(take_while1(valid_char))
}

/// Recognizes a valid identifier.
/// A valid identifier starts with an alphabetic character or underscore,
/// and is followed by 0 or more alphanumeric characters, underscores, or hyphens.
pub fn identifier<I, E: ParseError<I>>() -> impl Parser<I, Output = I, Error = E>
    where I: Input + Clone + Offset, <I as Input>::Item: AsChar
{
    preceded(
        multispace0,
        recognize(
            preceded(
                // First character must be alpha or underscore
                satisfy(|c: char| {
                    let c = c.as_char();
                    c.is_alphabetic() || c == '_'
                }), // Rest can include numbers and hyphens
                opt(
                    take_while1(|c: I::Item| {
                        let c = c.as_char();
                        c.is_alphanumeric() || c == '_' || c == '-'
                    })
                )
            )
        )
    )
}

/// Recognizes a valid integer value.
/// A valid numeric value starts with an optional sign ('+' or '-'), followed by one or more digits.
pub fn integer<'a, I, E: ParseError<I>>() -> impl Parser<I, Output = I, Error = E>
    where I: Compare<&'a str> + Input + Clone + Offset, <I as Input>::Item: AsChar
{
    preceded(
        multispace0,
        recognize(
            preceded(
                // Optional single + or - sign
                opt(alt((tag("+"), tag("-")))),
                // One or more digits
                take_while1(|c: I::Item| {
                    let c = c.as_char();
                    c.is_dec_digit()
                })
            )
        )
    )
}

pub fn as_integer<'a, I, T: FromStr, E: ParseError<I>>() -> impl Parser<I, Output = T, Error = E>
    where
        I: Compare<&'a str> + Input + Clone + Offset + AsRef<str>,
        E: FromExternalError<I, <T as FromStr>::Err>,
        <I as Input>::Item: AsChar
{
    map_res(integer(), |v: I| v.as_ref().parse::<T>())
}

#[cfg(test)]
mod tests {
    use nom::{ error::Error, IResult };

    use super::*;

    #[test]
    fn test_valid_keyword() {
        let to_parse = "const x = 10";
        let result = keyword::<&str, &str, Error<&str>>("const").parse(to_parse);
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "const");
        assert_eq!(remaining, "x = 10");
    }

    #[test]
    fn test_invalid_keyword() {
        let to_parse = "x = 10";
        let result = keyword::<&str, &str, Error<&str>>("const").parse(to_parse);
        assert!(result.is_err());
    }

    #[test]
    fn test_recognize_valid_chars() {
        let to_parse = "0123456789-+";
        let result: IResult<&str, &str> = recognize_with_valid_chars(|c: char|
            c.is_digit(10)
        ).parse(to_parse);
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "0123456789");
        assert_eq!(remaining, "-+");
    }

    #[test]
    fn test_recognize_invalid_chars() {
        let to_parse = "abcdefg";
        let result: IResult<&str, &str> = recognize_with_valid_chars(|c: char|
            c.is_digit(10)
        ).parse(to_parse);
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_identifier() {
        let to_parse = "test_identifier-123";
        let result = identifier::<&str, Error<&str>>().parse(to_parse);
        assert!(result.is_ok(), "{}", result.unwrap_err());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "test_identifier-123");
        assert_eq!(remaining, "");

        let to_parse = "Vector{Int64}";
        let result = identifier::<&str, Error<&str>>().parse(to_parse);

        assert!(result.is_ok(), "{}", result.unwrap_err());

        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "Vector");
        assert_eq!(remaining, "{Int64}");
    }

    #[test]
    fn test_invalid_identifier() {
        let to_parse = "123test_identifier";
        let result = identifier::<&str, Error<&str>>().parse(to_parse);
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_numeric() {
        let test_cases = vec![
            "123",
            "+123",
            "-123",
            "123abc" // should parse 123 and leave abc
        ];

        for case in test_cases {
            let result = integer::<&str, Error<&str>>().parse(case);
            assert!(result.is_ok(), "Expected ok {}", case);
            let (_, matched) = result.unwrap();
            assert!(matched.chars().all(|c| (c.is_ascii_digit() || c == '+' || c == '-')));
        }
    }

    #[test]
    fn test_invalid_numeric() {
        let test_cases = vec!["abc", "+-123", "--123", "abc123"];

        for case in test_cases {
            let result = integer::<&str, Error<&str>>().parse(case);
            assert!(result.is_err(), "Expected error for {}", case);
        }
    }
}
