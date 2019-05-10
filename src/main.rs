#[macro_use] extern crate nom;
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;
extern crate num;
extern crate hex;
extern crate core;
extern crate petgraph;
//extern crate llvm_sys as llvm;

use nom::types::CompleteStr;
use std::fs::File;
use std::str;
mod koce;

use koce::ir::{Path, Symbol, Implementation, Definition, Table};
use std::io::{Read, Cursor, BufReader};
use nom::AsBytes;

fn main() {
    let mut f = File::open("./koce_examples/exkoce_00.koce").unwrap();
    let mut v = Vec::new();
    f.read_to_end(&mut v).unwrap();
    let temp =  str::from_utf8(v.as_bytes()).unwrap();
    let (left, stc) = koce::ast::parse::parse_sentence_multiple(CompleteStr(temp)).unwrap();
    println!("Left : {}", left);
    println!("Sentence : {:?}", stc);

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