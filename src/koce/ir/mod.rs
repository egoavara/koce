use koce::ast::{Accessor};

pub mod table;
mod path;
mod symbol;
pub use self::path::Path;
pub use self::symbol::{Symbol, Definition, Implementation};
