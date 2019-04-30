

use std::fmt::{Display, Formatter, Error};
use super::Value;
use num::BigUint;
use nom::AsBytes;
use koce::ast::utils::hex_format;

impl Display for Value{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Value::Name(n) =>{
                f.write_fmt(format_args!("Name({})", n))
            }
            Value::Literal(l) =>{
                f.write_fmt(format_args!("Literal({})", l))
            }
            Value::Bytes(v) =>{
                f.write_fmt(format_args!("Bytes({})", hex_format(v, ", ")))
            }
            Value::Numeric(i) =>{
                f.write_fmt(format_args!("Numeric({})", i.to_str_radix(10)))
            }
        };
        return Ok(())
    }
}