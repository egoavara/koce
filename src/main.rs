extern crate core;
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;
extern crate hex;
#[macro_use] extern crate nom;
extern crate num;
extern crate petgraph;


mod koce;
mod lpath;
//use koce::ir::{Parser, Path};
//use koce::ast::parse;
use nom::types::CompleteStr;

fn main() {
    let p = lpath::Path::new();
//    let code = "fn main : () -> = {var a : i32 = 0x12345678}";
//    let code = "var a : i32 = -0x12345678";
//    let mut p = Parser::new();
//    p.core();
//    p.parse_str(Path::Root, code).unwrap();
//    println!("Ok)\n{}", p.root());
}