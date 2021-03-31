extern crate rust_db;
extern crate diesel;

use self::rust_db::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use rust_db::schema::posts::dsl::*;
    
    // Establish connection
    let connection = establish_connection();
    // Get results from db
    let results = posts
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");
    
    println!("Displaying {} posts", results.len());
    // Display each result
    for post in results {
        println!("{}: {}", post.id, post.title);
        println!("--------------------");
        println!("{}", post.body);
    }
}
