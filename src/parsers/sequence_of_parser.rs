use super::parser_traits::Parse;
use super::parser_state::ParserState;
use super::parser_state_result::ParserStateResult;

pub struct SequenceOfParser {
    parsers: Vec<Box<dyn Parse>>
}

impl SequenceOfParser {
    pub fn new(parsers: Vec<Box<dyn Parse>>) -> Self {
        SequenceOfParser {
            parsers
        }
    }

    pub fn run(&self, target_string: String) -> ParserState {
        let initial_state = ParserState::get_initial_state(target_string);
        return self.parse(initial_state);
    }

    pub fn format_result(&self, result: Vec<ParserStateResult>) -> ParserStateResult {
        ParserStateResult::Sequence(result)
    }
}

impl Parse for SequenceOfParser {
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