use std::fmt::{Display, Error, Formatter};
use std::string::String;

use nom::{multispace0, multispace1};
use nom::types::CompleteStr;
use num::bigint::BigInt;

use koce::{parse_value, Value};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum Expression {
    //
    Argument(Value),
    // '('
    Tuple(Vec<Expression>),
    // '['
    Array(Vec<Expression>),
    // <args> -> <return>
    FunctionShape(Vec<(Expression, Expression)>, Box<Option<Expression>>),
    // '<', 예외적으로 제네릭 형 안의 익스프레션은 논리레벨 이상의 표현식이 올 수 없다
    Generic(Vec<Expression>),
    // control = <address>(<args...>)
    Call(Box<Expression>, Vec<Expression>),
    // control = <src>.<dst>
    Member(Box<Expression>, Box<Expression>),
    // control = <from>@<to>
    Cast(Box<Expression>, Box<Expression>),
    // control, unary = $ <Expression>
    // arithmetic, unary = + <Expression>
    Pos(Box<Expression>),
    // arithmetic, unary = - <Expression>
    Neg(Box<Expression>),
    // arithmetic, binary = <Expression> + <Expression>
    Add(Box<Expression>, Box<Expression>),
    // arithmetic, binary = <Expression> - <Expression>
    Sub(Box<Expression>, Box<Expression>),
    // arithmetic, binary = <Expression> * <Expression>
    Mul(Box<Expression>, Box<Expression>),
    // arithmetic, binary = <Expression> / <Expression>
    Div(Box<Expression>, Box<Expression>),
    // arithmetic, binary = <Expression> % <Expression>
    Mod(Box<Expression>, Box<Expression>),
    // arithmetic, binary = <Expression> ** <Expression>
    Exp(Box<Expression>, Box<Expression>),

    // logical, unary = ! <Expression>
    Not(Box<Expression>),
    // logical, binary = <Expression> == <Expression>
    Eq(Box<Expression>, Box<Expression>),
    // logical, binary = <Expression> != <Expression>
    Neq(Box<Expression>, Box<Expression>),
    // logical, binary = <Expression> > <Expression>
    G(Box<Expression>, Box<Expression>),
    // logical, binary = <Expression> < <Expression>
    L(Box<Expression>, Box<Expression>),
    // logical, binary = <Expression> >= <Expression>
    Ge(Box<Expression>, Box<Expression>),
    // logical, binary = <Expression> <= <Expression>
    Le(Box<Expression>, Box<Expression>),

    // <logical/bitwise>, binary = <Expression> & <Expression>
    And(Box<Expression>, Box<Expression>),
    // <logical/bitwise>, binary = <Expression> | <Expression>
    Or(Box<Expression>, Box<Expression>),
    // <logical/bitwise>, binary = <Expression> ^ <Expression>
    Xor(Box<Expression>, Box<Expression>),

    // bitwise, binary = <Expression> << <Expression>
    ShL(Box<Expression>, Box<Expression>),
    // bitwise, binary = <Expression> >> <Expression>
    ShR(Box<Expression>, Box<Expression>),
}
named!(pub parse_expr<CompleteStr, Expression>, call!(parse_expr_binary));

named!(pub parse_expr_value<CompleteStr, Expression>,
    alt!(
        parse_expr_generic
        | parse_expr_function_shape
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
        pair!(parse_expr_binary_0, opt!(parse_expr_tuple)),
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


named!(pub parse_expr_function_shape<CompleteStr, Expression>,
    do_parse!(
        args : parse_function_arguments  >>
        ws!(tag!("->")) >>
        rets : opt!(parse_expr)  >>
        (Expression::FunctionShape(args, Box::new(rets)))
    )
);

named!(parse_function_arguments<CompleteStr, Vec<(Expression, Expression)>>,
    delimited!(char!('('), separated_list!(tag!(","), ws!(parse_function_argument_each)), char!(')'))
);
named!(parse_function_argument_each<CompleteStr, (Expression, Expression)>,
    pair!(parse_expr , preceded!(ws!(tag!(":")), parse_expr))
);