use std::io::{Read, stdin};
use diesel_demo::{create_post, establish_connection};

fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut body = String::new();

    println!("What is the title of the post?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end(); // We don't want the trailing \n

    println!("\nOk! Let's write {} (Press {} when you're done)\n",
        title,
        EOF
    );
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(connection, title, &body);
    println!("\nSaved draft {} with id {}", title, post.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
