use crate::parser;
use crate::parser::{
    lexer,
    lexer::Token
};
use crate::bcc;
use crate::bcc::{
    Bytecode,
    Nargs
};

use std::collections::HashSet;
use std::collections::HashMap;

// the deffinitions of all standard library functions go here.


pub fn get_std_funcs<'input>() -> HashMap<&'input str, Nargs> {
    let mut std_funcs = HashMap::new();

    std_funcs.insert("write", Nargs::INF);
    std_funcs.insert("terpri", Nargs::Num(1));

    return std_funcs;
}
