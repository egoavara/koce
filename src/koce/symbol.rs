use koce::{Path, Type};

pub enum Symbol{
    Unnamed(Description),
    UnnamedImplementation(Description, Implementation),
    NamedVirtual(String),
    Named(String, Description),
    NamedImplementation(String, Description, Implementation),
}
pub enum Description{
    Macro,
    Callable(Vec<Path>, Path),
    Memorable(Path),
    Structure(Vec<Type>),
    Layer,
    Define(Path)
}

pub enum Implementation{

}