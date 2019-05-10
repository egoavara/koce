use koce::ast::{Expression, Sentence, Value};
use koce::ast::Sentence::Define;
use koce::ir::{Definition, Path};

//pub fn is_valid(stc : &Sentence) -> bool{
//
//}
#[derive(Debug)]
pub enum SymbolizeFail {
    NotSymbolizable,
    NoDefinition,

    FunctionDefinitionUnmatched,
}

pub fn check_symbol(stc: &Sentence) -> Result<String, SymbolizeFail> {
    match stc {
        Sentence::Constant(_, expr_name, _, _) => {
            if let Expression::Argument(Value::Name(n)) = expr_name {
                Ok(n.clone())
            } else {
                Err(SymbolizeFail::NotSymbolizable)
            }
        }
        Sentence::Variable(_, expr_name, _, _) => {
            if let Expression::Argument(Value::Name(n)) = expr_name {
                Ok(n.clone())
            } else {
                Err(SymbolizeFail::NotSymbolizable)
            }
        }
        Sentence::Layer(_, opt_expr_name, _, _) => {
            match opt_expr_name {
                Some(expr_name) => {
                    if let Expression::Argument(Value::Name(n)) = expr_name {
                        Ok(n.clone())
                    } else {
                        Err(SymbolizeFail::NotSymbolizable)
                    }
                }
                None => { Err(SymbolizeFail::NotSymbolizable) }
            }
        }
        Sentence::Struct(_, opt_expr_name, _, _) => {
            match opt_expr_name {
                Some(expr_name) => {
                    if let Expression::Argument(Value::Name(n)) = expr_name {
                        Ok(n.clone())
                    } else {
                        Err(SymbolizeFail::NotSymbolizable)
                    }
                }
                None => { Err(SymbolizeFail::NotSymbolizable) }
            }
        }
        Sentence::Function(_, opt_expr_name, _, _) => {
            match opt_expr_name {
                Some(expr_name) => {
                    if let Expression::Argument(Value::Name(n)) = expr_name {
                        Ok(n.clone())
                    } else {
                        Err(SymbolizeFail::NotSymbolizable)
                    }
                }
                None => { Err(SymbolizeFail::NotSymbolizable) }
            }
        }
        _ => Err(SymbolizeFail::NotSymbolizable),
    }
}

pub fn check_indirect_symbol(stc: &Sentence) -> Option<Path> {
    match stc {
        Sentence::Library(_, n, _, _) => {
            Path::from_expr(n)
        }
        _ => None,
    }
}
pub fn check_definition(stc: &Sentence) -> Result<Option<Definition>, SymbolizeFail> {
    match stc {
        Sentence::Constant(_, _, bod, _) | Sentence::Variable(_, _, bod, _) => {
            if let Some(d) = AsRef::as_ref(bod) {
                match d {
                    Sentence::Mean(expr) => {
                        let_definition(expr).map_or(
                            Err(SymbolizeFail::NoDefinition),
                            |x| Ok(Some(x)),
                        )
                    }
                    Sentence::Layer(_, _, _, _) => {
                        // TODO
                        unimplemented!()
                    }
                    Sentence::Struct(_, _, _, _) => {
                        // TODO
                        unimplemented!()
                    }
                    Sentence::Function(_, _, _, _) => {
                        // TODO
                        unimplemented!()
                    }
                    Sentence::FunctionShape(_, _) => {
                        // TODO
                        unimplemented!()
                    }
                    _ => {
                        Err(SymbolizeFail::NoDefinition)
                    }
                }
            } else {
                Ok(None)
            }
        }
        Sentence::Define(_, _, bod) => {
            // TODO
            unimplemented!()
        }
        Sentence::Library(_, _, bod, _) => {
            // TODO
            unimplemented!()
        }
        Sentence::Layer(_, _, bod, _) => {
            // TODO
            unimplemented!()
        }
        Sentence::Struct(_, _, bod, _) => {
            // TODO
            unimplemented!()
        }
        Sentence::Function(_, _, bod, _) => {
            if let Some(def) = AsRef::as_ref(bod) {
                match def {
                    Sentence::FunctionShape(args, ret) => {
                        let defret = match ret {
                            None => {
                                Definition::Void
                            }
                            Some(some) => {
                                let_definition(some).ok_or(SymbolizeFail::FunctionDefinitionUnmatched)?
                            }
                        };
                        let mut defargs = Vec::new();
                        for (name, defarg) in args {
                            defargs.push(
                                (
                                    Path::from_expr(name)
                                        .ok_or(SymbolizeFail::FunctionDefinitionUnmatched)?
                                        .is_direct_current_path()
                                        .ok_or(SymbolizeFail::FunctionDefinitionUnmatched)?,
                                    match defarg {
                                        None => { Ok(Definition::Void) }
                                        Some(x) => { let_definition(x).ok_or(SymbolizeFail::FunctionDefinitionUnmatched)}
                                    }?,
                                ),
                            );
                        }
                        Ok(Some(Definition::Function(defargs, Box::new(defret))))
                    }
                    _ => {
                        Err(SymbolizeFail::FunctionDefinitionUnmatched)
                    }
                }
            } else {
                Ok(None)
            }
        }
        _ => {
            Err(SymbolizeFail::NoDefinition)
        }
    }
}
//pub fn recursive_children(stc: &Sentence) -> Vec<&Sentence>{
//    match stc {
//        Sentence::Define(_, _, i) => {},
//        Sentence::Library(_, _, _, _) => {},
//        Sentence::Constant(_, _, _, _) => {},
//        Sentence::Variable(_, _, _, _) => {},
//        Sentence::Layer(_, _, _, _) => {},
//        Sentence::Struct(_, _, _, _) => {},
//        Sentence::Function(_, _, _, _) => {},
//        Sentence::Comment(_) => {},
//        Sentence::Assign(_, _) => {},
//        Sentence::Mean(_) => {},
//        Sentence::Lambda(_, _) => {},
//        Sentence::If(_, _, _) => {},
//        Sentence::FunctionShape(_, _) => {},
//        Sentence::Return(_) => {},
//        Sentence::After(_) => {},
//        Sentence::Block(_) => {},
//    }
//}
pub fn let_definition(expr: &Expression) -> Option<Definition> {
    Path::from_expr(expr).map(|x| {
        if let Some(name) = x.is_direct_current_path() {
            match name.as_str() {
                "void" => { Definition::Void }
                "i8" => { Definition::I8 }
                "i16" => { Definition::I16 }
                "i32" => { Definition::I32 }
                "i64" => { Definition::I64 }
                "iu8" => { Definition::U8 }
                "u16" => { Definition::U16 }
                "u32" => { Definition::U32 }
                "u64" => { Definition::U64 }
                "f32" => { Definition::F32 }
                "f64" => { Definition::F64 }
                _ => { Definition::Shape(x) }
            }
        } else {
            Definition::Shape(x)
        }
    })
}