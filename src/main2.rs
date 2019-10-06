#[derive(Debug)]
enum ParserType {
    String
}

#[derive(Debug)]
struct ParserState {
    target_value: String,
    index: usize,
    result: Option<String>,
    error: Option<String>,
    is_error: bool
}

#[derive(Debug)]
struct Parser {
    value: String,
    parser_state: ParserState,
    parser_type: ParserType
}

impl Parser {
    fn parse_string(self) -> ParserState {

        let value = self.value;
        let parser_state = self.parser_state;
        let target_value = parser_state.target_value.clone();
        let index = parser_state.index;
        let new_index = value.len().clone();
        let len = target_value.len();
        let slice = &target_value[index..len];
        let is_error = !slice.starts_with(&value);
        let mut error: Option<String> = None;
        let mut result: Option<String> = None;

        if is_error {
            error = Some(<String>::from("String error"));
        } else {
            result = Some(value);
        }

        return ParserState {
            target_value,
            index: new_index,
            result,
            error,
            is_error
        }
    }
}

fn main() {

    let parser_state = ParserState {
        target_value: String::from(""),
        index: 0,
        result: None,
        error: None,
        is_error: false
    };

    let parser = Parser {
        value: String::from("hello, world"),
        parser_state: parser_state,
        parser_type: ParserType::String
    };

    println!("{:?}", parser.parse_string());
}

// fn sequence_of(parsers: Vec<Parser>) {
//     let results: Vec<String> = Vec::new();
//     let mut next_state = 

// }