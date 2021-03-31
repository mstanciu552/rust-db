extern crate rust_db;
extern crate diesel;

use diesel::prelude::*;
use self::rust_db::*;
use std::env::args;


// Deletes based on post title
fn main() {
    use rust_db::schema::posts::dsl::*;
    
    // Gets cli args
    let target = args().nth(1).expect("Expected id target");
    let pattern = format!("%{}%", target);
    let conn = establish_connection();

    // Delete posts and save the number of posts deleted
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&conn)
        .expect("Error deleting post");

    println!("Deleted {} posts", num_deleted);
}
