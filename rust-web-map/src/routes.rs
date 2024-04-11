use actix_web::{post, web, HttpResponse};
use log::{debug, error};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    db::{create_location, create_user_in_db, establish_connection, verify_user_in_db},
    models::{NewLocation, NewUser},
};
#[derive(Serialize, Deserialize)]
struct AuthData {
    username: String,
    password: String,
}

#[post("/register")]
async fn register(user: web::Json<AuthData>) -> HttpResponse {
    let new_user = NewUser {
        username: user.username.clone(),
        password_hash: user.password.clone(),
    };
    match create_user_in_db(new_user) {
        Ok(_) => HttpResponse::Created().json("User created successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Error creating user"),
    }
}

#[post("/login")]
async fn login(credentials: web::Json<AuthData>) -> HttpResponse {
    if verify_user_in_db(credentials.username.clone(), credentials.password.clone()) {
        HttpResponse::Ok().json("Mock token")
    } else {
        HttpResponse::Unauthorized().json("Invalid credentials")
    }
}

#[post("/mark_location")]
async fn mark_location(location_data: web::Json<NewLocation>) -> HttpResponse {
    let user_id = rand::thread_rng().gen_range(1..=100000);

    let new_location = NewLocation {
        name: location_data.name.clone(),
        description: location_data.description.clone(),
        latitude: location_data.latitude,
        longitude: location_data.longitude,
        user_id: Some(user_id),
    };

    let conn = establish_connection();
    match create_location(&conn, new_location) {
        Ok(location) => {
            debug!("Location marked successfully: {:?}", location);
            HttpResponse::Created().json(location)
        }
        Err(e) => {
            error!("Failed to mark location {}", e);
            HttpResponse::InternalServerError().json("Failed to mark location")
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register).service(login).service(mark_location);
}
