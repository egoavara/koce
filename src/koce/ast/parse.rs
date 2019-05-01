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
use nom::{alpha1, alphanumeric0, alphanumeric1, digit1, hex_digit1, multispace1, multispace0};
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

named!(pub parse_expr_value<CompleteStr, Expression>,
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
named!(pub parse_expr_binary_0<CompleteStr, Expression>,
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
        multispace0 >>
        v : parse_expr_call >>
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
named!(pub parse_expr_binary_5<CompleteStr, Expression>,
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
        | map!(tag!("exc"), |_|Accessor::Exclusive)
        | map!(tag!("pkg"), |_|Accessor::Package)
        | map!(tag!("pri"), |_|Accessor::Private)
    )
);

named!(pub parse_sentence<CompleteStr, Sentence>,
    alt!(
        parse_sentence_constant
        | parse_sentence_variable
        | parse_sentence_library
        | parse_sentence_layer
        | parse_sentence_struct
        | parse_sentence_function
        //
        | parse_sentence_lambda // must before parse_sentence_function_shape
        | parse_sentence_function_shape
        | parse_sentence_block
        | parse_sentence_assign
        | parse_sentence_return
        | parse_sentence_after
        | parse_sentence_if
        | parse_sentence_mean
    )
);

named!(pub parse_sentence_constant<CompleteStr, Sentence>,
    do_parse!(
        accessor : opt!(parse_accessor) >>
        ws!(tag!("const")) >>
        name : ws!(parse_expr) >>
        definition : opt!(preceded!(ws!(tag!(":")), parse_sentence)) >>
        assign : opt!(preceded!(ws!(tag!("=")), parse_sentence)) >>
        (Sentence::Constant(accessor.unwrap_or(Accessor::Private), name, Box::new(definition), Box::new(assign)))
    )
);

named!(pub parse_sentence_variable<CompleteStr, Sentence>,
    do_parse!(
        accessor : opt!(parse_accessor) >>
        ws!(tag!("var")) >>
        name : ws!(parse_expr) >>
        definition : opt!(preceded!(ws!(tag!(":")), parse_sentence)) >>
        assign : opt!(preceded!(ws!(tag!("=")), parse_sentence)) >>
        (Sentence::Variable(accessor.unwrap_or(Accessor::Private), name, Box::new(definition), Box::new(assign)))
    )
);

named!(pub parse_sentence_library<CompleteStr, Sentence>,
    do_parse!(
        accessor : opt!(parse_accessor) >>
        ws!(tag!("lib")) >>
        name : ws!(parse_expr) >>
        definition : opt!(preceded!(ws!(tag!(":")), parse_sentence)) >>
        assign : opt!(preceded!(ws!(tag!("=")), parse_sentence)) >>
        (Sentence::Library(accessor.unwrap_or(Accessor::Private), name, Box::new(definition), Box::new(assign)))
    )
);

named!(pub parse_sentence_layer<CompleteStr, Sentence>,
    do_parse!(
        accessor : opt!(parse_accessor) >>
        ws!(tag!("layer")) >>
        name : opt!(ws!(parse_expr)) >>
        definition : opt!(preceded!(ws!(tag!(":")), parse_sentence)) >>
        assign : opt!(preceded!(ws!(tag!("=")), parse_sentence)) >>
        (Sentence::Layer(accessor.unwrap_or(Accessor::Private), name, Box::new(definition), Box::new(assign)))
    )
);

named!(pub parse_sentence_struct<CompleteStr, Sentence>,
    do_parse!(
        accessor : opt!(parse_accessor) >>
        ws!(tag!("struct")) >>
        name : opt!(ws!(parse_expr)) >>
        definition : opt!(preceded!(ws!(tag!(":")), parse_sentence)) >>
        assign : opt!(preceded!(ws!(tag!("=")), parse_sentence)) >>
        (Sentence::Struct(accessor.unwrap_or(Accessor::Private), name, Box::new(definition), Box::new(assign)))
    )
);

named!(pub parse_sentence_function<CompleteStr, Sentence>,
    do_parse!(
        accessor : opt!(parse_accessor) >>
        ws!(tag!("fn")) >>
        name : opt!(ws!(parse_expr)) >>
        definition : opt!(preceded!(ws!(tag!(":")), parse_sentence)) >>
        assign : opt!(preceded!(ws!(tag!("=")), parse_sentence)) >>
        (Sentence::Function(accessor.unwrap_or(Accessor::Private), name, Box::new(definition), Box::new(assign)))
    )
);

named!(pub parse_sentence_block<CompleteStr, Sentence>,
    map!(
        delimited!(char!('{'), separated_list!(alt!(
            char!('\n')
            | char!(';')
        ), ws!(parse_sentence)), char!('}')),
        |x|Sentence::Block(x)
    )
);


named!(pub parse_sentence_assign<CompleteStr, Sentence>,
    alt!(
        parse_sentence_direct_assign
        | parse_sentence_op_assign
    )
);

named!(parse_sentence_direct_assign<CompleteStr, Sentence>,
    do_parse!(
        dst : parse_expr >>
        ws!(tag!("=")) >>
        src : parse_expr >>
        (Sentence::Assign(dst, src))
    )
);

named!(parse_sentence_op_assign<CompleteStr, Sentence>,
    do_parse!(
        dst : parse_expr >>
        op : terminated!(alt!(
            tag!("+")
            | tag!("-")
            | tag!("*")
            | tag!("/")
            | tag!("%")
            | tag!("**")
            | tag!("&")
            | tag!("|")
            | tag!("^")
            | tag!("<<")
            | tag!(">>")
        ), tag!("=")) >>
        src : parse_expr >>
        (match op.0{
            "+" => Sentence::Assign(dst.clone(), Expression::Add(Box::new(dst), Box::new(src))),
            "-" => Sentence::Assign(dst.clone(), Expression::Sub(Box::new(dst), Box::new(src))),
            "*" => Sentence::Assign(dst.clone(), Expression::Mul(Box::new(dst), Box::new(src))),
            "/" => Sentence::Assign(dst.clone(), Expression::Div(Box::new(dst), Box::new(src))),
            "%" => Sentence::Assign(dst.clone(), Expression::Mod(Box::new(dst), Box::new(src))),
            "**" => Sentence::Assign(dst.clone(), Expression::Exp(Box::new(dst), Box::new(src))),
            "&" => Sentence::Assign(dst.clone(), Expression::And(Box::new(dst), Box::new(src))),
            "|" => Sentence::Assign(dst.clone(), Expression::Or(Box::new(dst), Box::new(src))),
            "^" => Sentence::Assign(dst.clone(), Expression::Xor(Box::new(dst), Box::new(src))),
            "<<" => Sentence::Assign(dst.clone(), Expression::ShL(Box::new(dst), Box::new(src))),
            ">>" => Sentence::Assign(dst.clone(), Expression::ShR(Box::new(dst), Box::new(src))),
            _ => unreachable!()
        })
    )
);

//named!(pub parse_sentence_after<CompleteStr, Sentence>,
//    do_parse!(
//        tag!("before") >>
//        multispace1 >>
//        expr : parse_expr >>
//        (Sentence::Return(expr))
//    )
//);

named!(pub parse_sentence_after<CompleteStr, Sentence>,
    do_parse!(
        tag!("after") >>
        multispace1 >>
        expr : parse_expr >>
        (Sentence::After(expr))
    )
);

named!(pub parse_sentence_return<CompleteStr, Sentence>,
    do_parse!(
        tag!("return") >>
        multispace1 >>
        expr : parse_expr >>
        (Sentence::Return(expr))
    )
);

named!(pub parse_sentence_mean<CompleteStr, Sentence>,
    do_parse!(
        expr : parse_expr  >>
        (Sentence::Mean(expr))
    )
);

named!(pub parse_sentence_function_shape<CompleteStr, Sentence>,
    do_parse!(
        args : parse_function_arguments  >>
        ws!(tag!("->")) >>
        rets : opt!(parse_expr)  >>
        (Sentence::FunctionShape(args, rets))
    )
);

named!(parse_function_arguments<CompleteStr, Vec<(Expression, Option<Expression>)>>,
    delimited!(char!('('), separated_list!(tag!(","), ws!(parse_function_argument_each)), char!(')'))
);
named!(parse_function_argument_each<CompleteStr, (Expression, Option<Expression>)>,
    pair!(parse_expr , opt!(preceded!(ws!(tag!(":")), parse_expr)))
);


named!(pub parse_sentence_lambda<CompleteStr, Sentence>,
    do_parse!(
        shape : parse_sentence_function_shape  >>
        form : parse_sentence  >>
        (Sentence::Lambda(Box::new(shape), Box::new(form)))
    )
);

named!(pub parse_sentence_if<CompleteStr, Sentence>,
    do_parse!(
        tag!("if") >>
        multispace1 >>
        condition : parse_expr>>
        multispace1 >>
        ok : parse_sentence  >>
        not : opt!(preceded!(ws!(tag!("else")), parse_sentence))  >>
        (Sentence::If(condition, Box::new(ok), Box::new(not)))
    )
);

