extern crate core;
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;
extern crate hex;
#[macro_use] extern crate nom;
extern crate num;
extern crate petgraph;
extern crate xml5ever;

//use koce::ir::{Parser, Path};
//use koce::ast::parse;
use nom::types::CompleteStr;
use xml5ever::tendril::TendrilSink;

mod koce;
mod gom;

fn main() {
    let mut t = gom::GOM::setup("Hello");
    println!("{:?}", t);
    let a = t.explore().unwrap().add_child("no.0");
    a.add_child("no.0.0");
    a.add_child("no.0.1");
    t.explore().unwrap().add_child("no.1");
    t.explore().unwrap().add_child("no.2");
    t.explore().unwrap().add_child("no.3");

//    let code = "fn main : () -> = {var a : i32 = 0x12345678}";
//    let code = "var a : i32 = -0x12345678";
//    let mut p = Parser::new();
//    p.core();
//    p.parse_str(Path::Root, code).unwrap();
//    println!("Ok)\n{}", p.root());
}