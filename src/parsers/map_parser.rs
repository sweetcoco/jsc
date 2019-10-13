use super::parser_traits::Parse;
use super::parser_state::ParserState;
use super::parser_state_result::ParserStateResult;

pub struct MapParser {
    parser: Box<dyn Parse>,
    map_lambda: Box<dyn Fn(ParserStateResult) -> ParserStateResult>
}

impl Parse for MapParser {
    fn parse(&self, parser_state: ParserState) -> ParserState {
        let next_state: ParserState = self.parser.parse(parser_state);

        if next_state.is_error {
            return next_state;
        }

        let next_state_result = next_state.result.clone();
        let mapped_state_result = (self.map_lambda)(next_state_result);
        

        ParserState::update_parser_result(next_state, mapped_state_result)
    }
}


impl MapParser {
    pub fn new(parser: Box<dyn Parse>, map_lambda: impl Fn(ParserStateResult) -> ParserStateResult + 'static) -> Self {
        MapParser {
            parser: parser,
            map_lambda: Box::new(map_lambda)
        }
    }

    pub fn run(&self, target_string: String) -> ParserState {
        let initial_state = ParserState::get_initial_state(target_string);
        return self.parse(initial_state);
    }

    // pub fn format_result(&self, result: String) -> ParserStateResult {
    //     ParserStateResult::String(result)
    // }
}