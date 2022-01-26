use std::fs::File;
use std::env::args;
use std::io::{stdin,Read};
use regex::Regex;
use lazy_static::lazy_static;
lazy_static!{
    static ref IP_RE: Regex = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap();
}
fn main(){
    let mut output = String::new();
    match args().nth(1) {
        Some(v) => {
            File::open(v).expect("failed to open file").read_to_string(&mut output).expect("could not read from file");
        }
        None => {
            stdin().lock().read_to_string(&mut output).expect("could not read from stdin");
        }
    }
    IP_RE.captures_iter(&output).for_each(|v|println!("{}",v.get(0).map_or("",|m|m.as_str())));
}
