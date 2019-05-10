
use super::Path;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Definition {
    Function(Vec<(String, Definition)>, Box<Definition>),

    Shape(Path),

    Void,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    // TODO Tuple
    // TODO Array

}

#[derive(Debug, PartialEq, Clone)]
pub enum Implementation {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),

    Raw(Vec<u8>),
    Mapping(HashMap<Path, Vec<u8>>),
    Execution(Vec<Command>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    AssignDirect(Path, Implementation),
    Assign(Path, Path),
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
        Self {
            name : name.to_string(),
            definition : def,
            implementation : imp,
        }
    }
}