use koce::{Path, Type, Raw};
use koce::expression::Expression;
use std::fmt::{Debug, Formatter, Error, Display};

#[derive(Debug)]
pub enum Symbol{
    Unnamed(Description, Implementation),
    Named(String, Description, Implementation),
}

impl Symbol {
    pub fn get_name(&self) -> Option<&String>{
        match self {
            Symbol::Unnamed(_, _) => None,
            Symbol::Named(name, _, _) => Some(name),
        }
    }
    pub fn get_description(&self) -> &Description{
        match self {
            Symbol::Unnamed(desc, _) => desc,
            Symbol::Named(_, desc, _) => desc,
        }
    }
    pub fn get_description_mut(&mut self) -> &mut Description{
        match self {
            Symbol::Unnamed(desc, _) => desc,
            Symbol::Named(_, desc, _) => desc,
        }
    }
    pub fn get_implementation(&self) -> &Implementation{
        match self {
            Symbol::Unnamed(_, imple) => imple,
            Symbol::Named(_, _, imple) => imple,
        }
    }
    pub fn get_implementation_mut(&mut self) -> &mut Implementation{
        match self {
            Symbol::Unnamed(_, imple) => imple,
            Symbol::Named(_, _, imple) => imple,
        }
    }
}

impl Display for Symbol{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Symbol::Unnamed(desc, imple) => f.write_fmt(format_args!("Symbol( {}, {:?} )", desc, imple)),
            Symbol::Named(name, desc, imple) => f.write_fmt(format_args!("Symbol( '{}', {}, {:?} )", name, desc, imple)),
        }
    }
}
#[derive(Debug)]
pub enum Description{
    Virtual,
    Macro,
    Callable(Vec<Path>, Path),
    Memorable(Path),
    Structure(Vec<Type>),
    Layer,
    Define(Path)
}
impl Display for Description{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Description::Callable(args, ret) => f.write_fmt(format_args!("Callable( {:?} -> {} )", args, ret)),
            Description::Memorable(dst) => f.write_fmt(format_args!("Memorable( {} )", dst)),
            Description::Structure(components) => f.write_fmt(format_args!("Structure( {:?} )", components)),
            Description::Define(dst) => f.write_fmt(format_args!("Define( {} )", dst)),
            _ => f.write_fmt(format_args!("{:?}", self))
        }
    }
}

pub enum Implementation{
    Empty,
    Direct(Raw),
    Indirect(Path),
    Task(),
    Handler(Box<dyn Fn(Expression) -> Result<Expression, HandlerError>>),
}
impl Debug for Implementation{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Implementation::Empty => f.write_fmt(format_args!("Empty")),
            Implementation::Direct(raw) => f.write_fmt(format_args!("Direct( {:?} )", raw)),
            Implementation::Indirect(raw) => f.write_fmt(format_args!("Indirect( {:?} )", raw)),
            Implementation::Task() => f.write_fmt(format_args!("Task")),
            Implementation::Handler(_) => f.write_fmt(format_args!("Handler")),
        }
    }
}


#[derive(Debug)]
pub enum HandlerError{

    Custom(String)
}