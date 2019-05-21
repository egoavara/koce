use std::fmt::{Display, Error, Formatter};

use koce::ast::{Sentence, Value, Expression};
use koce::ir::{Code, ParserError, Path, Work};

#[derive(Debug)]
pub enum Symbol {
    Named(String, Frame),
    Unnamed(Frame),
}
//impl Drop for Symbol{
//    fn drop(&mut self) {
//        println!("{:?}", self);
//    }
//}
impl Symbol {
    pub fn get_frame(&self) -> &Frame {
        match self {
            Symbol::Named(_, f) => { f }
            Symbol::Unnamed(f) => { f }
        }
    }
    pub fn get_frame_mut(&mut self) -> &mut Frame {
        match self {
            Symbol::Named(_, f) => { f }
            Symbol::Unnamed(f) => { f }
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Symbol::Named(name, frame) => {
                f.write_fmt(format_args!("<Name : '{}' / Frame : {:?}>", name, frame))
            }
            Symbol::Unnamed(frame) => {
                f.write_fmt(format_args!("<Frame : {:?}>", frame))
            }
        }
    }
}

#[derive(Debug)]
pub struct Frame {
    pub description: Description,
    pub block: Work<Expression, Code>,
}

impl Frame {
    pub fn new(description: Description, block: Work<Expression, Code>) -> Self { Self { description, block } }
}

#[derive(Debug)]
pub enum Description {
    Virtual,
    Callable(Vec<Path>, Path),
    Memorable(Path),
    Structure(MemoryLayout),
    Layer,
    LayerDefine(Path),
    Generic,
    Macro,
}

#[derive(Debug)]
pub enum MemoryLayout{
    Padding(usize),
    I32,
    F32,
}

#[derive(Debug, Clone)]
pub enum KoceType {
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
    Pointer(KocePointerMetaData),
    Complex(Vec<KoceType>),
    Array(Box<KoceType>, usize),
    Indirect(Path),
}

#[derive(Debug, Clone)]
pub enum KocePointerMetaData {
    None,
    FunctionPointer(Vec<KoceType>, Box<KoceType>),
}

impl KoceType {
    pub fn default(&self) -> KoceRaw {
        match self {
            KoceType::Void => KoceRaw::Void,
            KoceType::I8 => KoceRaw::I8(0),
            KoceType::I16 => KoceRaw::I16(0),
            KoceType::I32 => KoceRaw::I32(0),
            KoceType::I64 => KoceRaw::I64(0),
            KoceType::U8 => KoceRaw::U8(0),
            KoceType::U16 => KoceRaw::U16(0),
            KoceType::U32 => KoceRaw::U32(0),
            KoceType::U64 => KoceRaw::U64(0),
            KoceType::F32 => KoceRaw::F32(0.0),
            KoceType::F64 => KoceRaw::F64(0.0),
            KoceType::Pointer(_) => KoceRaw::Pointer(0), // nullptr
            KoceType::Complex(inner) => KoceRaw::Complex(inner.iter().map(|x| x.default()).collect()),
            KoceType::Array(inner, size) => KoceRaw::Array(vec![inner.default(); *size]),
            KoceType::Indirect(_) => {
                //TODO
                unimplemented!()
            },
            _ => KoceRaw::Void,
        }
    }
}

#[derive(Debug, Clone)]
pub enum KoceRaw {
    Void,
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
    Pointer(usize),
    Complex(Vec<KoceRaw>),
    Array(Vec<KoceRaw>),
}