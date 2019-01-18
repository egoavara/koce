#[macro_use] extern crate nom;
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;
extern crate regex;
extern crate hex;
extern crate num;
extern crate llvm_sys;

mod expr;
use std::fs::File;
use std::io::prelude::*;
use nom::types::CompleteStr;
use expr::util::*;
use expr::word;

fn main() {
    let mut f = File::open("example/example_00.koce").expect("file not found");
    llvm_sys::support::
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    match expr::syntax::parse(contents.as_ref()) {
        Ok(ok) => {
            println!("value : {:?}", ok);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}