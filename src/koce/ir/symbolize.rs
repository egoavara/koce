use koce::ast::{Expression, Sentence, Value};
use koce::ir::Path;

//pub fn is_valid(stc : &Sentence) -> bool{
//
//}
pub enum SymbolizeFail{
    NotSymbolizable,
}
pub fn check_symbol(stc : &Sentence) -> Result<String, SymbolizeFail>{
    match stc{
//        Sentence::Define(_, _) => {},
//        Sentence::Alias(_, _, _) => {},
        Sentence::Library(_, expr_name, _, _) => {
            if let Expression::Argument(Value::Name(n)) = expr_name{
                Ok(n.clone())
            }else {
                Err(SymbolizeFail::NotSymbolizable)
            }
        },
        Sentence::Constant(_, expr_name, _, _) => {
            if let Expression::Argument(Value::Name(n)) = expr_name{
                Ok(n.clone())
            }else {
                Err(SymbolizeFail::NotSymbolizable)
            }
        },
        Sentence::Variable(_, expr_name, _, _) => {
            if let Expression::Argument(Value::Name(n)) = expr_name{
                Ok(n.clone())
            }else {
                Err(SymbolizeFail::NotSymbolizable)
            }
        },
        Sentence::Layer(_, opt_expr_name, _, _) => {
            match opt_expr_name {
                Some(expr_name) => {
                    if let Expression::Argument(Value::Name(n)) = expr_name{
                        Ok(n.clone())
                    }else {
                        Err(SymbolizeFail::NotSymbolizable)
                    }
                }
                None => {Err(SymbolizeFail::NotSymbolizable)}
            }
        },
        Sentence::Struct(_, opt_expr_name, _, _) => {
            match opt_expr_name {
                Some(expr_name) => {
                    if let Expression::Argument(Value::Name(n)) = expr_name{
                        Ok(n.clone())
                    }else {
                        Err(SymbolizeFail::NotSymbolizable)
                    }
                }
                None => {Err(SymbolizeFail::NotSymbolizable)}
            }
        },
        Sentence::Function(_, opt_expr_name, _, _) => {
            match opt_expr_name {
                Some(expr_name) => {
                    if let Expression::Argument(Value::Name(n)) = expr_name{
                        Ok(n.clone())
                    }else {
                        Err(SymbolizeFail::NotSymbolizable)
                    }
                }
                None => {Err(SymbolizeFail::NotSymbolizable)}
            }
        },
        _ => Err(SymbolizeFail::NotSymbolizable),
    }
}
pub fn check_indirect_symbol(stc : &Sentence) -> Option<Path>{
    match stc{
        Sentence::Define(_, n, _) => {
            Path::from_expr(n)
        },
        _ => None,
    }
}