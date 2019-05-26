use koce::{Symbol, Expression, Value, Description, Implementation, Raw, HandlerError, Type};
use gom::Explorer;
use num::traits::cast::ToPrimitive;
use koce::core;

pub fn load_i32(dst : Explorer<Symbol>) {
    let prim_i32 =dst.add_child(Symbol::Named("i32".to_string(), Description::Structure(vec![Type::I32]), Implementation::Empty));
    prim_i32.add_child(Symbol::Unnamed(Description::Define(core::handle::ARGUMENT.clone()), Implementation::Handler(Box::new(core_handle_argument))));
    prim_i32.add_child(Symbol::Unnamed(Description::Define(core::handle::NEGATIVE.clone()), Implementation::Handler(Box::new(core_handle_negative))));
}

pub fn core_handle_argument(expr : Expression) -> Result<Expression, HandlerError>{
    if let Expression::Argument(v) = expr {
        match v {
            Value::Name(_) => { Err(HandlerError::Custom("name can't parse by macro".to_string())) }
            Value::Literal(_) => { Err(HandlerError::Custom("literal can't parse by macro".to_string())) }
            Value::Bytes(bts) => {
                Ok(Expression::Primitive(Raw::I32(
                    bts.iter().fold(0, |res, x| {
                        (res << 8) | (*x as i32)
                    })
                )))
            }
            Value::Numeric(n) => {
                Ok(Expression::Primitive(Raw::I32(
                    n.to_i32().ok_or(HandlerError::Custom("literal can't parse by macro".to_string()))?
                )))
            }
        }
    } else {
        unreachable!()
    }
}
pub fn core_handle_negative(expr : Expression) -> Result<Expression, HandlerError> {
    if let Expression::Neg(inner) = expr {
        match *inner{
            Expression::Primitive(prim) => {
                match prim {
                    Raw::I32(value) => Ok(Expression::Primitive(Raw::I32(-value))),
                    _ => Err(HandlerError::Custom("literal can't parse by macro".to_string()))
                }
            },
            _ => Ok(expr)
        }
    } else {
        unreachable!()
    }
}
