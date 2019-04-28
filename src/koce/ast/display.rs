

use std::fmt::{Display, Formatter, Error};
use super::Value;

impl Display for Value{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Value::Name(_) =>{
                f.write_fmt(format_args!("{:?}", self))
            }
            Value::Exponential(_) =>{
                f.write_fmt(format_args!("{:?}", self))
            }
            Value::Binary(_) =>{
                f.write_fmt(format_args!("{:?}", self))
            }
            Value::Numeric(n) =>{
                f.write_fmt(format_args!("Numeric({})", n.to_str_radix(10)))
            }
            Value::Hexadecimal(_) =>{
                f.write_fmt(format_args!("{:?}", self))
            }
        };
        return Ok(())
    }
}