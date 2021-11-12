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

    let func_name = &funcs.get(&"write-line".to_string()).unwrap().1;
    let mut args = Vec::new();
    args.push(Token::Str("std_lib".to_string()));
    args.push(Token::Str("test printer".to_string()));

    // call_comp(
    //     &location.as_os_str().to_str().unwrap().to_string(),
    //     &func_name,
    //     args,
    // );

    return funcs;
}

fn import_lib<'a>(
    lib_name: &String,
) -> HashMap<
    String,
    (
        Nargs,
        String,
        // &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
    ),
> {
    let location = find_lib(lib_name);
    return get_lib_contents(&location);
}

fn call_comp<'a>(lib_name: &String, func_name: &String, args: Vec<Token>) {
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

    // return result;
}

fn run(sc_file: &String) {
    let ast = parser::parse(&read_source(&sc_file));
    /*
    println!("ast\n{:#?}\n", ast[0].data);
    for kid in &ast[0].children {
        println!("{:#?}", kid.data);
    }
    */

    import_lib(&"libstd_lib.so".to_string());

    // for fname in import_lib(&"libstd_lib.so".to_string()).keys() {
    //     println!("{}", fname);
    // }

    for glob in &ast {
        println!("{:?}", glob.data);
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
