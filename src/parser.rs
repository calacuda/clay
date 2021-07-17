/*  the parser:
    parses the token list prodused by the lexer and ouputs and outputs an ast.
*/

use crate::lexer;
// use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub parent_id: Option<NodeID>,
    pub id: NodeID,
    pub children: Vec<Node>,
    pub data: Option<lexer::Token>,
    // pub bytecode: Vec<Bytecode<'arb>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct NodeID {
    pub index: usize,
}

impl<'arb> Node {
    pub fn new(id: NodeID) -> Node {
        Node {
            id: id,
            parent_id: None,
            children: Vec::new(),
            data: None,
            // bytecode: Vec::new(),
        }
    }

    pub fn add_child(mut self, node_id: Node) -> Node {
        self.children.push(node_id);
        return self;
    }

    pub fn add_children(mut self, mut node_id: Vec<Node>) -> Node {
        self.children.append(&mut node_id);
        return self;
    }

    pub fn add_data(mut self, data: lexer::Token) -> Node {
        self.data = Some(data);
        return self;
    }

    pub fn set_parent(mut self, pid: NodeID) -> Node {
        self.parent_id = Some(pid);
        return self;
    }

    // pub fn set_bytecode(mut self, code: Vec<Bytecode<'arb>>) -> Node<'arb> {
    //     self.bytecode = code;
    //     return self;
    // }
}

impl NodeID {
    pub fn new(index: usize) -> NodeID {
        NodeID {
            index: index,
        }
    }
}

fn _parse<'arb>(lex: &mut lexer::Lexer<'arb>, token: lexer::Token, pid: usize, uid: usize,
                last_paren: char) -> (Vec<Node>, Vec<Node>, lexer::Token, (usize, usize)) { // id = id for the node.
    /*
     * this is a big, scarry, recurive function, with a loop in it. aka it's the two things
     * that should NEVER be used together. bassically it recurses though the source code based on
     * that it generates the ast. it does this by looping over tokens adn colecting them into a
     * vector if they are a symbol or number (ie. NOT left or right paren). it starts a new
     * recusion if the current token if an opening paren, and breaks the loop if the current token
     * is a clossing paren.
     *
     * I will however tell you that this functino is much less complicated then it used to be.
     * making this faster will improve the speed of the whole language!
     *
     * this outputs an AST in the form of an unsorted vector. said vector is sorted by the
     * function "parse" which is the entry point to this file and its functionality. this fuction
     * should never be called directly from outside this file.
     */
    let mut block = Vec::new();
    let mut created_ids = Vec::new();
    let mut u_id;
    let mut tok = token;
    let mut next_tok;
    let mut nt_cp;
    let mut lp = 0;
    let mut rp = 0;
    // println!("called on: {:?}", tok);

    loop {
        next_tok = lex.get_token();
        u_id = lex.pos;
        nt_cp = next_tok.clone();
        let id = NodeID::new(u_id);
        let mut node = Node::new(id).set_parent(NodeID::new(pid));

        // println!("{:?}, {:?}", tok, nt_cp);
        // println!("{:?}", tok);

        match tok {
            lexer::Token::LParen => {
                // lp += 1;
                // if next_tok == lexer::Token::RParen {
                //     // println!("reasigning next_tok");
                //     // next_tok = lex.get_token();
                //     println!("breaking");
                //     // break;
                // }

                if next_tok != lexer::Token::RParen {
                    // println!("added {:?} to node", next_tok);
                    node = node.add_data(next_tok);
                    u_id += 1;
                    next_tok = lex.get_token();
                    u_id = lex.pos;
                    // println!("before: {} {}", lp, rp);

                    let (mut child_block, cids, nt_cp, p_count) = _parse(lex, next_tok, uid, u_id + 1, 'l');

                    // let mut child_clone = child_block.clone();
                    block.append(&mut child_block);
                    node = node.add_children(cids);
                    let tmp_node = node.clone();
                    block.push(node);
                    created_ids.push(tmp_node);
                    lp += p_count.0 + 1;
                    rp += p_count.1;
                    if nt_cp == lexer::Token::EOF {
                        rp = 0;
                        lp = 0;
                    }
                    tok = nt_cp;
                    // println!("after: {} {}", lp, rp);
                }
                else {
                    next_tok = lex.get_token();
                }
            }

            lexer::Token::RParen => {
                rp += 1;
                tok = nt_cp;
                u_id += 1;
                if rp == lp {
                    rp = 0;
                    lp = 0;
                }
                break;
            }

            lexer::Token::Symbol(_) |
            lexer::Token::Number(_) |
            lexer::Token::Bool(_) |
            lexer::Token::Form(_) |
            lexer::Token::Str(_) => {
                // println!("added {:?} to node", tok);
                node = node.add_data(tok).set_parent(NodeID::new(pid));
                let tmp_node = node.clone();
                block.push(node);
                created_ids.push(tmp_node);
                tok = nt_cp;
                u_id += 1;
            }

            lexer::Token::EOF => {
                u_id += 1;
                // println!("{:?}, {:?}", tok, nt_cp);
                tok = nt_cp;
                if lp > rp { // may cause error pls error check.
                    panic!("ERROR: unclosed left parenthesis.")
                }
                else if lp < rp {
                    panic!("ERROR: too many right parenthesis.")
                }
                break;
            }
        }
    }

    return (block, created_ids, tok, (lp, rp));
}

pub fn parse(source_code: &String) -> Vec<Node> {

    let mut lex = lexer::Lexer::new(source_code);
    let next_tok = lex.get_token();
    let u_id = lex.pos;
    let (_, mut nodes, _, _) = _parse(&mut lex, next_tok, 0, u_id, 'l');
    nodes.sort_by_key(|d| d.id.index);
    return nodes;
}
