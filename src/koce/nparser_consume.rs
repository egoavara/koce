use std::path::{Path, PathBuf};

use gom::Explorer;
use koce::{Argument, Expression, Parser, ParserData, ParserError, RawData, Sentence, Task, ToSentences, Type, Value, IncompleteTaskMeta};

impl Parser {
    pub fn consume<P: AsRef<Path>, S: ToSentences>(&self, to: P, src: S) -> Result<(), ParserError> {
        for sentence in src.to_sentence()? {
            self.consume_to(
                self.root().follow(to.as_ref()).map_err(|_| ParserError::UnknownPath(PathBuf::from(to.as_ref())))?,
                sentence,
            )?;
        }
        Ok(())
    }
    fn consume_to(&self, to: Explorer<ParserData>, stc: Sentence) -> Result<(), ParserError> {
        match stc {
            Sentence::Define(_, name, desc, imple) => {
                let (name, generics) = consume_local_name(name)?;

                let child = match to.find_child(|x| {
                    use gom::PathMatcher;
                    x.is_matched(name.as_ref())
                }) {
                    Ok(ok) => {
                        // if there is target destination
                        ok
                    }
                    Err(err) => {
                        // TODO if imple is comment(= external)
                        err.add_child(ParserData::NamedVirtual(name))
                    }
                };
                let child = if let Some(desc) = *desc {
                    child.add_child(ParserData::Define(Type::from_expression(&desc)?))
                } else {
                    child
                };
                if let Some(stc) = *imple {
                    self.define_consume_to(child, stc);
                }
            }
            Sentence::Library(_, _, _, _) => {}
            Sentence::Constant(_, _, _, _) => {}
            Sentence::Variable(_, name, desc, imple) => {
                let (name, _) = consume_local_name(name)?;
                let child = to.add_child(ParserData::Variable(
                    name,
                    Type::from_expression(&desc.ok_or(ParserError::Unimplemented)?)?,
                ));
                if let Some(some) = *imple {
                    self.var_consume_to(child, some)?;
                }
            }
            Sentence::Layer(_, name, _, imple) => {
                // todo : layer hieracy
                let (name, generics) = consume_local_name(name)?;
                let child = to.add_child(ParserData::Layer(
                    name,
                ));
                for (gname, gcond) in generics {
                    child.add_child(ParserData::Generic(gname, gcond));
                }

                if let Some(some) = *imple {
                    if let Sentence::Block(block) = some {
                        for x in block {
                            self.layer_consume_to(child.clone(), x)?;
                        }
                    } else {
                        self.layer_consume_to(child, some)?;
                    }
                }
            }
            Sentence::Struct(_, name, _, _) => {}
            Sentence::Enum(_, name, _, imple) => {
                let (name, generics) = consume_local_name(name)?;
                let child = to.add_child(ParserData::Enum(
                    name, None, // TODO Defined Enum
                ));
                for (gname, gcond) in generics {
                    child.add_child(ParserData::Generic(gname, gcond));
                }
                if let Some(some) = *imple {
                    if let Sentence::Block(block) = some {
                        for x in block {
                            self.enum_consume_to(child.clone(), x)?;
                        }
                    } else {
                        return Err(ParserError::Unimplemented);
                    }
                }
            }
            Sentence::Function(_, name, desc, imple) => self.util_function(to, name, *desc, *imple, false)?,
            Sentence::Macro(_, _, _) => {}
            Sentence::Comment(_) => {}
            Sentence::Assign(left, right) => {}
            Sentence::Mean(expr) => {}
            Sentence::Lambda(_, _) => {}
            Sentence::If(_, _, _) => {}
            Sentence::Return(expr) => {}
            Sentence::After(_) => {}
            Sentence::Block(lines) => {
                for line in lines {
                    self.consume_to(to.clone(), line);
                }
            }
        };
        Ok(())
    }
    fn var_consume_to(&self, to: Explorer<ParserData>, stc: Sentence) -> Result<(), ParserError> {
        match stc {
            Sentence::Comment(_) => {
                // External variable
                Ok(())
            }
            Sentence::Mean(expr) => {
                // is compile parsable variable check
                to.add_child(ParserData::Raw(RawData::I32(1234)));
                Ok(())
            }
//            Sentence::If(_, _, _) => {},
//            Sentence::Return(_) => {},
//            Sentence::After(_) => {},
//            Sentence::Block(_) => {},
//            Sentence::Lambda(_, _) => {},
            // TODO more specific error
            _ => Err(ParserError::ImplementationFail)
        }
    }
    fn enum_consume_to(&self, to: Explorer<ParserData>, stc: Sentence) -> Result<(), ParserError> {
        match stc {
            Sentence::Comment(_) => Ok(()),
            // TODO Defined Enum
//            Sentence::Assign(_, _) => {},
            Sentence::Mean(expr) => {
                match expr {
                    Expression::Argument(value) => {
                        match value {
                            Value::Name(name) => {
                                to.add_child(ParserData::Field(name, Type::Reference(PathBuf::from("."))));
                                Ok(())
                            }
                            _ => Err(ParserError::EnumSubSymbolError("not allowed argument".to_string()))
                        }
                    }
                    // TODO Argumented Enum Field
                    Expression::Call(name, args) => {
                        if let Expression::Argument(Value::Name(name)) = *name {
                            to.add_child(
                                ParserData::Field(
                                    name,
                                    Type::Complex(
                                        args.into_iter().map(|x| Type::from_expression(&x)).collect::<Result<Vec<Type>, ParserError>>()?
                                    ),
                                )
                            );
                            Ok(())
                        } else {
                            Err(ParserError::EnumSubSymbolError("Argumented Enum must use local name".to_string()))
                        }
                    }
                    _ => Err(ParserError::EnumSubSymbolError("not allowed sentence".to_string()))
                }
            }

//            Sentence::If(_, _, _) => {},
//            Sentence::Return(_) => {},
//            Sentence::After(_) => {},
//            Sentence::Block(_) => {},
//            Sentence::Lambda(_, _) => {},
            // TODO more specific error
            _ => Err(ParserError::EnumSubSymbolError("not allowed sentence".to_string()))
        }
    }
    fn layer_consume_to(&self, to: Explorer<ParserData>, stc: Sentence) -> Result<(), ParserError> {
        match stc {
            Sentence::Comment(_) => Ok(()),
            Sentence::Variable(_, name, desc, _) => { Ok(()) }
            Sentence::Constant(_, name, desc, _) => { Ok(()) }
            Sentence::Function(_, name, desc, imple) => self.util_function(to, name, *desc, *imple, true),
            Sentence::Define(_, name, desc, _) => { Ok(()) }
            _ => Err(ParserError::LayerConditionalSymbolError("not allowed sentence".to_string()))
        }
    }
    fn fn_consume_to(&self, to: Explorer<ParserData>, stc: Sentence) -> Result<(), ParserError> {
        match stc {
            Sentence::Comment(comment) => {
                Ok(())
            },
            Sentence::Assign(left, right) => {
                use koce::ExpressionPath;
                let dst = PathBuf::from_expression(&left).map_err(|x| ParserError::ParsePathError(x))?;
                let mut result = Vec::new();
                convert_expr_tasks(&mut result, right);
                result.push(Task::Store(Argument::Indirect(dst), Argument::Temporary(result.len() - 1)));
                to.add_child(ParserData::Works(result));
                Ok(())
            }

            Sentence::Return(expr) => {
                let mut result = Vec::new();
                convert_expr_tasks(&mut result, expr);
                result.push(Task::Return(Argument::Temporary(result.len() - 1)));
                to.add_child(ParserData::Works(result));
                Ok(())
            }

            Sentence::Block(blocks) => {
                let child = to.add_child(ParserData::Virtual);
                for x in blocks {
                    self.fn_consume_to(child.clone(), x)?;
                }
                Ok(())
            }
            _ => self.consume_to(to, stc)
        }
    }
    fn define_consume_to(&self, to: Explorer<ParserData>, stc: Sentence) -> Result<(), ParserError> {
        match stc {
//            Sentence::Comment(_) => Ok(()),
//            Sentence::Block(_) => Ok(()),
            _ => self.consume_to(to, stc)
        }
    }
    fn util_function(&self, to: Explorer<ParserData>, name: Expression, desc: Option<Expression>, imple: Option<Sentence>, allow_unimplemented: bool) -> Result<(), ParserError> {
        let (name, generics) = consume_local_name(name)?;
        let child = to.add_child(ParserData::Function(
            name,
        ));
        for (gname, gcond) in generics {
            child.add_child(ParserData::Generic(gname, gcond));
        }
        if let Expression::FunctionShape(args, ret) = desc.ok_or(ParserError::Unimplemented)? {
            for (arg_name, arg_type) in args {
                if let Expression::Argument(Value::Name(name)) = arg_name {
                    child.add_child(ParserData::Parameter(name, Type::from_expression(&arg_type)?));
                } else {
                    return Err(ParserError::Unimplemented);
                }
            }
            if let Some(some) = *ret {
                child.add_child(ParserData::Return(
                    Type::from_expression(&some)?
                ));
            }
        } else {
            return Err(ParserError::Unimplemented);
        }
        match imple {
            None => {
                if !allow_unimplemented {
                    return Err(ParserError::Unimplemented);
                }
            }
            Some(imple) => {
                self.fn_consume_to(child, imple)?;
            }
        }
        Ok(())
    }
}

// name, generics
pub fn consume_local_name(expr: Expression) -> Result<(String, Vec<(String, Vec<Type>)>), ParserError> {
    match expr {
        Expression::Argument(value) => {
            match value {
                Value::Name(name) => Ok((name, Vec::new())),
                _ => Err(ParserError::Unimplemented),
            }
        }
        Expression::Cast(a, b) => {
            if let (Expression::Argument(value), Expression::Generic(generics)) = (*a, *b) {
                let name = match value {
                    Value::Name(name) => name,
                    _ => return Err(ParserError::Unimplemented),
                };
                let gens = generics.into_iter().map(|x| {
                    let mut a = add_multiple_to_vectorize(x).ok_or(ParserError::Unimplemented)?;
                    if let Some(Type::Reference(some)) = a.pop(){
                        Ok((format!("{:?}", some), a))
                    }else{
                        Err(ParserError::Unimplemented)
                    }
                }).collect::<Result<Vec<(String, Vec<Type>)>, ParserError>>()?;
                Ok((name, gens))
            } else {
                Err(ParserError::Unimplemented)
            }
        }
        _ => Err(ParserError::Unimplemented),
    }
}

pub fn consume_imple(stc: Sentence) -> (Vec<Sentence>, Vec<Sentence>) {
    let mut ret_work = Vec::new();
    let mut ret_sym = Vec::new();
    match stc {
        Sentence::Define(_, _, _, _) |
        Sentence::Library(_, _, _, _) |
        Sentence::Layer(_, _, _, _) |
        Sentence::Struct(_, _, _, _) |
        Sentence::Function(_, _, _, _) |
        Sentence::Macro(_, _, _) => {
            ret_sym.push(stc);
        }
        Sentence::Constant(_, _, _, _) |
        Sentence::Variable(_, _, _, _) => {
            ret_sym.push(stc.clone());
            ret_work.push(stc);
        }
        Sentence::Block(b) => {
            b.into_iter().for_each(|x| {
                match x {
                    Sentence::Define(_, _, _, _) |
                    Sentence::Library(_, _, _, _) |
                    Sentence::Layer(_, _, _, _) |
                    Sentence::Struct(_, _, _, _) |
                    Sentence::Function(_, _, _, _) |
                    Sentence::Macro(_, _, _) => ret_sym.push(x),
                    Sentence::Constant(_, _, _, _) |
                    Sentence::Variable(_, _, _, _) => {
                        ret_sym.push(x.clone());
                        ret_work.push(x);
                    }
                    Sentence::Block(_) => { unreachable!("inner block not allowed") }
                    _ => ret_work.push(x),
                }
            })
        }
        _ => {
            ret_work.push(stc)
        }
    }
    (ret_work, ret_sym)
}

// () -> temporary index
pub fn convert_expr_tasks(result: &mut Vec<Task>, expr: Expression) -> Argument {
    match expr {
        Expression::Argument(v) => {
            match v {
                Value::Name(name) => {
                    Argument::Indirect(PathBuf::from(name))
                }
                _ => {
                    Argument::Direct(v)
                }
            }
        }
        Expression::Member(a, b) => {
            let (a, b) = (convert_expr_tasks(result, *a), convert_expr_tasks(result, *b));
            result.push(Task::Member(Argument::Temporary(result.len()), a, b));
            Argument::Temporary(result.len() - 1)
        }
//        Expression::Tuple(_) => {},
//        Expression::Array(_) => {},
//        Expression::FunctionShape(_, _) => {},
//        Expression::Generic(_) => {},
//        Expression::Call(_, _) => {},
        Expression::Cast(a, b) => {
            let (a, b) = (convert_expr_tasks(result, *a), convert_expr_tasks(result, *b));
            result.push(
                Task::Incomplete(
                    IncompleteTaskMeta::Cast,
                    Argument::Temporary(result.len()), a, b,
                )
            );
            Argument::Temporary(result.len() - 1)
        },
//        Expression::Pos(_) => {},
//        Expression::Neg(_) => {},
        Expression::Add(a, b) => {
            let (a, b) = (convert_expr_tasks(result, *a), convert_expr_tasks(result, *b));
            result.push(Task::Add(Argument::Temporary(result.len()), a, b));
            Argument::Temporary(result.len() - 1)
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
        _ => {Argument::UnHandled(expr)}
    }
}

fn add_multiple_to_vectorize(expr : Expression) -> Option<Vec<Type>>{
    match expr{
        Expression::Add(a, b) => {
            let mut a = add_multiple_to_vectorize(*a)?;
            a.extend(add_multiple_to_vectorize(*b)?);
            Some(a)
        },
        _ => {
            Some(vec![Type::from_expression(&expr).ok()?])
        }
    }
}