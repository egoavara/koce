
use super::Path;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Definition {
    Constant(Path),
    Variable(Path),
    Library(Path),

    Function,
    Struct,
    Layer,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Implementation {
    Raw(Vec<u8>),
    Mapping(HashMap<Path, Vec<u8>>),
    Execution(Vec<Command>),
}
pub enum Command {
    Return(Path),
    Neg(Path, Path),
    Add(Path, Path, Path),
}

#[derive(Debug)]
pub struct Symbol{
    pub name : String,
    pub definition : Option<Definition>,
    pub implementation : Option<Implementation>,
}

impl Symbol {
    pub fn new(name : &str, def : Option<Definition>, imp : Option<Implementation>) -> Self{
        Self{
            name : name.to_string(),
            definition : def,
            implementation : imp,
        }
    }
}