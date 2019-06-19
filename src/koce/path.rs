use std::path::{Path, PathBuf};
use std::ops::Deref;
use koce::{Expression, Value};
#[derive(Debug)]
pub enum PathError{
    PathRuleViolation
}
impl ExpressionPath for PathBuf{

}
pub trait ExpressionPath{
    fn from_expression(expr: &Expression) -> Result<PathBuf, PathError> {
            match expr {
                Expression::Argument(Value::Name(name)) => {
                    Ok(PathBuf::from(name.as_str()))
                }
                Expression::Member(l, r) => {
                    Ok(Self::from_expression(l.deref())?.join(Self::from_expression(r.deref())?))
                }
                Expression::Cast(_, _) => {
                    //TODO
                    unimplemented!()
                }
                _ => Err(PathError::PathRuleViolation)
            }
    }
}