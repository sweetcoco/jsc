use super::parser::Parser;
use super::parser::ParserClass;
use super::parser_state::ParserState;
use super::parser_state_result::ParserStateResult;

pub struct ManyParser {
    parser: Parser,
    require_one: bool
}

impl ManyParser {
    pub fn new(parser: Parser, require_one: bool) -> Self {
        ManyParser {
            parser,
            require_one: require_one
        }
    }

    pub fn format_result(&self, result: Vec<ParserStateResult>) -> ParserStateResult {
        ParserStateResult::Sequence(result)
    }
}

impl ParserClass for ManyParser {
    fn parse(&self, parser_state: ParserState) -> ParserState {
        if parser_state.is_error {
            return parser_state;
        }
        
        let mut next_state = parser_state.clone();
        let mut results: Vec<ParserStateResult> = Vec::new();
        let mut done = false;

        while !done {
            let test_state = self.parser.parse(next_state.clone());
            if !test_state.is_error {
                results.push(test_state.result.clone());
                next_state = test_state;
            } else {
                done = true;
            }
        }

        if self.require_one && results.len() < 1 {
            let index = parser_state.index;
            return ParserState::update_parser_error(parser_state, format!("Unabled to match any input using parser @ index {}", index));
        }

        return ParserState::update_parser_result(next_state, self.format_result(results));
    }
}