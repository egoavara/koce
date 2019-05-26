extern crate core;
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;
extern crate hex;
#[macro_use] extern crate nom;
extern crate num;
extern crate petgraph;
extern crate xml5ever;
#[macro_use] extern crate lazy_static;

use nom::types::CompleteStr;
use xml5ever::tendril::TendrilSink;
use koce::{Path, PathNode, Parser};
use gom::GOM;

mod koce;
mod gom;

fn main() {
//    let code = "fn main : () -> = {var a : i32 = 0x12345678}";
    let code = "var a : i32 = -0x12345678";
    let mut p = Parser::new();
    p.use_core();
    p.parse_str(code).unwrap();
    println!("{}", p.root.explore().unwrap())
}