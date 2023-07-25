use std::env::args;
use diesel_demo::*;
use diesel::prelude::*;
use self::models::Post;

fn main() {
    use self::schema::posts::dsl::posts;

    let post_id = args()
        .nth(1)
        .expect("get_posts requires a post id")
        .parse::<i32>()
        .expect("Invalid id");

    let connection = &mut establish_connection();

    let post = posts
        .find(post_id)
        .select(Post::as_select())
        .first(connection)
        .optional(); // Avoids throwing an error

    match post {
        Ok(Some(post)) => println!("Post with id: {} has a title: {}", post.id, post.title),
        Ok(None) => println!("Unable to find post {}", post_id),
        Err(_) => println!("An error occurred while fetching post {}", post_id),
    }
}