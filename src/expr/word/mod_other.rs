use super::super::syntax::Token;
use super::*;

use nom::{multispace0};
use nom::types::CompleteStr;

named!(pub call<CompleteStr, Token>,
    map_res!(do_parse!(
        cmd : value >>
        args : tuple >>
        ({
            if let Token::Tuple(data) = args{
                Ok(Token::Call(
                    Box::new(cmd),
                    data,
                ))
            }else{
                Err("not callable")
            }
        })
    ), |x|x)
);
named!(pub cast<CompleteStr, Token>,
    map_res!(do_parse!(
        cmd : value >>
        args : tuple >>
        ({
            if let Token::Tuple(data) = args{
                Ok(Token::Call(
                    Box::new(cmd),
                    data,
                ))
            }else{
                Err("not callable")
            }
        })
    ), |x|x)
);
