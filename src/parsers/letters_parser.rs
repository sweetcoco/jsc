use super::parser::ParserClass;
use super::parser_state::ParserState;
use super::parser_state_result::ParserStateResult;

const LETTERS: [char; 52] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

#[derive(Debug)]
pub struct LettersParser {

}

impl LettersParser {
    pub fn new() -> Self {
        LettersParser {}
    }

    fn format_result(&self, result: String) -> ParserStateResult {
        ParserStateResult::String(result)
    }
}

impl ParserClass for LettersParser {
    fn parse(&self, parser_state: ParserState) -> ParserState {
        if parser_state.is_error {
            return parser_state;
        }
        
        let target_string = parser_state.target_string.to_owned();
        let index = parser_state.index;
        let sliced_target = &target_string[index..];

        if sliced_target.len() == 0 {
            return ParserState::update_parser_error(parser_state, format!("letters: got Unexpected end of input."));
        }

        let mut string = String::new();
        for c in sliced_target.chars() {
            if LETTERS.contains(&c) {
                string.push(c);
            } else {
                break;
            }
        }

        if string.len() > 0 {
            return ParserState::update_parser_state(parser_state, index + string.len(), self.format_result(string));
        }

        let truncated_slice = if sliced_target.len() < 11 { sliced_target } else { &sliced_target[..10] };

        return ParserState::update_parser_error(parser_state, format!("letters: Couldn't match letters at index {}. Here: {}", index, truncated_slice));
    }
}