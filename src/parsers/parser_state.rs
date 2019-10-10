use super::parser_state_result::ParserStateResult;

#[derive(Debug, Clone)]
pub struct ParserState {
    pub target_string: String,
    pub result: ParserStateResult,
    pub index: usize,
    pub error: Option<String>,
    pub is_error: bool
}

impl ParserState {
    pub fn get_initial_state(target_string: String) -> ParserState {
        ParserState {
            target_string,
            result: ParserStateResult::None,
            index: 0,
            error: None,
            is_error: false
        }
    }

    pub fn update_parser_state(parser_state: ParserState, index: usize, result: ParserStateResult) -> ParserState {

        ParserState {
            result,
            index,
            ..parser_state
        }
    }

    pub fn update_parser_result(parser_state: ParserState, result: ParserStateResult) -> ParserState {

        ParserState {
            result,
            ..parser_state
        }
    }

    pub fn update_parser_error(parser_state: ParserState, error: String) -> ParserState {

        ParserState {
            error: Some(error),
            is_error: true,
            ..parser_state
        }
    }
}