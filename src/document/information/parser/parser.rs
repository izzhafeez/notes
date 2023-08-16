use crate::document::information::parser::benefit_parser::benefit_parser;

pub enum Parser {
    ANCESTOR,
    BENEFIT,
    CHILD,
    DEFINITION,
    EXAMPLE,
    EXPLANATION,
    NEGATIVE,
    PROPERTY,
    PURPOSE,
    RELATED,
}

pub fn get(c: char) -> Result<Parser, ()> {
    match c {
        'A' => Ok(Parser::ANCESTOR),
        'B' => Ok(Parser::BENEFIT),
        'C' => Ok(Parser::CHILD),
        'D' => Ok(Parser::DEFINITION),
        'E' => Ok(Parser::EXPLANATION),
        'N' => Ok(Parser::NEGATIVE),
        'R' => Ok(Parser::RELATED),
        'T' => Ok(Parser::PROPERTY),
        'U' => Ok(Parser::PURPOSE),
        'X' => Ok(Parser::EXAMPLE),
        _ => Err(())
    }
}

pub fn parse(c: Parser, s: &str) -> Result<String, ()> {
    Ok(benefit_parser(s))
}