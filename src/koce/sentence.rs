use koce::{Accessor, Expression, parse_expr, parse_accessor};
use nom::types::CompleteStr;
use nom::{multispace0, multispace1, line_ending};

#[derive(Debug, Clone)]
pub enum Sentence {
    Define(Accessor, Expression, Box<Option<Expression>>, Box<Option<Sentence>>),
    // accessor, name, definition, form
    Library(Accessor, Expression, Box<Option<Expression>>, Box<Option<Sentence>>),
    // accessor, name, definition, form
    Constant(Accessor, Expression, Box<Option<Expression>>, Box<Option<Sentence>>),
    // accessor, name, definition, form
    Variable(Accessor, Expression, Box<Option<Expression>>, Box<Option<Sentence>>),
    // accessor, name, definition, form
    Layer(Accessor, Expression, Box<Option<Expression>>, Box<Option<Sentence>>),
    // accessor, name, definition, form
    Struct(Accessor, Expression, Box<Option<Expression>>, Box<Option<Sentence>>),
    Enum(Accessor, Expression, Box<Option<Expression>>, Box<Option<Sentence>>),
    // accessor, name, definition(argument, return), form
    Function(Accessor, Expression, Box<Option<Expression>>, Box<Option<Sentence>>),
    // accessor, name, form
    Macro(Accessor, Option<Expression>, Box<Option<Sentence>>),
    // //~
    Comment(String),
    // <dst> = <src>, <dst> <op>= <src>
    Assign(Expression, Expression),
    // <expression>
    Mean(Expression),
    // <ft:FunctionType> <form>
    Lambda(Box<Sentence>, Box<Sentence>),
    // if <condition> <ok> else <not>
    If(Expression, Box<Sentence>, Box<Option<Sentence>>),
    // return
    Return(Expression),
    //    Match(Expression, Box<Sentence>),
    After(Expression),
    //    Before(Expression),
    //
    Block(Vec<Sentence>),
}


named!(pub parse_sentence_multiple<CompleteStr, Vec<Sentence>>,
    do_parse!(
        result : separated_list!(many1!(alt!(tag!("\r\n") | tag!("\n") | tag!(";"))), parse_sentence) >>
        multispace0 >>
        (result)
    )
);
named!(pub parse_sentence<CompleteStr, Sentence>,
    alt!(
        parse_sentence_comment
        | parse_sentence_constant
        | parse_sentence_variable
        | parse_sentence_library
        | parse_sentence_layer
        | parse_sentence_struct
        | parse_sentence_enum
        | parse_sentence_function
        | parse_sentence_macro
        | parse_sentence_define
        | parse_sentence_block
        | parse_sentence_return
        | parse_sentence_after
        | parse_sentence_if
        | parse_sentence_assign
        | parse_sentence_mean
    )
);

#[inline]
pub fn is_not_space(chr: char) -> bool {
    !(chr == '\r' || chr == '\n')
}

named!(pub parse_sentence_comment<CompleteStr, Sentence>,
    do_parse!(
        tag!("//") >>
        comment : take_while!(is_not_space) >>
        (Sentence::Comment(comment.to_string()))
    )
);
named!(pub parse_sentence_constant<CompleteStr, Sentence>,
    do_parse!(
        accessor : opt!(parse_accessor) >>
        ws!(tag!("const")) >>
        name : ws!(parse_expr) >>
        definition : opt!(preceded!(ws!(tag!(":")), parse_expr)) >>
        assign : opt!(preceded!(ws!(tag!("=")), parse_sentence)) >>
        (Sentence::Constant(accessor.unwrap_or(Accessor::Private), name, Box::new(definition), Box::new(assign)))
    )
);

named!(pub parse_sentence_variable<CompleteStr, Sentence>,
    do_parse!(
        accessor : opt!(parse_accessor) >>
        ws!(tag!("var")) >>
        name : ws!(parse_expr) >>
        definition : opt!(preceded!(ws!(tag!(":")), parse_expr)) >>
        assign : opt!(preceded!(ws!(tag!("=")), parse_sentence)) >>
        (Sentence::Variable(accessor.unwrap_or(Accessor::Private), name, Box::new(definition), Box::new(assign)))
    )
);

// TODO parse_sentence_symboltype ws! to multispace1
named!(pub parse_sentence_library<CompleteStr, Sentence>,
    do_parse!(
        accessor : opt!(terminated!(parse_accessor, multispace1)) >>
        tag!("lib") >>
        multispace1 >>
        name : parse_expr >>
        definition : opt!(preceded!(ws!(tag!(":")), parse_expr)) >>
        assign : opt!(preceded!(ws!(tag!("=")), parse_sentence)) >>
        (Sentence::Library(accessor.unwrap_or(Accessor::Private), name, Box::new(definition), Box::new(assign)))
    )
);

named!(pub parse_sentence_define<CompleteStr, Sentence>,
    do_parse!(
        accessor : opt!(terminated!(parse_accessor, multispace1)) >>
        tag!("def") >>
        multispace1 >>
        name : parse_expr >>
        definition : opt!(preceded!(ws!(tag!(":")), parse_expr)) >>
        assign : opt!(preceded!(ws!(tag!("=")), parse_sentence)) >>
        (Sentence::Define(accessor.unwrap_or(Accessor::Private), name, Box::new(definition), Box::new(assign)))
    )
);

named!(pub parse_sentence_layer<CompleteStr, Sentence>,
    do_parse!(
        accessor : opt!(parse_accessor) >>
        ws!(tag!("layer")) >>
        name : ws!(parse_expr) >>
        definition : opt!(preceded!(ws!(tag!(":")), parse_expr)) >>
        assign : opt!(preceded!(ws!(tag!("=")), parse_sentence)) >>
        (Sentence::Layer(accessor.unwrap_or(Accessor::Private), name, Box::new(definition), Box::new(assign)))
    )
);

named!(pub parse_sentence_struct<CompleteStr, Sentence>,
    do_parse!(
        accessor : opt!(parse_accessor) >>
        ws!(tag!("struct")) >>
        name : ws!(parse_expr) >>
        definition : opt!(preceded!(ws!(tag!(":")), parse_expr)) >>
        assign : opt!(preceded!(ws!(tag!("=")), parse_sentence)) >>
        (Sentence::Struct(accessor.unwrap_or(Accessor::Private), name, Box::new(definition), Box::new(assign)))
    )
);

named!(pub parse_sentence_enum<CompleteStr, Sentence>,
    do_parse!(
        accessor : opt!(parse_accessor) >>
        ws!(tag!("enum")) >>
        name : ws!(parse_expr) >>
        definition : opt!(preceded!(ws!(tag!(":")), parse_expr)) >>
        assign : opt!(preceded!(ws!(tag!("=")), parse_sentence)) >>
        (Sentence::Enum(accessor.unwrap_or(Accessor::Private), name, Box::new(definition), Box::new(assign)))
    )
);


named!(pub parse_sentence_macro<CompleteStr, Sentence>,
    do_parse!(
        accessor : opt!(parse_accessor) >>
        ws!(tag!("macro")) >>
        name : opt!(ws!(parse_expr)) >>
        assign : opt!(preceded!(ws!(tag!("=")), parse_sentence)) >>
        (Sentence::Macro(accessor.unwrap_or(Accessor::Private), name, Box::new(assign)))
    )
);
named!(pub parse_sentence_function<CompleteStr, Sentence>,
    do_parse!(
        accessor : opt!(parse_accessor) >>
        ws!(tag!("fn")) >>
        name : ws!(parse_expr) >>
        definition : opt!(preceded!(ws!(tag!(":")), parse_expr)) >>
        assign : opt!(preceded!(ws!(tag!("=")), parse_sentence)) >>
        (Sentence::Function(accessor.unwrap_or(Accessor::Private), name, Box::new(definition), Box::new(assign)))
    )
);

named!(pub parse_sentence_block<CompleteStr, Sentence>,
    map!(
        delimited!(
            char!('{'),
            separated_list!(many1!(alt!(tag!("\r\n") | tag!("\n") | tag!(";"))), preceded!(multispace0, parse_sentence)),
            pair!(many0!(alt!(tag!("\r\n") | tag!("\n") | tag!(";") | tag!(" "))), char!('}'))
        ),
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

