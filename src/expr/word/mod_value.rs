use super::super::syntax::Word;
use super::super::util::{naming as u_naming, integer as u_integer, float as u_float, literal as u_literal};
use super::*;

use nom::{multispace0};
use nom::types::CompleteStr;

named!(pub reference<CompleteStr, Word>,
    do_parse!(
        root : u_naming >>
        next : opt!(reference_next) >>
        (Word::Reference(root.0.to_string(), match next{Some(data) => Some(Box::new(data)), None => None,}))
    )
);
named!(pub typeref<CompleteStr, Word>,
    do_parse!(
        root : u_naming >>
        next : opt!(reference_name) >>
        (Word::Reference(root.0.to_string(), match next{Some(data) => Some(Box::new(data)), None => None,}))
    )
);

named!(reference_next<CompleteStr, Word>,
    alt!(reference_index | reference_name)
);

named!(reference_name<CompleteStr, Word>,
    do_parse!(
        tag!(".") >>
        name : u_naming >>
        next : opt!(reference_next) >>
        (Word::Reference(name.0.to_string(), match next{Some(data) => Some(Box::new(data)), None => None,}))
    )
);
named!(reference_index<CompleteStr, Word>,
    do_parse!(
        index : delimited!(
            tag!("["),
            binary,
            tag!("]")
        ) >>
        next : opt!(reference_next) >>
        (Word::Index(Box::new(index), match next{Some(data) => Some(Box::new(data)), None => None,}))
    )
);

named!(pub integer<CompleteStr, Word>,
    do_parse!(
        i : u_integer >>
        (Word::Integer(i))
    )
);
named!(pub float<CompleteStr, Word>,
    do_parse!(
        f : u_float >>
        (Word::Float(f))
    )
);
named!(pub numaric<CompleteStr, Word>,
    alt!(float | integer)
);
named!(pub literal<CompleteStr, Word>,
    do_parse!(
        s : u_literal >>
        (Word::Literal(s))
    )
);
named!(pub tuple<CompleteStr, Word>,
    do_parse!(
        v : delimited!(
            tag!("("),
            separated_list!(tag!(","), ws!(binary)),
            tag!(")")
        ) >>
        (Word::Tuple(v))
    )
);
named!(pub array<CompleteStr, Word>,
    do_parse!(
        v : delimited!(
            tag!("["),
            separated_list!(tag!(","), ws!(binary)),
            tag!("]")
        ) >>
        (Word::Array(v))
    )
);
named!(pub value<CompleteStr, Word>,
    alt!(tuple | array | numaric | literal | reference)
);