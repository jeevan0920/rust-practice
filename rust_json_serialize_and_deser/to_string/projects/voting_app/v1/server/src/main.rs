use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware};
use serde::{Deserialize, Serialize};
use actix_cors::Cors;
use env_logger::Env;
// use qldb::QldbClient;
use aws_config::load_from_env;

#[derive(Debug, Serialize, Deserialize)]
struct LogLevelRequest {
    level: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Candidate {
    id: String,
    name: String,
    photo: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct VoteRequest {
    voterId: String,
    candidateId: String,
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    mobileNumber: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    success: bool,
    message: String,
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct VoteResponse {
    success: bool,
    message: String,
}

// Mock database for candidates
fn get_candidate_by_id(candidate_id: &str) -> Option<Candidate> {
    // Implement your database interaction logic here
    // This is a mock implementation, replace it with your actual database queries
    match candidate_id {
        "1" => Some(Candidate {
            id: "1".to_string(),
            name: "Candidate 1".to_string(),
            photo: "url_to_candidate_1_photo".to_string(),
        }),
        "2" => Some(Candidate {
            id: "2".to_string(),
            name: "Candidate 2".to_string(),
            photo: "url_to_candidate_2_photo".to_string(),
        }),
        _ => None,
    }
}

async fn candidate_login(info: web::Json<LoginRequest>) -> impl Responder {
    // Implement candidate login logic here
    // Generate and return a token upon successful login
    let token = "candidate_access_token".to_string();
    log::info!("Candidate logged in: {:?}", info);
    HttpResponse::Ok().json(LoginResponse {
        success: true,
        message: "Login successful".to_string(),
        token,
    })
}

async fn get_candidate_info(path: web::Path<String>) -> impl Responder {
    // Fetch candidate details from the database
    if let Some(candidate) = get_candidate_by_id(&path) {
        log::info!("Candidate info retrieved: {:?}", candidate);
        HttpResponse::Ok().json(candidate)
    } else {
        log::warn!("Candidate not found for ID: {}", &path);
        HttpResponse::NotFound().body("Candidate not found")
    }
}

async fn submit_vote(vote: web::Json<VoteRequest>) -> impl Responder {
    // Implement logic to verify the vote and store it in the database
    // Return success message
    let success_message = "Vote submitted successfully".to_string();
    log::info!("Vote submitted: {:?}", vote);
    HttpResponse::Ok().json(VoteResponse {
        success: true,
        message: success_message,
    })
}

async fn set_log_level(data: web::Json<LogLevelRequest>) -> HttpResponse {
    match data.level.parse::<log::LevelFilter>() {
        Ok(level) => {
            log::set_max_level(level);
            HttpResponse::Ok().body("Log level set successfully")
        }
        Err(err) => {
            log::error!("Failed to parse log level: {}", err);
            HttpResponse::InternalServerError().body("Failed to set log level")
        }
    }
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let _config = load_from_env().await;

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive()) // Enable CORS middleware
            .wrap(middleware::Logger::default()) // Enable request logging
            .service(web::resource("/api/candidate/login").route(web::post().to(candidate_login)))
            .service(web::resource("/api/candidate/{id}").route(web::get().to(get_candidate_info)))
            .service(web::resource("/api/vote").route(web::post().to(submit_vote)))
            .service(web::resource("/api/set_log_level").route(web::post().to(set_log_level)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
