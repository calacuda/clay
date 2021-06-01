use std::fs::read_to_string;
use std::env;

mod parser;
mod bcc;
// mod iterpreter;
mod std_lib;
mod bci;
mod lexer;

fn read_source(fname: &str) -> String {
    let mut cwd = env::current_dir().unwrap();
    if fname.chars().next().unwrap() == '~' ||
       fname.chars().next().unwrap() == '/' ||
       fname.chars().next().unwrap() == '\\' {
        return read_to_string(fname).unwrap();
    }
    else {
        cwd.push(fname);
        return read_to_string(cwd).unwrap();
    }

}

fn test_parser(parserd_sc: &Vec<parser::Node>) {
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

fn test_parser2(node: &parser::Node) {
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

fn test_parser3(nodes: &Vec<parser::Node>) {
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

fn pp_bytecode<'input>(bytecode: &Vec<Vec<bcc::Bytecode<'input>>>) {
    println!("bcc output:");
    for global in bytecode {
        for code in global {
            println!("{:?}", code);
        }
        println!();
    }
}

fn test_lexer(lex: &mut lexer::Lexer) {
    println!("lexer tokens:\n");
    //for global in bytecode {
    loop {
        let tok = lex.get_token();
        println!("{:?}", tok);
        match tok {
            lexer::Token::EOF => break,
            _ => {}
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let scf = args[1].clone();
    let sc = read_source(&scf);
    // let mut lex = lexer::Lexer::new(&sc);

    // test_lexer(&mut lex);
    // println!();
    println!("{}", i64::max());
    let parsed = parser::parse(&sc);
    let stdlib = std_lib::get_std_funcs();
    // test_parser(&parsed);
    // test_parser2(&parsed[0]);
    // test_parser2(&parsed[0].children[0]);
    // test_parser2(&parsed[0].children[1]);
    // test_parser2(&parsed[0].children[2]);
    // test_parser3(&parsed);
    // println!("bcc output:\n{:?}", bcc::get_bytecode(&parsed, &stdlib));
    // println!();
    let bytecode = bcc::get_bytecode(&parsed, &stdlib);
    // println!("{} bytecode:\n", scf);
    // pp_bytecode(&bytecode);
    // println!();
    bci::do_the_things(bytecode, &stdlib);
}
