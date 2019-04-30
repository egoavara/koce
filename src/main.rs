#[macro_use] extern crate nom;
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;
extern crate num;
extern crate hex;
extern crate core;
//extern crate llvm_sys as llvm;

use nom::types::CompleteStr;

mod expr;
mod koce;

fn main() {
//    let a = "1";
//    println!("{:?}", koce::ast::parse::parse_expr(CompleteStr(a)));
//    let t = "( 1, \"hello\" )";
//    println!("{:?}", koce::ast::parse::parse_expr(CompleteStr(t)));
//    let a = "[ 1, \"hello\" ]";
//    println!("{:?}", koce::ast::parse::parse_expr(CompleteStr(a)));
//    let ta = "( ( 1, 2, 3, 4), \"hello\")";
//    println!("{:?}", koce::ast::parse::parse_expr(CompleteStr(ta)));
    let ta = "-fib@<T + Reader - Writer>()";
    println!("{:?}", koce::ast::parse::parse_expr(CompleteStr(ta)));
    let m = "1 + -2 * -3 ** -math.fib(1 << 2)";
    println!("{:?}", koce::ast::parse::parse_expr(CompleteStr(m)));
    let acc = "pkg";
    println!("{:?}", koce::ast::parse::parse_accessor(CompleteStr(acc)));
//    let s_const = "const a : T";
//    println!("{:?}", koce::ast::parse::parse_sentence_constant(CompleteStr(s_const)));



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