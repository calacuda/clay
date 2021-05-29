use std::fs::read_to_string;

mod parser;
mod bcc;
// mod iterpreter;
mod std_lib;

fn read_source(fname: String) -> String {
    read_to_string(fname).unwrap()
}

fn test_parser(parserd_sc: &Vec<parser::Node>) {
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

fn main() {
    let sc = read_source("test.lisp".to_string());
    let parsed = parser::parse(&sc);
    let stdlib = std_lib::get_std_funcs();
    // test_parser(&parsed);
    // test_parser2(&parsed[0]);
    // test_parser2(&parsed[0].children[2]);
    println!("bcc output:\n{:?}", bcc::get_bytecode(&parsed, &stdlib));
}
