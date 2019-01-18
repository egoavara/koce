
use super::super::syntax::Token;
use super::*;

use nom::{multispace0};
use nom::types::CompleteStr;

named!(pub unary<CompleteStr, Token>,
    map_res!(
        do_parse!(
            op:one_of!("+-!") >>
            multispace0 >>
            w : value >>
            (match op{
                '+' => Ok(Token::Pos(Box::new(w))),
                '-' => Ok(Token::Neg(Box::new(w))),
                '!' => Ok(Token::Not(Box::new(w))),
                _ => Err("not unary word")
            })
        ),
        |x|x
    )
);
