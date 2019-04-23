use std::env;
use regex::{Regex,Captures};
use std::iter::Iterator;
use std::fs;
use std::io::{self,Read};
use std::process::exit;

fn envsubst(s: &String) -> String {
    let re = Regex::new("\\$(\\w+)").unwrap();
    re.replace_all(s, |caps: &Captures| {
        let var = &caps[1];
        env::var(var)
            .expect(format!("Invalid variable ${}", var).as_str())
            .to_string()

    }).to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut content: String;

    if args.len() > 1{
        let input_file_path = &args[1];
        content = match fs::read_to_string(input_file_path){
            Ok(c) => c,
            Err(_err) => {
                println!("File not found: {}", input_file_path);
                exit(1);
            }
        }
    } else {
        content = String::new();
        match io::stdin().read_to_string(&mut content) {
            Ok(_) => {},
            Err(_err) => {
                println!("Expecting stdin content or filepath argument");
                exit(1);
            }
        }
    }

    println!("{}", envsubst(&content));
    
}
