use koce::ast::{Expression, Value};

pub fn make_local_name(expr : &Expression) -> Option<String>{
    match expr{
        Expression::Argument(value) => {
            match value {
                Value::Name(name) => {Some(name.clone())},
                _ => {None}
            }
        },
        _ => {
            None
        }
    }
}