use std::fmt::{Debug, Error, Formatter};

use koce::ast::{Expression, Sentence};
use koce::ir::{Description, Frame, KoceRaw, KoceType, MacroError, ParserError, MacroResult};

#[derive(Debug)]
pub enum Work<S, T> {
    NotStart,
    Incomplete(S),
    Complete(T),
    Empty,
}
pub enum Code {
    // works, after,
    FunctionCode(Vec<Task>),
    BinaryCode(KoceRaw),
    MacroCode(Box<dyn Fn(&Expression, Option<MacroResult<Expression, Code>>) -> Result<MacroResult<Expression, Code>, MacroError>>),
}

impl Debug for Code {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Code::FunctionCode(tasks) => {
                f.write_fmt(format_args!("Code::FunctionCode({:?})", tasks))
            }
            Code::BinaryCode(raw) => {
                f.write_fmt(format_args!("Code::BinaryCode({:?})", raw))
            }
            Code::MacroCode(_) => { f.write_str("Code::MacroCode") }
        }
    }
}

#[derive(Debug)]
pub enum Task {
    Return(KoceRaw)
}