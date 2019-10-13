use super::parser_state::ParserState;
use super::parser_state_result::ParserStateResult;
use super::string_parser::StringParser;
use super::sequence_of_parser::SequenceOfParser;
use super::letters_parser::LettersParser;
use super::digits_parser::DigitsParser;
use super::choice_parser::ChoiceParser;
use super::many_parser::ManyParser;

pub struct Parser {
    parser_class: Box<dyn ParserClass>,
    map_lambda: Option<Box<dyn Fn(ParserStateResult) -> ParserStateResult>>,
    error_map_lambda: Option<Box<dyn Fn(ParserStateResult, usize) -> String>>,
}

impl Parser {
    pub fn new_string_parser(string: String) -> Parser {
        Parser {
            parser_class: Box::new(StringParser::new(string)),
            ..Parser::get_default_parser_state()
        }
        
    }

    pub fn new_sequence_of_parser(parsers: Vec<Parser>) -> Parser {
        Parser {
            parser_class: Box::new(SequenceOfParser::new(parsers)),
            ..Parser::get_default_parser_state()
        }
    }

    pub fn new_letters_parser() -> Parser {
        Parser {
            parser_class: Box::new(LettersParser::new()),
            ..Parser::get_default_parser_state()
        }
    }

    pub fn new_digits_parser() -> Parser {
        Parser {
            parser_class: Box::new(DigitsParser::new()),
            ..Parser::get_default_parser_state()
        }
    }

    pub fn new_choice_parser(parsers: Vec<Parser>) -> Parser {
        Parser {
            parser_class: Box::new(ChoiceParser::new(parsers)),
            ..Parser::get_default_parser_state()
        }
    }

    pub fn new_many_parser(parser: Parser, require_one: bool) -> Parser {
        Parser {
            parser_class: Box::new(ManyParser::new(parser, require_one)),
            ..Parser::get_default_parser_state()
        }
    }

    pub fn run(&self, target_string: String) -> ParserState {
        let initial_state = ParserState::get_initial_state(target_string);
        return self.parse(initial_state);
    }

    pub fn map(self,  map_lambda: impl Fn(ParserStateResult) -> ParserStateResult + 'static) -> Parser {
        Parser {
            parser_class: self.parser_class,
            map_lambda: Some(Box::new(map_lambda)),
            ..Parser::get_default_parser_state()
        }
    }

    pub fn error_map(self,  error_map_lambda: impl Fn(ParserStateResult, usize) -> String + 'static) -> Parser {
        Parser {
            parser_class: self.parser_class,
            error_map_lambda: Some(Box::new(error_map_lambda)),
            ..Parser::get_default_parser_state()
        }
    }

    fn get_default_parser_state() -> Parser {
        Parser {
            parser_class: Box::new(StringParser::new(String::from(""))),
            map_lambda: None,
            error_map_lambda: None
        }
    }
}

pub trait ParserClass {
    fn parse(&self, parser_state: ParserState) -> ParserState;

    fn run(&self, target_string: String) -> ParserState {
        let initial_state = ParserState::get_initial_state(target_string);
        return self.parse(initial_state);
    }
}

impl ParserClass for Parser {
    fn parse(&self, parser_state: ParserState) -> ParserState {
        let mut next_state = self.parser_class.parse(parser_state);

        if next_state.is_error {

            match &self.error_map_lambda {
                Some(lambda) => {
                    let next_state_result = next_state.result.clone();
                    let error_string = (lambda)(next_state_result, next_state.index);
                    next_state = ParserState::update_parser_error(next_state, error_string);
                }
                None => {
                    println!("No error map.");
                }
            }
            return next_state;
        }

        match &self.map_lambda {
            Some(lambda) => {
                let next_state_result = next_state.result.clone();
                let mapped_state_result = (lambda)(next_state_result);
                next_state = ParserState::update_parser_result(next_state, mapped_state_result);
            }
            None => {
                println!("No map.");
            }
        }

        next_state
    }
}