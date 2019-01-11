
use super::super::syntax::Word;
use super::*;

use nom::{multispace0};
use nom::types::CompleteStr;

named!(pub unary<CompleteStr, Word>,
    map_res!(
        do_parse!(
            op:one_of!("+-!") >>
            multispace0 >>
            w : value >>
            (match op{
                '+' => Ok(Word::Pos(Box::new(w))),
                '-' => Ok(Word::Neg(Box::new(w))),
                '!' => Ok(Word::Not(Box::new(w))),
                _ => Err("not unary word")
            })
        ),
        |x|x
    )
);
