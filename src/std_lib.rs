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

// type func = fn(Vec<Token>) -> Option<()>;

// the deffinitions of all standard library functions go here.

// pub fn eq<'input>(operands: &mut Vec<Token>) -> Result<Option<Token<'input>>, &'input str>{
//     /*
//     takes an infinate number of args returns if theyre all equal or not.
//     work inprogress.
//     */
//     let mut answer = true;
//     for _ in 0..operands.len() {
//         let operand = operands.pop().unwrap();
//         // answer = answer ||
//     }
// }

pub fn not<'input>(truth: Vec<Token>) -> Result<Option<Token<'input>>, &'input str>{
    /*
    takes a boolean val as an arguement returns its negation.
    work inprogress.
    */
    let negation: bool = match truth[0] {
        Token::Bool(val) => !val,
        _ => panic!("cannot negate a non boolean value."),
    };
    return Ok(Some(Token::Bool(negation)));
}

pub fn write_line<'input>(lines: Vec<Token>) -> Result<Option<Token<'input>>, &'input str> {
    write(lines);
    println!();
    return Ok(None);
}


pub fn write<'input>(lines: Vec<Token>) -> Result<Option<Token<'input>>, &'input str> {
    // println!("write args: {:?}", lines);
    let mut loc_lines = lines.clone();
    for _ in 0..loc_lines.len() {
        let line = loc_lines.pop().unwrap();
        match line {
            Token::Symbol(output) => print!("{}", output),
            Token::Number(output) => print!("{}", output),
            Token::Str(output) => print!("{}", output),
            Token::Bool(truth) => print!("{}", match truth {true => "t", false => "nil"}),
            _ => {} //return Err("ERROR: on write_line. you can't print that."),
        }
    }
    return Ok(None);
}

pub fn terpri<'input>(_things: Vec<Token>) -> Result<Option<Token<'input>>, &'input str> {
    println!();
    return Ok(None);
}

pub fn get_std_funcs<'input>() -> HashMap<&'input str, (Nargs, &'input (dyn Fn(Vec<Token<'input>>) -> Result<Option<Token<'input>>, &'input str>))> {
    let mut std_funcs: HashMap<&'input str, (Nargs, &'input (dyn Fn(Vec<Token<'input>>) -> Result<Option<Token<'input>>, &'input str>))> = HashMap::new();

    std_funcs.insert("write", (Nargs::INF, &write));
    std_funcs.insert("write-line", (Nargs::INF, &write_line));
    std_funcs.insert("terpri", (Nargs::Num(0), &terpri));
    std_funcs.insert("not", (Nargs::Num(1), &not));


    return std_funcs;
}
