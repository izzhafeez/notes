use crate::document::information::parser::ancestor_parser::parse_ancestor;
use crate::document::information::parser::benefit_parser::parse_benefit;
use crate::document::information::parser::child_parser::parse_child;
use crate::document::information::parser::definition_parser::parse_definition;
use crate::document::information::parser::example_parser::parse_example;
use crate::document::information::parser::explanation_parser::parse_explanation;
use crate::document::information::parser::negative_parser::parse_negative;
use crate::document::information::parser::property_parser::parse_property;
use crate::document::information::parser::purpose_parser::parse_purpose;
use crate::document::information::parser::related_parser::parse_related;

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

pub fn parse(p: Parser, raw_string: &str) -> Result<String, ()> {
    let s = &raw_string[3..];
    match p {
        Parser::ANCESTOR => Ok(parse_ancestor(s)),
        Parser::BENEFIT => Ok(parse_benefit(s)),
        Parser::CHILD => Ok(parse_child(s)),
        Parser::DEFINITION => Ok(parse_definition(s)),
        Parser::EXAMPLE => Ok(parse_example(s)),
        Parser::EXPLANATION => Ok(parse_explanation(s)),
        Parser::NEGATIVE => Ok(parse_negative(s)),
        Parser::PROPERTY => Ok(parse_property(s)),
        Parser::PURPOSE => Ok(parse_purpose(s)),
        Parser::RELATED => Ok(parse_related(s)),
    }
}