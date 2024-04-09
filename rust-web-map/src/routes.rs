use actix_web::{post, web, HttpResponse};
use log::{debug, error};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    db::{create_location, establish_connection},
    models::NewLocation,
};

#[derive(Serialize, Deserialize)]
struct AuthData {
    username: String,
    password: String,
}

// Mock implementations for demonstration
async fn create_user(_user: AuthData) -> Result<(), ()> {
    Ok(())
}

async fn verify_user(_username: String, _password: String) -> bool {
    true
}

#[post("/register")]
async fn register(user: web::Json<AuthData>) -> HttpResponse {
    let new_user = user.into_inner(); // Directly using received JSON data for demonstration
    match create_user(new_user).await {
        Ok(_) => HttpResponse::Created().json("User created successfully"),
        Err(_) => HttpResponse::BadRequest().json("Error creating user"),
    }
}

#[post("/login")]
async fn login(credentials: web::Json<AuthData>) -> HttpResponse {
    if verify_user(credentials.username.clone(), credentials.password.clone()).await {
        HttpResponse::Ok().json("Mock token")
    } else {
        HttpResponse::Unauthorized().json("Invalid credentials")
    }
}

#[post("/mark_location")]
async fn mark_location(location_data: web::Json<NewLocation>) -> HttpResponse {
    // Generate a random number between 1 and 100
    let user_id = rand::thread_rng().gen_range(1..=100000);

    // Convert the data received from the request into your NewLocation struct
    let new_location = NewLocation {
        name: location_data.name.clone(),
        description: location_data.description.clone(),
        latitude: location_data.latitude,
        longitude: location_data.longitude,

        // Assuming that user_id is available here. Otherwise, you need to modify your logic to handle user identification.
        user_id: Some(user_id), // Placeholder user_id, replace with actual logic to get the current user's id
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

// Update your config function to include the new route
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register).service(login).service(mark_location);
}
