use std;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
extern crate rustc_serialize;
use self::rustc_serialize::json;

pub fn load_users() -> std::io::Result<(HashMap<String,String>)> {
    let mut f = try!(File::open("users.json"));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    println!("{}", s);
    let decoded: HashMap<String, String> = json::decode(&s).unwrap();
    Ok(decoded)
}