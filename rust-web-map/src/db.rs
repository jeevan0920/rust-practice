use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::models::{Location, NewLocation, NewUser, User};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &PgConnection, new_user: NewUser) -> QueryResult<User> {
    use crate::schema::users;

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

pub fn create_location(conn: &PgConnection, new_location: NewLocation) -> QueryResult<Location> {
    use crate::schema::locations;

    diesel::insert_into(locations::table)
        .values(&new_location)
        .get_result(conn)
}
