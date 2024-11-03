// Import required external crates
use askama::Template;
use axum::{
    extract,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

// Structure for experimental data containing ID and condition
#[derive(Deserialize, Serialize)]
struct ExperimentData {
    id: String,
    condition: String,
}

// Template structure for the index page
#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {
    name: String,
}

// Wrapper struct for HTML templates
struct HtmlTemplate<T>(T);

// Implementation for converting templates to HTTP responses
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

// Structure representing a single experimental trial with various data fields
#[derive(Debug, Deserialize, Serialize)]
struct Trial {
    success: Option<bool>,
    timeout: Option<bool>,
    failed_images: Option<Vec<String>>,
    failed_audio: Option<Vec<String>>,
    failed_video: Option<Vec<String>>,
    trial_type: String,
    trial_index: u32,
    plugin_version: String,
    time_elapsed: u32,
    rt: Option<u32>,
    stimulus: Option<String>,
    response: Option<String>,
    task: Option<String>,
    correct_response: Option<String>,
    correct: Option<bool>,
}

// Structure containing a collection of trials
#[derive(Debug, Deserialize, Serialize)]
struct TrialData {
    trials: Vec<Trial>,
}

// Main function to set up and run the web server
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Configure routes and static file serving
    let app = Router::new()
        .route("/:name", get(root))
        .route("/trial", post(collect_trial_data))
        .nest_service("/static", ServeDir::new("static"));

    // Create and bind TCP listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // Start the server
    axum::serve(listener, app).await.unwrap();
}

// Handler for the root path that renders the template
async fn root(extract::Path(name): extract::Path<String>) -> impl IntoResponse {
    let template = HelloTemplate { name };
    HtmlTemplate(template)
}

// Handler for collecting trial data via POST requests
async fn collect_trial_data(Json(_json_value): Json<serde_json::Value>) -> StatusCode {
    if let Ok(trial_data) = serde_json::from_value::<TrialData>(_json_value) {
        let trials = trial_data.trials;
        // Process trial data and return success status
        println!("{:?}", trials[0].success);
        return StatusCode::OK;
    } else {
        // Return error status if data parsing fails
        return StatusCode::BAD_REQUEST;
    }
}
