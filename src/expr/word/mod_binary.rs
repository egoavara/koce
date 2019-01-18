use super::super::syntax::Token;
use super::*;

use nom::{multispace0};
use nom::types::CompleteStr;


//
named!(pub single<CompleteStr, Token>,
    alt!(call | unary | value)
);

fn fold_word(a : Token, v : Vec<(String, Token)>) -> Token {
    v.into_iter().fold(a, |r, x|{
        match x.0.as_ref() {
            "+" => Token::Add(Box::new(r), Box::new(x.1)),
            "-" => Token::Sub(Box::new(r), Box::new(x.1)),
            "*" => Token::Mul(Box::new(r), Box::new(x.1)),
            "/" => Token::Div(Box::new(r), Box::new(x.1)),
            "%" => Token::Mod(Box::new(r), Box::new(x.1)),
            "<<" => Token::LShift(Box::new(r), Box::new(x.1)),
            ">>" => Token::RShift(Box::new(r), Box::new(x.1)),
            "<=" => Token::LesserEqual(Box::new(r), Box::new(x.1)),
            ">=" => Token::GreaterEqual(Box::new(r), Box::new(x.1)),
            "<" => Token::Lesser(Box::new(r), Box::new(x.1)),
            ">" => Token::Greater(Box::new(r), Box::new(x.1)),
            "==" => Token::Equal(Box::new(r), Box::new(x.1)),
            "!=" => Token::NotEqual(Box::new(r), Box::new(x.1)),
            _ => Token::Add(Box::new(r), Box::new(x.1)),
        }
    })
}

named!(binary_0<CompleteStr, Token>,
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
named!(binary_1<CompleteStr, Token>,
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
named!(binary_2<CompleteStr, Token>,
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
named!(binary_3<CompleteStr, Token>,
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
named!(binary_4<CompleteStr, Token>,
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
named!(binary_5<CompleteStr, Token>,
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
named!(binary_6<CompleteStr, Token>,
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
named!(binary_7<CompleteStr, Token>,
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

named!(pub binary<CompleteStr, Token>, call!(binary_7));