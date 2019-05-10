use koce::ast::{Accessor};

mod table;
mod path;
mod symbol;
mod symbolize;

pub use self::path::Path;
pub use self::table::{Table, Permission};
pub use self::symbol::{Symbol, Definition, Implementation};
pub use self::symbolize::*;
