use super::parser_traits::Parse;
use super::parser_state::ParserState;
use super::parser_state_result::ParserStateResult;

#[derive(Debug)]
pub struct StringParser {
    string: String
}

impl Parse for StringParser {
    fn parse(&self, parser_state: ParserState) -> ParserState {
        if parser_state.is_error {
            return parser_state;
        }
        
        let target_string = parser_state.target_string.to_owned();
        let index = parser_state.index;
        let sliced_target = &target_string[index..];

        if sliced_target.len() == 0 {
            return ParserState::update_parser_error(parser_state, format!("string_parser: Tried to match `{}`, but got Unexpected end of input.", &self.string));
        }

        let new_index = index + self.string.len();

        if sliced_target.starts_with(&self.string) {
            return ParserState::update_parser_state(parser_state, new_index, self.format_result(self.string.to_owned()));
        }

        let truncated_slice = if sliced_target.len() < 11 { sliced_target } else { &sliced_target[..10] };

        return ParserState::update_parser_error(parser_state, format!("Tried to match `{}`, but got `{}`", &self.string, truncated_slice));
    }
}

impl StringParser {
    pub fn new(string: String) -> Self {
        StringParser {
            string
        }
    }

    pub fn run(&self, target_string: String) -> ParserState {
        let initial_state = ParserState::get_initial_state(target_string);
        return self.parse(initial_state);
    }

    pub fn format_result(&self, result: String) -> ParserStateResult {
        ParserStateResult::String(result)
    }
}