#[derive(Debug)]
struct ParserState<T> {
    target_string: String,
    result: Option<T>,
    index: usize,
    error: Option<String>,
    is_error: bool
}

impl ParserState<String> {
    fn to_vec_type(&self) -> ParserState<Vec<String>> {
        let uresult: String = self.result.clone().unwrap();

        return ParserState {
            target_string: self.target_string.clone(),
            result: Some(vec![uresult]),
            index: self.index,
            error: self.error.clone(),
            is_error: self.is_error
        }
    }
}

fn string_parser (string: String) -> impl FnOnce(ParserState<String>) -> ParserState<String> {
    return move |parser_state| {

        if parser_state.is_error {
            return parser_state;
        }
        
        let ParserState { target_string, index, .. } = parser_state;
        let sliced_target = &target_string[index..];

        if sliced_target.len() == 0 {
            return ParserState {
                target_string,
                error: Some(format!("string_parser: Tried to match {}, but got unexpected end of input.", string)),
                is_error: true,
                ..parser_state
            }
        }

        let new_index = index + string.len();

        if sliced_target.starts_with(&string) {
            return ParserState {
                target_string,
                result: Option::Some(string),
                index: new_index,
                ..parser_state
            };
        }

        let error = Some(format!("Tried to match \"{}\", but got \"{}\"", string, target_string));

        return ParserState {
                target_string,
                result: Option::Some(string),
                index: index,
                error: error,
                is_error: true
            }
    }
}

fn main() {

    let string1 = String::from("hello");
    let string2 = String::from("goodbye");
    
    // get parser for a single string:
    // let parser = string_parser(string1);
    //
    // run parser for single string
    // println!("{:#?}", run(parser, String::from("hello")));

    // parse a sequence of parsers
    let parser = sequence_of(vec![string_parser(string1), string_parser(string2)]);
    println!("{:#?}", run_sequence(parser, String::from("hellogoodbye")));
}

fn sequence_of(parsers: Vec<impl FnOnce(ParserState<String>) -> ParserState<String>>) -> impl FnOnce(ParserState<String>) -> ParserState<Vec<String>> {
    return move |parser_state| {

        if parser_state.is_error {
            return parser_state.to_vec_type();
        }

        let mut results: Vec<String> = Vec::new();
        let mut next_state = parser_state;
        
        for parser in parsers {
            next_state = parser(next_state);
            let result_option = next_state.result.clone();
            let result = result_option.unwrap_or(String::from(""));
            results.push(result);
        }

        return ParserState {
            result: Some(results),
            target_string: next_state.target_string,
            index: next_state.index,
            error: next_state.error,
            is_error: next_state.is_error
        }
    }
}

fn run(parser: impl FnOnce(ParserState<String>) -> ParserState<String>, target_string: String) -> ParserState<String> {
    let initial_state = get_initial_state(target_string);
    return parser(initial_state);
}

fn run_sequence(parser: impl FnOnce(ParserState<String>) -> ParserState<Vec<String>>, target_string: String) -> ParserState<Vec<String>> {
    let initial_state = get_initial_state(target_string);
    return parser(initial_state);
}

fn get_initial_state(target_string: String) -> ParserState<String> {
    let initial_state = ParserState {
        target_string,
        result: None,
        index: 0,
        error: None,
        is_error: false
    };

    initial_state
}