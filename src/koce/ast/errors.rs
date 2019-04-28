
#[derive(Debug)]
enum DefinedError{
    UnrecognizableValue(String),
    InvalidHexadecimal(String),
}