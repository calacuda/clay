use std::fs::read_to_string;

// mod lexer;
mod parser;

fn read_source(fname: String) -> String {
    read_to_string(fname).unwrap()
}

// fn main() {
//     let sc = read_source("test.lisp".to_string());
//     let mut lexer = parser::lexer::Lexer::new(&sc);
//     for i in 0..39 {
// 	println!("{:?}", lexer.get_token());
//     }
// }

fn main() {
    let sc = read_source("test.lisp".to_string());
    let parsed = parser::parse(&sc);
    // println!("{}", parsed.len());
    // println!("{:?}\n", parsed[0]);

    // for node in parsed.iter() {
    //     match node.data {
    //         Some(parser::lexer::Token::Symbol("defun")) => {
    //             println!("defun found with id: {}", node.id.index);
    //         }
    //         _ => {}
    //     }
    // }

    // let mut start = 0;

    // for start in 0..4 {
    //     println!("===========================");
    //     println!("Parent:\n{:?}:  {}", parsed[start].data, parsed[start].id.index);
    //     println!("===========================");
    //     for kid in parsed[start].children.iter() {
    //         let mut id = kid.index;
    //         for node in parsed.iter() {
    //             if node.id.index == id {
    //                 println!("{:?}:  {}", node.data, node.id.index);
    //             }
    //         }
    //         //println!("{:?}", kid);
    //     }
    //     println!("\n");
    // }

    for child in parsed.iter() {
        println!("===========================");
        println!("Parent:\n{:?}, with nodeID: {}", child.data, child.id.index);
        println!("===========================");
        println!("children of node {}", child.id.index);
        for kid in child.children.iter() {
            let id = kid.index;
            for node in parsed.iter() {
                if node.id.index == id {
                    println!("node: {:?}, with nodeID: {}", node.data, node.id.index);
                    break;
                }
            }
            //println!("{:?}", kid);
        }
        println!("\n");
    }

    // for tok in parsed.iter() {
    //     println!("{:?}\n", tok);
    // }
}
