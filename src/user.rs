//! Code related to user management

use std;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use rustc_serialize::json;
use std::io::{Result, Error, ErrorKind};

/// Attempts to load a HashMap of users from `users.json`.
///
/// Now that I think about it, it should probably be private.
///
/// # Examples
///
/// ```
/// use telnet_server::user;
/// assert!(user::load_users().is_ok());
/// ```
pub fn load_users() -> Result<(HashMap<String, String>)> {
    let mut f = try!(File::open("users.json"));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    println!("{}", s);
    let decoded: HashMap<String, String> = json::decode(&s).unwrap();
    Ok(decoded)
}

/// Attempts to save a HashMap of users as JSON.
pub fn save_users(users: HashMap<String, String>) -> Result<()> {
    let s = match json::encode(&users) {
        Ok(v) => v,
        Err(_) => return Err(Error::new(ErrorKind::Other, "JSON encoding exploded"))
    };
    let mut f = try!(File::create("users.json"));
    try!(f.write_all(&s.into_bytes()));
    Ok(())
}

/// Creates a test user and saves it.
fn create_test_users() -> Result<()> {
    let mut users = HashMap::new();
    users.insert("colin", "12345");
    let s = json::encode(&users).unwrap();
    println!("{}", s);
    let decoded: HashMap<String, String> = json::decode(&s).unwrap();
    println!("{:?}", decoded.get("colin"));
    let mut f = try!(File::create("users.json"));
    try!(f.write_all(&s.into_bytes()));
    Ok(())
}

/// Add a user with the given username and password.
pub fn add_user(user: &str, pwd: &str) -> Result<()> {
    let mut cur = try!(load_users());
    cur.insert(user.to_string(), pwd.to_string());
    save_users(cur)
}

/// Check if a supplied password is correct.
pub fn check_user(user: &str, pwd: &str) -> bool {
    let users = load_users().unwrap();
    users[user] == pwd.to_string()
}