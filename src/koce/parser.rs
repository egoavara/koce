use gom::GOM;
use koce::symbol::Symbol;
use koce::symbol::Description;
use koce::sentence::Sentence;
use koce::symbol::HandlerError;
use nom::types::CompleteStr;

pub struct Parser{
    root : GOM<Symbol>,
}
#[derive(Debug)]
pub enum ParserError{
    FailHandle(HandlerError),
    InvalidSyntax(String),
    NotConsumeAll(String),
}
impl Parser {
    pub fn new() -> Self {
        Parser { root: GOM::setup(Symbol::Unnamed(Description::Virtual)) }
    }
}

impl Parser {
    pub fn parse_str(&mut self, s : &str) -> Result<(), ParserError>{
        let (left, res) = super::parse_sentence(CompleteStr(s)).map_err(|_|ParserError::InvalidSyntax("".to_string()))?;
        if left.0.len() > 0{
            return Err(ParserError::NotConsumeAll(left.0.to_string()))
        }
        self.parse_sentence(a?)
    }
    pub fn parse_sentence(&mut self, stc : Sentence) -> Result<(), ParserError>{
        match stc {
            Sentence::Define(_, _, _, _) => {},
            Sentence::Library(_, _, _, _) => {},
            Sentence::Constant(_, _, _, _) => {},
            Sentence::Variable(_, _, _, _) => {},
            Sentence::Layer(_, _, _, _) => {},
            Sentence::Struct(_, _, _, _) => {},
            Sentence::Function(_, _, _, _) => {},
            Sentence::Comment(_) => {},
            Sentence::Assign(_, _) => {},
            Sentence::Mean(_) => {},
            Sentence::Lambda(_, _) => {},
            Sentence::If(_, _, _) => {},
            Sentence::Return(_) => {},
            Sentence::After(_) => {},
            Sentence::Block(_) => {},
        }
    }
}