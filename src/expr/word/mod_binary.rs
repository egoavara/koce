use super::super::syntax::Word;
use super::*;

use nom::{multispace0};
use nom::types::CompleteStr;


//
named!(pub single<CompleteStr, Word>,
    alt!(call | unary | value)
);

fn fold_word(a : Word, v : Vec<(String, Word)>) -> Word{
    v.into_iter().fold(a, |r, x|{
        match x.0.as_ref() {
            "+" => Word::Add(Box::new(r), Box::new(x.1)),
            "-" => Word::Sub(Box::new(r), Box::new(x.1)),
            "*" => Word::Mul(Box::new(r), Box::new(x.1)),
            "/" => Word::Div(Box::new(r), Box::new(x.1)),
            "%" => Word::Mod(Box::new(r), Box::new(x.1)),
            "<<" => Word::LShift(Box::new(r), Box::new(x.1)),
            ">>" => Word::RShift(Box::new(r), Box::new(x.1)),
            "<=" => Word::LesserEqual(Box::new(r), Box::new(x.1)),
            ">=" => Word::GreaterEqual(Box::new(r), Box::new(x.1)),
            "<" => Word::Lesser(Box::new(r), Box::new(x.1)),
            ">" => Word::Greater(Box::new(r), Box::new(x.1)),
            "==" => Word::Equal(Box::new(r), Box::new(x.1)),
            "!=" => Word::NotEqual(Box::new(r), Box::new(x.1)),
            _ => Word::Add(Box::new(r), Box::new(x.1)),
        }
    })
}

named!(binary_0<CompleteStr, Word>,
    do_parse!(
        a : single >>
        multispace0 >>
        v : many0!(do_parse!(
            op : alt!(
                tag!("*") |
                tag!("/") |
                tag!("%")
            )>>
            multispace0 >>
            b : single >>
            ((op.0.to_string(), b))
        )) >>
        (fold_word(a,v))
    )
);
named!(binary_1<CompleteStr, Word>,
    do_parse!(
        a : binary_0 >>
        multispace0 >>
        v : many0!(do_parse!(
            op : alt!(
                tag!("+") |
                tag!("-")
            )>>
            multispace0 >>
            b : binary_0 >>
            ((op.0.to_string(), b))
        )) >>
        (fold_word(a,v))
    )
);
named!(binary_2<CompleteStr, Word>,
    do_parse!(
        a : binary_1 >>
        multispace0 >>
        v : many0!(do_parse!(
            op : alt!(
                tag!("<<") |
                tag!(">>")
            )>>
            multispace0 >>
            b : binary_1 >>
            ((op.0.to_string(), b))
        )) >>
        (fold_word(a,v))
    )
);
named!(binary_3<CompleteStr, Word>,
    do_parse!(
        a : binary_2 >>
        multispace0 >>
        v : many0!(do_parse!(
            op : alt!(
                tag!("<=") |
                tag!(">=") |
                tag!("<") |
                tag!(">")
            )>>
            multispace0 >>
            b : binary_2 >>
            ((op.0.to_string(), b))
        )) >>
        (fold_word(a,v))
    )
);
named!(binary_4<CompleteStr, Word>,
    do_parse!(
        a : binary_3 >>
        multispace0 >>
        v : many0!(do_parse!(
            op : alt!(
                tag!("==") |
                tag!("!=")
            ) >>
            multispace0 >>
            b : binary_3 >>
            ((op.0.to_string(), b))
        )) >>
        (fold_word(a,v))
    )
);
named!(binary_5<CompleteStr, Word>,
    do_parse!(
        a : binary_4 >>
        multispace0 >>
        v : many0!(do_parse!(
            op : alt!(
                tag!("&")
            ) >>
            multispace0 >>
            b : binary_4 >>
            ((op.0.to_string(), b))
        )) >>
        (fold_word(a,v))
    )
);
named!(binary_6<CompleteStr, Word>,
    do_parse!(
        a : binary_5 >>
        multispace0 >>
        v : many0!(do_parse!(
            op : alt!(
                tag!("^")
            ) >>
            multispace0 >>
            b : binary_5 >>
            ((op.0.to_string(), b))
        )) >>
        (fold_word(a,v))
    )
);
named!(binary_7<CompleteStr, Word>,
    do_parse!(
        a : binary_6 >>
        multispace0 >>
        v : many0!(do_parse!(
            op : alt!(
                tag!("|")
            ) >>
            multispace0 >>
            b : binary_6 >>
            ((op.0.to_string(), b))
        )) >>
        (fold_word(a,v))
    )
);

named!(pub binary<CompleteStr, Word>, call!(binary_7));