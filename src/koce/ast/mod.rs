pub mod parse;
pub mod display;
pub mod errors;
pub mod utils;

use std::string::String;
use num::bigint::BigUint;
use nom::types::CompleteStr;

#[derive(Debug)]
pub enum Sentence {
    // <name/namepath>, path
    Library(Expression, Option<Expression>),
    // to, external defines
    Define(Expression, Option<Expression>),
    // accessor, nickname, realname
    Alias(Accessor, Expression, Expression),
    // accessor, name, definition, form
    Module(Accessor, Expression, Option<Box<Sentence>>, Option<Box<Sentence>>),
    // accessor, name, definition, form
    Constant(Accessor, Expression, Option<Box<Sentence>>, Option<Box<Sentence>>),
    // accessor, name, definition, form
    Variable(Accessor, Expression, Option<Box<Sentence>>, Option<Box<Sentence>>),
    // accessor, name, definition, form
    Layer(Accessor, Option<Expression>, Option<Box<Sentence>>, Option<Box<Sentence>>),
    // accessor, name, definition, form
    Struct(Accessor, Option<Expression>, Option<Box<Sentence>>, Option<Box<Sentence>>),
    // accessor, name, definition(argument, return), form
    Function(Accessor, Option<Expression>, (Expression, Expression), Option<Box<Sentence>>),
    // <dst> = <src>, <dst> <op>= <src>
    Assign(Expression, Expression),
    // <expression>
    Mean(Expression),
    // if <condition> <do> else <else>
    If(Expression, Box<Sentence>, Option<Box<Sentence>>),
    // return
    Return(Expression),
    Match(Expression, Box<Sentence>),
    Defer(Expression, Box<Sentence>),
    //
    Block(Vec<Sentence>),
}

#[derive(Debug)]
pub enum Expression {
    Argument(Value),
    // '('
    Tuple(Vec<Expression>),
    // '['
    Array(Vec<Expression>),
    // '<', 예외적으로 제네릭 형 안의 익스프레션은 논리레벨 이상의 표현식이 올 수 없다
    Generic(Vec<Expression>),
    // control = <address>(<args...>)
    Call(Box<Expression>, Vec<Expression>),
    // control = <src>.<dst>
    Member(Box<Expression>, Box<Expression>),
    // control = <from>@<to>
    Cast(Box<Expression>, Box<Expression>),
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

#[derive(Debug)]
pub enum Value {
    Name(String),
    Literal(String),
    Bytes(Vec<u8>),
    Numeric(BigUint),
}


#[derive(Debug)]
pub enum Accessor {
    Public,
    Exclusive,
    Package,
    Private,
}