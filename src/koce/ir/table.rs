use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::SymbolType;

pub enum Permission{
    No,
    Read,
    ReadWrite,
}
pub enum Symbol {
    Root(Vec<Rc<RefCell<Symbol>>>, String, SymbolType, MemoryType, Permission),
    // parent, children, name
    Node(Rc<RefCell<Symbol>>, Vec<Rc<RefCell<Symbol>>>, String, SymbolType, MemoryType, Permission),
    Leaf(Rc<RefCell<Symbol>>, Operation, Permission),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PrimitiveType {
    Void,

    U8,
    U16,
    U32,
    U64,

    I8,
    I16,
    I32,
    I64,

    F32,
    F64,
}

impl PrimitiveType {
    fn size(&self) -> usize {
        match self {
            PrimitiveType::Void => 0,
            PrimitiveType::U8 | PrimitiveType::I8 => 1,
            PrimitiveType::U16 | PrimitiveType::I16 => 2,
            PrimitiveType::U32 | PrimitiveType::I32 | PrimitiveType::F32=> 4,
            PrimitiveType::U64 | PrimitiveType::I64 | PrimitiveType::F64=> 8,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MemoryType {
    NoMemory,
    Primitive(PrimitiveType),
    Array(PrimitiveType, usize),
    FunctionPointer,
}
impl MemoryType {
    fn size(&self, os_bit : usize) -> usize {
        match self {
            MemoryType::NoMemory => 0,
            MemoryType::Primitive(pt) => pt.size(),
            MemoryType::Array(pt, count) => pt.size() * count,
            MemoryType::FunctionPointer => os_bit,
        }
    }
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SymbolType {
    Struct,
    Function,
    Layer,
    Constant,
    Variable,
    Library,
}


pub enum Operation {
    Op(Rc<RefCell<Symbol>>),
    UnaryOp(Rc<RefCell<Symbol>>, UnaryOperator, Rc<RefCell<Symbol>>),
    BinaryOp(Rc<RefCell<Symbol>>, BinaryOperator, Rc<RefCell<Symbol>>, Rc<RefCell<Symbol>>),
}

pub enum UnaryOperator {
    Neg
}

pub enum BinaryOperator {
    Add
}