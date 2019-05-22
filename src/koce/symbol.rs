use koce::{Path, Type};
use koce::expression::Expression;
use std::fmt::{Debug, Formatter, Error};

#[derive(Debug)]
pub enum Symbol{
    Unnamed(Description),
    UnnamedImplementation(Description, Implementation),
    NamedVirtual(String),
    Named(String, Description),
    NamedImplementation(String, Description, Implementation),
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


pub enum Implementation{
    Task(),

    Handler(Box<dyn Fn(Expression) -> Result<Handling<Expression, Implementation>, HandlerError>>),
}
impl Debug for Implementation{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Implementation::Task() => f.write_fmt(format_args!("Task")),
            Implementation::Handler(_) => f.write_fmt(format_args!("Handler")),
        }
    }
}

#[derive(Debug)]
pub enum Handling<S, T>{
    Incomplete(S),
    Complete(T)
}

#[derive(Debug)]
pub enum HandlerError{
    Custom(String)
}