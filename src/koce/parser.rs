use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};

use nom::types::CompleteStr;
use walkdir::WalkDir;

use gom::{Explorer, GOM};
use koce::{Expression, Implementation, ShallowExpression, Value};
use koce::{ExpressionPath, PathError};
use koce::sentence::Sentence;
use koce::symbol::Description;
use koce::symbol::HandlerError;
use koce::symbol::Symbol;

pub struct Parser {
    pub root: GOM<Symbol>,
}

#[derive(Debug)]
pub enum ParserError {
    FailHandle(HandlerError),
    FailPath(PathError),
    InvalidSyntax(String),
    NotConsumeAll(String),
    MustLocalName,
    InferenceUnavailable,
    ImplementsSubSymbolNotAllow,
    VariableMultipleAssignNotAllow,
    NoProperHandlerForExpression,
    UnknownPath(PathBuf),
    UnexpectedExpression,
}

impl Parser {
    pub fn new() -> Self {
        Parser { root: GOM::setup(Symbol::Unnamed(Description::Virtual, Implementation::Empty)) }
    }
}

impl Parser {
    //    pub fn parse_path<P : AsRef<std::path::Path>>(&mut self, to: Explorer<Symbol>, p : P) -> Result<(), ParserError>{
//        fn inner()
//    }
    pub fn parse_str<P: AsRef<Path>>(&mut self, p: P, s: &str) -> Result<(), ParserError> {
        let (left, res) = super::parse_sentence(CompleteStr(s)).map_err(|_| ParserError::InvalidSyntax("".to_string()))?;
        if left.0.len() > 0 {
            return Err(ParserError::NotConsumeAll(left.0.to_string()));
        }
        self.parse_sentence(p, res)
    }
    pub fn parse_sentence<P: AsRef<Path>>(&mut self, p: P, stc: Sentence) -> Result<(), ParserError> {
        let to = search(self.root.explore().unwrap(), p.as_ref()).ok_or(ParserError::UnknownPath(p.as_ref().to_owned()))?;
        self.parse_sentence_by_explorer(to, stc)
    }
    fn parse_sentence_by_explorer(&mut self, to: Explorer<Symbol>, stc: Sentence) -> Result<(), ParserError> {
        match stc {
//            Sentence::Define(_, _, _, _) => {},
//            Sentence::Library(_, _, _, _) => {},
//            Sentence::Constant(_, _, _, _) => {},
            Sentence::Variable(_, name, desc, imple) => {
                let name = PathBuf::from_expression(&name).map_err(|x| ParserError::FailPath(x))?;
                let desc = parse_desc(desc.ok_or(ParserError::InferenceUnavailable)?)?;
                let (imple, sub) = imple.map_or((None, Vec::new()), |x|sub_symbol(x));
                match local_symbol_name(name) {
                    None => { Err(ParserError::MustLocalName) }
                    Some(localname) => {
                        to.add_child(
                            Symbol::Named(
                                localname,
                                desc,
                                Implementation::Unhandled(imple),
                            )
                        );
                        Ok(())
                    }
                }
            }
//            Sentence::Layer(_, _, _, _) => {},
//            Sentence::Struct(_, _, _, _) => {},
            Sentence::Function(_, name, desc, imple) => {
                let name = match name {
                    None => return Err(ParserError::MustLocalName),
                    Some(some) => local_symbol_name(PathBuf::from_expression(&some).map_err(|x| ParserError::FailPath(x))?),
                }.ok_or(ParserError::MustLocalName)?;
                let desc = parse_desc(desc.ok_or(ParserError::InferenceUnavailable)?)?;
                let (imple, sub) = imple.map_or((None, Vec::new()), |x|sub_symbol(x));
                let symfn = to.add_child(
                    Symbol::Named(
                        name,
                        desc,
                        Implementation::Unhandled(imple),
                    )
                );
                for elem in sub {
                    self.parse_sentence_by_explorer(symfn.clone(), elem)?;
                }
                Ok(())
            }
//            Sentence::Comment(_) => {},
//            Sentence::Assign(_, _) => {},
//            Sentence::Mean(_) => {},
//            Sentence::Lambda(_, _) => {},
//            Sentence::If(_, _, _) => {},
//            Sentence::Return(_) => {},
//            Sentence::After(_) => {},
//            Sentence::Block(_) => {},
            _ => Ok(()),
        }
    }
}

fn search(src: Explorer<Symbol>, p: &Path) -> Option<Explorer<Symbol>> {
    p.iter().fold(Some(src), |res, a| {
        match a.to_str() {
            Some("\\") => Some(res?.root()),
            Some("") | Some(".") => Some(res?),
            Some("..") => res?.parent().ok(),
            Some(name) => res?.find_child(|x| {
                match x.get_name() {
                    None => false,
                    Some(some) => some == name,
                }
            }).ok(),
            _ => None,
        }
    })
}

fn search_define_for_layer(src: Explorer<Symbol>, preset: &Path) -> Option<Explorer<Symbol>> {
    src.find_child(|x| {
        match x.get_description() {
            Description::Define(test) => test == &preset,
            _ => false
        }
    }).ok()
}

fn search_define_for_core_handle(src: Explorer<Symbol>, preset: &Path, arg: Expression) -> Result<Expression, ParserError> {
    let src_clone = src.clone();
    src.find_child(|x| {
        match x.get_description() {
            Description::Define(test) => {
                test == preset
            }
            _ => false
        }
    }).map_or_else(|_| {
        Err(ParserError::NoProperHandlerForExpression)
    }, |x| {
        x.child(0).map_or_else(|_| {
            Err(ParserError::NoProperHandlerForExpression)
        }, |x| {
            if let Implementation::Handler(handle) = x.inside().data.get_implementation() {
                handle(src_clone, arg).map_err(|x| ParserError::FailHandle(x))
            } else {
                unreachable!("Must be Handler")
            }
        })
    })
}

fn parse_desc(expr: Expression) -> Result<Description, ParserError> {
    match expr {
        Expression::Argument(_) |
        Expression::Member(_, _) => { Ok(Description::Memorable(PathBuf::from_expression(&expr).map_err(|x| ParserError::FailPath(x))?)) }
        Expression::Reference(_) |
        Expression::Primitive(_) => { unreachable!() }
        Expression::Tuple(_) => { unimplemented!() }
        Expression::Array(_) => { unimplemented!() }
        Expression::FunctionShape(args, ret) => {
            let nargs = args.into_iter().map(|(_, arg_type)| parse_desc(arg_type.unwrap_or(Expression::Argument(Value::Name("void".to_string()))))).
                collect::<Result<Vec<Description>, ParserError>>();

            Ok(Description::Callable(
                nargs?,
                Box::new(parse_desc(ret.unwrap_or(Expression::Argument(Value::Name("void".to_string()))))?),
            ))
        }
        _ => { Err(ParserError::UnexpectedExpression) }
    }
}
//
fn sub_symbol_for_fn(stc: Sentence) -> Result<(Option<Sentence>, Vec<Sentence>), ParserError> {
    match stc {
        Sentence::Define(_, _, _, _) |
        Sentence::Library(_, _, _, _) |
        Sentence::Layer(_, _, _, _) |
        Sentence::Struct(_, _, _, _) |
        Sentence::Function(_, _, _, _) |
        Sentence::Variable(_, _, _, _) |
        Sentence::Constant(_, _, _, _) |
        Sentence::Macro(_, _, _) => Err(ParserError::ImplementsSubSymbolNotAllow),
        Sentence::Block(v) => {

        }
        _ => (Some(stc), Vec::new()),
    }
}

fn parse_var_imple(desc: Explorer<Symbol>, stc: Sentence) -> Result<Implementation, ParserError> {
    match stc {
        Sentence::Assign(_, _) => { Err(ParserError::VariableMultipleAssignNotAllow) }
        Sentence::Comment(_) => {
            // TODO External Raw Data
            unimplemented!()
        }
        Sentence::Mean(expr) => {
            let handled = handle_expr(desc, expr)?;
            // TODO runtime evaluation require
            match handled {
                Expression::Primitive(raw) => Ok(Implementation::Direct(raw)),
                _ => unimplemented!()
            }
        }
//        Sentence::Lambda(_, _) => {}
//        Sentence::If(_, _, _) => {}
//        Sentence::Return(_) => {}
//        Sentence::After(_) => {}
//        Sentence::Block(_) => {}
        _ => {
            Err(ParserError::ImplementsSubSymbolNotAllow)
        }
    }
}

fn separate(stc: Sentence) -> Vec<Sentence> {
    match stc {
        Sentence::Block(v) => v,
        _ => vec![stc],
    }
}

fn handle_expr(desc: Explorer<Symbol>, expr: Expression) -> Result<Expression, ParserError> {
    match expr {
        Expression::Primitive(_) |
        Expression::Reference(_) => Ok(expr),
        Expression::Argument(_) => {
            use koce::predefines::handle::ARGUMENT;
            search_define_for_core_handle(desc.clone(), ARGUMENT.clone(), expr)
        }
//        Expression::Tuple(_) => {},
//        Expression::Array(_) => {},
//        Expression::FunctionShape(_, _) => {},
//        Expression::Generic(_) => {},
//        Expression::Call(_, _) => {},
//        Expression::Member(_, _) => {},
//        Expression::Cast(_, _) => {},
        Expression::Pos(inner0) => {
            use koce::predefines::handle::POSITIVE;
            let inner0_handled = handle_expr(desc.clone(), *inner0)?;
            search_define_for_core_handle(desc.clone(), POSITIVE.clone(), Expression::Pos(Box::new(inner0_handled)))
        }
        Expression::Neg(inner0) => {
            use koce::predefines::handle::NEGATIVE;
            let inner0_handled = handle_expr(desc.clone(), *inner0)?;
            search_define_for_core_handle(desc.clone(), NEGATIVE.clone(), Expression::Neg(Box::new(inner0_handled)))
        }
        Expression::Add(inner0, inner1) => {
            use koce::predefines::handle::ADDITION;
            let inner0 = handle_expr(desc.clone(), *inner0)?;
            let inner1 = handle_expr(desc.clone(), *inner1)?;
            search_define_for_core_handle(desc.clone(), ADDITION.clone(), Expression::Add(Box::new(inner0), Box::new(inner1)))
        }
//        Expression::Sub(_, _) => {},
//        Expression::Mul(_, _) => {},
//        Expression::Div(_, _) => {},
//        Expression::Mod(_, _) => {},
//        Expression::Exp(_, _) => {},
//        Expression::Not(_) => {},
//        Expression::Eq(_, _) => {},
//        Expression::Neq(_, _) => {},
//        Expression::G(_, _) => {},
//        Expression::L(_, _) => {},
//        Expression::Ge(_, _) => {},
//        Expression::Le(_, _) => {},
//        Expression::And(_, _) => {},
//        Expression::Or(_, _) => {},
//        Expression::Xor(_, _) => {},
//        Expression::ShL(_, _) => {},
//        Expression::ShR(_, _) => {},
        _ => unimplemented!()
    }
    // Normal handle
}

fn local_symbol_name<P: AsRef<std::path::Path>>(p: P) -> Option<String> {
    if p.as_ref().components().count() != 1 {
        return None;
    }
    match p.as_ref().components().nth(0) {
        Some(Component::Normal(n)) => {
            Some(n.to_str().unwrap().to_string())
        }
        _ => { None }
    }
}

impl Parser {
    pub fn use_core(&self) {
        use super::predefines;
        predefines::i32::load_i32(self.root.explore().unwrap());
    }
}
