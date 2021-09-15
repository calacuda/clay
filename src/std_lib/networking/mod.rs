use reqwest;
use std::io::Read;
use clay_lib::{Token};

// fn post() -> String {
    
// }

pub fn get<'a>(data: &Vec<Token>) -> Result<Option<Token>, &'a str> {
    let url = match &data[0] {
	Token::Str(text) => text,
	_ => panic!("thats no a url."),
    };
    
    let mut res = match reqwest::blocking::get(url) {
	Ok(text) => text,
	Err(mesg) => panic!(mesg),
    };
    let mut body = String::new();
    res.read_to_string(&mut body);
    
    return Ok(Some(Token::Str(body)));
}
