//bytecode interpreter

use crate::bcc::{Bytecode, Nargs};
use crate::lexer;
use crate::lexer::Token;
use std::collections::HashMap;

// fn function(params: Vec<Token>, body_code: Vec<Token>, enviornment: HashMap<Token, Token>) {
//
// }

#[derive(Debug, Clone)]
pub enum StackData<'input> {
    Func(Function<'input>),
    StdFunc(Token<'input>),
    Tok(Token<'input>),
}

#[derive(Debug, Clone)]
pub struct Function<'foobar> {
    pub name: &'foobar str,
    pub nargs: Nargs,
    pub params: Vec<Token<'foobar>>,
    pub code: Vec<Bytecode<'foobar>>,
    // pub tok: Option<lexer::Token<'foobar>>
}

impl<'foobar> Function<'foobar> {
    fn new<'input>() -> Function<'input> {
        Function {
            name: "",
            nargs: Nargs::Num(0),
            params: Vec::new(),
            code: Vec::new(),
            // tok: None
        }
    }

    fn set_name<'input>(&mut self, name: &'foobar str) {
        self.name = name;
    }

    fn set_nargs<'input>(&mut self, nargs: Nargs) {
        self.nargs = nargs;
    }

    fn add_param<'input>(&mut self, param: Token<'foobar>) {
        self.params.push(param);
    }

    fn add_code(&mut self, tok: Bytecode<'foobar>) {
        self.code.push(tok);
    }

    // fn set_tok(&mut self, tok: lexer::Token<'foobar>) {
    //     self.tok = Some(tok);
    // }
}

fn get_params<'input>(
    nargs: usize,
    stack: &mut Vec<StackData<'input>>,
    user_names: &mut HashMap<&str, StackData<'input>>,
    stdlib: &HashMap<&'input str, (Nargs, &'input (dyn Fn(Vec<Token<'input>>) -> Result<Option<Token<'input>>, &'input str>))>,
) -> (&'input str, Vec<Token<'input>>) {
    let mut params = Vec::new();
    let mut fname = "DEFAULT";
    // println!("get_params stack: {:?}", stack);
    // println!("get_params stack: {:?}", stack.len());

    for _ in 0..stack.len() {
        let dat = stack.pop();
        // println!("stack elm: {:?}", dat);
        match dat {
            Some(StackData::Func(f)) => {
                // println!("asigning fname to: {}", f.name);
                fname = f.name;
                break;
            }

            Some(StackData::StdFunc(lexer::Token::Symbol(f))) => {
                // println!("asigning fname to: {}", f);
                fname = f;
                break;
            }

            Some(StackData::StdFunc(_)) => {
                panic!("nondescript StackData::StdFunc");
            }

            Some(StackData::Tok(tok)) => {
                // println!("adding \"{:?}\" to the params list.", tok);
                params.push(tok);
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

fn make_args<'input>(
    args: Vec<Token<'input>>,
    params: &Vec<Token<'input>>,
) -> HashMap<&'input str, StackData<'input>> {
    // println!("make_args args: {:?}", args);
    // println!("make_args params: {:?}", params);
    assert_eq!(args.len(), params.len());
    let mut enviornment = HashMap::new();
    // println!("args: {:?}", args);
    for i in [0..args.len()] {
        let i_2 = i.clone();
        let param_name = match params[i].to_vec()[0] {
            lexer::Token::Symbol(sym) => {
                enviornment.insert(sym, StackData::Tok(args[i_2].to_vec()[0].clone()))
            }
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

fn call_func<'input>(
    nargs: usize,
    stack: &mut Vec<StackData<'input>>,
    _envrnmt: &mut HashMap<&'input str, StackData>,
    user_names: &mut HashMap<&'input str, StackData<'input>>,
    stdlib: &HashMap<&'input str, (Nargs, &'input (dyn Fn(Vec<Token<'input>>) -> Result<Option<Token<'input>>, &'input str>))>,
) {
    // println!("envrnmt: {:?}", envrnmt);
    let (func_name, mut params) = get_params(nargs, stack, user_names, stdlib);
    // println!("call_func stack: {:?}", stack);
    // println!("call_func func_name: {:?}", func_name);
    // println!("call_func params: {:?}", params);

    // println!("call_func u_names: {:?}", user_names);
    let working_user_name = user_names.clone();

    if stdlib.contains_key(func_name) {
        // println!("f {}", f);
        let func_details = stdlib.get(func_name);
        match func_details.unwrap().0 {
            Nargs::INF => {}
            Nargs::Num(num) => {
                if num != nargs {
                    panic!("wrong number of args");
                }
            }
        }
        // println!("calling function {:?}", func_name);
        let return_val = func_details.unwrap().1(params).clone();
        match return_val.clone() {
            Ok(Some(tok)) => stack.push(StackData::Tok(tok)),
            Ok(None) => {},
            Err(_) => {},
            // Err(mesg) => panic!(mesg),
        }
        return;
    } else if working_user_name.contains_key(func_name) {
        // println!("calling function: {:?}", func_name);
        let func_details = working_user_name.get(func_name).unwrap();
        let func;
        match func_details {
            StackData::Func(f) => {
                func = f;
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
                // println!("func_name: \"{}\" was not called", func_name);
                panic!("func not called");
            }
        }

        // let env = make_args(params, func.params);
        // println!("func_details: {:?}", func_details);
    }
}

fn bin_add<'input>(stack: &mut Vec<StackData<'input>>) {
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

fn bin_sub<'input>(stack: &mut Vec<StackData<'input>>) {
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

fn bin_mul<'input>(stack: &mut Vec<StackData<'input>>) {
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

fn bin_div<'input>(stack: &mut Vec<StackData<'input>>) {
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

fn store_name<'input>(
                      stack: &mut Vec<StackData<'input>>,
                      name_space: &mut HashMap<&'input str, StackData<'input>>,
                      name: &'input str,
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
    name_space.insert(name, tmp_var);
}

fn bin_num_comp<'input>(stack: &mut Vec<StackData<'input>>, sign: char) {
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

fn do_the_thing<'input>(
    thing: &Vec<Bytecode<'input>>,
    stack: &mut Vec<StackData<'input>>,
    envrnmt: &mut HashMap<&'input str, StackData<'input>>,
    user_names: &mut HashMap<&'input str, StackData<'input>>,
    stdlib: &HashMap<&'input str, (Nargs, &'input (dyn Fn(Vec<Token<'input>>) -> Result<Option<Token<'input>>, &'input str>))>,
    func_mode: bool,
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
                        if user_names.contains_key(func_name) {
                            stack.push(user_names.get(func_name).unwrap().clone());
                        } else if stdlib.contains_key(func_name) {
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
                let value = if envrnmt.contains_key(name) {
                    envrnmt.get(name)
                } else if user_names.contains_key(name) {
                    user_names.get(name)
                } else {
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
                if func_mode {
                    store_name(stack, envrnmt, name.clone());
                }
                else {
                    store_name(stack, user_names, name.clone());
                }
            }
            Some(Bytecode::StoreName(Token::LParen | Token::RParen)) => panic!("ERROR: parenthesis are a reserved token."),
            Some(Bytecode::StoreName(_)) => panic!("ERROR: can't us a data type as a variable name."),
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
                StackData::Tok(lexer::Token::Bool(false)) => {
                    // println!("jumping if");
                    for _ in 0..distance.clone() {
                        loc_thing.next();
                    }
                }
                StackData::Tok(lexer::Token::Bool(true)) => {}, // println!("not jumping if");}
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

fn function_com<'input>(
    thing: &mut Vec<Bytecode<'input>>,
    stack: &mut Vec<StackData<'input>>,
    envrnmt: &mut HashMap<&'input str, StackData<'input>>,
    user_names: &mut HashMap<&'input str, StackData<'input>>,
    stdlib: &HashMap<&'input str, (Nargs, &'input (dyn Fn(Vec<Token<'input>>) -> Result<Option<Token<'input>>, &'input str>))>,
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
        Bytecode::MakeFunc(_) => {thing.pop();}
        _ => {
            thing.push(Bytecode::StoreName(Token::Symbol(f.name)));
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
                    f.add_param(lexer::Token::Symbol(name));
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
    f.set_nargs(Nargs::Num(nargs));
    user_names.insert(f.name, StackData::Func(f));
}

pub fn do_the_things<'input>(
    things: Vec<Vec<Bytecode<'input>>>,
    stdlib: &HashMap<&'input str, (Nargs, &'input (dyn Fn(Vec<Token<'input>>) -> Result<Option<Token<'input>>, &'input str>))>,
) {
    /*
    do the things:

    the entry point for the byte code interpreter. run this function to interpret bytecode.
    */
    let mut stack: Vec<StackData> = Vec::new();
    let mut user_names: HashMap<&'input str, StackData> = HashMap::new();
    let mut envrnmt: HashMap<&'input str, StackData> = HashMap::new();

    for g_block in things {
        // println!("g_block[0]: {:?}", g_block[0]);
        match g_block.last().unwrap() {
            Bytecode::StoreName(_) => {
                function_com(&mut g_block.clone(), &mut stack, &mut envrnmt, &mut user_names, stdlib)
            }
            _ => do_the_thing(&g_block, &mut stack, &mut envrnmt, &mut user_names, stdlib, false),
        }
        // println!("user_funcs: {:?}", user_names.keys());
        // println!("times-two: {:?}", user_names.get("times-two"));

        // break;
    }
}
