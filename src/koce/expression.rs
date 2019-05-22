use std::fmt::{Display, Error, Formatter};
use std::string::String;

use nom::{multispace0, multispace1};
use nom::types::CompleteStr;
use num::bigint::BigInt;

use koce::{parse_value, Path, Raw, Value};

#[derive(Debug, Clone)]
pub enum Expression {
    // No Parser allow
    Binary(Raw),
    Reference(Path),
    //
    Argument(Value),
    // '('
    Tuple(Vec<Expression>),
    // '['
    Array(Vec<Expression>),
    // <args> -> <return>
    FunctionShape(Vec<(Expression, Option<Expression>)>, Box<Option<Expression>>),
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

impl Expression {
    pub fn make_path_priority(self) -> Self {
        match self {
            Expression::Binary(_) => self,
            Expression::Reference(_) => self,
            Expression::Argument(_) => self,

            Expression::Tuple(mut v) => {
                if v.len() == 1{
                    v.remove(0)
                }else{
                    Expression::Tuple(v.into_iter().map(|x| {
                        match Path::from_expression(&x) {
                            Ok(ok) => Expression::Reference(ok),
                            Err(_) => x,
                        }
                    }).collect())
                }
            }
            Expression::Generic(v) |
            Expression::Array(v) => {
                Expression::Tuple(v.into_iter().map(|x| {
                    match Path::from_expression(&x) {
                        Ok(ok) => Expression::Reference(ok),
                        Err(_) => x,
                    }
                }).collect())
            }
            Expression::FunctionShape(args, ret) => {
                // TODO
                unimplemented!()
            }

            Expression::Call(_, _) => {
                // TODO
                unimplemented!()
            }
            Expression::Member(_, _) => {
                Expression::Reference(Path::from_expression(&self).unwrap())
            }

            Expression::Pos(v) => {
                Expression::Pos(Box::new(Path::from_expression(&v).map(|x| Expression::Reference(x)).unwrap_or(*v)))
            }
            Expression::Neg(v) => {
                Expression::Neg(Box::new(Path::from_expression(&v).map(|x| Expression::Reference(x)).unwrap_or(*v)))
            }
            Expression::Cast(l, r) => {
                Expression::Cast(
                    Box::new(Path::from_expression(&l).map(|x| Expression::Reference(x)).unwrap_or(*l)),
                    Box::new(Path::from_expression(&r).map(|x| Expression::Reference(x)).unwrap_or(*r)),
                )
            }

            Expression::Add(l, r) => {
                Expression::Add(
                    Box::new(Path::from_expression(&l).map(|x| Expression::Reference(x)).unwrap_or(*l)),
                    Box::new(Path::from_expression(&r).map(|x| Expression::Reference(x)).unwrap_or(*r)),
                )
            }
            Expression::Sub(l, r) => {
                Expression::Sub(
                    Box::new(Path::from_expression(&l).map(|x| Expression::Reference(x)).unwrap_or(*l)),
                    Box::new(Path::from_expression(&r).map(|x| Expression::Reference(x)).unwrap_or(*r)),
                )
            }
            Expression::Mul(l, r) => {
                Expression::Mul(
                    Box::new(Path::from_expression(&l).map(|x| Expression::Reference(x)).unwrap_or(*l)),
                    Box::new(Path::from_expression(&r).map(|x| Expression::Reference(x)).unwrap_or(*r)),
                )
            }
            Expression::Div(l, r) => {
                Expression::Div(
                    Box::new(Path::from_expression(&l).map(|x| Expression::Reference(x)).unwrap_or(*l)),
                    Box::new(Path::from_expression(&r).map(|x| Expression::Reference(x)).unwrap_or(*r)),
                )
            }
            Expression::Mod(l, r) => {
                Expression::Mod(
                    Box::new(Path::from_expression(&l).map(|x| Expression::Reference(x)).unwrap_or(*l)),
                    Box::new(Path::from_expression(&r).map(|x| Expression::Reference(x)).unwrap_or(*r)),
                )
            }
            Expression::Exp(l, r) => {
                Expression::Exp(
                    Box::new(Path::from_expression(&l).map(|x| Expression::Reference(x)).unwrap_or(*l)),
                    Box::new(Path::from_expression(&r).map(|x| Expression::Reference(x)).unwrap_or(*r)),
                )
            }
            Expression::Not(v) => {
                Expression::Not(Box::new(Path::from_expression(&v).map(|x| Expression::Reference(x)).unwrap_or(*v)))
            }
            Expression::Eq(l, r) => {
                Expression::Eq(
                    Box::new(Path::from_expression(&l).map(|x| Expression::Reference(x)).unwrap_or(*l)),
                    Box::new(Path::from_expression(&r).map(|x| Expression::Reference(x)).unwrap_or(*r)),
                )
            }
            Expression::Neq(l, r) => {
                Expression::Neq(
                    Box::new(Path::from_expression(&l).map(|x| Expression::Reference(x)).unwrap_or(*l)),
                    Box::new(Path::from_expression(&r).map(|x| Expression::Reference(x)).unwrap_or(*r)),
                )
            }
            Expression::G(l, r) => {
                Expression::G(
                    Box::new(Path::from_expression(&l).map(|x| Expression::Reference(x)).unwrap_or(*l)),
                    Box::new(Path::from_expression(&r).map(|x| Expression::Reference(x)).unwrap_or(*r)),
                )
            }
            Expression::L(l, r) => {
                Expression::L(
                    Box::new(Path::from_expression(&l).map(|x| Expression::Reference(x)).unwrap_or(*l)),
                    Box::new(Path::from_expression(&r).map(|x| Expression::Reference(x)).unwrap_or(*r)),
                )}
            Expression::Ge(l, r) => {
                Expression::Ge(
                    Box::new(Path::from_expression(&l).map(|x| Expression::Reference(x)).unwrap_or(*l)),
                    Box::new(Path::from_expression(&r).map(|x| Expression::Reference(x)).unwrap_or(*r)),
                )}
            Expression::Le(l, r) => {
                Expression::Le(
                    Box::new(Path::from_expression(&l).map(|x| Expression::Reference(x)).unwrap_or(*l)),
                    Box::new(Path::from_expression(&r).map(|x| Expression::Reference(x)).unwrap_or(*r)),
                )}
            Expression::And(l, r) => {
                Expression::And(
                    Box::new(Path::from_expression(&l).map(|x| Expression::Reference(x)).unwrap_or(*l)),
                    Box::new(Path::from_expression(&r).map(|x| Expression::Reference(x)).unwrap_or(*r)),
                )}
            Expression::Or(l, r) => {
                Expression::Or(
                    Box::new(Path::from_expression(&l).map(|x| Expression::Reference(x)).unwrap_or(*l)),
                    Box::new(Path::from_expression(&r).map(|x| Expression::Reference(x)).unwrap_or(*r)),
                )}
            Expression::Xor(l, r) => {
                Expression::Xor(
                    Box::new(Path::from_expression(&l).map(|x| Expression::Reference(x)).unwrap_or(*l)),
                    Box::new(Path::from_expression(&r).map(|x| Expression::Reference(x)).unwrap_or(*r)),
                )}
            Expression::ShL(l, r) => {
                Expression::ShL(
                    Box::new(Path::from_expression(&l).map(|x| Expression::Reference(x)).unwrap_or(*l)),
                    Box::new(Path::from_expression(&r).map(|x| Expression::Reference(x)).unwrap_or(*r)),
                )}
            Expression::ShR(l, r) => {
                Expression::ShR(
                    Box::new(Path::from_expression(&l).map(|x| Expression::Reference(x)).unwrap_or(*l)),
                    Box::new(Path::from_expression(&r).map(|x| Expression::Reference(x)).unwrap_or(*r)),
                )}
        }
    }
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


named!(pub parse_expr_function_shape<CompleteStr, Expression>,
    do_parse!(
        args : parse_function_arguments  >>
        ws!(tag!("->")) >>
        rets : opt!(parse_expr)  >>
        (Expression::FunctionShape(args, Box::new(rets)))
    )
);

named!(parse_function_arguments<CompleteStr, Vec<(Expression, Option<Expression>)>>,
    delimited!(char!('('), separated_list!(tag!(","), ws!(parse_function_argument_each)), char!(')'))
);
named!(parse_function_argument_each<CompleteStr, (Expression, Option<Expression>)>,
    pair!(parse_expr , opt!(preceded!(ws!(tag!(":")), parse_expr)))
);