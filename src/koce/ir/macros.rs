use koce::ast::{Sentence, Expression};
use koce::ir::{Work, Code};

#[derive(Debug)]
pub enum MacroError{
    Fail,
    PrimitiveExceedRange
}
pub enum MacroResult<A, B>{
    Switching(A),
    Modified(B),
}