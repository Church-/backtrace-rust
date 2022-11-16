use std::collections::HashMap;
use std::env;

use backtraceio::ResultExt;

fn main() {
    let args: Vec<String> = env::args().collect();
    let progname = args.get(0).expect("should always have a program name");
    if args.len() < 3 {
        println!("Usage: {} <url> <token>", progname);
        std::process::exit(1);
    }
    let url = args.get(1).expect("should have an args[1]");
    let token = args.get(2).expect("should have an args[2]");
    let panic_msg = env::var("PANIC_MSG").unwrap_or("I am a teapot!".to_string());
    let env_attributes = env::var("ATTRIBUTES");
    let mut attributes: HashMap<String, String> = HashMap::new();
    if let Ok(attr_str) = env_attributes {
        for kv_pair in attr_str.split(" ") {
            let kv: Vec<&str> = kv_pair.split("=").collect();
            let key = *kv.get(0).expect("should have a key");
            let val = *kv.get(1).expect("should have a value");
            attributes.insert(key.into(), val.into());
        }
    }

    backtraceio::init(&token, &url, None, Some(attributes.clone()));

    backtraceio::register_error_handler(url, token, move |r: &mut backtraceio::Report, _| {
        for (key, value) in &attributes.clone() {
            r.attributes.insert(key.to_string(), value.to_string());
        }
    });

    match std::fs::File::open("./do_not_exist").submit_error() {
        Ok(_) => {
            eprintln!("No error");
        }
        Err(_) => {
            eprintln!("Error");
        }
    }

    println!("Hello, world!");
    panic!("{}", panic_msg);
}
