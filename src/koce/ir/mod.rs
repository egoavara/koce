use core::borrow::Borrow;
use std::ops::Deref;

use nom::types::CompleteStr;

use koce::ir::util::make_local_name;
use koce::ir::Work::NotStart;
use koce::strt::TreeGraph;

pub use self::code::*;
pub use self::macros::*;
pub use self::path::*;
pub use self::symbol::*;

mod code;
mod path;
mod symbol;
mod util;
mod inline;
mod macros;

#[derive(Debug)]
pub enum ParserError {
    ASTFail(String),
    ASTIncomplete(String),
    MustLocalName(Expression),
    Undefined(Path),
    RequireSymbol(Sentence),
    InvalidDescription,
    PathRuleViolation,
    InvalidFunctionDescription,
    InvalidConstantOrVariableDescription,
    NotInferenced,
    NotImplemented,
    UnmatchedValue(String),
    ModifyCompleteCode,
    NotImplementMacroLayer(String),
    MacroFail(MacroError),
    UnknownType(Path),
    Unexpected,
    Unimplemented,
    VariableCodeSymbolNotAllow,
    VariableDefinitionUnknown,
    VariableCodeReturnNotAllow,
    VariableCodeMultipleAssignNotAllow,
    VariableUnhandlableExpression,
    VariableArgumentMustBeRaw,
}

pub struct Parser {
    e_root: TreeGraph<Symbol>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            e_root: TreeGraph::new(Symbol::Unnamed(Frame::new(Description::Virtual, Work::Empty)))
        }
    }
    pub fn root(&self) -> &TreeGraph<Symbol> {
        &self.e_root
    }
    pub fn core(&mut self){
        inline::koceprimitive_i32(&mut self.e_root)
    }
    pub fn parse_str(&mut self, to: Path, sentence: &str) -> Result<(), ParserError> {
        let (left, stcs) = parse::parse_sentence_multiple(CompleteStr(sentence)).map_err(|x| ParserError::ASTFail(format!("{:?}", x)))?;
        if left.0.len() > 0 {
            Err(ParserError::ASTIncomplete(left.0.to_string()))
        } else {
            for x in stcs {
                self.parse_sentence(to.clone(), x)?;
            }
            Ok(())
        }
    }
    pub fn parse_sentence(&mut self, to: Path, sentence: Sentence) -> Result<&mut TreeGraph<Symbol>, ParserError> {
        let dst = self.e_root.exact_follow_mut(&to).ok_or(ParserError::Undefined(to))?;
        Self::parse_recur_sentence(dst, sentence)
    }
    fn parse_recur_sentence(dst: &mut TreeGraph<Symbol>, sentence: Sentence) -> Result<&mut TreeGraph<Symbol>, ParserError> {
        match sentence {
//            Sentence::Define(_, name, optlayer, code) => {
//                // symbol for layer
//                // symbol for redefine
//            },
//            Sentence::Library(_, _, _, _) => {},
//            Sentence::Constant(_, _, _, _) => {},
            Sentence::Variable(_, name, desc, code) => {
                let temp = dst.append(Symbol::Named(
                    Path::from_expression(&name)?.get_local().ok_or(ParserError::MustLocalName(name))?,
                    Frame::new(
                        Description::from_expression(&desc.ok_or(ParserError::Unimplemented)?)?,
                        match *code {
                            None => Work::Empty,
                            Some(_) => Work::NotStart,
                        },
                    ),
                ));
                if let Some(code) = *code {
                    Self::parse_recur_sentence_var(temp, code)?;
                };
                Ok(temp)
            }
//            Sentence::Layer(_, _, _, _) => {},
//            Sentence::Struct(_, _, _, _) => {},
//            Sentence::Function(_, _, _, _) => {},
//            Sentence::Comment(_) => {},
//            Sentence::Assign(_, _) => {},
            Sentence::Mean(mean) => {
                Ok(dst)
            }
//            Sentence::Lambda(_, _) => {},
//            Sentence::If(_, _, _) => {},
//            Sentence::Return(_) => {},
//            Sentence::After(_) => {},
//            Sentence::Block(_) => {},
            _ => {
                // TODO
                unimplemented!()
            }
        }
    }
    fn parse_recur_sentence_var(dst: &mut TreeGraph<Symbol>, sentence: Sentence) -> Result<(), ParserError> {
        match sentence {
            Sentence::Mean(mean) => {
                fn recur_macro(dst: &mut TreeGraph<Symbol>, mean: &Expression) -> Result<MacroResult<Expression, Code>, ParserError> {
                    match mean {
                        Expression::Argument(_) => {
                            if let Description::Memorable(ref p) = dst.data.get_frame().description{
                                match Parser::find_layerdefine(dst.follow(p).ok_or(ParserError::UnknownType(p.clone()))?, Path::Root.child("core").child("op").child("MacroArgument")){
                                    None => {
                                        Err(ParserError::VariableUnhandlableExpression)
                                    },
                                    Some(somefn) => {
                                        somefn(mean, None).map_err(|x|ParserError::MacroFail(x))
                                    },
                                }
                            }else{
                                Err(ParserError::VariableDefinitionUnknown)
                            }
                        },
                        Expression::Neg(data) => {
                            let prev_result = recur_macro(dst, data.as_ref())?;
                            if let Description::Memorable(ref p) = dst.data.get_frame().description{
                                match Parser::find_layerdefine(dst.follow(p).ok_or(ParserError::UnknownType(p.clone()))?, Path::Root.child("core").child("op").child("MacroNeg")){
                                    None => {
                                        Err(ParserError::VariableUnhandlableExpression)
                                    },
                                    Some(somefn) => {
                                        somefn(&mean, Some(prev_result)).map_err(|x|ParserError::MacroFail(x))
                                    },
                                }
                            }else{
                                Err(ParserError::VariableDefinitionUnknown)
                            }
                        }
                        _ => Err(ParserError::NotImplemented)
                    }
                };
                match recur_macro(dst, &mean)? {
                    MacroResult::Switching(s) => {
                        dst.data.get_frame_mut().block = Work::Incomplete(s);
                    },
                    MacroResult::Modified(m) => {
                        dst.data.get_frame_mut().block = Work::Complete(m);
                    },
                }
                Ok(())
            },
//            Sentence::Lambda(_, _) => {
//                // TODO
//                unimplemented!()
//            },
//            Sentence::Block(v) => {
//                // TODO
//                unimplemented!()
//            },
//            Sentence::Comment(_) => {
//                // TODO
//                unimplemented!()
//            },
//            Sentence::If(_, _, _) => {
//                // TODO
//                unimplemented!()
//            },
            Sentence::Return(_) | Sentence::After(_) => {
                Err(ParserError::VariableCodeReturnNotAllow)
            },
            Sentence::Assign(_, _) => {
                Err(ParserError::VariableCodeMultipleAssignNotAllow)
            },
            _ =>{
                Err(ParserError::VariableCodeSymbolNotAllow)
            }
        }
    }

    fn find_layerdefine(from :&TreeGraph<Symbol>, def : Path) -> Option<&Box<dyn Fn(&Expression, Option<MacroResult<Expression, Code>>) -> Result<MacroResult<Expression, Code>, MacroError>>>{
        if let Work::Complete(Code::MacroCode(ref bx)) = from.children().iter().find(|x|{
            if let Description::LayerDefine(ref p) = x.data.get_frame().description{
                *p == def
            }else {
                false
            }
        })?.child(0).unwrap().data.get_frame().block{
            Some(bx)
        }else{
            None
        }
    }
}

impl Description {
    fn from_expression(expr: &Expression) -> Result<Self, ParserError> {
        match expr {
//            Expression::Tuple(_) => {},
//            Expression::Array(_) => {},
            Expression::FunctionShape(args, ret) => {
                let dargs: Result<Vec<Path>, ParserError> = args.iter().map(|(name, def)| {
                    match def {
                        None => {
                            Err(ParserError::Unimplemented)
                        }
                        Some(some) => {
                            Ok(Path::from_expression(some)?)
                        }
                    }
                }).collect();
                let ret = match ret.as_ref() {
                    None => {
                        Path::Current.child("void")
                    }
                    Some(some) => {
                        Path::from_expression(some)?
                    }
                };
                Ok(Description::Callable(dargs?, ret))
            }
//            Expression::Generic(_) => {},
            _ => {
                Path::from_expression(expr).map(|x| { Description::Memorable(x) })
            }
        }
    }
}



impl TreeGraph<Symbol> {
    pub fn follow(&self, p: &Path) -> Option<&TreeGraph<Symbol>> {
        if p.is_absolute() {
            self.exact_follow(p)
        } else {
            self.inner_follow(p)
        }
    }
    fn inner_follow(&self, p: &Path) -> Option<&TreeGraph<Symbol>> {
        match self.parent() {
            None => { self.exact_follow(p) }
            Some(some) => {
                some.inner_follow(p).or_else(|| self.exact_follow(p))
            }
        }
    }
    pub fn exact_follow(&self, p: &Path) -> Option<&TreeGraph<Symbol>> {
        match p {
            Path::Root => {
                Some(self.root())
            }
            Path::Current => {
                Some(self)
            }
            Path::Child(prev, value) => {
                self.exact_follow(prev.deref()).map(|x| {
                    x.children().into_iter().find(|x| {
                        match x.data {
                            Symbol::Named(ref name, _) => { value == name }
                            Symbol::Unnamed(_) => { false }
                        }
                    })
                })?
            }
            Path::Temporary(prev, value) => {
                self.exact_follow(prev.deref()).map(|x| {
                    x.children().into_iter().filter(|x| {
                        match x.data {
                            Symbol::Named(_, _) => { false }
                            Symbol::Unnamed(_) => { true }
                        }
                    }).nth(*value)
                })?
            }
        }
    }
    pub fn exact_follow_mut(&mut self, p: &Path) -> Option<&mut TreeGraph<Symbol>> {
        match p {
            Path::Root => {
                Some(self.root_mut())
            }
            Path::Current => {
                Some(self)
            }
            Path::Child(prev, value) => {
                self.exact_follow_mut(prev.deref()).map(|x| {
                    x.children_mut().into_iter().find(|x| {
                        match x.data {
                            Symbol::Named(ref name, _) => { value == name }
                            Symbol::Unnamed(_) => { false }
                        }
                    })
                })?
            }
            Path::Temporary(prev, value) => {
                self.exact_follow_mut(prev.deref()).map(|x| {
                    x.children_mut().into_iter().filter(|x| {
                        match x.data {
                            Symbol::Named(_, _) => { false }
                            Symbol::Unnamed(_) => { true }
                        }
                    }).nth(*value)
                })?
            }
        }
    }
}