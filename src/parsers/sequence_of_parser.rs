use super::parser::Parser;
use super::parser::ParserClass;
use super::parser_state::ParserState;
use super::parser_state_result::ParserStateResult;

pub struct SequenceOfParser {
    parsers: Vec<Parser>
}

impl SequenceOfParser {
    pub fn new(parsers: Vec<Parser>) -> Self {
        SequenceOfParser {
            parsers
        }
    }

    pub fn format_result(&self, result: Vec<ParserStateResult>) -> ParserStateResult {
        ParserStateResult::Sequence(result)
    }
}

impl ParserClass for SequenceOfParser {
    fn parse(&self, parser_state: ParserState) -> ParserState {
        if parser_state.is_error {
            return parser_state;
        }

        let mut results: Vec<ParserStateResult> = Vec::new();
        let mut next_state = parser_state;

        for parser in &self.parsers {
            next_state = parser.parse(next_state);
            results.push(next_state.result.clone());
        }

        return ParserState::update_parser_result(next_state, self.format_result(results));
    }
}