use crate::parser;
use crate::lexer;
use crate::lexer::Token;
use crate::bcc;
use crate::bcc::{
    Bytecode,
    Nargs
};

use std::collections::HashSet;
use std::collections::HashMap;

type func = fn(Vec<Token>) -> Option<()>;

// the deffinitions of all standard library functions go here.
pub fn write_line<'input>(lines: &mut Vec<Token>) { //-> Result<Option<Token<'input>>, &'input str> {
    for _ in 0..lines.len() {
        let line = lines.pop().unwrap();
        match line {
            Token::Symbol(output) => print!("{}", output),
            Token::Number(output) => print!("{}", output),
            Token::Str(output) => print!("{}", output),
            _ => {} //return Err("ERROR: on write_line. you can't print that."),
        }
    }
    println!();
    // return Ok(None);
}


pub fn write(lines: &mut Vec<Token>) {
    // println!("write args: {:?}", lines);
    for _ in 0..lines.len() {
        let line = lines.pop().unwrap();
        match line {
            Token::Symbol(output) => print!("{}", output),
            Token::Number(output) => print!("{}", output),
            Token::Str(output) => print!("{}", output),
            _ => {} //return Err("ERROR: on write_line. you can't print that."),
        }
    }
    // return Ok(None);
}

pub fn terpri(_things: &mut Vec<Token>) {
    println!();
    // return Ok(None);
}

pub fn get_std_funcs<'input>() -> HashMap<&'input str, (Nargs, &'input (dyn for<'r, 's> Fn(&'r mut Vec<Token<'s>>)))> {
    let mut std_funcs: HashMap<&'input str, (Nargs, &'input (dyn for<'r, 's> Fn(&'r mut Vec<Token<'s>>) -> _))> = HashMap::new();

    std_funcs.insert("write", (Nargs::INF, &write));
    std_funcs.insert("write-line", (Nargs::INF, &write_line));
    std_funcs.insert("terpri", (Nargs::Num(0), &terpri));

    return std_funcs;
}
