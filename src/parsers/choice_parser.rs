use super::parser::Parser;
use super::parser::ParserClass;
use super::parser_state::ParserState;
use super::parser_state_result::ParserStateResult;

pub struct ChoiceParser {
    parsers: Vec<Parser>
}

impl ChoiceParser {
    pub fn new(parsers: Vec<Parser>) -> Self {
        ChoiceParser {
            parsers
        }
    }

    // pub fn format_result(&self, result: Vec<ParserStateResult>) -> ParserStateResult {
    //     ParserStateResult::Sequence(result)
    // }
}

impl ParserClass for ChoiceParser {
    fn parse(&self, parser_state: ParserState) -> ParserState {
        if parser_state.is_error {
            return parser_state;
        }

        for parser in &self.parsers {
            let next_state = parser.parse(parser_state.clone());
            
            if !next_state.is_error {
                return next_state;
            }
        }

        let index = parser_state.index;

        return ParserState::update_parser_error(parser_state, format!("choice: unable to match with any parser at index {}.", index));
    }
}