//pub enum Value {
//    Name(String),
//    Literal(String),
//    Comment(String),
//    Exponential(String),
//    Binary(Vec<u8>),
//    Numeric(BigUint),
//    Hexadecimal(Vec<u8>),
//}
//
use super::{Value, Expression, Accessor, Sentence};
use nom::types::CompleteStr;
use nom::{IResult, ErrorKind, InputTakeAtPosition, AsChar};
use nom::{alpha1, alphanumeric0, alphanumeric1, digit1, hex_digit1};
use num::{BigUint, Num};
named!(pub parse_value<CompleteStr, Value>,
    alt!(
        parse_value_name
        | parse_value_literal
        | parse_value_numeric
        | parse_value_bytes
    )
);

named!(pub parse_value_name<CompleteStr, Value>,
    map!(
        recognize!(map!(tuple!(alpha1, alphanumeric0), |x|x.0.to_string())),
        |a:CompleteStr|Value::Name(a.0.parse().unwrap())
    )
);

named!(pub parse_value_literal<CompleteStr, Value>,
    alt!(
        parse_value_literal_quotation
        | parse_value_literal_astrophe
    )
);

named!(parse_value_literal_quotation<CompleteStr, Value>,
    map!(
        delimited!(char!('\"'), escaped!(is_not!("\""), '\\', one_of!("\'\"\\nNuUxo")), char!('\"')),
        |a:CompleteStr|Value::Literal(a.0.parse().unwrap())
    )
);

named!(parse_value_literal_astrophe<CompleteStr, Value>,
    map!(
        delimited!(char!('\''), escaped!(is_not!("\'"), '\\', one_of!("\'\"\\nNuUxo")), char!('\'')),
        |a:CompleteStr|Value::Literal(a.0.parse().unwrap())
    )
);

named!(pub parse_value_numeric<CompleteStr, Value>,
    map!(
        digit1,
        |i|Value::Numeric(BigUint::from_str_radix(i.0, 10).unwrap())
    )
);
named!(pub parse_value_bytes<CompleteStr, Value>,
    alt!(
        parse_value_bytes_binary
        | parse_value_bytes_hexadecimal
    )
);
named!(parse_value_bytes_binary<CompleteStr, Value>,
    map!(
        preceded!(tag!("0b"), re_find!("[0-1_]+")),
        |a|Value::Bytes(a.replace("_", "").as_bytes().rchunks(8).map(|x|u8::from_str_radix(String::from_utf8_lossy(x).as_ref(), 2).unwrap()).rev().collect::<Vec<u8>>())
    )
);
named!(parse_value_bytes_hexadecimal<CompleteStr, Value>,
    map!(
        preceded!(tag!("0x"), re_find!("[0-9a-zA-Z_]+")),
        |a|Value::Bytes(a.replace("_", "").as_bytes().rchunks(2).map(|x|u8::from_str_radix(String::from_utf8_lossy(x).as_ref(), 16).unwrap()).rev().collect::<Vec<u8>>())
    )
);

named!(pub parse_expr<CompleteStr, Expression>, call!(parse_expr_binary));

named!(parse_expr_value<CompleteStr, Expression>,
    alt!(
        parse_expr_generic
        | parse_expr_tuple
        | parse_expr_array
        | parse_expr_argument
    )
);

named!(pub parse_expr_argument<CompleteStr, Expression>,
    map!(
        parse_value,
        |v|Expression::Argument(v)
    )
);

named!(pub parse_expr_tuple<CompleteStr, Expression>,
    map!(
        delimited!(char!('('), separated_list!(tag!(","), ws!(parse_expr)), char!(')')),
        |v|Expression::Tuple(v)
    )
);
named!(pub parse_expr_array<CompleteStr, Expression>,
    map!(
        delimited!(char!('['), separated_list!(tag!(","), ws!(parse_expr)), char!(']')),
        |v|Expression::Array(v)
    )
);
// Special case
named!(pub parse_expr_generic<CompleteStr, Expression>,
    map!(
        delimited!(char!('<'), separated_list!(tag!(","), ws!(parse_expr_binary_4)), char!('>')),
        |v|Expression::Generic(v)
    )
);




named!(pub parse_expr_binary<CompleteStr, Expression>, call!(parse_expr_binary_9));
named!(parse_expr_binary_0<CompleteStr, Expression>,
    do_parse!(
        a : parse_expr_value >>
        v_op_b : many0!(pair!(ws!(alt!(tag!(".") | tag!("@"))), parse_expr_value )) >>
        (v_op_b.into_iter().fold(a, |a, (op, b)|{
            match op.0{
                "." => Expression::Member(Box::new(a), Box::new(b)),
                "@" => Expression::Cast(Box::new(a), Box::new(b)),
                _ => unreachable!(),
            }
        }))
    )
);
named!(pub parse_expr_call<CompleteStr, Expression>,
    map!(
        pair!(parse_expr_binary_0, opt!(ws!(parse_expr_tuple))),
        |(address, tuple)|match tuple{
            Some(args) => {
                Expression::Call(
                    Box::new(address),
                    if let Expression::Tuple(argsv) = args{argsv}else { unreachable!() },
                )
            }
            None => {
                address
            }
        }
    )
);
named!(pub parse_expr_unary<CompleteStr, Expression>,
    do_parse!(
        op : opt!(alt!(tag!("+") | tag!("-") | tag!("!"))) >>
        v : ws!(parse_expr_call) >>
        (
            match op{
                Some(some) => match some.0{
                    "+" =>Expression::Pos(Box::new(v)),
                    "-" =>Expression::Neg(Box::new(v)),
                    "!" =>Expression::Not(Box::new(v)),
                    _ => unreachable!()
                }
                None => v
            }
        )
    )
);

named!(parse_expr_binary_1<CompleteStr, Expression>,
    do_parse!(
        a : parse_expr_unary >>
        v_op_b : many0!(pair!(ws!(alt!(
            tag!("**")
        )), parse_expr_unary )) >>
        (v_op_b.into_iter().fold(a, |a, (op, b)|{
            match op.0{
                "**" => Expression::Exp(Box::new(a), Box::new(b)),
                _ => unreachable!(),
            }
        }))
    )
);
named!(parse_expr_binary_2<CompleteStr, Expression>,
    do_parse!(
        a : parse_expr_binary_1 >>
        v_op_b : many0!(pair!(ws!(alt!(
            tag!("*")
            | tag!("/")
            | tag!("%")
        )), parse_expr_binary_1 )) >>
        (v_op_b.into_iter().fold(a, |a, (op, b)|{
            match op.0{
                "*" => Expression::Mul(Box::new(a), Box::new(b)),
                "/" => Expression::Div(Box::new(a), Box::new(b)),
                "%" => Expression::Mod(Box::new(a), Box::new(b)),
                _ => unreachable!(),
            }
        }))
    )
);
named!(parse_expr_binary_3<CompleteStr, Expression>,
    do_parse!(
        a : parse_expr_binary_2 >>
        v_op_b : many0!(pair!(ws!(alt!(
            tag!("+")
            | tag!("-")
        )), parse_expr_binary_2 )) >>
        (v_op_b.into_iter().fold(a, |a, (op, b)|{
            match op.0{
                "+" => Expression::Add(Box::new(a), Box::new(b)),
                "-" => Expression::Sub(Box::new(a), Box::new(b)),
                _ => unreachable!(),
            }
        }))
    )
);
named!(pub parse_expr_binary_4<CompleteStr, Expression>,
    do_parse!(
        a : parse_expr_binary_3 >>
        v_op_b : many0!(pair!(ws!(alt!(
            tag!("<<")
            | tag!(">>")
        )), parse_expr_binary_3 )) >>
        (v_op_b.into_iter().fold(a, |a, (op, b)|{
            match op.0{
                "<<" => Expression::ShL(Box::new(a), Box::new(b)),
                ">>" => Expression::ShR(Box::new(a), Box::new(b)),
                _ => unreachable!(),
            }
        }))
    )
);
named!(parse_expr_binary_5<CompleteStr, Expression>,
    do_parse!(
        a : parse_expr_binary_4 >>
        v_op_b : many0!(pair!(ws!(alt!(
            tag!("<=")
            | tag!(">=")
            | tag!("<")
            | tag!(">")
        )), parse_expr_binary_4 )) >>
        (v_op_b.into_iter().fold(a, |a, (op, b)|{
            match op.0{
                "<" => Expression::L(Box::new(a), Box::new(b)),
                ">" => Expression::G(Box::new(a), Box::new(b)),
                "<=" => Expression::Le(Box::new(a), Box::new(b)),
                ">=" => Expression::Ge(Box::new(a), Box::new(b)),
                _ => unreachable!(),
            }
        }))
    )
);
named!(parse_expr_binary_6<CompleteStr, Expression>,
    do_parse!(
        a : parse_expr_binary_5 >>
        v_op_b : many0!(pair!(ws!(alt!(
            tag!("==")
            | tag!("!=")
        )), parse_expr_binary_5 )) >>
        (v_op_b.into_iter().fold(a, |a, (op, b)|{
            match op.0{
                "==" => Expression::Eq(Box::new(a), Box::new(b)),
                "!=" => Expression::Neq(Box::new(a), Box::new(b)),
                _ => unreachable!(),
            }
        }))
    )
);
named!(parse_expr_binary_7<CompleteStr, Expression>,
    do_parse!(
        a : parse_expr_binary_6 >>
        v_op_b : many0!(pair!(ws!(alt!(
            tag!("&")
        )), parse_expr_binary_6 )) >>
        (v_op_b.into_iter().fold(a, |a, (op, b)|{
            match op.0{
                "&" => Expression::And(Box::new(a), Box::new(b)),
                _ => unreachable!(),
            }
        }))
    )
);
named!(parse_expr_binary_8<CompleteStr, Expression>,
    do_parse!(
        a : parse_expr_binary_7 >>
        v_op_b : many0!(pair!(ws!(alt!(
            tag!("^")
        )), parse_expr_binary_7 )) >>
        (v_op_b.into_iter().fold(a, |a, (op, b)|{
            match op.0{
                "^" => Expression::Xor(Box::new(a), Box::new(b)),
                _ => unreachable!(),
            }
        }))
    )
);
named!(parse_expr_binary_9<CompleteStr, Expression>,
    do_parse!(
        a : parse_expr_binary_8 >>
        v_op_b : many0!(pair!(ws!(alt!(
            tag!("|")
        )), parse_expr_binary_8 )) >>
        (v_op_b.into_iter().fold(a, |a, (op, b)|{
            match op.0{
                "|" => Expression::Or(Box::new(a), Box::new(b)),
                _ => unreachable!(),
            }
        }))
    )
);


named!(pub parse_accessor<CompleteStr, Accessor>,
    alt!(
        map!(tag!("pub"), |_|Accessor::Public)
        | map!(tag!("pkg"), |_|Accessor::Package)
        | map!(tag!("exc"), |_|Accessor::Exclusive)
        | map!(tag!("pri"), |_|Accessor::Private)
    )
);


//named!(pub parse_sentence_constant<CompleteStr, Sentence>,
//    do_parse!(
//        tag!("const") >>
//        name : ws!(parse_expr) >>
//        definition : opt!(preceded!(ws!(tag!(":")), parse_expr)) >>
//        assign : opt!(preceded!(ws!(tag!("=")), parse_expr)) >>
//        (Sentence::Constant(name, definition, assign))
//    )
//);