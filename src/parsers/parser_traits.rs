use super::parser_state::ParserState;

pub trait Parse {
    fn parse(&self, parser_state: ParserState) -> ParserState;
}

pub trait Map {
    fn map(&self, map_lamda: impl FnOnce(ParserState) -> ParserState) -> dyn Parse;
}