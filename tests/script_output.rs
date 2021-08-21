use std::fs;

fn test_file(name: &str) {
    use std::process::Command;
 
    let dif = Command::new("bash")
        .arg("-c")
	//.arg("pwd")                                                                                                                               
        .arg(format!("./target/debug/clay jit test_inputs/{fname}.lisp | diff test_output/{fname}.out -", fname=&name))
        //.arg("|")
	//.arg(format!("diff test_ouput/{fname}.out -\"", fname=&name))
        .output()
        //.expect("error is checking difference");
	.unwrap();
    //panic!("name :  {:?}\ndif : {:?}", name, String::from_utf8(dif.stdout).unwrap());
    assert!(String::from_utf8(dif.stdout).unwrap() == String::from(""));
    assert!(String::from_utf8(dif.stderr).unwrap() == String::from(""));
}

#[test]
fn scripts() {
    use std::process::Command;
    
    let paths = fs::read_dir("./test_inputs").unwrap();
    Command::new("cargo build");
    for fname in paths {
        let name = match fname.unwrap().file_name().into_string() {
            Ok(f) => {
                println!("\nrunning test on file :  {:?}", f);
                f.split(".").collect::<Vec<&str>>()[0].to_owned()
            },
            Err(_) => panic!("an error ocured in the testing script."),
        };                                                      
        test_file(&name);
    }
}

fn main() {
    scripts();
} 
