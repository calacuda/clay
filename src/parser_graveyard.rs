// pub fn parse(lex: lexer::Lexer) -> Vec<Node> {
//     let mut nodes = Vec::new();
//     let i = 0;
//     //for tok in lex.tok_buf.iter() {\
//     let tok;
//     while i <= lex.tok_buf.len() {
//         tok = nodes[i];
//         match tok {
//             lexer::Token::&LParen => {
//                 let block = Vec::new();
//                 while tok != lexer::Token::
//             }
//             lexer::Token::LParen => {}
//             lexer::Token::RParen => {}
//             lexer::Token::Symbol(_) => {}
//             lexer::Token::Number(_) => {}
//             lexer::Token::EOF => break,
//         }
//         //i += 1;
//     }
//     return nodes;
// }


// fn _parse<'arb>(lex: &mut lexer::Lexer<'arb>, pid: usize) -> Vec<Node<'arb>> { // pid = id ID
//     //while tok != lexer::Token::EOF {  // could also be set to an infinate loop (ie. "while true {...)
//     let mut block = Vec::new();
//     let nid = NodeID::new(pid);
//     let cid = NodeID::new(pid + 1);
//     //let node = Node::new(nid);
//     //loop {
//     let tok = lex.get_token();
//     match tok {
//         lexer::Token::LParen => {
//             // let block = Vec::new();
//             let mut child_block = _parse(lex, pid + 1);
//             block.append(&mut child_block);
//             //let node = Node::new(nid).add_data(tok).add_child(cid);
//             //block.push(node)
//         }
//         lexer::Token::RParen => {
//             // let node = Node::new(nid).add_data(tok);
//             // block.push(node);
//             // let mut child_block = _parse(lex, pid + 1);
//             // block.append(&mut child_block);
//             // // break
//             let mut child_block = _parse(lex, pid + 1);
//             block.append(&mut child_block);
//         }
//         lexer::Token::Symbol(_) => {
//             let node = Node::new(nid).add_data(tok);
//             block.push(node);
//             let mut child_block = _parse(lex, pid + 1);
//             block.append(&mut child_block);
//         }
//         lexer::Token::Number(_) => {
//             let node = Node::new(nid).add_data(tok);
//             block.push(node);
//             let mut child_block = _parse(lex, pid + 1);
//             block.append(&mut child_block);_child_nids
//         }
//         lexer::Token::EOF => {
//             // let node = Node::new(nid).add_data(tok);
//             // block.push(node);
//             // let mut child_block = _parse(lex, pid + 1);
//             // block.append(&mut child_block);
//         },
//     }
//         //pid += 1;
//     //}
//     return block; //something
// }

// fn _parse<'arb>(lex: &mut lexer::Lexer<'arb>, token: lexer::Token<'arb>, pid: usize, uid: usize,
//                 last_paren: char) -> Vec<Node<'arb>> { // id = id for the node.
//     let mut block = Vec::new();
//     let mut u_id = uid;
//     //let pid = id;
//     //let nid = NodeID::new(uid);
//     //let cid = NodeID::new(pid + 1);
//     let mut next_tok;
//     let mut nt_cp;
//     let mut tok = token;
//
//     loop {
//         next_tok = lex.get_token();
//         nt_cp = next_tok.clone();
//         match tok {
//             lexer::Token::LParen => {
//                 let mut child_block = _parse(lex, next_tok, pid, u_id, 'l');
//                 block.append(&mut child_block);
//                 break;
//             }
//
//             lexer::Token::RParen => {
//                 let mut child_block = _parse(lex, next_tok, pid, u_id, 'r');
//                 block.append(&mut child_block);
//                 break;
//             }
//
//             lexer::Token::Symbol(_) | lexer::Token::Number(_) => {
//                 // match tok {
//                 //     lexer::Token::Symbol("defun") => {
//                 //         println!("defun found!");
//                 //     }
//                 //     _ => {}
//                 // }
//                 let mut node = Node::new(NodeID::new(u_id))
//                                     .add_data(tok)
//                                     .set_parent(NodeID::new(pid));
//                 let mut child_block = _parse(lex, next_tok, pid + 1, u_id + 1, last_paren);
//
//                 let mut child_ids = Vec::new();
//                 for nd in child_block.clone() {
//                     child_ids.push(nd.id);
//                 }
//
//                 node = node.add_children(child_ids);
//                 block.push(node);
//                 block.append(&mut child_block);
//                 //tok = lex.get_token();
//             }
//
//             lexer::Token::EOF => {
//                 break
//             }
//         }
//         tok = nt_cp;_child_nids
//         u_id += 1;
//     }
//     return block;
// }

// fn _parse<'arb>(lex: &mut lexer::Lexer<'arb>, token: lexer::Token<'arb>, pid: usize, uid: usize,
//                 last_paren: char) -> (Vec<Node<'arb>>, Vec<NodeID>) { // id = id for the node.
//     let mut block = Vec::new();
//     let mut created_ids = Vec::new();
//     let mut u_id = uid;
//     let mut next_tok;
//     let mut nt_cp;
//     let mut tok = token;
//
//     loop {
//         next_tok = lex.get_token();
//         nt_cp = next_tok.clone();
//         match tok {
//             lexer::Token::LParen => {
//                 let (mut child_block, _child_nids) = _parse(lex, next_tok, pid, u_id, 'l');
//                 block.append(&mut child_block);
//                 created_ids.push(NodeID::new(u_id));
//                 break;
//             }
//
//             lexer::Token::RParen => {
//                 let (mut child_block, _child_nids) = _parse(lex, next_tok, pid, u_id, 'r');
//                 block.append(&mut child_block);
//                 created_ids.push(NodeID::new(u_id));
//                 break;
//             }
//
//             lexer::Token::Symbol(_) | lexer::Token::Number(_) => {
//                 let id = NodeID::new(u_id);
//                 let cp_id = id.clone();
//                 let mut node = Node::new(id)
//                                     .add_data(tok)
//                                     .set_parent(NodeID::new(pid));
//                 created_ids.push(cp_id);
//                 let (mut child_block, child_nids) = _parse(lex, next_tok, pid + 1,
//                                                            u_id + 1, last_paren);
//
//                 node = node.add_children(child_nids);
//                 block.push(node);
//                 block.append(&mut child_block);
//             }
//
//             lexer::Token::EOF => {
//                 break
//             }
//         }
//         tok = nt_cp;
//         u_id += 1;
//     }
//     return (block, created_ids);
// }

// fn _parse<'arb>(lex: &mut lexer::Lexer<'arb>, tok: lexer::Token<'arb>, parent_id: usize,
//                 child_id: usize, lp_count: usize, rp_count: usize) -> Vec<Node<'arb>> { // pid = id ID
// fn _parse<'arb>(tokens: Vec<lexer::Token>) -> Vec<Node<'arb>> {

// fn split_globals<'arb>(lex: &mut lexer::Lexer<'arb>) -> Vec<Vec<Node<'arb>>> {
//
//     let mut program = Vec::new();
//     // let mut nodes = Vec::new();
//     let mut block = Vec::new();
//     let mut num_lp = 0;
//     let mut num_rp = 0;
//     let mut tok;
//     let mut node;
//     let mut pid = 0;
//
//     loop{
//         tok = lex.get_token();
//         node = Node::new(NodeID::new(pid));
//         match tok {
//             lexer::Token::LParen => {
//                 block.push(node.add_data(tok));
//                 num_lp += 1;
//             }
//
//             lexer::Token::RParen => {
//                 block.push(node.add_data(tok));
//                 num_rp += 1;
//                 if num_rp == num_lp {
//                     program.push(block);
//                     block = Vec::new();
//
//                 }
//             }
//
//             lexer::Token::Symbol(_) | lexer::Token::Number(_) => {
//                 block.push(node.add_data(tok));
//             }
//
//             // lexer::Token::Number(_) => {
//             //     block.push(tok);
//             // }
//
//             lexer::Token::EOF => {
//                 break
//             }
//         }
//     }
//     // println!("{:?}\nlen: {}", program[0], program.len());
//     return program;
// }

// fn parse_global(global: &Vec<Node>) {
//     let mut tok;
//     let mut last_paren: char = 'l';
//     for node in global.iter() {
//         tok = node.data.as_ref();
//         match tok {
//             Some(lexer::Token::LParen) => {
//                 last_paren = 'l';
//             }
//
//             Some(lexer::Token::RParen) => {
//                 if last_paren == 'l' {
//
//                 }
//                 else {
//                 last_paren = 'r';
//                 }
//             }
//
//             Some(lexer::Token::Symbol(_)) | Some(lexer::Token::Number(_)) => {
//
//             }
//
//             _ => {
//
//             }
//
//             // lexer::Token::Number(_) => {
//             //     block.push(tok);
//             // }
//         }
//     };
// }

// fn parse_globals(globals: &Vec<Vec<Node>>) { //-> Vec<Node> {
//     // println!("{:?}", globals);
//     for form in globals.iter() {
//         // println!("{:?}", form)
//         parse_global(form)
//     }
// }

// pub fn parse(source_code: &String) -> Vec<Vec<Node>> {
//     let mut lex = lexer::Lexer::new(source_code);
//     let globals = split_globals(&mut lex);
//     let ast = parse_globals(&globals);
//     return globals;
// }

// fn _parse<'arb>(lex: &mut lexer::Lexer<'arb>, token: lexer::Token<'arb>, pid: usize, uid: usize,
//                 last_paren: char) -> (Vec<Node<'arb>>, Vec<NodeID>) { // id = id for the node.
//     let mut block = Vec::new();
//     let mut created_ids = Vec::new();
//     let mut u_id = uid;
//     let mut next_tok;
//     let mut nt_cp;
//     let mut tok = token;
//
//     loop {
//         next_tok = lex.get_token();
//         nt_cp = next_tok.clone();
//         match tok {
//             lexer::Token::LParen => {
//                 let (mut child_block, _child_nids) = _parse(lex, next_tok, pid, u_id, 'l');
//                 block.append(&mut child_block);
//                 created_ids.push(NodeID::new(u_id));
//                 break;
//             }
//
//             lexer::Token::RParen => {
//                 let (mut child_block, _child_nids) = _parse(lex, next_tok, pid, u_id, 'r');
//                 block.append(&mut child_block);
//
//                 // created_ids.push(NodeID::new(u_id));
//                 break;
//             }
//
//             lexer::Token::Symbol(_) | lexer::Token::Number(_) => {
//                 let id = NodeID::new(u_id);
//                 let cp_id = id.clone();
//                 let mut node = Node::new(id)
//                                     .add_data(tok)
//                                     .set_parent(NodeID::new(pid));
//                 created_ids.push(cp_id);
//                 let (mut child_block, child_nids) = _parse(lex, next_tok, pid + 1,
//                                                            u_id + 1, last_paren);
//
//                 // if last_paren == '
//                 node = node.add_children(child_nids);
//                 block.push(node);
//                 block.append(&mut child_block);
//             }
//
//             lexer::Token::EOF => {
//                 break
//             }
//         }
//         tok = nt_cp;
//         u_id += 1;
//     }
//     return (block, created_ids);
// }

// pub fn globals(sc: &String) -> Vec<String> {
//     let mut lp = 0;
//     let mut rp = 0;
//     // let mut start: usize;
//     let mut globes = Vec::new();
//     let mut form = Vec::new();
//
//     for c in sc.chars() {
//         form.push(c);
//         match c {
//             '(' => {
//                 lp += 1;
//                 // if lp == rp + 1 {
//                 //     form = Vec::new();
//                 // }
//             }
//             ')' => {
//                 rp += 1;
//                 if lp == rp {
//                     globes.push(form.clone().into_iter().collect::<String>());
//                     form = Vec::new();
//                 }
//             }
//             _ => {}
//         };
//     }
//     return globes;
// }

// fn read_seq(lex: &mut lexer::Lexer<'arb>, token: lexer::Token<'arb>, uid: usize,)

fn _parse<'arb>(lex: &mut lexer::Lexer<'arb>, token: lexer::Token<'arb>, pid: usize, uid: usize,
                last_paren: char) -> (Vec<Node<'arb>>, lexer::Token<'arb>, (usize, usize)) { // id = id for the node.
    /*
     * this version is designed to use a vector of vector of tokens (Vec<Vec<lexer::Token>>)
     * this is so the global scope can be split first into the forms that exsist in the clobal
     * scope. then this is called on etch of those forms. this function recursively generates the
     * AST.
     *
     * expected problems:
     * 1. lossing where the loop is in the list of tokens, this can be solved with a for loop
     */
    // println!("processing: {:?}", token);
    if token == lexer::Token::Symbol("defun") {
        println!("defun found");
    }
    let mut block = Vec::new();
    let mut u_id = uid;
    let mut tok = token;
    let mut next_tok;
    let mut nt_cp;
    let mut lp = 0;
    let mut rp = 0;

    loop {
        next_tok = lex.get_token();
        nt_cp = next_tok.clone();
        let id = NodeID::new(u_id);
        let cp_id = id.clone();
        let mut node = Node::new(id);

        match (&tok, &next_tok) {
            (&lexer::Token::LParen, _) => {
                lp += 1;
                let (mut child_block, nt_cp, p_count) = _parse(lex, next_tok, pid, u_id + 1, 'l');
                block.append(&mut child_block);
                lp += p_count.0;
                rp += p_count.1;
                // let node = Node::new(NodeID::new(u_id))
                //                 .add_data(tok)
                //                 .set_parent(NodeID::new(pid))
                //                 .add_children(_child_nids);
                // block.push(node);

                tok = nt_cp;
                u_id += 1;
                // break;
            }

            (&lexer::Token::RParen, _) => {
                //let (mut child_block, nt_cp, p_count) = _parse(lex, next_tok, pid, u_id, 'r');
                //block.append(&mut child_block);
                // lp += p_count.0;
                // rp += p_count.1;
                rp += 1;
                tok = nt_cp;
                u_id += 1;
                break;
            }

            (&lexer::Token::Symbol(_) | &lexer::Token::Number(_), &lexer::Token::RParen) => {
                let id = NodeID::new(u_id);
                let cp_id = id.clone();
                let mut node = Node::new(id)
                                    .add_data(tok)
                                    .set_parent(NodeID::new(pid));
                block.push(node);
                //block.append(&mut child_block);
                tok = nt_cp;
                u_id += 1;
            }

            (&lexer::Token::Symbol(_) | &lexer::Token::Number(_), _) => {
                if tok == lexer::Token::Symbol("defun") {
                    println!("defun found");
                }
                node = node.add_data(tok).set_parent(NodeID::new(pid));
                // let (mut child_block, child_nids, nt_cp) = _parse(lex, next_tok, pid + 1,
                //                                            u_id + 1, last_paren);
                // node = node.add_children(child_nids);
                block.push(node);
                // block.append(&mut child_block);
                tok = nt_cp;
                u_id += 1;
            }

            // lexer::Token::Symbol(_) | lexer::Token::Number(_) => {
            //     let mut nt_cp;
            //     loop {
            //         next_tok = lex.get_token();
            //         nt_cp = next_tok.clone();
            //         match tok {
            //             lexer::Token::LParen => {
            //                 let (mut child_block, _child_nids) = _parse(lex, next_tok, pid, u_id + 1, 'l');
            //                 block.append(&mut child_block);
            //                 let node = Node::new(NodeID::new(u_id))
            //                                 .set_parent(NodeID::new(pid))
            //                                 .add_children(_child_nids);
            //                 created_ids.push(NodeID::new(u_id));
            //                 block.push(node);
            //                 break;
            //             }
            //
            //             lexer::Token::RParen => {
            //                 // let (mut child_block, _child_nids) = _parse(lex, next_tok, pid, u_id + 1, 'r');
            //                 // block.append(&mut child_block);
            //                 // created_ids.push(NodeID::new(u_id));
            //                 break;
            //             }
            //
            //             lexer::Token::Symbol(_) | lexer::Token::Number(_) => {
            //                 let id = NodeID::new(u_id);
            //                 let cp_id = id.clone();
            //                 let node = Node::new(id)
            //                                     .add_data(tok)
            //                                     .set_parent(NodeID::new(pid));
            //                 created_ids.push(cp_id);
            //                 block.push(node);
            //             }
            //
            //             lexer::Token::EOF => {
            //                 //panic!("unmatched opening parenthesis.");
            //                 break;
            //             }
            //         }
            //         tok = nt_cp;
            //         u_id += 1;
            //     }
            // }

            (&lexer::Token::EOF, _) => {
                u_id += 1;
                tok = nt_cp;
                break;
            }
        }
    }
    return (block, tok, (lp, rp));
}
