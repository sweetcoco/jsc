mod parsers;

use parsers::parser::Parser;
use parsers::parser_state_result::ParserStateResult;

fn main() {
    let string1 = String::from("hello there!");
    // let string2 = String::from("goodbye there!");
    // let parser = Parser::new_string_parser(string1);
    // let parser = Parser::new_sequence_of_parser(vec![Parser::new_digits_parser(), Parser::new_letters_parser(), Parser::new_digits_parser()]);
    // let parser = Parser::new_digits_parser();
    // let parser = Parser::new_choice_parser(vec![Parser::new_digits_parser(), Parser::new_letters_parser()]);
    let parser = Parser::new_many_parser(Parser::new_choice_parser(vec![Parser::new_digits_parser(), Parser::new_letters_parser()]), false);

    println!("{:#?}", parser.run(String::from("4356dskjds")));
}







    // let parser = Parser::new_string_parser(string1)
    // .map(move |parser_state_result| {
    //     let new_result = match parser_state_result {
    //         ParserStateResult::String(string) => {
    //             ParserStateResult::String(string.to_uppercase())
    //         },
    //         _ => { panic!("oh no!")}
    //     };

    //     new_result
    // })
    // .error_map(move |parser_state_result, index| {
    //     match parser_state_result {
    //         ParserStateResult::String(string) => {
    //             return format!("Expected a greeting @ index {}, in String: {}", index, string);
    //         },
    //         _ => {
    //             return format!("Expected a greeting @ index {}", index);
    //             }
    //     };
    // });