extern crate rust_db;
extern crate diesel;

use self::rust_db::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();
    
    println!("Title: ");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end(); // Drop the newline character
    
    println!("\nOk! Let's write {} (Press {} when finished\n)", title, EOF);
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(&connection, title, &body);
    println!("\nSaved draft {} with id {}", title, post.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
