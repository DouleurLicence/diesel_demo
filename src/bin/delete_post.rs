use std::env::args;
use diesel_demo::*;
use diesel::prelude::*;

fn main() {
    use self::schema::posts::dsl::*;

    args()
        .nth(1)
        .expect("Expected a target to match against");

    let connection = &mut establish_connection();

    let num_deleted = diesel::delete(title.like(pattern))
        .execute(connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}