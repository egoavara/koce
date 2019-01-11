use super::super::syntax::Word;
use super::*;

use nom::{multispace0};
use nom::types::CompleteStr;

named!(pub call<CompleteStr, Word>,
    map_res!(do_parse!(
        cmd : value >>
        args : tuple >>
        ({
            if let Word::Tuple(data) = args{
                Ok(Word::Call(
                    Box::new(cmd),
                    data,
                ))
            }else{
                Err("not callable")
            }
        })
    ), |x|x)
);
named!(pub cast<CompleteStr, Word>,
    map_res!(do_parse!(
        cmd : value >>
        args : tuple >>
        ({
            if let Word::Tuple(data) = args{
                Ok(Word::Call(
                    Box::new(cmd),
                    data,
                ))
            }else{
                Err("not callable")
            }
        })
    ), |x|x)
);
//named!(pub cast<CompleteStr, Word>,
//    do_parse!(
//        w0 : alt!(single) >>
//        multispace0 >>
//        op : tag!("as") >>
//        multispace0 >>
//        w1 : typeref >>
//        (Word::Cast(Box::new(w0), Box::new(w1)))
//    ),
//);
