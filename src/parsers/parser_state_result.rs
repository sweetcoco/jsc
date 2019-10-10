#[derive(Debug, Clone)]
pub enum ParserStateResult {
    String(String),
    Sequence(Vec<ParserStateResult>),
    None
}

// #[derive(Debug, Clone)]
// pub struct StringResult {
//     pub result: String
// }

// #[derive(Debug, Clone)]
// pub struct SequenceResult {
//     pub result: Vec<ParserStateResult>
// }