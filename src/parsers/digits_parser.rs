use super::parser::ParserClass;
use super::parser_state::ParserState;
use super::parser_state_result::ParserStateResult;

const DIGITS: [char; 10] = ['0','1','2','3','4','5','6','7','8','9'];

#[derive(Debug)]
pub struct DigitsParser {

}

impl DigitsParser {
    pub fn new() -> Self {
        DigitsParser {}
    }

    fn format_result(&self, result: String) -> ParserStateResult {
        ParserStateResult::String(result)
    }
}

impl ParserClass for DigitsParser {
    fn parse(&self, parser_state: ParserState) -> ParserState {
        if parser_state.is_error {
            return parser_state;
        }
        
        let target_string = parser_state.target_string.to_owned();
        let index = parser_state.index;
        let sliced_target = &target_string[index..];

        if sliced_target.len() == 0 {
            return ParserState::update_parser_error(parser_state, format!("digits: got Unexpected end of input."));
        }

        let mut string = String::new();
        for c in sliced_target.chars() {
            if DIGITS.contains(&c) {
                string.push(c);
            } else {
                break;
            }
        }

        if string.len() > 0 {
            return ParserState::update_parser_state(parser_state, index + string.len(), self.format_result(string));
        }

        let truncated_slice = if sliced_target.len() < 11 { sliced_target } else { &sliced_target[..10] };

        return ParserState::update_parser_error(parser_state, format!("digits: Couldn't match digits at index {}. Here: {}", index, truncated_slice));
    }
}