use num::BigInt;
use std::fmt::{Display, Formatter, Error};
use nom::types::CompleteStr;
use nom::{alpha1, alphanumeric0, digit1};
use num::Num;

#[derive(Debug, Clone)]
pub enum Value {
    Name(String),
    Literal(String),
    Bytes(Vec<u8>),
    Numeric(BigInt),
}
pub fn hex_format
<R: AsRef<[u8]>>(raw:R, sep : &str) -> String{
    raw.as_ref().iter().fold(String::new(), |mut res, d|{
        res += format!("0x{:02X}{}", d, sep).as_str();
        res
    }).trim_end_matches(sep).to_string()
}



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

named!(pub parse_value<CompleteStr, Value>,
    alt!(
        parse_value_name
        | parse_value_literal
        | parse_value_bytes
        | parse_value_numeric
    )
);
named!(pub parse_value_name<CompleteStr, Value>,
    map!(
        recognize!(map!(tuple!(alpha1, alphanumeric0), |x|x.0.to_string())),
        |a:CompleteStr|Value::Name(a.0.parse().unwrap())
    )
);

named!(pub parse_value_literal<CompleteStr, Value>,
    alt!(
        parse_value_literal_quotation
        | parse_value_literal_astrophe
    )
);

named!(parse_value_literal_quotation<CompleteStr, Value>,
    map!(
        delimited!(char!('\"'), escaped!(is_not!("\""), '\\', one_of!("\'\"\\nNuUxo")), char!('\"')),
        |a:CompleteStr|Value::Literal(a.0.parse().unwrap())
    )
);

named!(parse_value_literal_astrophe<CompleteStr, Value>,
    map!(
        delimited!(char!('\''), escaped!(is_not!("\'"), '\\', one_of!("\'\"\\nNuUxo")), char!('\'')),
        |a:CompleteStr|Value::Literal(a.0.parse().unwrap())
    )
);

named!(pub parse_value_numeric<CompleteStr, Value>,
    map!(
        digit1,
        |i|Value::Numeric(BigInt::from_str_radix(i.0, 10).unwrap())
    )
);
named!(pub parse_value_bytes<CompleteStr, Value>,
    alt!(
        parse_value_bytes_binary
        | parse_value_bytes_hexadecimal
    )
);
named!(parse_value_bytes_binary<CompleteStr, Value>,
    map!(
        preceded!(tag!("0b"), re_find!("[0-1_]+")),
        |a|Value::Bytes(a.replace("_", "").as_bytes().rchunks(8).map(|x|u8::from_str_radix(String::from_utf8_lossy(x).as_ref(), 2).unwrap()).rev().collect::<Vec<u8>>())
    )
);
named!(parse_value_bytes_hexadecimal<CompleteStr, Value>,
    map!(
        preceded!(tag!("0x"), re_find!("[0-9a-zA-Z_]+")),
        |a|Value::Bytes(a.replace("_", "").as_bytes().rchunks(2).map(|x|u8::from_str_radix(String::from_utf8_lossy(x).as_ref(), 16).unwrap()).rev().collect::<Vec<u8>>())
    )
);