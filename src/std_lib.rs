use crate::parser;
use crate::parser::{
    lexer,
    lexer::Token
};
use crate::bcc;
use crate::bcc::Bytecode;

use std::collections::HashSet;

// the deffinitions of all standard library functions go here.


pub fn get_std_funcs<'input>() -> HashSet<&'input str> {
    let mut std_funcs = HashSet::new();
    std_funcs.insert("write");
    std_funcs.insert("terpri");

    return std_funcs;
}
