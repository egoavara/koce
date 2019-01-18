use super::super::syntax::Token;
use super::super::util::{naming as u_naming, integer as u_integer, float as u_float, literal as u_literal};
use super::*;

use nom::{multispace0};
use nom::types::CompleteStr;

named!(pub reference<CompleteStr, Token>,
    do_parse!(
        root : u_naming >>
        next : opt!(reference_next) >>
        (Token::Reference(root.0.to_string(), match next{Some(data) => Some(Box::new(data)), None => None,}))
    )
);
named!(pub typeref<CompleteStr, Token>,
    do_parse!(
        root : u_naming >>
        next : opt!(reference_name) >>
        (Token::Reference(root.0.to_string(), match next{Some(data) => Some(Box::new(data)), None => None,}))
    )
);

named!(reference_next<CompleteStr, Token>,
    alt!(reference_index | reference_name)
);

named!(reference_name<CompleteStr, Token>,
    do_parse!(
        tag!(".") >>
        name : u_naming >>
        next : opt!(reference_next) >>
        (Token::Reference(name.0.to_string(), match next{Some(data) => Some(Box::new(data)), None => None,}))
    )
);
named!(reference_index<CompleteStr, Token>,
    do_parse!(
        index : delimited!(
            tag!("["),
            binary,
            tag!("]")
        ) >>
        next : opt!(reference_next) >>
        (Token::Index(Box::new(index), match next{Some(data) => Some(Box::new(data)), None => None,}))
    )
);

named!(pub integer<CompleteStr, Token>,
    do_parse!(
        i : u_integer >>
        (Token::Integer(i))
    )
);
named!(pub float<CompleteStr, Token>,
    do_parse!(
        f : u_float >>
        (Token::Float(f))
    )
);
named!(pub numaric<CompleteStr, Token>,
    alt!(float | integer)
);
named!(pub literal<CompleteStr, Token>,
    do_parse!(
        s : u_literal >>
        (Token::Literal(s))
    )
);
named!(pub tuple<CompleteStr, Token>,
    do_parse!(
        v : delimited!(
            tag!("("),
            separated_list!(tag!(","), ws!(binary)),
            tag!(")")
        ) >>
        (Token::Tuple(v))
    )
);
named!(pub array<CompleteStr, Token>,
    do_parse!(
        v : delimited!(
            tag!("["),
            separated_list!(tag!(","), ws!(binary)),
            tag!("]")
        ) >>
        (Token::Array(v))
    )
);
named!(pub value<CompleteStr, Token>,
    alt!(tuple | array | numaric | literal | reference)
);