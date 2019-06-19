//pub mod ast;
//pub mod ir;
//pub mod strt;

mod accessor;
mod sentence;
mod expression;
mod value;
mod path;
mod nparser;
mod nparser_consume;
mod cores;

pub use self::accessor::*;
pub use self::sentence::*;
pub use self::expression::*;
pub use self::value::*;
pub use self::path::*;
pub use self::nparser::*;
pub use self::nparser_consume::*;


use std::io::Read;

pub fn read_to_string<T : Read>(t : &mut T) -> Option<String>{
    let mut temp : String = "".to_string();
    t.read_to_string(&mut temp).ok()?;
    Some(temp)
}