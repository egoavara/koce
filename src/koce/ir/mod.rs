use koce::ast::{Accessor};

mod table;

pub enum SymbolType{
    Struct,
    Function,
    Layer,
    Constant,
    Variable,
    Library,
}