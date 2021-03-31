extern crate rust_db;
extern crate diesel;

use self::rust_db::*;
use diesel::prelude::*;
use self::models::Post;
use std::env::args;

fn main() {
    use rust_db::schema::posts::dsl::{posts, published};
    
    // Get command line arguments
    let id = args().nth(1).expect("publish_post requires post id")
        .parse::<i32>().expect("Invalid ID");
    let conn = establish_connection();
    
    // Update specified post
    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&conn)
        .expect(&format!("Unable to find post {}", id));

    println!("Published post {}", post.title);

}
