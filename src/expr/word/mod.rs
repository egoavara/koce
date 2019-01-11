mod mod_value;
mod mod_unary;
mod mod_binary;
mod mod_other;

pub use self::mod_value::*;
pub use self::mod_unary::*;
pub use self::mod_binary::*;
pub use self::mod_other::*;

use super::syntax::Word;
use nom::{multispace0};
use nom::types::CompleteStr;
use super::util::{naming};

named!(pub Word<CompleteStr, Word>, call!(binary));
named!(pub Type<CompleteStr, Word>, call!(typeref));
named!(pub LocalVariable<CompleteStr, Word>, map!(naming, |x|Word::Reference(x.0.to_string(), None)));
named!(pub Variable<CompleteStr, Word>, call!(reference));