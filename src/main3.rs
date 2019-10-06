#[derive(Debug)]
enum ParserType {
    String
}

#[derive(Debug)]
struct ParserState {
    target_string: String
}

#[derive(Debug)]
struct Parser {
    value: String,
    parser_type: ParserType
}

impl Parser {
    fn run(&self, parser_state: ParserState) -> ParserState {
        let state = self.parse_string(parser_state);
        state
    }

    fn parse_string(&self, parser_state: ParserState) -> ParserState {

        if parser_state.target_string.eq(&self.value) {
            return parser_state;
        }

        panic!("bad!");
    }
}

fn main() {
    let parser = Parser {
        value: String::from("Hello, world!"),
        parser_type: ParserType::String
    };

    let parser_state = ParserState {
        target_string: String::from("Hello, world!")
    };

    let state = parser.run(parser_state);

    println!("{:#?}", state);
}