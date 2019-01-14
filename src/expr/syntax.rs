use std::string::String;


#[derive(Debug, Clone)]
pub enum Word{
    Call(Box<Word>, Vec<Word>),
    Cast(Box<Word>, Box<Word>),
    //
    Pos(Box<Word>),
    Neg(Box<Word>),
    Not(Box<Word>),
    //
    Add(Box<Word>, Box<Word>),
    Sub(Box<Word>, Box<Word>),
    Mul(Box<Word>, Box<Word>),
    Div(Box<Word>, Box<Word>),
    Mod(Box<Word>, Box<Word>),
    LShift(Box<Word>, Box<Word>),
    RShift(Box<Word>, Box<Word>),
    And(Box<Word>, Box<Word>),
    Or(Box<Word>, Box<Word>),
    Xor(Box<Word>, Box<Word>),
    Greater(Box<Word>, Box<Word>),
    GreaterEqual(Box<Word>, Box<Word>),
    Lesser(Box<Word>, Box<Word>),
    LesserEqual(Box<Word>, Box<Word>),
    Equal(Box<Word>, Box<Word>),
    NotEqual(Box<Word>, Box<Word>),
    //
    Reference(String, Option<Box<Word>>),
    Index(Box<Word>, Option<Box<Word>>),
    Tuple(Vec<Word>),
    Array(Vec<Word>),
    Integer(i128),
    Float(f64),
    Literal(String),
}
#[derive(Debug, Clone)]
pub enum Expr {
    // Import(Variable, LocalVariable)
    // Import := 'import' <Variable> ( 'as' <LocalVariable>)?
    Import(Word, Option<Word>),
    // Var(LocalVariable, Type, Word)
    // Val := 'var' <LocalVariable> ( ':' <Type>)? ( '=' <Word>)?
    Var(Word, Option<Word>, Option<Word>),
    // Const(LocalVariable, Type, Word)
    // Const := 'const' <LocalVariable> ( ':' <Type>)? ( '=' <Word>)?
    Const(Word, Option<Word>, Option<Word>),
    // Assign(Variable, Word)
    // Assign := <Variable> '=' <Word>
    Assign(Word, Word),
    // Return(Word)
    // Return := 'return' <Word>
    Return(Word),
    // Block(Vec<Expr>)
    // Block := '{' sep(<Expr>, ';' | '\n' ) '}'
    Block(Vec<Expr>),
    // If(<Condition>, <Do>, <Next>)
    // If := 'if' <Condition> <Do> <Next>
    If(Word, Box<Expr>, Option<Box<Expr>>),
    // ElseIf(<Condition>, <Do>, <Next>)
    // ElseIf := 'else' 'if' <Condition> <Do> <Next>
    ElseIf(Word, Box<Expr>, Option<Box<Expr>>),
    // Else(<Do>)
    // Else := 'else' <Do>
    Else(Box<Expr>),

    // Fn(LocalVariable, Vec<(LocalVariable, Type)>, <Type>, <Expr>)
    // Fn := 'fn' <LocalVariable> '(' (<Type> | <LocalVariable> : <Type>) ( ',' <Type> | <LocalVariable> : <Type>)* ')' ('->' <Type>)? <Expr>
    Fn(Word, Vec<(Word, Word)>, Option<Word>, Box<Expr>),
}


use word::{LocalVariable, Variable, Type, Word};
use nom::types::CompleteStr;
use nom::{multispace0, multispace1};
use nom::{InputTakeAtPosition, IResult, AsChar, ErrorKind};

pub fn ms0<T>(input: T) -> IResult<T, T>
    where
        T: InputTakeAtPosition,
        <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position(|item| {
        let c = item.clone().as_char();
        !(c == ' ' || c == '\t' || c == '\r')
    })
}
/// Recognizes one or more spaces, tabs, carriage returns and line feeds
pub fn ms1<T>(input: T) -> IResult<T, T>
    where
        T: InputTakeAtPosition,
        <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position1(
        |item| {
            let c = item.clone().as_char();
            !(c == ' ' || c == '\t' || c == '\r')
        },
        ErrorKind::MultiSpace,
    )
}

named!(pub Import<CompleteStr, Expr>,
    do_parse!(
        tag!("import") >>
        ms1 >>
        target : Variable >>
        ms1 >>
        alias : opt!(preceded!(ws!(tag!("as")), Type)) >>
        (Expr::Import(target, alias))
    )
);

named!(pub Var<CompleteStr, Expr>,
    do_parse!(
        tag!("var") >>
        ms1 >>
        lc : LocalVariable >>
        ms0 >>
        tp : opt!(preceded!(ws!(tag!(":")), Type)) >>
        wd : opt!(preceded!(ws!(tag!("=")), Word)) >>
        (Expr::Var(lc, tp, wd))
    )
);
named!(pub Const<CompleteStr, Expr>,
    do_parse!(
        tag!("const") >>
        ms1 >>
        lc : LocalVariable >>
        ms0 >>
        tp : opt!(preceded!(ws!(tag!(":")), Type)) >>
        wd : opt!(preceded!(ws!(tag!("=")), Word)) >>
        (Expr::Var(lc, tp, wd))
    )
);
named!(pub Assign<CompleteStr, Expr>,
    do_parse!(
        dst : Variable >>
        ms0 >>
        tag!("=") >>
        ms0 >>
        src : Word >>
        (Expr::Assign(dst, src))
    )
);
named!(pub Return<CompleteStr, Expr>,
    do_parse!(
        tag!("return") >>
        ms1 >>
        result : Word >>
        (Expr::Return(result))
    )
);

named!(pub SingleExpr<CompleteStr, Expr>, alt!(
    Import | Var | Const | Assign | Return | If
));

named!(pub Block<CompleteStr, Expr>,
    do_parse!(
        v : delimited!(
            do_parse!(tag!("{") >> multispace0 >> ()),
            splitBlock,
            tag!("}")
        ) >>
        (Expr::Block(v))
    )
);
//named!(pub splitSingleBlock<CompleteStr, Expr>,
//    SingleExor
//);

named!(splitBlock<CompleteStr, Vec<Expr>>,
    many0!(
        terminated!(SingleExpr, multispace0)
    )
);

named!(pub If<CompleteStr, Expr>,
    do_parse!(
        tag!("if") >>
        ms1 >>
        cond : Word >>
        ms0 >>
        dobl : alt!(SingleExpr| Block) >>
        ms0 >>
        next : opt!(alt!(ElseIf | Else)) >>
        (Expr::If(cond, Box::new(dobl), {
            match next {
                Some(some) => Some(Box::new(some)),
                None => None,
            }
        }))
    )
);
named!(ElseIf<CompleteStr, Expr>,
    do_parse!(
        tag!("else") >>
        ms1 >>
        tag!("if") >>
        ms1 >>
        cond : Word >>
        ms0 >>
        dobl : alt!(SingleExpr| Block) >>
        ms0 >>
        next : opt!(alt!(ElseIf | Else)) >>
        (Expr::ElseIf(cond, Box::new(dobl), {
            match next {
                Some(some) => Some(Box::new(some)),
                None => None,
            }
        }))
    )
);
named!(Else<CompleteStr, Expr>,
    do_parse!(
        tag!("else") >>
        ms1 >>
        dobl : alt!(SingleExpr| Block) >>
        (Expr::Else(Box::new(dobl)))
    )
);