use actix_web::{web, App, HttpServer, Responder};
use prometheus::{register, Encoder, Histogram, TextEncoder, HistogramOpts, default_registry};
use std::sync::Mutex;

#[derive(Debug)]
struct AppState {
    version_histograms: Mutex<Vec<Histogram>>,
}

#[derive(Debug, serde::Deserialize)]
struct Payload {
    version: u64,
    size: f64,
}

async fn handle_payload(payload: web::Json<Payload>, data: web::Data<AppState>) -> impl Responder {
    let version = payload.version;
    let size = payload.size;

    // Update the appropriate version histogram
    let mut histograms = data.version_histograms.lock().unwrap();
    if version > 0 && version <= histograms.len() as u64 {
        histograms[version as usize - 1].observe(size);
    }

    "Payload received successfully"
}

async fn metrics(data: web::Data<AppState>) -> impl Responder {
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();

    // Gather metrics from the AppState or default registry
    let histograms = data.version_histograms.lock().unwrap();
    if let Err(e) = encoder.encode(&histograms.iter().map(|histogram| histogram.gather()).collect::<Vec<_>>(), &mut buffer) {
        eprintln!("Failed to encode metrics: {}", e);
    }

    // Consume the encoder
    let encoded_metrics = String::from_utf8_lossy(&buffer).into_owned();

    // Return the encoded metrics
    encoded_metrics
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize version histograms
    let version_histograms: Vec<Histogram> = (1..=3)
        .map(|_| {
            let histogram_opts = HistogramOpts::new("payload_size_version", "Payload size version");
            Histogram::with_opts(histogram_opts).unwrap()
        })
        .collect();

    // Create the Actix Web App
    let app_state = web::Data::new(AppState {
        version_histograms: Mutex::new(version_histograms),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(web::resource("/payload").route(web::post().to(handle_payload)))
            .service(web::resource("/metrics").to(metrics))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}