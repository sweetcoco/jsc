trait Parse {
    fn parse(&self, parser_state: ParserState) -> ParserState;
}

#[derive(Debug, Clone)]
struct ParserState {
    target_string: String,
    result: Option<ParserStateResult>,
    index: usize,
    error: Option<String>,
    is_error: bool
}

#[derive(Debug, Clone)]
enum ParserStateResult {
    String(StringResult),
    Sequence(SequenceResult),
    None
}

#[derive(Debug, Clone)]
struct StringResult {
    result: String
}

#[derive(Debug, Clone)]
struct SequenceResult {
    result: Vec<ParserStateResult>
}

#[derive(Debug)]
struct StringParser {
    string: String
}

impl Parse for StringParser {
    fn parse(&self, parser_state: ParserState) -> ParserState {
        if parser_state.is_error {
            return parser_state;
        }
        
        let ParserState { target_string, index, .. } = parser_state;
        let sliced_target = &target_string[index..];

        if sliced_target.len() == 0 {
            return ParserState {
                target_string,
                error: Some(format!("string_parser: Tried to match {}, but got unexpected end of input.", &self.string)),
                is_error: true,
                ..parser_state
            }
        }

        let new_index = index + self.string.len();

        if sliced_target.starts_with(&self.string) {
            return ParserState {
                target_string,
                result: Option::Some(self.format_result(self.string.to_owned())),
                index: new_index,
                ..parser_state
            };
        }

        let error = Some(format!("Tried to match \"{}\", but got \"{}\"", &self.string, target_string));

        return ParserState {
                target_string,
                result: Option::Some(self.format_result(self.string.to_owned())),
                index: index,
                error: error,
                is_error: true
            }
    }
}

impl StringParser {
    fn new(string: String) -> Self {
        StringParser {
            string
        }
    }

    fn run(&self, target_string: String) -> ParserState {
        let initial_state = get_initial_state(target_string);
        return self.parse(initial_state);
    }

    fn format_result(&self, result: String) -> ParserStateResult {
        ParserStateResult::String(StringResult {
            result
        })
    }
}

// #[derive(Debug)]
struct SequenceOfParser {
    parsers: Vec<Box<dyn Parse>>
}

impl SequenceOfParser {
    fn new(parsers: Vec<Box<dyn Parse>>) -> Self {
        SequenceOfParser {
            parsers
        }
    }

    fn run(&self, target_string: String) -> ParserState {
        let initial_state = get_initial_state(target_string);
        return self.parse(initial_state);
    }

    fn format_result(&self, result: Vec<ParserStateResult>) -> ParserStateResult {
        ParserStateResult::Sequence(SequenceResult {
            result
        })
    }
}

impl Parse for SequenceOfParser {
    fn parse(&self, parser_state: ParserState) -> ParserState {

        if parser_state.is_error {
            return parser_state;
        }

        let mut results: Vec<ParserStateResult> = Vec::new();
        let mut next_state = parser_state;
        let parsers_iter = self.parsers.iter();
        for parser in parsers_iter {
            let next_state_clone = next_state.clone();
            next_state = parser.parse(next_state_clone);
            let result_option = next_state.result.clone();
            let result = match result_option {
                None => ParserStateResult::None,
                Some(r) => r
            };


            // match checked_division(dividend, divisor) {
            //     None => println!("{} / {} failed!", dividend, divisor),
            //     Some(quotient) => {
            //         println!("{} / {} = {}", dividend, divisor, quotient)
            //     },
            // }

            results.push(result);
        }

        return ParserState {
            result: Some(self.format_result(results)),
            target_string: next_state.target_string,
            index: next_state.index,
            error: next_state.error,
            is_error: next_state.is_error
        }
    }
}

fn main() {
    let string1 = String::from("hello");
    let string2 = String::from("goodbye");
    // let parser = StringParser::new(string1);
    let parser = SequenceOfParser::new(vec![Box::new(StringParser::new(string1)), Box::new(StringParser::new(string2))]);

    println!("{:#?}", parser.run(String::from("hello")));
}

fn get_initial_state(target_string: String) -> ParserState {
    let initial_state = ParserState {
        target_string,
        result: None,
        index: 0,
        error: None,
        is_error: false
    };

    initial_state
}