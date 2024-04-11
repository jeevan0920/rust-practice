use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::{
    models::{Location, NewLocation, NewUser, User},
    schema::users,
};

pub fn create_user_in_db(user: NewUser) -> Result<User, diesel::result::Error> {
    let connection = establish_connection();
    create_user(&connection, user)
}

pub fn verify_user_in_db(username: String, password: String) -> bool {
    let connection = establish_connection();
    let user = users::table
        .filter(users::username.eq(username))
        .filter(users::password_hash.eq(password))
        .first::<User>(&connection)
        .optional()
        .expect("Failed to verify user");

    user.is_some()
}

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
