use std::rc::Rc;

use nom::error::Error;
use nom_locate::LocatedSpan;
use crate::common::parser_types::ParsedType;

use super::parser_impl::parse_all;

type Span<'a> = LocatedSpan<&'a str>;

fn read_file(full_path: &str) -> Result<String, std::io::Error> {
    use std::{ fs::File, io::prelude::* };

    let mut file = File::open(full_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub struct Parser {
    pub file_name: Option<String>,
    all_types: Vec<Rc<ParsedType>>,
}

impl Parser {
    pub fn from_lines(lines: String) -> Result<Parser, String> {
        let span = Span::new(&lines);

        match parse_all::<Span, Error<Span>>(span) {
            Ok((_, types)) =>
                Ok(Parser {
                    file_name: None,
                    all_types: types
                        .iter()
                        .map(|t| Rc::new(t.clone()))
                        .collect(),
                }),
            Err(err) => {
                let err = match err {
                    nom::Err::Error(e) | nom::Err::Failure(e) => e,
                    nom::Err::Incomplete(_) => {
                        return Err("Incomplete input".to_string());
                    }
                };

                println!("{:#?}", err);

                let location = err.input;
                Err(
                    format!(
                        "Parse error at line {}, column {}: {}, {}",
                        location.location_line(),
                        location.get_column(),
                        err.input,
                        match err.code {
                            nom::error::ErrorKind::Tag => "Expected specific text",
                            nom::error::ErrorKind::Alt =>
                                "None of the expected alternatives matched",
                            nom::error::ErrorKind::Eof => "Unexpected end of input",
                            _ => "Invalid syntax",
                        }
                    )
                )
            }
        }
    }

    pub fn from_file(base: &str, filename: &str, ext: &str) -> Result<Parser, String> {
        let full_path = format!("{}{}.{}", base, filename, ext);

        let contents = read_file(&full_path).map_err(|open_err|
            format!("Error opening file: {:?}", open_err)
        )?;

        Parser::from_lines(contents).map(|p| Parser {
            file_name: Some(filename.to_string()),
            all_types: p.all_types,
        })
    }

    pub fn get_types(&self) -> &Vec<Rc<ParsedType>> {
        &self.all_types
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_definition() {
        let book_def =
            r#"@enum BookUpdateType Update Snapshot
# Republish is only applicable to books with limited depth where a delete occured and level shifts back into the depth
# See Delete Messages section at https://support.kraken.com/hc/en-us/articles/360027821131-How-to-maintain-a-valid-order-book
@enum LevelUpdateType New Change Delete Republish

struct Level
    price::Price
    size::Size
    recent_size::Float64
    last_update::Timestamp
end

const Levels = Vector{Level}

# TODO: Books....

mutable struct NewOrder
    timestamp::Timestamp
    timestamp_exch::Optional{Timestamp}
    id::InstrumentId
    price::Price
    size::Optional{Size}
    side::BidOrAsk
end

struct LevelUpdate
    type::LevelUpdateType
    level::Level
end

struct BookUpdate
    timestamp::Timestamp
    timestamp_exch::Optional{Timestamp}
    id::InstrumentId
    bids::Vector{LevelUpdate}
    asks::Vector{LevelUpdate}
    upd_type::BookUpdateType
end

struct FullBookUpdate
    timestamp::Timestamp
    timestamp_exch::Optional{Timestamp}
    id::InstrumentId
    bids::Levels
    asks::Levels
end
        "#;

        let parser = Parser::from_lines(book_def.to_string()).unwrap();
        let types = parser.get_types();
        assert_eq!(types.len(), 8);
    }

    #[test]
    fn test_error_definition() {
        let book_def = r#"@enum BookUpdateType Update Snapshot::xxx"#;

        let parser = Parser::from_lines(book_def.to_string());
        assert!(parser.is_err());
        assert_eq!(
            "Parse error at line 1, column 37: ::xxx, Unexpected end of input",
            parser.map(|_| "").unwrap_err()
        );
    }
}
