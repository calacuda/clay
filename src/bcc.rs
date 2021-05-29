/*
takes the parsed code outputs bytecode.
*/

use crate::parser;
use crate::parser::lexer;
// use crate::std_lib;

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Bytecode<'input> {
    //Stack stuff
    Push(lexer::Token<'input>),
    Pop,
    Clr,
    Block(&'input str),

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
    StoreName(lexer::Token<'input>),
    LoadName(lexer::Token<'input>),
    DropName,

    //Functions
    CallFunc,
    MakeFunc(usize),
    LoadFunc(&'input str),

    //fLoW cOnTrOl
    JumpIfTrue(&'input str),
    Jump(&'input str), // could be done with "Push True" then "JumpIfTrue" but I too tierd to
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

fn _get_bytecode<'input>(parsed: Vec<parser::Node<'input>>, user_funcs: &HashSet<&'input str>, stdlib: &HashSet<&'input str>) -> Vec<Bytecode<'input>> {
    let mut gen_funcs = Vec::new();
    let mut code = Vec::new();
    // println!("\ndat:{:?}", user_funcs);

    for node in parsed.iter() {
        println!("{:?}", node.data);
        match node.data {
            Some(lexer::Token::Symbol("+" | "*" | "/" | "-")) => { // math operators.
                let mut bcode = _get_bytecode(node.children.clone(), user_funcs, stdlib);
                code.append(&mut bcode);
                match node.data {
                    Some(lexer::Token::Symbol("+")) => code.push(Bytecode::BinAdd),
                    Some(lexer::Token::Symbol("*")) => code.push(Bytecode::BinMul),
                    Some(lexer::Token::Symbol("-")) => code.push(Bytecode::BinSub),
                    Some(lexer::Token::Symbol("/")) => code.push(Bytecode::BinDiv),
                    _ => {},
                }
            }

            Some(lexer::Token::Symbol("<" | ">" | ">=" | "<=" | "==" | "!=")) => { // comparisons
                let mut bcode = _get_bytecode(node.children.clone(), user_funcs, stdlib);
                code.append(&mut bcode);
                match node.data {
                    Some(lexer::Token::Symbol("<")) => code.push(Bytecode::BinLess),
                    Some(lexer::Token::Symbol(">")) => code.push(Bytecode::BinGrtr),
                    // Some(lexer::Token::Symbol(">=")) => code.push(Bytecode::),
                    // Some(lexer::Token::Symbol("<=")) => code.push(Bytecode::),
                    Some(lexer::Token::Symbol("==")) => code.push(Bytecode::BinEqu),
                    Some(lexer::Token::Symbol("!=")) => {
                        code.push(Bytecode::BinEqu);
                        code.push(Bytecode::LogNot);
                    },
                    _ => {},
                }
            }

            Some(lexer::Token::Symbol("defun")) => {
                gen_funcs.push(node.children[0].data.clone().unwrap());
                // dat.add_func(node.children[0].data.clone().unwrap());

                let mut bcode = _get_bytecode(node.children[1..].to_vec(), user_funcs, stdlib);
                code.append(&mut bcode);

                code.push(Bytecode::MakeFunc(node.children[1].children.len() + 1));
                code.push(Bytecode::StoreName(node.children[1].data.clone().unwrap()));
                for child in &node.children[1].children {
                    code.push(Bytecode::StoreName(child.data.clone().unwrap()));
                }
                //break;
            }

            Some(lexer::Token::Symbol("if")) => {
                // code.push(Bytecode::Block("CONDITION_BLOCK")); // uncomment if conditional are broken
                let mut bcode = _get_bytecode(vec![node.children[0].clone()], user_funcs, stdlib);
                code.append(&mut bcode); // the condition form
                code.push(Bytecode::JumpIfTrue("ELSE_BLOCK"));

                let mut bcode = _get_bytecode(vec![node.children[1].clone()], user_funcs, stdlib);
                code.append(&mut bcode); // the if block
                code.push(Bytecode::Jump("END_IF"));
                code.push(Bytecode::Block("ELSE_BLOCK"));

                let mut bcode = _get_bytecode(vec![node.children[2].clone()], user_funcs, stdlib);
                code.append(&mut bcode); // the else block
                code.push(Bytecode::Block("END_IF"));
            }

            Some(lexer::Token::Symbol(name)) => {
                //println!("ERROR: generic symbol detected, fix yo broken code!");
                // let mut bcode = _get_bytecode(node.children.clone(), user_funcs);
                // code.append(&mut bcode);

                // if dat.clone().has_func(&node.data.as_ref().unwrap()) {
                //     code.push(Bytecode::CallFunc);
                // }
                if user_funcs.contains(name) | stdlib.contains(name) {
                    code.push(Bytecode::LoadFunc(name));
                    let mut bcode = _get_bytecode(node.children.clone(), user_funcs, stdlib);
                    code.append(&mut bcode);
                    code.push(Bytecode::CallFunc);
                }
                else {
                    let mut bcode = _get_bytecode(node.children.clone(), user_funcs, stdlib);
                    code.append(&mut bcode);
                    code.push(Bytecode::LoadName(lexer::Token::Symbol(name)));
                }
            }

            Some(lexer::Token::Number(n)) => {
                code.push(Bytecode::Push(node.data.clone().unwrap()));
                // break;
            }

            _ => {
                panic!("ERROR: and not even I know why!");
            }
        }
    }
    return code;
}

fn _get_user_funcs<'input>(parsed: &Vec<parser::Node<'input>>) -> HashSet<&'input str>{
    let mut user_funcs = HashSet::new();

    for globe in parsed.iter() {
        match globe.data {
            Some(lexer::Token::Symbol("defun")) => {
                match globe.children[0].data {
                    Some(lexer::Token::Symbol(func_name)) => user_funcs.insert(func_name),
                    _ => panic!("ERROR: no function name after function declaration."),
                };
            }
            _ => {}
        }
    }

    return user_funcs;
}



pub fn get_bytecode<'input>(parsed: &Vec<parser::Node<'input>>, stdlib: &HashSet<&'input str>) -> Vec<Vec<Bytecode<'input>>> { //-> Vec<Statments> {
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
        println!("===================================");
        // break;
    }
    return code;
}
