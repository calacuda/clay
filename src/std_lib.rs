/*
TODO's:
* add a raw_write and a raw_write_line functions. will be the same as write and write_lines, but
will print with out the space in between the tokens.
*/

use crate::bcc;
use crate::bcc::{Bytecode, Nargs};
// use crate::lexer;
// use crate::lexer::Token;
use crate::parser;
use clay_lib::Token;

use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::path::Path;

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

pub fn from<'a>(imports: &Vec<Token>) -> Result<Option<Token>, &'a str> {
    /*
    (from LIB `(FUNC ARG_1 ... ARG_n))
    loads LIB.so, calls FUNC form LIB.so with ARGS 1 through n.
    FUNC return val must be in the form of a string like this "42=str", or "42=num". this
    will be translated into a Str Token or a number token respectively.
     */
    let site_packs = format!(
        "{}/.local/lib/clay/site-packages/",
        env::var("HOME").unwrap()
    );
    let cur_dir = env::current_dir().unwrap();
    // println!("{:?}", &imports[1]);
    let mut lib_name = match &imports[1] {
        Token::Symbol(l_name) => l_name.to_string(),
        Token::Str(l_name) => l_name.to_string(),
        _ => panic!("I lib name must be a symbol or string, can't be other data literal"),
    };

    if lib_name.contains('~') {
        lib_name = lib_name.replace("~", env::var("HOME").unwrap().as_str().as_ref());
    }

    let pack_path = Path::new(&site_packs).join(Path::new(&lib_name));
    // .as_os_str();

    let cur_path = Path::new(cur_dir.as_os_str()).join(Path::new(&lib_name));
    //.as_os_str();

    let full_path = Path::new(&lib_name);

    let data = match &imports[0] {
        Token::Form(atoms) => atoms,
        _ => panic!("a form must come after the library name."),
    };

    let func_name = match &data[0] {
        Token::Symbol(lib_name) => lib_name,
        Token::Str(lib_name) => lib_name,
        _ => panic!("I func name must be a symbol or string, can't be other data literal"),
    };

    // println!("cur_path :  {:?}", &cur_path);
    // println!("full_path :  {:?}", &full_path);
    // println!("pack_path :  {:?}", &pack_path);

    let lib_path = if cur_path.exists() {
        // println!("cur_path");
        cur_path.as_os_str()
    } else if full_path.exists() {
        // println!("full_path");
        full_path.as_os_str()
    } else if pack_path.exists() {
        // println!("pack_path");
        pack_path.as_os_str()
    } else {
        panic!("the requested module is not available");
    };

    let args = make_f_args(data[1..].to_vec());

    unsafe {
        // let lib = libloading::Library::new(libloading::library_filename(lib_path)).unwrap();
        let lib = libloading::Library::new(lib_path).unwrap();
        let func: libloading::Symbol<
            unsafe extern "C" fn(
                Vec<(String, String)>,
            ) -> Result<Option<(String, String)>, &'a str>,
        > = lib.get(func_name.as_bytes()).unwrap();
        let tok: Token;
        // println!("{:?}", args);
        let raw_out = match func(args) {
            Ok(Some(val)) => val,
            Ok(None) => return Ok(None),
            Err(error) => panic!("{:?}", error),
        };
        match raw_out.1.as_str() {
            "str" => tok = Token::Str(raw_out.0),
            "symbol" => tok = Token::Symbol(raw_out.0),
            "bool" => tok = Token::Bool(make_bool(&raw_out.0)),
            "num" => tok = Token::Number(raw_out.0),
            ty => panic!("you cant return the datatype, {}", ty),
        };

        Ok(Some(tok))
    }
}

fn make_f_args(clay_in: Vec<Token>) -> Vec<(String, String)> {
    let mut rust_args: Vec<(String, String)> = Vec::new();
    for arg in clay_in.iter() {
        match arg {
            Token::Str(data) => rust_args.push((data.to_owned(), "str".to_string())),
            Token::Number(data) => rust_args.push((data.to_owned(), "num".to_string())),
            Token::Bool(data) => {
                if data.to_owned() {
                    rust_args.push(("t".to_string(), "bool".to_string()))
                } else {
                    rust_args.push(("nil".to_string(), "bool".to_string()))
                }
            }
            Token::Form(_) => panic!("you can't pass Forms as arguments to external libs yet"),
            Token::Symbol(_) => {
                panic!("you can't pass Symbols as arguments to external libs yet")
            }
            _ => panic!("syntax error, make sure your arguments are formated corectly."),
        }
    }
    return rust_args;
}

fn make_bool(input: &str) -> bool {
    match input {
        "t" => true,
        "nill" => false,
        _ => panic!("external library function returned a not boolean but clamed it as a bool."),
    }
}

pub fn not<'a>(truth: &Vec<Token>) -> Result<Option<Token>, &'a str> {
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

pub fn write_line<'a>(lines: &Vec<Token>) -> Result<Option<Token>, &'a str> {
    let _ = write(lines);
    println!();
    return Ok(None);
}

pub fn write<'a>(lines: &Vec<Token>) -> Result<Option<Token>, &'a str> {
    // println!("write args: {:?}", lines);
    let mut loc_lines = lines.clone();
    let last_i = loc_lines.len();
    for i in 0..last_i {
        let line = loc_lines.pop().unwrap();
        print_line(line);
        if i != last_i - 1 {
            print!(" ");
        }
    }
    return Ok(None);
}

fn print_line(line: Token) {
    match line {
        Token::Symbol(output) => print!("{}", output),
        Token::Number(output) => print!("{}", output),
        Token::Str(output) => print!("{}", output),
        Token::Form(form) => print_form(*form),
        Token::Bool(truth) => print!(
            "{}",
            match truth {
                true => "t",
                false => "nil",
            }
        ),
        _ => {} //return Err("ERROR: on write_line. you can't print that."),
    }
}

fn print_form(form: Vec<Token>) {
    let mut loc_form = form.clone();
    loc_form.reverse();
    print!("`(");
    let _ = write(&loc_form);
    print!(")");
}

pub fn terpri<'a>(_things: &Vec<Token>) -> Result<Option<Token>, &'a str> {
    println!();
    return Ok(None);
}

pub fn get_std_funcs<'a>() -> HashMap<
    &'a str,
    (
        Nargs,
        &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
    ),
> {
    let mut std_funcs: HashMap<
        &'a str,
        (
            Nargs,
            &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
        ),
    > = HashMap::new();

    std_funcs.insert("write", (Nargs::INF, &write));
    std_funcs.insert("write-line", (Nargs::INF, &write_line));
    std_funcs.insert("terpri", (Nargs::Num(0), &terpri));
    std_funcs.insert("not", (Nargs::Num(1), &not));
    std_funcs.insert("from", (Nargs::Num(2), &from));

    return std_funcs;
}
