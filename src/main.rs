mod parsers;

use parsers::string_parser::StringParser;
use parsers::sequence_of_parser::SequenceOfParser;

fn main() {
    let string1 = String::from("hello there!");
    let string2 = String::from("goodbye there!");
    // let mut parser = StringParser::new(string1);
    
    let parser = SequenceOfParser::new(vec![Box::new(StringParser::new(string1)), Box::new(StringParser::new(string2))]);

    println!("{:#?}", parser.run(String::from("hello there!goodbye there!")));
}