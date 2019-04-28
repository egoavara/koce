pub mod parse;
pub mod display;
pub mod errors;

use std::string::String;
use num::bigint::BigUint;

#[derive(Debug)]
pub enum Sentence {
    Constant { name: Expression, datatype: Option<Expression>, assignment: Option<Expression> },
    Variable { name: Expression, datatype: Option<Expression>, assignment: Option<Expression> },
    Library { name: Expression, alias: Expression },
    Interface { name: Expression, definitions: Expression },
    Struct { name: Expression, definitions: Expression },
    Function { name: Expression, definitions: Expression },
    Implements { name: Expression, definitions: Expression },
    Define { name: Expression, definitions: Expression },
    //
    Assign { dst: Expression, src: Expression },
    Mean(Expression),
    If { condition: Expression, case: Box<Sentence>, none_case: Option<Box<Sentence>> },
    ElseIf { condition: Expression, case: Box<Sentence>, none_case: Option<Box<Sentence>> },
    Else(Box<Sentence>),
    Return(Expression),
    Match(Expression, Box<Sentence>),
    Defer(Expression, Box<Sentence>),
    //
    Block(Vec<Sentence>),
}

#[derive(Debug)]
pub enum Expression {
    Argument(Value),
    // control
    Call(Box<Expression>, Box<Expression>),
    // control
    Cast(Box<Expression>, Box<Expression>),
    // control
    Reference(Box<Expression>),
    // control
    Dereference(Box<Expression>),
    // control
    Member(Box<Expression>, Box<Expression>),
    // control
    Wrap(Box<Expression>),
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
    // arithmetic, binary = <Expression> // <Expression>
    Floor(Box<Expression>, Box<Expression>),

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
    Exponential(String),
    Binary(Vec<u8>),
    Numeric(BigUint),
    Hexadecimal(Vec<u8>),
}

