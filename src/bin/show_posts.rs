use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;

fn main() {
    // Easing naming
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();

    // posts == posts::table and published == posts::published
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
