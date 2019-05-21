use num::traits::cast::ToPrimitive;

use koce::ast::{Expression, Value};
use koce::ir::{Code, Description, Frame, KoceRaw, MacroError, MacroResult, MemoryLayout, Path, Symbol, Work};
use koce::strt::TreeGraph;

fn add_lambda_layerdefine<F>(dst: &mut TreeGraph<Symbol>, p: Path, f: F)
    where F : 'static + Fn(&Expression, Option<MacroResult<Expression, Code>>) -> Result<MacroResult<Expression, Code>, MacroError>
{
    dst.append(
        Symbol::Unnamed(
            Frame::new(
                Description::LayerDefine(p),
                Work::Empty,
            )
        )
    ).append(
        Symbol::Unnamed(
            Frame::new(
                Description::Macro,
                Work::Complete(Code::MacroCode(Box::new(f))),
            )
        )
    );
}

pub fn koceprimitive_i32(dst: &mut TreeGraph<Symbol>) {
    let root = dst.append(
        Symbol::Named(
            "i32".to_string(),
            Frame::new(
                Description::Structure(MemoryLayout::I32),
                Work::Complete(Code::BinaryCode(KoceRaw::I32(0))),
            ),
        )
    );
    add_lambda_layerdefine(root, Path::Root.child("core").child("op").child("MacroArgument"), |expr, _| {
        if let Expression::Argument(v) = expr {
            match v {
                Value::Bytes(bts) => {
                    Ok(MacroResult::Modified(Code::BinaryCode(KoceRaw::I32(bts.iter().take(4).fold(0, |res, x| (res << 8) | (*x as i32))))))
                }
                Value::Numeric(n) => Ok(MacroResult::Modified(Code::BinaryCode(KoceRaw::I32(n.to_i32().ok_or(MacroError::PrimitiveExceedRange)?)))),
                _ => Err(MacroError::Fail)
            }
        } else {
            Err(MacroError::Fail)
        }
    });
}