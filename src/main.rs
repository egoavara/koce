
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;
extern crate hex;
#[macro_use] extern crate nom;
extern crate num;
extern crate petgraph;
extern crate xml5ever;
#[macro_use] extern crate lazy_static;
extern crate walkdir;

mod koce;
mod gom;

use nom::types::CompleteStr;
use xml5ever::tendril::TendrilSink;
use gom::{GOM, IterRule, Explorer};
use std::path::{Path, PathBuf};
use koce::{Parser, ParserData, ToSentences};
use std::fs::File;


fn main() {
    let mut cc0 = File::open("./lib/core/compile/_enums.koce").unwrap();
    let mut ch0 = File::open("./lib/core/handle/_layers.koce").unwrap();
    let mut cii32 = File::open("./lib/core/int/i32.koce").unwrap();
    let mut ex0 = File::open("./koce_examples/exkoce_00.koce").unwrap();
    let par = koce::Parser::new();
    par.consume("/", koce::read_to_string(&mut cc0).unwrap()).unwrap();
    par.consume("/", koce::read_to_string(&mut ch0).unwrap()).unwrap();
    par.consume("/", koce::read_to_string(&mut cii32).unwrap()).unwrap();
    par.consume("/", koce::read_to_string(&mut ex0).unwrap()).unwrap();
//    println!("{:?}", koce::parse_sentence_define(CompleteStr(cii32.as_str())));
    println!("{}", par.root());
}