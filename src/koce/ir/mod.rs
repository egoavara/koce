use koce::ast::{Accessor};

pub enum IR{
    Symbol(Accessor, String),
    // symbol, type,
    Define(Accessor, DefineType, Option<String>,  Form),
    Allocation,
    Comment,
}

pub enum DefineType{
    Struct(),
    // arguments, return
    Function(Vec<DefineItem>, TypeItem),
    Interface,
    Constant(TypeItem),
    Variable(TypeItem),
    Library,
}
pub struct Form{

}
pub struct DefineItem{

}
pub struct TypeItem{

}