use nom::types::CompleteStr;

use gom::{Explorer, GOM};
use koce::{Implementation, Path, PathError, Expression, PathNode};
use koce::sentence::Sentence;
use koce::symbol::Description;
use koce::symbol::HandlerError;
use koce::symbol::Symbol;
use super::core;

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
    VariableSubSymbolNotAllow,
    VariableMultipleAssignNotAllow,
    NoProperHandlerForExpression,
}

impl Parser {
    pub fn new() -> Self {
        Parser { root: GOM::setup(Symbol::Unnamed(Description::Virtual, Implementation::Empty)) }
    }
}

impl Parser {
    pub fn parse_str(&mut self, s: &str) -> Result<(), ParserError> {
        let (left, res) = super::parse_sentence(CompleteStr(s)).map_err(|_| ParserError::InvalidSyntax("".to_string()))?;
        if left.0.len() > 0 {
            return Err(ParserError::NotConsumeAll(left.0.to_string()));
        }
        self.parse_sentence_to(self.root.explore().unwrap(), res)
    }
    pub fn parse_sentence(&mut self, stc: Sentence) -> Result<(), ParserError> {
        self.parse_sentence_to(self.root.explore().unwrap(), stc)
    }
    pub fn parse_sentence_to(&mut self, to: Explorer<Symbol>, stc: Sentence) -> Result<(), ParserError> {
        match stc {
//            Sentence::Define(_, _, _, _) => {},
//            Sentence::Library(_, _, _, _) => {},
//            Sentence::Constant(_, _, _, _) => {},
            Sentence::Variable(_, name, desc, imple) => {
                let name = Path::from_expression(&name).map_err(|x| ParserError::FailPath(x))?.trim();
                let desc = Path::from_expression(&desc.ok_or(ParserError::InferenceUnavailable)?).map_err(|x| ParserError::FailPath(x))?;
                let imple = parse_var_imple(
                    search(self.root.explore().unwrap(), desc.clone()).ok_or(ParserError::InferenceUnavailable)?,
                    imple.unwrap_or(Sentence::Comment("default".to_string()))
                )?;
                match name.root_local() {
                    None => { Err(ParserError::MustLocalName) }
                    Some(localname) => {
                        to.add_child(
                            Symbol::Named(
                                localname,
                                Description::Memorable(desc),
                                imple,
                            )
                        );
                        Ok(())
                    }
                }
            }
//            Sentence::Layer(_, _, _, _) => {},
//            Sentence::Struct(_, _, _, _) => {},
//            Sentence::Function(_, _, _, _) => {},
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
fn search(src : Explorer<Symbol>, p : Path) -> Option<Explorer<Symbol>>{
    p.iter().fold(Some(src), |res, a|{
        match a {
            PathNode::Current => Some(res?),
            PathNode::Parent => res?.parent().ok(),
            PathNode::Node(name) => res?.find_child(|x|{
                match x.get_name() {
                    None => false,
                    Some(some) => some == name,
                }
            }).ok(),
        }
    })
}
fn search_define_for_layer(src : Explorer<Symbol>, preset : Path) -> Option<Explorer<Symbol>>{
    src.find_child(|x|{
        match x.get_description() {
            Description::Define(test) => test == &preset,
            _ => false
        }
    }).ok()
}
fn parse_var_imple(desc : Explorer<Symbol>, stc: Sentence) -> Result<Implementation, ParserError> {
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
            Err(ParserError::VariableSubSymbolNotAllow)
        }
    }
}
fn handle_expr(desc : Explorer<Symbol>, expr : Expression) -> Result<Expression, ParserError>{
    match &expr {
        Expression::Primitive(_) |
        Expression::Reference(_) => Ok(expr),
        Expression::Argument(_) => {
            use koce::core::handle::ARGUMENT;
            match search_define_for_layer(desc.clone(), ARGUMENT.clone()){
                Some(res) => {
                    if let Implementation::Handler(handle) = res.inside().data.get_implementation(){
                        handle(expr).map_err(|x| ParserError::FailHandle(x))
                    }else{
                        unreachable!("Must be Handler")
                    }
                },
                None => Err(ParserError::NoProperHandlerForExpression),
            }
        },
//        Expression::Tuple(_) => {},
//        Expression::Array(_) => {},
//        Expression::FunctionShape(_, _) => {},
//        Expression::Generic(_) => {},
//        Expression::Call(_, _) => {},
//        Expression::Member(_, _) => {},
//        Expression::Cast(_, _) => {},
//        Expression::Pos(_) => {},
        Expression::Neg(inner0) => {
            use koce::core::handle::NEGATIVE;
            let inner0_handled = handle_expr(desc.clone(), expr)?;
            match search_define_for_layer(desc, NEGATIVE.clone()){
                Some(res) => {
                    if let Implementation::Handler(handle) = res.inside().data.get_implementation(){
                        handle(Expression::Neg(Box::new(inner0_handled))).map_err(|x| ParserError::FailHandle(x))
                    }else{
                        unreachable!("Must be Handler")
                    }
                },
                None => Err(ParserError::NoProperHandlerForExpression),
            }
        },
//        Expression::Add(_, _) => {},
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

impl Parser{
    pub fn use_core(&self){
        use super::core;
        core::i32::load_i32(self.root.explore().unwrap());
    }
}