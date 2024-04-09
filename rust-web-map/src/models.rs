use crate::schema::locations;
use crate::schema::users;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

// Placeholder for models
#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String, // In practice, store hashed passwords
}

#[derive(Debug, Queryable, Serialize, Deserialize, Associations, Identifiable)]
#[belongs_to(User)]
#[table_name = "locations"]
pub struct Location {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    // Ensure the types match with your database schema for latitude and longitude
    pub latitude: f64,
    pub longitude: f64,
    // This field should match the type and nullability of the user_id column in your locations table
    pub user_id: Option<i32>, // Or Option<i32> if it's nullable
}

#[derive(Insertable, Deserialize)]
#[table_name = "locations"]
pub struct NewLocation {
    pub name: String,
    pub description: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub user_id: Option<i32>, // Or Option<i32> if it's nullable
}

#[derive(Debug)]
pub struct Endorsement {
    pub id: i32,
    pub location_id: i32,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password_hash: String, // Consider storing only hashed passwords
}
