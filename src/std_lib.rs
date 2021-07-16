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
use std::path::Path;
use std::env;

use libloading;

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

pub fn from<'input>(imports: Vec<Token>) -> Result<Option<Token>, &'input str> {
    /*
    (from LIB `(FUNC ARG_1 ... ARG_n))
    loads LIB.so, calls FUNC form LIB.so with ARGS 1 through n.
    FUNC return val must be in the form of a string like this "42=str", or "42=num". this 
    will be translated into a Str Token or a number token respectively.
     */
    let site_packs = format!("{}/.local/lib/clay/site-packages/", env::var("HOME").unwrap());
    let cur_dir = env::current_dir().unwrap();
    let mut pack_path = Path::new(&site_packs);
    let lib_name = match imports[0] {
	Token::Symbol(lib_name) => lib_name,
	Token::Str(lib_name) => lib_name,
	_ => panic!("I lib name must be a symbol or string, can't be other data literal"),
    };

    pack_path.join(Path::new(&lib_name));
    
    let mut cur_path = Path::new(cur_dir.as_os_str());
    cur_path.join(Path::new(&lib_name));
    let func_name = match imports[1] {
	Token::Symbol(lib_name) => lib_name,
	Token::Str(lib_name) => lib_name,
	_ => panic!("I func name must be a symbol or string, can't be other data literal"),
    };

    let lib_path =
	if cur_path.exists() {
	    cur_path
	} else if pack_path.exists() {
	    pack_path
	} else {
	    panic!("the requested module is not available");
	};
    
    unsafe {
        let lib = libloading::Library::new(libloading::library_filename(lib_path)).unwrap();
        let func: libloading::Symbol<unsafe extern fn(Vec<Token>) -> String> = lib.get(func_name
										       .as_bytes())
	    .unwrap();
	let raw_out = func(imports[2..].to_vec());
	let (output, ty) = match raw_out.rsplit_once('=') {
	    Some(something) => something,
	    _ => panic!("external function lacks either data or data type.")
	};
	//output = ;
	Ok(Some(make_tok(output.to_string(), ty)))	
    }
}

fn make_tok(output: String, ty: &str) -> Token {
    match ty {
	    "str" => Token::Str(output),
	    "STR" => Token::Str(output),
	    "bool" => Token::Bool(make_bool(&output)),
	    "BOOL" => Token::Bool(make_bool(&output)),
	    "symbol" => Token::Symbol(output),
	    "SYMBOL" => Token::Symbol(output),
	    "num" => Token::Number(output),
	    "NUM" => Token::Number(output),
	    _ => panic!("you cant return the datatype, {}", ty)
	}
}

fn make_bool(input: &str) -> bool {
    match input {
	"t" => true,
	"nill" => false,
	_ => panic!("external library function returned a not boolean but clamed it as a bool."),
    }
}

pub fn not<'input>(truth: Vec<Token>) -> Result<Option<Token>, &'input str> {
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


pub fn write_line<'input>(lines: Vec<Token>) -> Result<Option<Token>, &'input str> {
    write(lines);
    println!();
    return Ok(None);
}


pub fn write<'input>(lines: Vec<Token>) -> Result<Option<Token>, &'input str> {
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


pub fn terpri<'input>(_things: Vec<Token>) -> Result<Option<Token>, &'input str> {
    println!();
    return Ok(None);
}


pub fn get_std_funcs<'input>() -> HashMap<String, (Nargs, &'input (dyn Fn(Vec<Token>) -> Result<Option<Token>, &'input str>))> {
    let mut std_funcs: HashMap<String, (Nargs, &'input (dyn Fn(Vec<Token>) -> Result<Option<Token>, &'input str>))> = HashMap::new();

    std_funcs.insert("write".to_string(), (Nargs::INF, &write));
    std_funcs.insert("write-line".to_string(), (Nargs::INF, &write_line));
    std_funcs.insert("terpri".to_string(), (Nargs::Num(0), &terpri));
    std_funcs.insert("not".to_string(), (Nargs::Num(1), &not));


    return std_funcs;
}
