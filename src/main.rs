#[macro_use] extern crate nom;
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;
extern crate regex;
extern crate hex;
extern crate num;

mod expr;

use nom::types::CompleteStr;
use expr::util::*;
use expr::word;

fn main() {

    match expr::syntax::If (CompleteStr("if a == 1 {
    if b == 2{

    }
}")) {
        Ok((remain, value)) => {
            println!("remain : '{}', value : {:?}", remain, value);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
