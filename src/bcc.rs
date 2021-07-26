/*
takes the parsed code outputs bytecode.
*/

// use crate::lexer;
use crate::bci::StackData;
use crate::parser;
use crate::parser::Node;
// use crate::lexer::Token;
use clay_lib::Nargs;
use clay_lib::Token;
// use crate::std_lib;

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Bytecode {
    //Stack stuff
    Push(Token), // pushes a const to the stack

    //Binary math
    BinAdd,
    BinSub,
    BinMul,
    BinDiv,

    //Binary Comparisons
    BinLess,
    BinGrtr,
    BinEqu,
    // BinNEqu, //implemented with "BinEqu" and "LogNot"

    //Binary logic
    LogOr,
    LogAnd,
    LogNot,

    //Names
    StoreName(Token),
    LoadName(Token),

    //Functions
    CallFunc(usize),
    MakeFunc(usize),
    LoadFunc(Token),

    //fLoW cOnTrOl
    JumpIfTrue(usize),
    Jump(usize), // could be done with "Push True" then "JumpIfTrue" but I too tierd to
                 // figure out the mechanics of that.
}

// #[derive(Debug, Clone)]
// pub struct Data<'input> {
//     pub funcs: Vec<lexer::Token<'input>>,
//     pub vars: Vec<lexer::Token<'input>>,
// }
//
// impl<'input> Data<'input> {
//     pub fn new() -> Data<'input> {
//         Data {
//             funcs: Vec::new(),
//             vars: Vec::new(),
//         }
//     }
//
//     pub fn add_func(mut self, func: lexer::Token<'input>) {
//         println!("adding function: {:?}", func);
//         self.funcs.push(func);
//     }
//
//     pub fn add_var(mut self, var: lexer::Token<'input>) {
//         self.vars.push(var);
//     }
//
//     pub fn has_func(self, func: &lexer::Token<'input>) -> bool{
//         self.funcs.contains(func)
//     }
// }

// pub fn get_bytecode<'input>(parsed: &Vec<parser::Node<'input>>) -> Vec<parser::Bytecode<'input>> { //-> Vec<Statments> {
//     /*
//     returns a vector of instructions to execute.
//     this is interpreted later.
//     */
//     let mut code = Vec::new();
//
//     for node in parsed.iter() {
//         println!("{:?}", node.data);
//         match node.data {
//             Some(lexer::Token::Symbol("+" | "*" | "/" | "-")) => {
//                 code.append(&mut get_bytecode(&node.children));
//                 match node.data {
//                     Some(lexer::Token::Symbol("+")) => code.push(Bytecode::BinAdd),
//                     Some(lexer::Token::Symbol("*")) => code.push(Bytecode::BinMul),
//                     Some(lexer::Token::Symbol("-")) => code.push(Bytecode::BinSub),
//                     Some(lexer::Token::Symbol("/")) => code.push(Bytecode::BinDiv),
//                     _ => {},
//                 }
//                 //code.push('\n');
//                 // break;
//             }
//
//             Some(lexer::Token::Number(n)) => {
//                 code.push(Bytecode::Push(node.data.clone().unwrap()));
//                 // break;
//             }
//
//             Some(lexer::Token::RParen) => {
//                 break;
//             }
//
//             _ => {
//                 println!("ERROR: and not even I know why!");
//             }
//         }
//     }
//     return code;
// }

fn _get_bc_jit<'input>(
    parsed: Vec<parser::Node>,
    user_funcs: &mut HashMap<String, StackData>,
    // user_names: &HashSet<String>,
    stdlib: &HashMap<
        &'input str,
        (
            Nargs,
            &'input (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'input str>),
        ),
    >,
) -> Vec<Bytecode> {
    let mut gen_funcs = Vec::new();
    let mut code = Vec::new();
    // println!("\ndat:{:?}", user_funcs);

    // println!("\n{:?}\n", parsed);

    for node in parsed.iter() {
        // println!("node data: {:?}", node.data);
        match &node.data {
            Some(Token::Symbol(sym))
                if (sym == &"+") | (sym == &"*") | (sym == &"/") | (sym == &"-") =>
            {
                // math operators.
                let mut bcode = _get_bc_jit(node.children.clone(), user_funcs, stdlib);
                code.append(&mut bcode);
                match &node.data {
                    Some(Token::Symbol(sym)) if sym == &"+" => code.push(Bytecode::BinAdd),
                    Some(Token::Symbol(sym)) if sym == &"*" => code.push(Bytecode::BinMul),
                    Some(Token::Symbol(sym)) if sym == &"-" => code.push(Bytecode::BinSub),
                    Some(Token::Symbol(sym)) if sym == &"/" => code.push(Bytecode::BinDiv),
                    _ => {}
                }
            }

            Some(Token::Symbol(sym))
                if (sym == &"<")
                    | (sym == &">")
                    | (sym == &">=")
                    | (sym == &"<=")
                    | (sym == &"=")
                    | (sym == &"!=") =>
            {
                // comparisons
                let mut bcode = _get_bc_jit(node.children.clone(), user_funcs, stdlib);
                code.append(&mut bcode);
                match &node.data {
                    Some(Token::Symbol(sym)) if sym == &"<" => code.push(Bytecode::BinLess),
                    Some(Token::Symbol(sym)) if sym == &">" => code.push(Bytecode::BinGrtr),
                    Some(Token::Symbol(sym)) if sym == &">=" => {
                        panic!("greater then or equal to is not implemented yet")
                    } // code.push(Bytecode::),
                    Some(Token::Symbol(sym)) if sym == &"<=" => {
                        panic!("less then or equal to is not implemented yet")
                    } // code.push(Bytecode::),
                    Some(Token::Symbol(sym)) if sym == &"=" => code.push(Bytecode::BinEqu),
                    Some(Token::Symbol(sym)) if sym == &"!=" => {
                        code.push(Bytecode::BinEqu);
                        code.push(Bytecode::LogNot);
                    }
                    _ => {}
                }
            }

            Some(Token::Symbol(sym)) if sym == &"defun" => {
                gen_funcs.push(node.children[0].data.clone().unwrap());
                // dat.add_func(node.children[0].data.clone().unwrap());

                // println!("first child of defun: {:?}", node.children[2]);

                if node.children.len() > 2 {
                    let mut bcode = _get_bc_jit(node.children[2..].to_vec(), user_funcs, stdlib);
                    code.append(&mut bcode);

                    code.push(Bytecode::StoreName(node.children[1].data.clone().unwrap())); // first parameter

                    // println!("node.children[1].data:  {:?}", node.children[1].data);

                    for child in &node.children[1].children {
                        // the rest of the params.
                        code.push(Bytecode::StoreName(child.data.clone().unwrap()));
                    }
                    code.push(Bytecode::MakeFunc(node.children[1].children.len() + 1)); // makes the function

                    code.push(Bytecode::StoreName(node.children[0].data.clone().unwrap()));
                // stores function
                } else {
                    let mut bcode = _get_bc_jit(node.children[1..].to_vec(), user_funcs, stdlib);
                    code.append(&mut bcode);

                    code.push(Bytecode::StoreName(node.children[0].data.clone().unwrap()));
                    code.push(Bytecode::MakeFunc(0));

                    // for child in &node.children[0].children {
                    //     code.push(Bytecode::StoreName(child.data.clone().unwrap()));
                    // }
                }

                //break;
            }

            Some(Token::Symbol(sym)) if sym == &"if" => {
                // code.push(Bytecode::Block("CONDITION_BLOCK")); // uncomment if conditional are broken
                let mut cond_bcode =
                    _get_bc_jit(vec![node.children[0].clone()], user_funcs, stdlib);
                // let distance = bcode.len();
                // println!("if block bytecode {:?}", bcode);

                let mut bcode = _get_bc_jit(vec![node.children[1].clone()], user_funcs, stdlib);
                let distance = bcode.len();
                code.append(&mut cond_bcode); // the condition form
                code.push(Bytecode::JumpIfTrue(distance + 1));
                code.append(&mut bcode); // the if block

                // code.push(Bytecode::Jump("END_IF"));
                // code.push(Bytecode::Block("ELSE_BLOCK"));

                let mut bcode = _get_bc_jit(vec![node.children[2].clone()], user_funcs, stdlib);
                code.push(Bytecode::Jump(bcode.len()));
                code.append(&mut bcode); // the else block
                                         // code.push(Bytecode::Block("END_IF"));
            }

            Some(Token::Symbol(sym)) if sym == &"return" => {
                let mut bcode = _get_bc_jit(node.children.clone(), user_funcs, stdlib);
                code.append(&mut bcode);
                break;
            }

            Some(Token::Symbol(sym)) if sym == &"let" => {
                // let mut bcode = _get_bytecode(node.children.clone(), user_funcs, stdlib);
                // // code.append(&mut bcode);
                // println!("\nlet begin:");
                // for code in bcode {
                //     println!("bcode code: {:?}", code);
                // }
                // println!(":let end\n");
                for child in node.children.clone() {
                    if child.children.len() < 1 {
                        match child.data {
                            Some(Token::Symbol(var_name)) => {
                                panic!("ERROR: variable \"{}\" needs a value", var_name)
                            }
                            Some(Token::EOF) => panic!("ERROR: unexpected EOF"),
                            Some(Token::LParen | Token::RParen) => {
                                panic!("ERROR: unexpected parenthesis")
                            }
                            Some(
                                Token::Str(_) | Token::Bool(_) | Token::Form(_) | Token::Number(_),
                            ) => panic!("ERROR: can't use data as a variable name"),
                            None => panic!("can't read variable name"),
                        }
                    }
                    let mut bcode = _get_bc_jit(child.children.clone(), user_funcs, stdlib);
                    code.append(&mut bcode);
                    code.push(Bytecode::StoreName(child.data.unwrap()));
                }
            }

            Some(Token::Symbol(name)) => {
                //println!("ERROR: generic symbol detected, fix yo broken code!");
                // let mut bcode = _get_bytecode(node.children.clone(), user_funcs);
                // code.append(&mut bcode);

                // if dat.clone().has_func(&node.data.as_ref().unwrap()) {
                //     code.push(Bytecode::CallFunc);
                // }
                //println!("name: {}, {}", name, stdlib.contains_key(name));

                if user_funcs.contains_key(name) {
                    code.push(Bytecode::LoadFunc(node.data.clone().unwrap()));
                    let mut bcode = _get_bc_jit(node.children.clone(), user_funcs, stdlib);
                    code.append(&mut bcode);
                    code.push(Bytecode::CallFunc(node.children.len()));
                } else if stdlib.contains_key(&name.as_ref()) {
                    let nargs = node.children.len();
                    match stdlib.get(&name.as_ref()).unwrap().0 {
                        Nargs::Num(num) => {
                            if nargs == num {
                                code.push(Bytecode::LoadFunc(node.data.clone().unwrap()));
                                let mut bcode =
                                    _get_bc_jit(node.children.clone(), user_funcs, stdlib);
                                code.append(&mut bcode);
                                code.push(Bytecode::CallFunc(node.children.len()));
                            } else {
                                panic!(
                                    "wrong number of arguments fed to the function: \"{}\"",
                                    name
                                );
                            }
                        }

                        Nargs::INF => {
                            code.push(Bytecode::LoadFunc(node.data.clone().unwrap()));
                            let mut bcode = _get_bc_jit(node.children.clone(), user_funcs, stdlib);
                            code.append(&mut bcode);
                            code.push(Bytecode::CallFunc(node.children.len()));
                        } // _ => {
                          //     code.push(Bytecode::LoadFunc(node.data.clone().unwrap()));
                          //     let mut bcode = _get_bytecode(node.children.clone(), user_funcs, stdlib);
                          //     code.append(&mut bcode);
                          //     code.push(Bytecode::CallFunc(node.children.len()));
                          // }
                    }
                } else {
                    let mut bcode = _get_bc_jit(node.children.clone(), user_funcs, stdlib);
                    code.append(&mut bcode);
                    // println!("adding LoadName(\"{}\"), from children: {:?}", name, node.children);
                    // println!("");
                    code.push(Bytecode::LoadName(Token::Symbol(name.to_owned())));
                }
            }

            Some(Token::Number(_)) | Some(Token::Bool(_)) | Some(Token::Str(_)) => {
                code.push(Bytecode::Push(node.data.clone().unwrap()));
                // break;
            }
            Some(Token::Form(_)) => code.push(Bytecode::Push(node.data.clone().unwrap())),
            _ => {
                println!("{:?}", node.data);
                panic!("ERROR: you should not be seeing this.");
            }
        }
    }
    return code;
}

fn _get_bytecode<'input>(
    parsed: Vec<parser::Node>,
    user_funcs: &HashSet<String>,
    stdlib: &HashMap<
        &'input str,
        (
            Nargs,
            &'input (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'input str>),
        ),
    >,
) -> Vec<Bytecode> {
    let mut gen_funcs = Vec::new();
    let mut code = Vec::new();
    // println!("\ndat:{:?}", user_funcs);

    // println!("\n{:?}\n", parsed);

    for node in parsed.iter() {
        // println!("node data: {:?}", node.data);
        match &node.data {
            Some(Token::Symbol(sym))
                if (sym == &"+") | (sym == &"*") | (sym == &"/") | (sym == &"-") =>
            {
                // math operators.
                let mut bcode = _get_bytecode(node.children.clone(), user_funcs, stdlib);
                code.append(&mut bcode);
                match &node.data {
                    Some(Token::Symbol(sym)) if sym == &"+" => code.push(Bytecode::BinAdd),
                    Some(Token::Symbol(sym)) if sym == &"*" => code.push(Bytecode::BinMul),
                    Some(Token::Symbol(sym)) if sym == &"-" => code.push(Bytecode::BinSub),
                    Some(Token::Symbol(sym)) if sym == &"/" => code.push(Bytecode::BinDiv),
                    _ => {}
                }
            }

            Some(Token::Symbol(sym))
                if (sym == &"<")
                    | (sym == &">")
                    | (sym == &">=")
                    | (sym == &"<=")
                    | (sym == &"=")
                    | (sym == &"!=") =>
            {
                // comparisons
                let mut bcode = _get_bytecode(node.children.clone(), user_funcs, stdlib);
                code.append(&mut bcode);
                match &node.data {
                    Some(Token::Symbol(sym)) if sym == &"<" => code.push(Bytecode::BinLess),
                    Some(Token::Symbol(sym)) if sym == &">" => code.push(Bytecode::BinGrtr),
                    Some(Token::Symbol(sym)) if sym == &">=" => {
                        panic!("greater then or equal to is not implemented yet")
                    } // code.push(Bytecode::),
                    Some(Token::Symbol(sym)) if sym == &"<=" => {
                        panic!("less then or equal to is not implemented yet")
                    } // code.push(Bytecode::),
                    Some(Token::Symbol(sym)) if sym == &"=" => code.push(Bytecode::BinEqu),
                    Some(Token::Symbol(sym)) if sym == &"!=" => {
                        code.push(Bytecode::BinEqu);
                        code.push(Bytecode::LogNot);
                    }
                    _ => {}
                }
            }

            Some(Token::Symbol(sym)) if sym == &"defun" => {
                gen_funcs.push(node.children[0].data.clone().unwrap());
                // dat.add_func(node.children[0].data.clone().unwrap());

                // println!("first child of defun: {:?}", node.children[2]);

                if node.children.len() > 2 {
                    let mut bcode = _get_bytecode(node.children[2..].to_vec(), user_funcs, stdlib);
                    code.append(&mut bcode);

                    code.push(Bytecode::StoreName(node.children[1].data.clone().unwrap())); // first parameter

                    // println!("node.children[1].data:  {:?}", node.children[1].data);

                    for child in &node.children[1].children {
                        // the rest of the params.
                        code.push(Bytecode::StoreName(child.data.clone().unwrap()));
                    }
                    code.push(Bytecode::MakeFunc(node.children[1].children.len() + 1)); // makes the function

                    code.push(Bytecode::StoreName(node.children[0].data.clone().unwrap()));
                // stores function
                } else {
                    let mut bcode = _get_bytecode(node.children[1..].to_vec(), user_funcs, stdlib);
                    code.append(&mut bcode);

                    code.push(Bytecode::StoreName(node.children[0].data.clone().unwrap()));
                    code.push(Bytecode::MakeFunc(0));

                    // for child in &node.children[0].children {
                    //     code.push(Bytecode::StoreName(child.data.clone().unwrap()));
                    // }
                }

                //break;
            }

            Some(Token::Symbol(sym)) if sym == &"if" => {
                // code.push(Bytecode::Block("CONDITION_BLOCK")); // uncomment if conditional are broken
                let mut cond_bcode =
                    _get_bytecode(vec![node.children[0].clone()], user_funcs, stdlib);
                // let distance = bcode.len();
                // println!("if block bytecode {:?}", bcode);

                let mut bcode = _get_bytecode(vec![node.children[1].clone()], user_funcs, stdlib);
                let distance = bcode.len();
                code.append(&mut cond_bcode); // the condition form
                code.push(Bytecode::JumpIfTrue(distance + 1));
                code.append(&mut bcode); // the if block

                // code.push(Bytecode::Jump("END_IF"));
                // code.push(Bytecode::Block("ELSE_BLOCK"));

                let mut bcode = _get_bytecode(vec![node.children[2].clone()], user_funcs, stdlib);
                code.push(Bytecode::Jump(bcode.len()));
                code.append(&mut bcode); // the else block
                                         // code.push(Bytecode::Block("END_IF"));
            }

            Some(Token::Symbol(sym)) if sym == &"return" => {
                let mut bcode = _get_bytecode(node.children.clone(), user_funcs, stdlib);
                code.append(&mut bcode);
                break;
            }

            Some(Token::Symbol(sym)) if sym == &"let" => {
                // let mut bcode = _get_bytecode(node.children.clone(), user_funcs, stdlib);
                // // code.append(&mut bcode);
                // println!("\nlet begin:");
                // for code in bcode {
                //     println!("bcode code: {:?}", code);
                // }
                // println!(":let end\n");
                for child in node.children.clone() {
                    if child.children.len() < 1 {
                        match child.data {
                            Some(Token::Symbol(var_name)) => {
                                panic!("ERROR: variable \"{}\" needs a value", var_name)
                            }
                            Some(Token::EOF) => panic!("ERROR: unexpected EOF"),
                            Some(Token::LParen | Token::RParen) => {
                                panic!("ERROR: unexpected parenthesis")
                            }
                            Some(
                                Token::Str(_) | Token::Bool(_) | Token::Form(_) | Token::Number(_),
                            ) => panic!("ERROR: can't use data as a variable name"),
                            None => panic!("can't read variable name"),
                        }
                    }
                    let mut bcode = _get_bytecode(child.children.clone(), user_funcs, stdlib);
                    code.append(&mut bcode);
                    code.push(Bytecode::StoreName(child.data.unwrap()));
                }
            }

            Some(Token::Symbol(name)) => {
                //println!("ERROR: generic symbol detected, fix yo broken code!");
                // let mut bcode = _get_bytecode(node.children.clone(), user_funcs);
                // code.append(&mut bcode);

                // if dat.clone().has_func(&node.data.as_ref().unwrap()) {
                //     code.push(Bytecode::CallFunc);
                // }
                //println!("name: {}, {}", name, stdlib.contains_key(name));
                if user_funcs.contains(&name.to_string()) {
                    code.push(Bytecode::LoadFunc(node.data.clone().unwrap()));
                    let mut bcode = _get_bytecode(node.children.clone(), user_funcs, stdlib);
                    code.append(&mut bcode);
                    code.push(Bytecode::CallFunc(node.children.len()));
                } else if stdlib.contains_key(&name.as_ref()) {
                    let nargs = node.children.len();
                    match stdlib.get(&name.as_ref()).unwrap().0 {
                        Nargs::Num(num) => {
                            if nargs == num {
                                code.push(Bytecode::LoadFunc(node.data.clone().unwrap()));
                                let mut bcode =
                                    _get_bytecode(node.children.clone(), user_funcs, stdlib);
                                code.append(&mut bcode);
                                code.push(Bytecode::CallFunc(node.children.len()));
                            } else {
                                panic!(
                                    "wrong number of arguments fed to the function: \"{}\"",
                                    name
                                );
                            }
                        }

                        Nargs::INF => {
                            code.push(Bytecode::LoadFunc(node.data.clone().unwrap()));
                            let mut bcode =
                                _get_bytecode(node.children.clone(), user_funcs, stdlib);
                            code.append(&mut bcode);
                            code.push(Bytecode::CallFunc(node.children.len()));
                        } // _ => {
                          //     code.push(Bytecode::LoadFunc(node.data.clone().unwrap()));
                          //     let mut bcode = _get_bytecode(node.children.clone(), user_funcs, stdlib);
                          //     code.append(&mut bcode);
                          //     code.push(Bytecode::CallFunc(node.children.len()));
                          // }
                    }
                } else {
                    let mut bcode = _get_bytecode(node.children.clone(), user_funcs, stdlib);
                    code.append(&mut bcode);
                    // println!("adding LoadName(\"{}\"), from children: {:?}", name, node.children);
                    // println!("");
                    code.push(Bytecode::LoadName(Token::Symbol(name.to_owned())));
                }
            }

            Some(Token::Number(_)) | Some(Token::Bool(_)) | Some(Token::Str(_)) => {
                code.push(Bytecode::Push(node.data.clone().unwrap()));
                // break;
            }
            Some(Token::Form(_)) => code.push(Bytecode::Push(node.data.clone().unwrap())),
            _ => {
                println!("{:?}", node.data);
                panic!("ERROR: you should not be seeing this.");
            }
        }
    }
    return code;
}

fn _get_user_funcs<'input>(parsed: &Vec<parser::Node>) -> HashSet<String> {
    let mut user_funcs = HashSet::new();

    for globe in parsed.iter() {
        match &globe.data {
            Some(Token::Symbol(sym)) if sym == "defun" => {
                match &globe.children[0].data {
                    Some(Token::Symbol(func_name)) => user_funcs.insert(func_name.to_string()),
                    _ => panic!("ERROR: no function name after function declaration."),
                };
            }
            _ => {}
        }
    }

    return user_funcs;
}

// fn _get_user_funcs<'input>(parsed: &Vec<parser::Node<'input>>) -> HashMap<&'input str, usize>{
//     let mut user_funcs = HashMap::new();
//
//     for globe in parsed.iter() {
//         match globe.data {
//             Some(lexer::Token::Symbol("defun")) => {
//                 match globe.children[0].data {
//                     Some(lexer::Token::Symbol(func_name)) => {
//                         user_funcs.insert(func_name, globe.children[1]);
//                     }
//                     _ => panic!("ERROR: no function name after function declaration."),
//                 };
//             }
//             _ => {}
//         }
//     }
//
//     return user_funcs;
// }

pub fn get_bytecode<'input>(
    parsed: &Vec<parser::Node>,
    stdlib: &HashMap<
        &'input str,
        (
            Nargs,
            &'input (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'input str>),
        ),
    >,
) -> Vec<Vec<Bytecode>> {
    //-> Vec<Statments> {
    /*
    returns a vector of instructions to execute.
    this is interpreted later.
    */
    let mut code = Vec::new();
    // let data = Data::new();
    // let mut funcs = Vec::new();
    // let mut new_funcs = Vec::new();
    // let mut bytecode;
    let user_funcs = _get_user_funcs(parsed);

    for globes in parsed.iter() {
        let bytecode = _get_bytecode(vec![globes.clone()], &user_funcs, stdlib);
        code.push(bytecode);
        // println!("===================================");
        // break;
    }
    return code;
}

pub fn get_bc_jit<'input>(
    parsed: &parser::Node,
    user_funcs: &mut HashMap<String, StackData>,
    // envrnmt: &HashMap<String, StackData>,
    stdlib: &HashMap<
        &'input str,
        (
            Nargs,
            &'input (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'input str>),
        ),
    >,
) -> Vec<Bytecode> {
    /*
    returns a vector of instructions to execute.
    this is interpreted later.
    */
    // let user_funcs = _get_user_funcs(parsed);

    let bytecode = _get_bc_jit(vec![parsed.clone()], user_funcs, stdlib);

    return bytecode;
}
