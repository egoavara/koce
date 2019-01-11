use nom::{AsChar, InputTakeAtPosition, IResult, ErrorKind, Err};
use nom::types::CompleteStr;
use nom::simple_errors::Context;

pub use nom::{hex_digit1, digit1, digit0, alpha1, multispace0};

use hex::{decode, FromHexError};

use std::str::FromStr;

pub fn bin_digit1<T>(input: T) -> IResult<T, T>
    where
        T: InputTakeAtPosition,
        <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position1(|item| {
        match item.as_char() {
            '0' | '1' => false,
            _ => true,
        }
    }, ErrorKind::Custom(0))
}

fn is_naming(c : char) -> bool{
    match c {
        c if c.is_alphanumeric() => true,
        '_' => true,
        _ => false,
    }
}
named!(pub naming<CompleteStr, CompleteStr>,
    recognize!(do_parse!(
        alpha1 >>
        take_while!(is_naming) >>
        ()
    ))
);


fn hex_parse(a : CompleteStr) -> Result<i128, FromHexError>{
    Ok(decode(a.0.trim_left_matches("0x"))?.iter().fold(0, |result, &data|result<<8 | data as i128))
}
fn bin_parse(a : CompleteStr) -> Result<i128, FromHexError>{
    Ok(a.0.trim_left_matches("0b").chars().into_iter().fold(0, |result, data|match data {'0' =>result << 1,'1' =>result << 1 | 1,_ => result,}))
}


// Interger
named!(pub dec_integer<CompleteStr, i128>,
    map_res!(
        digit1,
        |a:CompleteStr|a.0.parse()
    )
);

named!(pub hex_integer<CompleteStr, i128>,
    map_res!(
        recognize!(
            do_parse!(
                tag!("0x") >>
                hex_digit1 >>
                ()
            )
        ),
        hex_parse
    )
);
named!(pub bin_integer<CompleteStr, i128>,
    map_res!(
        recognize!(
            do_parse!(
                tag!("0b") >>
                bin_digit1 >>
                ()
            )
        ),
        bin_parse
    )
);
named!(pub integer<CompleteStr, i128>,
    alt!(bin_integer | hex_integer | dec_integer)
);

// Float
named!(pub float<CompleteStr, f64>,
    map_res!(
        recognize!(
            do_parse!(
                dec_integer >>
                tag!(".") >>
                opt!(digit1) >>
                ()
            )
        ),
        |a:CompleteStr|a.0.parse()
    )
);
fn convert_numaric<T>(data : CompleteStr) -> Result<T, T::Err>
    where T : FromStr
{
    let mut temp = String::from(data.0);
    if temp.starts_with("0x"){
        match hex_parse(data) {
            Ok(ok) => {
                temp = ok.to_string();
            },
            Err(err) =>{
                temp = err.to_string();
            },
        }
    }else if temp.starts_with("0b"){
        match bin_parse(data) {
            Ok(ok) => {
                temp = ok.to_string();
            },
            Err(err) =>{
                temp = err.to_string();
            },
        }
    }
    temp.parse()
}
// numaric
named!(hide_numaric<CompleteStr, CompleteStr>,
    alt!(
        recognize!(integer)|
        recognize!(float)
    )
);
pub fn numaric<T>(input: CompleteStr) -> IResult<CompleteStr, T>
    where
        T: FromStr,
{
    match hide_numaric(input) {
        Ok((remain, val)) =>{
            match convert_numaric(val) {
                Ok(ok) => {
                    Ok((remain, ok))
                }
                Result::Err(err) =>{
                    Result::Err(Err::Error(Context::Code(input, ErrorKind::Custom(0))))
                }
            }
        }
        Result::Err(err) => {
            Result::Err(err)
        }
    }
}

named!(literal_0<CompleteStr, String>, map!(
    delimited!(
        tag!("\'"),
        take_until!("\'"),
        tag!("\'")
    ),
    |x| x.0.to_string()
));
named!(literal_1<CompleteStr, String>, map!(
    delimited!(
        tag!("\""),
        take_until!("\""),
        tag!("\"")
    ),
    |x| x.0.to_string()
));
named!(pub literal<CompleteStr, String>, alt!(literal_0 | literal_1));
