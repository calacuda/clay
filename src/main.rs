use clay_lib::{Nargs, Token};
use libloading::{Library, Symbol};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::io::stdin;
use std::path::{Path, PathBuf};

// mod bcc;
mod lexer;
mod parser;

enum LangLibrary {
    Compiled(
        HashMap<
            String,
            (
                Nargs,
                String,
                // &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
            ),
        >,
    ),
    Lisp(String),
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum KnownThing {
    // LispFunc(parser::Node),
    LispFunc(clay_lib::Token),
    CompiledFunc((PathBuf, String)), // (library, function name)
    Var(Token),
}

fn read_source(fname: &str) -> String {
    let mut cwd = env::current_dir().unwrap();
    if fname.chars().next().unwrap() == '~'
        || fname.chars().next().unwrap() == '/'
        || fname.chars().next().unwrap() == '\\'
    {
        return read_to_string(fname).unwrap();
    } else {
        cwd.push(fname);
        return read_to_string(cwd).unwrap();
    }
}

/*
fn _test_parser(parserd_sc: &Vec<parser::Node>) {
    println!("Parsed Source Code:\n");
    for child in parserd_sc.iter() {
        println!("===========================");
        println!("Parent:\n{:?}, with nodeID: {}", child.data, child.id.index);
        println!("===========================");
        println!("children of node {}", child.id.index);
        for kid in child.children.iter() {
            // let id = kid.index;
            // for node in parserd_sc.iter() {
            //     if node.id.index == id {
            //         println!("node: {:?}, with nodeID: {}", node.data, node.id.index);
            //         break;
            //     }
            // }
            println!("{:?}, with nodeID: {}", kid.data, kid.id.index);
        }
        println!("\n");
    }
}

fn _test_parser2(node: &parser::Node) {
    println!("Parsed Source Code:\n");

    println!("===========================");
    println!("Parent:\n{:?}, with nodeID: {}", node.data, node.id.index);
    println!("===========================");
    println!("children of node {}", node.id.index);
    for kid in node.children.iter() {
        // let id = kid.index;
        // for node in parserd_sc.iter() {
        //     if node.id.index == id {
        //         println!("node: {:?}, with nodeID: {}", node.data, node.id.index);
        //         break;
        //     }
        // }
        println!("{:?}, with nodeID: {}", kid.data, kid.id.index);
    }
    println!("\n");
}

fn _test_parser3(nodes: &Vec<parser::Node>) {
    println!("Parsed Source Code:\n");
    for node in nodes {
        println!("===========================");
        println!("Parent:\n{:?}, with nodeID: {}", node.data, node.id.index);
        println!("===========================");
        println!("children of node {}", node.id.index);
        for kid in node.children.iter() {
            let id = kid.id;
            for node in nodes.iter() {
                if node.id == id {
                    println!("node: {:?}, with nodeID: {}", node.data, node.id.index);
                    break;
                }
            }
            println!("{:?}, with nodeID: {}", kid.data, kid.id.index);
        }
        println!("\n");
    }
}
*/

// fn test_lexer(lex: &mut lexer::Lexer) {
//     println!("lexer tokens:\n");
//     //for global in bytecode {
//     loop {
//         let tok = lex.get_token();
//         println!("{:?}", tok);
//         match tok {
//             Token::EOF => break,
//             _ => {}
//         }
//     }
// }

fn find_lib(common_name: &String) -> PathBuf {
    let site = format!(
        "{}/.local/lib/clay/site-packages/{}",
        env::var("HOME").unwrap(),
        common_name,
    );
    let site_packs = Path::new(&site);
    let cur_dir = Path::new(env::current_dir().unwrap().as_os_str()).join(Path::new(
        &common_name.replace("~", env::var("HOME").unwrap().as_str().as_ref()),
    ));

    if cur_dir.exists() {
        cur_dir
    } else if site_packs.exists() {
        site_packs.to_path_buf()
    } else {
        panic!("the library <{}> can't be found.", common_name);
    }
}

fn get_lib_contents<'a>(
    location: &PathBuf,
) -> HashMap<
    String,
    (
        Nargs,
        String,
        // &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
    ),
> {
    let funcs = unsafe {
        let lib = Library::new(location).unwrap();
        let func: Symbol<
            fn() -> HashMap<
                String,
                (
                    Nargs,
                    String,
                    // &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
                ),
            >,
        > = lib.get(b"get_funcs").unwrap();
        let funcs = func();
        funcs
    };

    // let func_name = &funcs.get(&"write-line".to_string()).unwrap().1;
    // let mut args = Vec::new();
    // args.push(Token::Str("std_lib".to_string()));
    // args.push(Token::Str("test printer".to_string()));
    //
    // call_comp(
    //     &location.as_os_str().to_str().unwrap().to_string(),
    //     &func_name,
    //     args,
    // );

    return funcs;
}

fn import_comp_lib<'a>(
    known_things: &mut HashMap<String, KnownThing>,
    libs: &mut HashMap<String, LangLibrary>,
    lib_name: &String,
) {
    let location = find_lib(lib_name);
    let lib = get_lib_contents(&location);

    for (func_name, val) in lib.iter() {
        let thing = KnownThing::CompiledFunc((location.clone(), val.1.clone()));
        known_things.insert(func_name.clone(), thing);
    }
    libs.insert(lib_name.clone(), LangLibrary::Compiled(lib));
}

fn import(statement: &Token) {}

fn def_let(
    known_things: &mut HashMap<String, KnownThing>,
    tmp_known_things: &mut HashMap<String, KnownThing>,
    libs: &mut HashMap<String, LangLibrary>,
    s_exp: &Token,
) -> Token {
    let (_, mut vars, code) = match s_exp {
        Token::Form(list) => (
            list[0].clone(),
            match &list[1] {
                Token::Form(l) => l,
                _ => panic!("let takes a list of variables to assign."),
            },
            list[2..].to_vec(),
        ),
        _ => panic!("thats not a let statement!"),
    };

    for var in vars {
        let (var_name, val) = match var {
            Token::Form(list) => {
                let evaled = match list[1] {
                    Token::Form(_) => evaluate(known_things, tmp_known_things, libs, &list[1]),
                    _ => list[1].clone(),
                };

                let name = match &list[0] {
                    Token::Symbol(sym) => sym.clone(),
                    _ => panic!("var name cant be a datatype."),
                };

                (name, evaled)
            }
            _ => panic!("let claus needs to be a form of forms."),
        };

        // println!("name: {:?} --- val: {:?}", var_name, val);
        tmp_known_things.insert(var_name, KnownThing::Var(val));
    }

    // for statement in code {
    //     evaluate(known_things, tmp_known_things, libs, &statement);
    // }
    return call_code(known_things, tmp_known_things, libs, code);
}

fn def(
    known_things: &mut HashMap<String, KnownThing>,
    tmp_known_things: &mut HashMap<String, KnownThing>,
    libs: &mut HashMap<String, LangLibrary>,
    s_exp: &Token,
) {
    /*
     * used to define functons or variables
     */
    // todo: write it!
    match s_exp {
        Token::Form(list) if list[0] == Token::Symbol("let".to_string()) => {
            def_let(known_things, tmp_known_things, libs, s_exp);
            return;
        }
        Token::Form(list) if list[0] == Token::Symbol("defun".to_string()) => {}
        _ => panic!("non form recieved by interpreter function def as s_exp"),
    }

    let thing = KnownThing::LispFunc(Token::Form(match s_exp {
        Token::Form(list) => list[2..].to_vec(),
        _ => panic!("ERROR Will Robinson!"),
    }));
    // println!("{:?}", s_exp);
    let name = match s_exp {
        Token::Form(l) => match &l[1] {
            Token::Symbol(name) => name.clone(),
            _ => panic!("function name can not be a datatype."),
        },
        _ => panic!("ERROR Will Robinson!"),
    };
    // println!("{:?}", thing);
    known_things.insert(name, thing);
}

fn call_code(
    known_things: &mut HashMap<String, KnownThing>,
    tmp_known_things: &mut HashMap<String, KnownThing>,
    libs: &mut HashMap<String, LangLibrary>,
    lisp_code: Vec<Token>,
) -> Token {
    let mut res = Token::Bool(false);

    for statement in lisp_code {
        res = evaluate(known_things, tmp_known_things, libs, &statement);
    }

    return res;
}

fn call_lisp(
    known_things: &mut HashMap<String, KnownThing>,
    tmp_known_things: &mut HashMap<String, KnownThing>,
    libs: &mut HashMap<String, LangLibrary>,
    lisp_code: &Token,
    args: Vec<Token>,
) -> Token {
    /*
     * calls a lisp function.
     */
    // println!("{:?}", lisp_code);
    let (params, algorithm) = match lisp_code {
        Token::Form(list) => {
            let p = match &list[0] {
                Token::Form(l) => l,
                _ => panic!("param list must be a form"),
            };
            let algo = list[1..].to_vec();
            (p, algo)
        }
        _ => panic!("not a lisp function"),
    };

    if args.len() != params.len() {
        panic!("you must have the same number of arguyments as you do parameters");
    }

    for i in 0..args.len() {
        tmp_known_things.insert(
            match &params[i] {
                Token::Symbol(sym) => sym.clone(),
                _ => panic!("parameter must not be a datatype"),
            },
            KnownThing::Var(args[i].clone()),
        );
    }

    call_code(known_things, tmp_known_things, libs, algorithm)
}

fn call_comp<'a>(lib_name: &PathBuf, func_name: &String, args: Vec<Token>) -> Token {
    /*
     * calls a compiled rust/c/golang/whatever function from the
     * .so file stored in lib_name.
     */

    let result = unsafe {
        let lib = Library::new(lib_name).unwrap();
        let func: Symbol<fn(&Vec<Token>) -> Result<Option<Token>, &'a str>> =
            lib.get(func_name.as_bytes()).unwrap();
        func(&args)
    };

    return match result {
        Ok(Some(data)) => data,
        Ok(None) => Token::Bool(false),
        Err(err) => panic!(
            "function: {} form library: {:?} return with error <{}>",
            func_name, lib_name, err
        ),
    };
    // return result;
}

// fn call_func(
//     known_things: &mut HashMap<String, KnownThing>,
//     tmp_known_things: &mut HashMap<String, KnownThing>,
//     libs: &mut HashMap<String, LangLibrary>,
//     func: &Token,
//     args: Vec<Token>,
// ) -> Token {
//     // println!("call_func :  {:?}", func);
//     let f = match func {
//         Token::Symbol(thing) => thing,
//         _ => panic!("function name must be a symbol, not a data type or EOF!"),
//     };
//
//     let thing: KnownThing = match known_things.get(f) {
//         Some(thing) => thing.clone(),
//         None => panic!("that function does not exist."),
//     };
//
//     return match thing {
//         KnownThing::LispFunc(s_exp) => call_lisp(), // evaluate(known_things, libs, &s_exp),
//         KnownThing::CompiledFunc(comp_func) => call_comp(&comp_func.0, &comp_func.1, args),
//         KnownThing::Var(name) => panic!("you cant call a variable!"),
//     };
// }

fn get_dual_hashmap(
    known_things: &mut HashMap<String, KnownThing>,
    tmp_known_things: &mut HashMap<String, KnownThing>,
    sym: &String,
) -> KnownThing {
    return match (tmp_known_things.get(sym), known_things.get(sym)) {
        (Some(thing), None) => thing,
        (None, Some(thing)) => thing,
        (Some(thing1), Some(thing2)) => thing1,
        (None, None) => panic!(
            "that function or variable does not exist. function/variable name [{:?}]",
            sym
        ),
        // tmp_known_things.get(&sym)
    }
    .clone();
}

fn evaluate(
    known_things: &mut HashMap<String, KnownThing>,
    tmp_known_things: &mut HashMap<String, KnownThing>,
    libs: &mut HashMap<String, LangLibrary>,
    s_exp: &Token,
) -> Token {
    // println!("s_exp :  {:?}", s_exp);
    let (mut action, args) = match s_exp {
        Token::Form(list) => (&list[0], list[1..].to_vec()),
        _ => return s_exp.clone(), // panic!("trying to eval a non-form data type from the global scope as an action."),
    };

    match action {
        Token::Symbol(s) if s == "let" => {
            return def_let(known_things, tmp_known_things, libs, s_exp);
            // Token::Bool(false)
        }
        _ => {}
    }

    // let args = &s_exp.children;
    let mut evaled_args = Vec::new();

    for arg in args {
        match arg {
            Token::Form(_) => {
                evaled_args.push(evaluate(known_things, tmp_known_things, libs, &arg))
            }
            Token::Symbol(ref sym) => {
                let var_val = get_dual_hashmap(tmp_known_things, known_things, sym);

                match var_val {
                    KnownThing::Var(val) => evaled_args.push(val.clone()),
                    KnownThing::LispFunc(_) => evaled_args.push(arg),
                    KnownThing::CompiledFunc(_) => evaled_args.push(arg),
                };
            }
            _ => evaled_args.push(arg),
        }
    }


    // println!("evaluate :  {:?}", action);
    // println!("known_things :  {:?}", known_things.keys());

    // println!("action :  {:?}", action);

    return match action {
        Token::Symbol(sym) => {
            let thing: KnownThing = get_dual_hashmap(known_things, tmp_known_things, sym);

            // match known_things.get(sym) {
            //     Some(thing) => thing.clone(),
            //     None => panic!("that function or variable does not exist."),
            // };

            match thing {
                KnownThing::LispFunc(s_exp) => {
                    // println!("calling a lisp function");
                    call_lisp(known_things, tmp_known_things, libs, &s_exp, evaled_args)
                } // evaluate(known_things, libs, &s_exp),
                KnownThing::CompiledFunc(f) => call_comp(&f.0, &f.1, evaled_args),
                KnownThing::Var(name) => panic!("you cant call a variable!"),
            }
        }
        Token::Str(_) | Token::Bool(_) | Token::Number(_) => action.clone(),
        Token::LParen | Token::RParen | Token::EOF => {
            panic!("there should be no parens or EOF tokens here.")
        }
        Token::Form(list) => {
            panic!("trying to eval a form as an action.");
        }
    };

    // return call_func(known_things, libs, &action, evaled_args);
}

fn run(sc_file: &String) {
    let ast = parser::parse(&read_source(&sc_file));
    /*
    println!("ast\n{:#?}\n", ast[0].data);
    for kid in &ast[0].children {
        println!("{:#?}", kid.data);
    }
    */
    let mut libraries: HashMap<String, LangLibrary> = HashMap::new(); // holds all libraries except the std_lib,
                                                                      // which gets dumped into known_things

    let mut known_things: HashMap<String, KnownThing> = HashMap::new();

    // let std_lib = import_comp_lib(
    //     &mut known_things,
    //     &mut libraries,
    //     &"libstd_lib.so".to_string(),
    // );

    import_comp_lib(
        &mut known_things,
        &mut libraries,
        &"libstd_lib.so".to_string(),
    );

    // libraries.insert("std_lib".to_string(), LangLibrary::compiled(std_lib));

    // for fname in import_lib(&"libstd_lib.so".to_string()).keys() {
    //     println!("{}", fname);
    // }

    // println!("run: known_things :  {:?}", known_things.keys());

    for glob in &ast {
        // for kid in &glob.children {
        //     println!("{:?}", kid.data);
        // }

        // let root_exec = match &glob.children[0].data {
        //     Some(thing) => thing,
        //     None => panic!("global does not exist"),
        // };

        let root_exec = match glob {
            Token::Form(form) => &form[0],
            _ => panic!("cant handle a non-form data type in the global scope."),
        };

        // println!("root_exec :  {:?}", root_exec);

        match root_exec {
            Token::Symbol(s) if s == "import" => import(glob),
            Token::Symbol(s) if s == "defun" || s == "let" => {
                def(&mut known_things, &mut HashMap::new(), &mut libraries, glob)
            }
            _ => {
                let _ = evaluate(&mut known_things, &mut HashMap::new(), &mut libraries, glob);
            }
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = args[1..].to_vec();

    // if args.len() == 0 {
    //     repl();
    //     return;
    // }

    // let test_mode = args.contains(&"-test".to_string());
    run(&args[0]);
}
