/*  the parser:
    parses the token list prodused by the lexer and ouputs and outputs an ast.
*/

pub mod lexer;
// use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node<'arb> {
    pub parent_id: Option<NodeID>,
    pub id: NodeID,
    pub children: Vec<NodeID>,
    pub data: Option<lexer::Token<'arb>>,
}

#[derive(Debug, Clone)]
pub struct NodeID {
    pub index: usize,
}

impl<'arb> Node<'arb> {
    pub fn new(id: NodeID) -> Node<'arb> {
        Node {
            id: id,
            parent_id: None,
            children: Vec::new(),
            data: None,
        }
    }

    pub fn add_child(mut self, node_id: NodeID) -> Node<'arb> {
        self.children.push(node_id);
        return self;
    }

    pub fn add_children(mut self, mut node_id: Vec<NodeID>) -> Node<'arb> {
        self.children.append(&mut node_id);
        return self;
    }

    pub fn add_data(mut self, data: lexer::Token<'arb>) -> Node<'arb> {
        self.data = Some(data);
        return self;
    }

    pub fn del_child(mut self) -> Node<'arb> {
        self.children = Vec::new();
        return self;
    }

    pub fn set_parent(mut self, pid: NodeID) -> Node<'arb> {
        self.parent_id = Some(pid);
        return self;
    }
}

impl NodeID {
    pub fn new(index: usize) -> NodeID {
        NodeID {
            index: index,
        }
    }
}

// fn merge_sort<'arb>(alpha: Vec<Node<'arb>>, beta: Vec<Node<'arb>>) -> Vec<Node<'arb>> {
//     let mut merged = Vec::new();
//     let mut a = 0;
//     let mut b = 0;
//     let total_len = alpha.len() + beta.len();
//
//     while merged.len() < total_len {
//         if alpha[a].id.index <= beta[b].id.index {
//             merged.push(alpha[a].clone());
//             a += 1;
//         }
//         else {
//             merged.push(beta[b].clone());
//             b += 1;
//         }
//     }
//     // for i in 0..std::cmp::max(alpha.len(), beta.len()) {
//     //
//     // }
//     return merged;
// }

fn partition<'arb>(arr: &mut Vec<Node<'arb>>, low: usize, high: usize) -> usize {
    let mut i = low - 1;
    let pivot = arr[high].id.index;

    for j in low..high {
        if arr[j].id.index <= pivot {
            i += 1;
            arr.swap(i, j);
        }
    }
    arr.swap(i+1, high);
    return i + 1;
}

// fn quick_sort<'arb>(arr: &mut Vec<Node<'arb>>, low: usize, high: usize) -> Vec<Node<'arb>> {
//     if low < high {
//         let pi = partition(&mut arr, low, high);
//         quick_sort(arr, low, pi-1);
//         quick_sort(arr, pi+1, high);
//     }
//     return arr;
// }

fn _parse<'arb>(lex: &mut lexer::Lexer<'arb>, token: lexer::Token<'arb>, pid: usize, uid: usize,
                last_paren: char) -> (Vec<Node<'arb>>, Vec<NodeID>, lexer::Token<'arb>, (usize, usize)) { // id = id for the node.
    /*
     * this version is designed to use a vector of vector of tokens (Vec<Vec<lexer::Token>>)
     * this is so the global scope can be split first into the forms that exsist in the clobal
     * scope. then this is called on etch of those forms. this function recursively generates the
     * AST.
     *
     * expected problems:
     * 1. lossing where the loop is in the list of tokens, this can be solved with a for loop
     */
    // if token == lexer::Token::Symbol("defun") {
    //     println!("defun found");
    // }
    let mut block = Vec::new();
    let mut created_ids = Vec::new();
    let mut u_id = uid;
    let tt = token.clone();
    let mut tok = token;
    let mut next_tok;
    let mut nt_cp;
    let mut lp = 0;
    let mut rp = 0;
    let mut sym_seen = false;

    loop {
        next_tok = lex.get_token();
        u_id = lex.pos;
        nt_cp = next_tok.clone();
        let id = NodeID::new(u_id);
        let cp_id = id.clone();
        let mut node = Node::new(id).set_parent(NodeID::new(pid));
        if next_tok == lexer::Token::Symbol("if") {
            println!("if is next_tok.");
        }

        match (&tok, &next_tok) {
            (&lexer::Token::LParen, _) => {
                lp += 1;
                // println!("(lp, _):  {:?}", next_tok);
                // println!("left_paren: tok {:?}", next_tok);
                // print!("{:?} : {:?} : ", tok, next_tok);
                node = node.add_data(next_tok);
                u_id += 1;
                next_tok = lex.get_token();
                u_id = lex.pos;
                // nt_cp = next_tok.clone();
                println!("{:?}", next_tok);
                let (mut child_block, cids, nt_cp, p_count) = _parse(lex, next_tok, uid, u_id + 1, 'l');
                block.append(&mut child_block);
                node = node.add_children(cids);
                block.push(node);
                created_ids.push(cp_id);
                lp += p_count.0;
                rp += p_count.1;
                tok = nt_cp;
            }

            (&lexer::Token::RParen, _) => {
                rp += 1;
                tok = nt_cp;
                u_id += 1;
                if rp == lp {
                    rp = 0;
                    lp = 0;
                }
                break;
            }

            (&lexer::Token::Symbol(_) | &lexer::Token::Number(_), &lexer::Token::RParen) => {
                // println!("(s/n, rp):  {:?}", tok);
                if tok == lexer::Token::Symbol("if") {
                    println!("find me in your code, {:?}:  {:?}", tok, created_ids);
                }
                // println!("tok {:?}", tok);
                node = node.add_data(tok)
                           .set_parent(NodeID::new(pid));
                block.push(node);
                created_ids.push(cp_id);
                tok = nt_cp;
                u_id += 1;
            }

            (&lexer::Token::Symbol(_) | &lexer::Token::Number(_), &lexer::Token::LParen) => {
                // if tok == lexer::Token::Symbol("defun") {
                //     println!("defun found");
                // }
                // println!("(s/n, _):  {:?}", tok);
                // println!("before lp: tok: {:?}, next_tok: {:?}", tok, next_tok);

                node = node.add_data(tok).set_parent(NodeID::new(pid));
                block.push(node);
                created_ids.push(cp_id);
                tok = nt_cp;
                u_id += 1;
            }

            (&lexer::Token::Symbol(_) | &lexer::Token::Number(_), _) => {
                // if tok == lexer::Token::Symbol("defun") {
                //     println!("defun found");
                // }
                // println!("(s/n, _):  {:?}", tok);
                // println!("NOT before Rp: tok {:?}", tok);

                node = node.add_data(tok).set_parent(NodeID::new(pid));
                block.push(node);
                created_ids.push(cp_id);
                tok = nt_cp;
                u_id += 1;
            }

            (&lexer::Token::EOF, _) => {
                u_id += 1;
                tok = nt_cp;
                break;
            }
        }
    }
    // if tt == lexer::Token::Symbol("if") {
    //     println!("{}:  {:?}", pid, created_ids);
    // }
    return (block, created_ids, tok, (lp, rp));
}

pub fn parse(source_code: &String) -> Vec<Node> {

    let mut lex = lexer::Lexer::new(source_code);
    let next_tok = lex.get_token();
    let u_id = lex.pos;
    let (mut nodes, _, _, _) = _parse(&mut lex, next_tok, 0, u_id, 'l');
    nodes.sort_by_key(|d| d.id.index);
    return nodes;
}
