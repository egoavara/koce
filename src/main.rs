#[macro_use] extern crate nom;
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;
extern crate num;
extern crate hex;
extern crate core;
extern crate petgraph;
//extern crate llvm_sys as llvm;

use nom::types::CompleteStr;
use std::cell::RefCell;
use std::rc::Rc;

mod koce;
use koce::ir::{Path, Symbol, Implementation, Definition};

fn main() {
    let a = Path::Root.extends(vec!["a", "b"]);
    println!("{}", a);
    let b = Symbol::new("Foo", None, None);
    println!("{:?}", b);

}

fn test_parse_value(){
    let l0 = CompleteStr("\"Hello, 안녕하살법, 你好\\n\"");
    println!("{}", koce::ast::parse::parse_value(l0).unwrap().1);
    let l1 = CompleteStr("\'Hello, 안녕하살법 받아치기, 你好\\n\'");
    println!("{}", koce::ast::parse::parse_value(l1).unwrap().1);
    let b = CompleteStr("foo");
    println!("{}", koce::ast::parse::parse_value(b).unwrap().1);
    let n = CompleteStr("123456789");
    println!("{}", koce::ast::parse::parse_value(n).unwrap().1);
    let bts_0 = CompleteStr("0x0123_4567_89ab_cdef");
    println!("{}", koce::ast::parse::parse_value(bts_0).unwrap().1);
    let bts_1 = CompleteStr("0b01010_00100010");
    println!("{}", koce::ast::parse::parse_value(bts_1).unwrap().1);
}