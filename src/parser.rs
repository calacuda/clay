/*  the parser:
    parses the token list prodused by the lexer and ouputs and outputs an ast.
*/

use crate::lexer;
use clay_lib::Token;
// use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub parent_id: Option<NodeID>,
    pub id: NodeID,
    pub children: Vec<Node>,
    pub data: Option<Token>,
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

    pub fn add_child(mut self, node: Node) -> Node {
        self.children.push(node);
        return self;
    }

    pub fn add_children(mut self, mut node: Vec<Node>) -> Node {
        self.children.append(&mut node);
        return self;
    }

    pub fn add_data(mut self, data: Token) -> Node {
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
        NodeID { index: index }
    }
}

fn _parse<'arb>(
    lex: &mut lexer::Lexer<'arb>,
    // token: Token,
    // pid: usize,
    // uid: usize,
    // last_paren: char,
) -> Vec<Node> {
    // id = id for the node.
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
    let mut block: Vec<Node> = Vec::new();

    loop {
        let tok = lex.get_token();
        match tok {
            Token::LParen => {
                let mut children = _parse(lex);
                let mut root = children[0].clone().add_children(children[1..].to_vec());
                block.push(root);
            }

            Token::RParen => {
                break;
            }

            Token::Str(_)
            | Token::Bool(_)
            | Token::Form(_)
            | Token::Number(_)
            | Token::Symbol(_) => {
                let mut this_node = Node::new(NodeID::new(lex.pos)).add_data(tok);
                block.push(this_node);
                // break;
            }

            Token::EOF => {
                // panic!("theres an unclosed open paren somewhere."),
                break;
            }
        }
    }
    return block;
}

pub fn parse(source_code: &String) -> Vec<Node> {
    let mut lex = lexer::Lexer::new(source_code);
    let u_id = lex.pos;
    let mut nodes = _parse(&mut lex);
    nodes.sort_by_key(|d| d.id.index);
    // let mut imports = match nodes[0].data {
    //     Some(Token::Symbol(sym)) if sym == "import" => import(nodes[0].children),
    //     _ => Vec::new(),
    // }
    // println!("nodes: {:?}", nodes);
    return nodes;
}
