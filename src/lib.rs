use std::env;
use diesel::{Connection, PgConnection};
use diesel::prelude::*;
use dotenvy::dotenv;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    // Loads the .env file
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Connects to the database using its URL
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
