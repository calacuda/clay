//bytecode interpreter

use crate::bcc;
use crate::bcc::Bytecode;
use crate::parser;
use crate::parser::Node;
// use crate::lexer;
// use crate::lexer::Token;
use clay_lib::{Nargs, Token};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
// fn function(params: Vec<Token>, body_code: Vec<Token>, enviornment: HashMap<Token, Token>) {
//
// }
// #[derive(Debug, Clone)]
// pub enum G_Block {
//     AST(Vec<Node>),
//     Bytecode(Vec<Bytecode>),
// }

#[derive(Debug, Clone)]
pub enum StackData {
    Func(Function),
    RawFunc(RawFunc),
    StdFunc(Token),
    Tok(Token),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub nargs: Nargs,
    pub params: Vec<Token>,
    pub code: Vec<Bytecode>,
    // pub tok: Option<lexer::Token<'foobar>>
}

impl<'foobar> Function {
    fn new<'a>() -> Function {
        Function {
            name: String::new(),
            nargs: Nargs::Num(0),
            params: Vec::new(),
            code: Vec::new(),
            // tok: None
        }
    }

    fn set_name<'a>(&mut self, name: String) {
        self.name = name;
    }

    fn set_nargs<'a>(&mut self, nargs: Nargs) {
        self.nargs = nargs;
    }

    fn add_param<'a>(&mut self, param: Token) {
        self.params.push(param);
    }

    fn add_code(&mut self, tok: Bytecode) {
        self.code.push(tok);
    }

    // fn set_tok(&mut self, tok: lexer::Token<'foobar>) {
    //     self.tok = Some(tok);
    // }
}

#[derive(Debug, Clone)]
pub struct RawFunc {
    pub name: String,
    // pub nargs: Nargs,
    // pub params: Vec<Token>,
    pub root_node: Node,
    // pub tok: Option<lexer::Token<'foobar>>
}

impl<'foobar> RawFunc {
    fn new<'a>(root_node: Node) -> RawFunc {
        RawFunc {
            name: String::new(),
            // nargs: Nargs::Num(0),
            // params: Vec::new(),
            root_node: root_node,
        }
    }

    fn set_name<'a>(&mut self, name: String) {
        self.name = name;
    }

    // fn set_nargs<'a>(&mut self, nargs: Nargs) {
    //     self.nargs = nargs;
    // }

    fn set_root(&mut self, root_node: Node) {
        self.root_node = root_node;
    }

    // fn add_param<'a>(&mut self, param: Token) {
    //     self.params.push(param);
    // }
}

fn get_params<'a>(
    // nargs: usize,
    stack: &mut Vec<StackData>,
    user_names: &mut HashMap<String, StackData>,
    envrnmt: &mut HashMap<String, StackData>,
    // user_names: &mut HashMap<&'input str, StackData<'a>>
    stdlib: &'a HashMap<
        &'a str,
        (
            Nargs,
            &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
        ),
    >,
) -> (String, Vec<Token>) {
    let mut params = Vec::new();
    let mut fname = "DEFAULT".to_string();
    // println!("get_params stack: {:?}", stack);
    // println!("get_params stack: {:?}", stack.len());
    // println!("88, stack:\n\n{:?}\n\n", stack);
    for _ in 0..stack.len() {
        let dat = stack.pop();
        // println!("stack elm: {:?}", dat);
        match dat {
            Some(StackData::Func(f)) => {
                // println!("asigning fname to: {}", f.name);
                fname = f.name.to_string();
                break;
            }

            Some(StackData::StdFunc(Token::Symbol(f))) => {
                // println!("asigning fname to: {}", f);
                fname = f.to_string();
                break;
            }

            Some(StackData::StdFunc(_)) => {
                panic!("nondescript StackData::StdFunc");
            }

            Some(StackData::Tok(tok)) => {
                // println!("adding \"{:?}\" to the params list.", tok);
                params.push(tok);
            }

            Some(StackData::RawFunc(f)) => {
                // println!("encountered a raw function fname :  {}", f.name);
                match user_names.clone().get(&f.name) {
                    Some(StackData::RawFunc(_raw_func)) => {
                        // println!("f.name :  {:?}", f.name);
                        let (_thing, _thing2) = make_user_funcs(
                            &f.name,
                            &f.root_node.clone(),
                            stack,
                            envrnmt,
                            user_names,
                            stdlib,
                        );
                        // println!("{:?}", _thing);
                    }
                    _ => {
                        // println!("{:?}", f);
                        panic!("temp text.")
                    }
                }
                fname = f.name.clone();
                // println!("func :  {:?}", f.name);
                break;
                // println!("func :  {:?}", user_names.get(&f.name));
                // panic!("RawFunc cant be sent to get_params."),
            }

            None => {
                panic!("place holder text")
            }
        };
        // println!("i: {}", i);
    }
    // println!("params: {:?}", params);
    return (fname, params);
}

fn make_args<'a>(args: Vec<Token>, params: &Vec<Token>) -> HashMap<String, StackData> {
    // println!("make_args args: {:?}", args);
    // println!("make_args params: {:?}", params);
    assert_eq!(args.len(), params.len());
    let mut enviornment = HashMap::new();
    // println!("args: {:?}", args);
    for i in [0..args.len()] {
        let i_2 = i.clone();
        // let param_name = match params[i].to_vec()[0] {
        match &params[i].to_vec()[0] {
            Token::Symbol(sym) => enviornment.insert(
                sym.to_owned(),
                StackData::Tok(args[i_2].to_vec()[0].clone()),
            ),
            // [lexer::Token::Number(num)] => &num.to_string(),
            _ => panic!("call the weewoo wagon."),
        };
        //println!("param_name: {}", param_name);
        // let arg = match &args[i_2] {
        //     [lexer::Token::Symbol(sym)] => sym.to_string(),
        //     [lexer::Token::Number(num)] => num.to_string(),
        //     _ => panic!("call the weewoo wagon.")
        // };
        // enviornment.insert(param_name, StackData::Tok(args[i_2].to_vec()[0].clone()));
    }
    return enviornment;
}

fn call_func<'a>(
    nargs: usize,
    stack: &mut Vec<StackData>,
    envrnmt: &mut HashMap<String, StackData>,
    user_names: &mut HashMap<String, StackData>,
    stdlib: &'a HashMap<
        &'a str,
        (
            Nargs,
            &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
        ),
    >,
) {
    // println!("envrnmt: {:?}", envrnmt);
    // println!("user_names: {:?}", user_names.keys());
    let (func_name, mut params) = get_params(stack, user_names, envrnmt, stdlib);
    // println!("params :  {:?}", params);
    // let needs_comp = match user_names.get(&func_name) {
    //     None => false,
    //     Some(StackData::RawFunc(_)) => true,
    //     _ => false,
    // };
    // // println!("needs_comp :  {:?}", needs_comp);
    // // println!("stack :  {:?}", stack);

    // let g_block;
    // g_block = match user_names.get(&func_name) {
    //     Some(StackData::RawFunc(f)) => {
    //         make_user_funcs(&f.root_node.clone(), stack, envrnmt, user_names, stdlib);
    //     }
    //     _ => panic!("temp text."),
    // };

    // let (_, func_name) = make_user_funcs(&g_block, stack, envrnmt, user_names, stdlib);

    // println!("call_func stack: {:?}", stack);
    // println!("call_func func_name: {:?}", func_name);
    // println!("call_func params: {:?}", params);

    // println!("call_func u_names: {:?}", user_names);
    // let working_user_names = user_names.clone();
    // println!("user_names keys :  {:?}", user_names.keys());
    // println!("func name :  {:?}", func_name);

    // if uncomp_func.contains_key(&func_name) {
    //     function_com(
    //         &mut bcc::get_bc_jit(uncomp_func.remove(&func_name).unwrap(), user_names, stdlib),
    //         stack,
    //         uncomp_func,
    //         envrnmt,
    //         user_names,
    //         stdlib,
    //     )
    // }

    if stdlib.contains_key(&func_name.as_ref()) {
        // println!("f {}", f);
        let func_details = stdlib.get(&func_name.as_ref());
        match func_details.unwrap().0 {
            Nargs::INF => {}
            Nargs::Num(num) => {
                if num != nargs {
                    panic!("wrong number of args");
                }
            }
        }
        // println!("calling function {:?}", func_name);
        let return_val = func_details.unwrap().1(&params).clone();
        match return_val {
            Ok(Some(tok)) => stack.push(StackData::Tok(tok)),
            Ok(None) => {}
            Err(_) => {} // Err(mesg) => panic!(mesg),
        }
        return;
    } else if user_names.contains_key(func_name.as_str()) {
        // println!("calling function: {:?}", func_name);
        // let func_details = working_user_name.get(func_name.as_str()).unwrap();
        let func;
        // let wun = user_names.clone();
        let data = user_names.get(func_name.as_str()).unwrap().clone();
        match data {
            StackData::Func(f) => {
                func = f.clone();
                // println!("call_func params: {:?}", params);
                let mut envrnmt = make_args(params.clone(), &func.params);
                // println!("call_func envrnmt: {:?}", envrnmt);
                // let nargs = match f.nargs {
                //     Nargs::INF => {15},
                //     Nargs::Num(num) => num,
                // };
                // println!("envrnmt: {:?}", envrnmt);
                // call_func(nargs, stack, &envrnmt, user_names, stdlib);
                do_the_thing(&f.code, stack, &mut envrnmt, user_names, stdlib, true);
            }
            _ => {
                println!("func_name: \"{}\" was not called", func_name);
                // println!("data: {:?} triggered an ERROR", data);
                println!("user_names: {:?}", user_names.keys());
                panic!("func not called");
            }
        }

        // let env = make_args(params, func.params);
        // println!("func_details: {:?}", func_details);
    } else {
        panic!("function not found");
    }
}

fn bin_add<'a>(stack: &mut Vec<StackData>) {
    let s1 = match stack.pop().unwrap() {
        StackData::Tok(Token::Number(num)) => num,
        _ => panic!("you can only add numbers"),
    };
    let s2 = match stack.pop().unwrap() {
        StackData::Tok(Token::Number(num)) => num,
        _ => panic!("you can only add numbers"),
    };

    let sum;

    if s1.contains(".") | s2.contains(".") {
        let n1: f64 = s1.parse().unwrap();
        let n2: f64 = s2.parse().unwrap();
        let tmp_sum = n2 + n1;
        sum = tmp_sum.to_string();
    } else {
        let n1: i64 = s1.parse().unwrap();
        let n2: i64 = s2.parse().unwrap();
        let tmp_sum = n2 + n1;
        sum = tmp_sum.to_string();
    };
    stack.push(StackData::Tok(Token::Number(sum)));
}

fn bin_sub<'a>(stack: &mut Vec<StackData>) {
    let s1 = match stack.pop().unwrap() {
        StackData::Tok(Token::Number(num)) => num,
        _ => panic!("you can only subtract numbers"),
    };
    let s2 = match stack.pop().unwrap() {
        StackData::Tok(Token::Number(num)) => num,
        _ => panic!("you can only subtract numbers"),
    };

    let sum;

    if s1.contains(".") | s2.contains(".") {
        let n1: f64 = s1.parse().unwrap();
        let n2: f64 = s2.parse().unwrap();
        let tmp_sum = n2 - n1;
        sum = tmp_sum.to_string();
    } else {
        let n1: i64 = s1.parse().unwrap();
        let n2: i64 = s2.parse().unwrap();
        let tmp_sum = n2 - n1;
        sum = tmp_sum.to_string();
    };
    stack.push(StackData::Tok(Token::Number(sum)));
}

fn bin_mul<'a>(stack: &mut Vec<StackData>) {
    // println!("bin_mul stack: {:?}", stack);
    let s1 = match stack.pop().unwrap() {
        StackData::Tok(Token::Number(num)) => num,
        _ => panic!("you can only multiply numbers"),
    };
    let s2 = match stack.pop().unwrap() {
        StackData::Tok(Token::Number(num)) => num,
        _ => panic!("you can only multiply numbers"),
    };
    // let (s1, s2) = get_two_num(stack);

    let sum;

    if s1.contains(".") | s2.contains(".") {
        let n1: f64 = s1.parse().unwrap();
        let n2: f64 = s2.parse().unwrap();
        let tmp_sum = n2 * n1;
        sum = tmp_sum.to_string();
    } else {
        let n1: i64 = s1.parse().unwrap();
        let n2: i64 = s2.parse().unwrap();
        let tmp_sum = n2 * n1;
        sum = tmp_sum.to_string();
    };
    stack.push(StackData::Tok(Token::Number(sum)));
}

fn bin_div<'a>(stack: &mut Vec<StackData>) {
    let s1 = match stack.pop().unwrap() {
        StackData::Tok(Token::Number(num)) => num,
        _ => panic!("you can only divide numbers"),
    };
    let s2 = match stack.pop().unwrap() {
        StackData::Tok(Token::Number(num)) => num,
        _ => panic!("you can only divide numbers"),
    };

    let n1: f64 = s1.parse().unwrap();
    let n2: f64 = s2.parse().unwrap();
    let tmp_sum = n2 / n1;
    let sum = tmp_sum.to_string();
    stack.push(StackData::Tok(Token::Number(sum)));
}

fn store_name<'a>(
    stack: &mut Vec<StackData>,
    name_space: &mut HashMap<String, StackData>,
    name: &'a str,
) {
    // for _ in [0..stack.len()] {
    //     let tok = stack.pop().clone().unwrap();
    //     match tok {
    //         _ => println!("tok: {:?}", tok)
    //     }
    // }
    // println!("store_name");
    let tmp_var = stack.pop().unwrap();
    // match tmp_var {
    //     StackData::Tok(Token::Symbol(name)) => {user_names.insert(name, tmp_var);},
    //     StackData::Tok(_) => panic!("idk what that is, but its not a function name."),
    //     StackData::Func(_) => panic!("found Function, not a function name."),
    //     _ => println!("tmp_var: {:?}", tmp_var),
    // }
    // match name
    name_space.insert(name.to_string(), tmp_var);
}

fn bin_num_comp<'a>(stack: &mut Vec<StackData>, sign: char) {
    let s1 = match stack.pop().unwrap() {
        StackData::Tok(Token::Number(num)) => num,
        _ => panic!("you can only compare numbers"),
    };
    let s2 = match stack.pop().unwrap() {
        StackData::Tok(Token::Number(num)) => num,
        _ => panic!("you can only compare numbers"),
    };

    if s1.contains(".") | s2.contains(".") {
        let n1: f64 = s1.parse().unwrap();
        let n2: f64 = s2.parse().unwrap();
        let answer = match sign {
            '<' => n1 > n2,
            '>' => n1 < n2,
            '=' => n1 == n2,
            _ => panic!("wrong sign provided"),
        };
        stack.push(StackData::Tok(Token::Bool(answer)));
    } else {
        let n1: i64 = s1.parse().unwrap();
        let n2: i64 = s2.parse().unwrap();
        let answer = match sign {
            '<' => n1 > n2,
            '>' => n1 < n2,
            '=' => n1 == n2,
            _ => panic!("wrong sign provided"),
        };
        stack.push(StackData::Tok(Token::Bool(answer)));
    };
}

fn do_the_thing<'a>(
    thing: &Vec<Bytecode>,
    stack: &mut Vec<StackData>,
    envrnmt: &mut HashMap<String, StackData>,
    user_names: &mut HashMap<String, StackData>,
    stdlib: &'a HashMap<
        &'a str,
        (
            Nargs,
            &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
        ),
    >,
    user_def_func: bool,
)
// -> Option<Token<'input>>
{
    /*
    the main doer of things, hence the not so clever name, this excecute teh byte code for a single
    form in the global scope.
    */

    // println!("doing the thing");
    // let interable = thing.clone().peekable();
    // println!("do_the_thing stack: {:?}", stack);
    // println!("do_the_thing thing: {:?}", thing);
    let mut loc_thing = thing.iter();
    let mut code;
    loop {
        code = loc_thing.next();
        // println!("code: {:?}", code);
        match code.clone() {
            Some(Bytecode::LoadFunc(func_tok)) => {
                // println!("func: {:?}", func);
                match func_tok {
                    Token::Symbol(func_name) => {
                        if user_names.contains_key(func_name.as_str()) {
                            stack.push(user_names.get(func_name.as_str()).unwrap().clone());
                        } else if stdlib.contains_key(func_name.as_str()) {
                            stack.push(StackData::StdFunc(func_tok.clone()));
                        } else {
                            panic!("\"{}\" is not a defined function", func_name);
                        }
                        // println!("stack: {:?}", stack);
                    }
                    _ => {}
                }
            }
            Some(Bytecode::Push(constant)) => stack.push(StackData::Tok(constant.clone())),
            // Some(Bytecode::LoadName(Token::Symbol(name))) => {
            //     match envrnmt.get(name) {
            //         Some(StackData::Func(func)) => {},
            //         Some(StackData::StdFunc(std_func)) => {},
            //         Some(StackData::Tok(tok)) => {}
            //         None => println!("name: {}", name) //panic!("ERROR of unknown origins.")
            //     };
            // }  // user_names.insert(k, v)
            Some(Bytecode::LoadName(Token::Symbol(name))) => {
                // println!("loading name: {:?}", name);

                // if uncomp_func.contains_key(name) {
                //     let mut bc = bcc::get_bc_jit(
                //         uncomp_func.remove(name).unwrap(),
                //         uncomp_func.clone(),
                //         stdlib,
                //     );
                //     println!("459, bc: {:?}", bc);
                //     function_com(&mut bc, stack, uncomp_func, envrnmt, user_names, stdlib)
                // }

                let value = if envrnmt.contains_key(name) {
                    envrnmt.get(name)
                } else if user_names.contains_key(name) {
                    user_names.get(name)
                } else {
                    println!("user_names :  {:?}", user_names.keys());
                    panic!("I don't of this: \"{}\" you speak of.", name)
                };

                match value {
                    Some(_) => stack.push(value.unwrap().clone()),
                    None => {
                        // println!("name: {:?}", name);
                        // println!("user_names: {:?}", user_names);
                        // panic!("variable or function not found");
                    }
                }
            }
            Some(Bytecode::StoreName(Token::Symbol(name))) => {
                if user_def_func {
                    store_name(stack, envrnmt, name);
                } else {
                    store_name(stack, user_names, name);
                }
            }
            Some(Bytecode::StoreName(Token::LParen | Token::RParen)) => {
                panic!("ERROR: parenthesis are a reserved token.")
            }
            Some(Bytecode::StoreName(_)) => {
                panic!("ERROR: can't us a data type as a variable name.")
            }
            // Some(Bytecode::StoreName(Token::EOF)) => panic!("ERROR: parenthesis are a reserved token."),
            // Bytecode::MakeFunc(nargs) => make_func(stack, nargs),
            Some(Bytecode::BinAdd) => bin_add(stack),
            Some(Bytecode::BinSub) => bin_sub(stack),
            Some(Bytecode::BinMul) => bin_mul(stack),
            Some(Bytecode::BinDiv) => bin_div(stack),
            Some(Bytecode::CallFunc(num)) => {
                call_func(num.clone(), stack, envrnmt, user_names, stdlib)
            }
            Some(Bytecode::BinLess) => bin_num_comp(stack, '<'),
            Some(Bytecode::BinGrtr) => bin_num_comp(stack, '>'),
            Some(Bytecode::BinEqu) => bin_num_comp(stack, '='),
            Some(Bytecode::JumpIfTrue(distance)) => match stack.pop().unwrap() {
                StackData::Tok(Token::Bool(false)) => {
                    // println!("jumping if");
                    for _ in 0..distance.clone() {
                        loc_thing.next();
                    }
                }
                StackData::Tok(Token::Bool(true)) => {} // println!("not jumping if");}
                _ => panic!("no bool found after if statement"),
            },
            Some(Bytecode::Jump(distance)) => {
                // println!("jumping");
                for _ in 0..distance.clone() {
                    loc_thing.next();
                }
            }
            Some(_) => panic!("bytecode \"{:?}\" not known", code),
            None => break,
        }
        // println!("stack: {:?}", stack);
        // println!();
    }
}

fn function_com<'a>(
    thing: &mut Vec<Bytecode>,
    stack: &mut Vec<StackData>,
    envrnmt: &mut HashMap<String, StackData>,
    user_names: &mut HashMap<String, StackData>,
    stdlib: &'a HashMap<
        &'a str,
        (
            Nargs,
            &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
        ),
    >,
) {
    /*
    function "compiler":

    this function builds up the bytecode for a function.
    */
    let mut f = Function::new();
    // println!("thing: {:?}", thing);
    match thing.pop() {
        Some(Bytecode::StoreName(Token::Symbol(name))) => f.set_name(name),
        _ => panic!("defun needs a function name."),
    }
    match thing.last().unwrap() {
        Bytecode::MakeFunc(_) => {
            thing.pop();
        }
        _ => {
            thing.push(Bytecode::StoreName(Token::Symbol(f.name.to_string())));
            // asign_globals(thing, f.name, stack, envrnmt, user_names, std_lib);
            do_the_thing(thing, stack, envrnmt, user_names, stdlib, true);
            return;
        }
    }
    // println!("thing: {:?}", thing);
    let mut nargs = 0;
    loop {
        let code = thing.pop();
        match code {
            Some(Bytecode::StoreName(Token::Symbol(name))) => {
                // println!("name: {:?}", name);
                if name != f.name {
                    f.add_param(Token::Symbol(name));
                } else {
                    nargs += 1;
                }
            }
            _ => {
                thing.push(code.unwrap());
                break;
            }
        }
        // println!("code: {:?}", code);
    }
    // let mut tmp_stack: Vec<StackData> = Vec::new();
    for code in thing {
        // println!("code: {:?}", code);
        match code.clone() {
            Bytecode::MakeFunc(_) => {
                // println!("function def inside function")
                panic!("ERROR: you can't declare functions inside other functions.")
            }
            bc => f.add_code(bc),
        }
        // println!("stack: {:?}", stack);
    }
    f.set_nargs(Nargs::Num(f.params.len()));
    user_names.insert(f.name.to_string(), StackData::Func(f));
}

pub fn do_the_things<'a>(
    things: &Vec<Vec<Bytecode>>,
    stdlib: &'a HashMap<
        &'a str,
        (
            Nargs,
            &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
        ),
    >,
) {
    /*
    do the things:

    the entry point for the byte code interpreter. run this function to interpret bytecode.
    */
    let mut stack: Vec<StackData> = Vec::new();
    let mut user_names: HashMap<String, StackData> = HashMap::new();
    let mut envrnmt: HashMap<String, StackData> = HashMap::new();
    // let mut g_block_clone: Vec<Bytecode>;
    let mut uncomp_func: HashMap<String, &Node> = HashMap::new();
    let funcs: HashSet<String> = HashSet::new();

    for g_block in things {
        // println!("g_block[0]: {:?}", g_block[0]);
        // g_block_clone = g_block.to_owned().clone();
        match g_block.last().unwrap() {
            Bytecode::StoreName(_) => function_com(
                &mut g_block.clone(),
                &mut stack,
                &mut envrnmt,
                &mut user_names,
                stdlib,
            ),
            _ => do_the_thing(
                &g_block,
                &mut stack,
                &mut envrnmt,
                &mut user_names,
                stdlib,
                false,
            ),
        }
        // println!("user_funcs: {:?}", user_names.keys());
        // println!("times-two: {:?}", user_names.get("times-two"));

        // break;
    }
}

fn find_lib(name: String) -> std::path::PathBuf {
    let site_packs = format!(
        "{}/.local/lib/clay/site-packages/{}",
        env::var("HOME").unwrap(),
        name
    );
    let site_path = Path::new(&site_packs).to_owned();

    let cur_dir = env::current_dir().unwrap();
    let cur_path = Path::new(cur_dir.as_os_str()).join(Path::new(&name));

    let full_path = Path::new(&name).to_owned();
    // println!("{:?}", site_path);
    if cur_path.exists() {
        return cur_path;
    } else if site_path.exists() {
        return site_path;
    } else if full_path.exists() {
        return full_path;
    } else {
        panic!("the library, {}, can not be found", name);
    }
}

fn find_libs(libs: Option<&Token>) -> Vec<(String, std::path::PathBuf)> {
    let lib_form = match libs.unwrap() {
        Token::Form(thing) => *thing.clone(),
        _ => panic!("the import function needs a form of files to import."),
    };

    let mut lib_paths = Vec::new();
    for lib in lib_form.iter() {
        match lib {
            Token::Str(name) => lib_paths.push((
                name.split('.').collect::<Vec<&str>>()[0].to_string(),
                find_lib(name.to_owned()),
            )),
            Token::Symbol(name) => lib_paths.push((
                name.split('.').collect::<Vec<&str>>()[0].to_string(),
                find_lib(name.to_owned()),
            )),
            _ => panic!("lib name must be either a string or symbol."),
        }
    }
    return lib_paths;
}

fn import_comp_rust<'a>(
    path: PathBuf,
) -> HashMap<
    &'a str,
    (
        Nargs,
        &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
    ),
> {
    unsafe {
        // let lib = libloading::Library::new(libloading::library_filename(lib_path)).unwrap();
        let lib = libloading::Library::new(path).unwrap();
        let get_funcs: libloading::Symbol<
            unsafe extern "C" fn() -> HashMap<
                &'a str,
                (
                    Nargs,
                    &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
                ),
            >,
        > = lib.get("get_funcs".as_bytes()).unwrap();
        return get_funcs();
    }
}

fn not_yet_compiled<'a>(
    // block: &'a Node,
    things: &'a Vec<Node>,
    user_names: &mut HashMap<String, StackData>,
    name_mod: String,
    // uncomp_func: &'a HashMap<String, &'a Node>,
) -> (
    HashMap<String, &'a Node>,
    HashMap<String, &'a Node>,
    HashSet<String>,
) {
    let mut raw_func: HashMap<String, &Node> = HashMap::new();
    let mut imports: HashMap<String, &Node> = HashMap::new();
    let mut funcs: HashSet<String> = HashSet::new();
    for block in things {
        match &block.data {
            Some(Token::Symbol(defun)) if defun == "defun" => {
                let fname = match &block.children[0].data {
                    Some(Token::Symbol(name)) => name,
                    _ => panic!("that is not a valid function name"),
                };
                // raw_func.insert(fname.to_owned(), block);
                // funcs.insert(fname.to_owned());
                let mut f = RawFunc::new(block.to_owned());
                // f.name = format!("{}{}", name_mod, fname.to_owned());
                f.name = fname.to_owned();
                user_names.insert(
                    fname.to_owned(),
                    // format!("{}{}", name_mod, fname.to_owned()),
                    StackData::RawFunc(f),
                );
            }
            Some(Token::Symbol(import)) if import == "import" => {
                let lib_paths = find_libs(block.children[0].data.as_ref());
                // println!("{:?}", lib_paths);
                for (name, path) in lib_paths {
                    match read_to_string(&path) {
                        Ok(source_code) => {
                            let ast = parser::parse(&source_code);
                            // not_yet_compiled(&ast, user_names, format!("{}::", name));
                            not_yet_compiled(&ast, user_names, name);
                        }

                        Err(_) => {
                            let funcs = import_comp_rust(path);
                        }
                    }
                } // make this arm work
            }
            // Some(Symbol(func)) => {}
            _ => {}
        }
    }
    return (raw_func, imports, funcs);
}

// fn jit_call<'a>(
//     func_name: String,
//     g_block: &'a Node,
//     stack: &mut Vec<StackData>,
//     user_names: &mut HashMap<String, StackData>,
//     envrnmt: &mut HashMap<String, StackData>,
//     stdlib: &'a HashMap<
//         &'a str,
//         (
//             Nargs,
//             &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
//         ),
//     >,
// ) {
//     // println!("{:#?}", g_block.last().unwrap());
//     let needs_comp = match user_names.get(&func_name) {
//         None => false,
//         Some(StackData::RawFunc(_)) => true,
//         _ => false,
//     };
//     println!("needs_comp :  {:?}", needs_comp);
//     println!("stack :  {:?}", stack);
//     let bc = if needs_comp {
//         let (_, func_name) = make_user_funcs(&g_block, stack, envrnmt, user_names, stdlib);
//         match user_names.clone().get(&func_name) {
//             Some(StackData::Func(f)) => f.code,
//             _ => panic!("something is wrong internally."),
//         }
//     } else if envrnmt.contains_key(&func_name) {
//         match envrnmt.get(&func_name) {
//             Some(StackData::Func(f)) => f.code,
//             // Some(StackData::StdFunc(tok)) => {}
//             _ => panic!("temp text"),
//         }
//     } else if stdlib.contains_key(&func_name) {
//         match stdlib.get(&func_name) {
//             Some(StackData::StdFunc(f)) => {
//                 return;
//             }
//             _ => panic!("temp text"),
//         }
//     } else {
//         panic!("temp text");
//     };
//     do_the_thing(&bc, stack, envrnmt, user_names, stdlib, true);
// }

fn make_user_funcs<'a>(
    _func_name: &String,
    uncomp_thing: &Node,
    stack: &mut Vec<StackData>,
    envrnmt: &mut HashMap<String, StackData>,
    user_names: &mut HashMap<String, StackData>,
    stdlib: &'a HashMap<
        &'a str,
        (
            Nargs,
            &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
        ),
    >,
) -> (StackData, String) {
    // let mut func = Function::new();
    // let uncomp_func_list = uncomp_func.keys().collect::<Vec<&String>>().to_vec();
    // let (func_name, mut params) = get_params(stack, user_names, stdlib);
    // func.set_name(func_name);
    // func.params = params;
    let func_name = match &uncomp_thing.children[0].data {
        Some(Token::Symbol(sym)) => {
            let mut bc = bcc::get_bc_jit(uncomp_thing, user_names, stdlib);
            // let mut tmp_user_names: HashMap<String, StackData> = HashMap::new();
            function_com(&mut bc, stack, envrnmt, user_names, stdlib); // function_comp adds the function to user_names
                                                                       // for (_old_func_name, stack_data) in tmp_user_names {
                                                                       //     // only iterates once.
                                                                       //     user_names.insert(func_name.to_owned(), stack_data);
                                                                       // }
            sym
        }
        _ => panic!("ERROR: in make_user_funcs function from bci."),
    };
    // add function to user_names

    return (
        user_names.get(func_name).unwrap().to_owned(),
        func_name.to_owned(),
    );
}

pub fn jit_run<'a>(
    things: &'a Vec<Node>,
    stdlib: &'a HashMap<
        &'a str,
        (
            Nargs,
            &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
        ),
    >,
) {
    let mut stack: Vec<StackData> = Vec::new();
    let mut user_names: HashMap<String, StackData> = HashMap::new();
    let mut envrnmt: HashMap<String, StackData> = HashMap::new();
    // let mut uncomp_func: HashMap<String, &Node> = HashMap::new();
    // parser

    let (mut _uncomp_stuff, _imports, mut _funcs) =
        not_yet_compiled(things, &mut user_names, String::new());
    // println!("not compiled :  {:?}", uncomp_stuff);

    for g_block in things {
        // let uncomp_stuff_list = uncomp_stuff.keys().collect::<Vec<&String>>().to_vec();
        match &g_block.data {
            Some(Token::Symbol(s)) if s != "import" && s != "defun" => {
                // println!("g_block :  {:?}", g_block.data);
                // jit_call(
                //     s.to_owned(),
                //     &g_block,
                //     &mut stack,
                //     &mut envrnmt,
                //     &mut user_names,
                //     stdlib,
                // );
                let bc = bcc::get_bc_jit(g_block, &mut user_names, stdlib);
                do_the_thing(
                    &bc,
                    &mut stack,
                    &mut envrnmt,
                    &mut user_names,
                    // &g_block,
                    stdlib,
                    true,
                );
            }

            Some(Token::Symbol(s)) if s == "import" || s == "defun" => {}

            _ => {} // should this panic.
        }
    }
    // println!("stack :  {:?}", stack);
}
