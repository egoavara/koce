use std::ffi::OsStr;
use std::fmt::{Display, Error, Formatter};
use std::path::{Path, PathBuf};

use nom::types::CompleteStr;

use gom::{Explorer, GOM, PathMatcher};
use koce::{Expression, PathError, Sentence, Value};
use num::traits::cast::ToPrimitive;
use std::io::Read;

#[derive(Debug)]
pub enum ParserData {
    Virtual,
    NamedVirtual(String),
    Generic(String, Vec<Type>),
    GenericImple(String, PathBuf),
    // name, type,
    Parameter(String, Type),
    Return(Type),
    Variable(String, Type),
    Enum(String, Option<PathBuf>),
    Field(String, Type),
    // name (args -> return) code
    Function(String),
    //
    Layer(String),
    //
    Define(Type),
    Works(Vec<Task>),
    Raw(RawData),
}
impl Display for ParserData {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_fmt(format_args!("{:?}", self))
    }
}
impl PathMatcher for ParserData {
    fn is_matched(&self, test: &OsStr) -> bool {
        match self {
            ParserData::Virtual => false,
            ParserData::NamedVirtual(name) => name.as_str() == test,
            ParserData::Generic(name, _) => name.as_str() == test,
            ParserData::GenericImple(_, _) => false,
            ParserData::Parameter(name, _) => name.as_str() == test,
            ParserData::Return(_) => false,
            ParserData::Variable(name, _) => name.as_str() == test,
            ParserData::Function(name) => name.as_str() == test,
            ParserData::Layer(name) => name.as_str() == test,
            ParserData::Define(_) => false,
            ParserData::Works(_) => false,
            ParserData::Raw(_) => false,
            ParserData::Enum(name, _) => name.as_str() == test,
            ParserData::Field(name, _) => name.as_str() == test,
        }
    }
}

#[derive(Debug)]
pub enum RawData {
    I32(i32)
}

#[derive(Debug)]
pub enum Task {
    // dst, compiler_todo_from, compiler_todo_to
    Incomplete(IncompleteTaskMeta, Argument, Argument, Argument),
    // dst, src0, src1
    Add(Argument, Argument, Argument),
    Call(Vec<Argument>),
    Return(Argument),
    ReturnVoid,
    Member(Argument, Argument, Argument),
    Store(Argument, Argument),
}
#[derive(Debug)]
pub enum IncompleteTaskMeta {
    Cast
}
#[derive(Debug)]
pub enum Argument {
    Direct(Value),
    Indirect(PathBuf),
    Temporary(usize),
    UnHandled(Expression),
}

#[derive(Debug)]
pub enum Type{
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
    Array(Box<Type>, usize),
    Complex(Vec<Type>),
    Reference(PathBuf),
}
impl Type{
    // (Type, [  Generics, ... ])
    pub fn from_expression(exor : &Expression) -> Result<Self, ParserError>{
        match exor {
            Expression::Argument(v) => match v {
                Value::Name(name) => {
                    match name.as_str() {
                        "i8" => Ok(Type::I8),
                        "i16" => Ok(Type::I16),
                        "i32" => Ok(Type::I32),
                        "i64" => Ok(Type::I64),
                        "u8" => Ok(Type::U8),
                        "u16" => Ok(Type::U16),
                        "u32" => Ok(Type::U32),
                        "u64" => Ok(Type::U64),
                        "f32" => Ok(Type::F32),
                        "f64" => Ok(Type::F64),
                        "Self" => Ok(Type::Reference(PathBuf::from("."))),
                        name => Ok(Type::Reference(PathBuf::from(name)))
                    }
                }
                _ => Err(ParserError::TempTypeError(0x00))
            },
            Expression::Tuple(inner) => {
                let a = inner.iter().map(|x|Self::from_expression(x)).collect::<Result<Vec<Type>, ParserError>>();
                Ok(Type::Complex(a?))
            },
            Expression::Array(inner) => {
                let def = inner.get(0);
                let def = match def {
                    None => {return Err(ParserError::TempTypeError(0x01))},
                    Some(some) => {
                        Self::from_expression(some)?
                    },
                };
                let length = inner.get(1);
                let length = match length {
                    None => {0},
                    Some(some) => {
                        if let Expression::Argument(Value::Numeric(num)) = some{
                            num.to_usize().unwrap()
                        }else{
                            return Err(ParserError::TempTypeError(0x02))
                        }
                    },
                };
                Ok(Type::Array(Box::new(def), length))
            },
            Expression::FunctionShape(_, _) => {
                unimplemented!()
            },
            Expression::Member(_, _) => {
                // Ignore generic Type
                use koce::ExpressionPath;
                Ok(Type::Reference(PathBuf::from_expression(&exor).map_err(|x|ParserError::ParsePathError(x))?))
            },
            _ => Err(ParserError::TempTypeError(0x03))
        }
    }
}

pub struct Parser {
    root: GOM<ParserData>
}
impl Parser {
    pub fn new() -> Self {
        Self {
            root: GOM::setup(ParserData::Virtual)
        }
    }
    pub fn root(&self) -> Explorer<ParserData> {
        self.root.explore().unwrap()
    }
}


#[derive(Debug)]
pub enum ParserError {
    Unimplemented,
    UnknownPath(PathBuf),
    ParsePathError(PathError),
    EnumSubSymbolError(String),
    LayerConditionalSymbolError(String),
    TempTypeError(usize),
    ImplementationFail,
}

pub trait ToSentences {
    fn to_sentence(&self) -> Result<Vec<Sentence>, ParserError>;
}
impl ToSentences for Sentence {
    fn to_sentence(&self) -> Result<Vec<Sentence>, ParserError> {
        Ok(vec![self.clone()])
    }
}
impl<T: AsRef<str>> ToSentences for T {
    fn to_sentence(&self) -> Result<Vec<Sentence>, ParserError> {
        let (left, stc) = super::parse_sentence_multiple(CompleteStr(self.as_ref())).map_err(|x| {
            ParserError::Unimplemented
        })?;
        if left.0.len() > 0 {
            Err(ParserError::Unimplemented)
        } else {
            Ok(stc)
        }
    }
}



// Spec
// EnumHeader
// - Total : 4 byte + (min : 0 byte, max : 256 byte)
//
//  0               1                ...
//  0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7  ...
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-
// | Enum ID       | Option Length | ...
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-