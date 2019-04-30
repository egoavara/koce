use std::string::String;


#[derive(Debug, Clone)]
pub enum Token {
    Call(Box<Token>, Vec<Token>),
    Cast(Box<Token>, Box<Token>),
    //
    Pos(Box<Token>),
    Neg(Box<Token>),
    Not(Box<Token>),
    //
    Add(Box<Token>, Box<Token>),
    Sub(Box<Token>, Box<Token>),
    Mul(Box<Token>, Box<Token>),
    Div(Box<Token>, Box<Token>),
    Mod(Box<Token>, Box<Token>),
    LShift(Box<Token>, Box<Token>),
    RShift(Box<Token>, Box<Token>),
    And(Box<Token>, Box<Token>),
    Or(Box<Token>, Box<Token>),
    Xor(Box<Token>, Box<Token>),
    Greater(Box<Token>, Box<Token>),
    GreaterEqual(Box<Token>, Box<Token>),
    Lesser(Box<Token>, Box<Token>),
    LesserEqual(Box<Token>, Box<Token>),
    Equal(Box<Token>, Box<Token>),
    NotEqual(Box<Token>, Box<Token>),
    //
    Reference(String, Option<Box<Token>>),
    Index(Box<Token>, Option<Box<Token>>),
    Tuple(Vec<Token>),
    Array(Vec<Token>),
    Integer(i128),
    Float(f64),
    Literal(String),
}
#[derive(Debug, Clone)]
pub enum Expr {
    // Import(Variable, LocalVariable)
    // Import := 'import' <Variable> ( 'as' <LocalVariable>)?
    Import(Token, Option<Token>),
    // Var(LocalVariable, Type, Word)
    // Val := 'var' <LocalVariable> ( ':' <Type>)? ( '=' <Word>)?
    Var(Token, Option<Token>, Option<Token>),
    // Const(LocalVariable, Type, Word)
    // Const := 'const' <LocalVariable> ( ':' <Type>)? ( '=' <Word>)?
    Const(Token, Option<Token>, Option<Token>),
    // Assign(Variable, Word)
    // Assign := <Variable> '=' <Word>
    Assign(Token, Token),
    // <Word>
    JustWord(Token),
    // Return(Word)
    // Return := 'return' <Word>
    Return(Token),
    // Block(Vec<Expr>)
    // Block := '{' sep(<Expr>, ';' | '\n' ) '}'
    Block(Vec<Expr>),
    // If(<Condition>, <Do>, <Next>)
    // If := 'if' <Condition> <Do> <Next>
    If(Token, Box<Expr>, Option<Box<Expr>>),
    // ElseIf(<Condition>, <Do>, <Next>)
    // ElseIf := 'else' 'if' <Condition> <Do> <Next>
    ElseIf(Token, Box<Expr>, Option<Box<Expr>>),
    // Else(<Do>)
    // Else := 'else' <Do>
    Else(Box<Expr>),

    // Fn(LocalVariable, Vec<(LocalVariable, Type)>, <Type>, <Expr>)
    // Fn := 'fn' <LocalVariable> '(' (<Type> | <LocalVariable> : <Type>) ( ',' <Type> | <LocalVariable> : <Type>)* ')' ('->' <Type>)? <Expr>
    Fn(Token, Vec<(Token, Token)>, Option<Token>, Box<Expr>),
}


use super::word::{LocalVariable, Variable, Type, Word};
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

pub fn parse(src : &str) ->Result<Expr, u32>{
    let a = format!("{{\n{inner}\n}}", inner = src);
    let b = a.as_ref();
    match DoExpressions(CompleteStr(b)){
        Ok((_, r)) =>{
            Ok(r)
        }
        Err(e) =>{
            Err(0)
        }
    }
}

named!(pub DoExpressions<CompleteStr, Expr>,
    alt!(Fn | Block | SingleExpr)
);

named!(pub Fn<CompleteStr, Expr>,
    do_parse!(
        tag!("fn") >>
        ms1 >>
        name : LocalVariable >>
        ms0 >>
        args : FnArgs >>
        ms0 >>
        ret: opt!(preceded!(
            do_parse!(tag!("->") >> ms1 >>()),
            Type
        )) >>
        block : Block >>
        (Expr::Fn(name, args, ret, Box::new(block)))
    )
);

named!(FnArgs<CompleteStr, Vec<(Token, Token)>>,
    delimited!(
        tag!("("),
        separated_list!(tag!(","), ws!(FnArgDef)),
        tag!(")")
    )
);

named!(FnArgDef<CompleteStr, (Token, Token)>,
    do_parse!(
        va : opt!(
            terminated!(
                LocalVariable,
                do_parse!(
                    ms0 >>
                    tag!(":")>>
                    ms0 >>
                    ()
                )
            )
        ) >>
        tp : Type >>
        ((va.unwrap_or(Token::Reference("".to_string(), None)), tp))
    )
);

named!(pub Import<CompleteStr, Expr>,
    do_parse!(
        tag!("import") >>
        ms1 >>
        target : Variable >>
        ms0 >>
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
named!(pub JustWord<CompleteStr, Expr>,
    map!(
        Word,
        |x| Expr::JustWord(x)
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
    Import | Var | Const | Assign | Return | If | JustWord
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

named!(splitBlock<CompleteStr, Vec<Expr>>,
    many0!(
        terminated!(DoExpressions, multispace0)
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