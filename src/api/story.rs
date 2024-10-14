use axum::{
    extract::{State, Json},
    routing::post,
    Router,
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
//use std::sync::Arc;

use crate::AppState;
//use crate::services::llm::LlmClient;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/generate", post(handle_story))
}

#[derive(Debug, Deserialize)]
pub struct UserChoice {
    choice: Option<usize>,
}

#[derive(Serialize)]
pub struct StoryResponse {
    story_segment: String,
    choices: Vec<String>,
}

async fn handle_story(
    State(state): State<AppState>,
    payload: Option<Json<UserChoice>>,
) -> impl IntoResponse {
    let user_choice = payload.unwrap_or(Json(UserChoice { choice: None }));
    println!("Received user choice: {:?}", user_choice);

    let llm_client = state.llm_client.clone();
    
    let current_situation = match user_choice.choice {
        Some(_) => "continues their journey",
        None => "begins their adventure",
    };

    println!("Current situation: {}", current_situation);

    match llm_client.generate_story_segment(current_situation).await {
        Ok(story_segment) => {
            println!("Generated story segment: {}", story_segment);

            // Extract choices from the story segment
            let mut choices = extract_choices(&story_segment);

            // Handle cases where we don't have exactly 3 choices
            match choices.len() {
                0 => {
                    // If no choices were extracted, generate generic ones
                    choices = vec![
                        "Explore further".to_string(),
                        "Proceed with caution".to_string(),
                        "Turn back".to_string(),
                    ];
                },
                1 | 2 => {
                    // If we have 1 or 2 choices, add generic ones to make it 3
                    let generic_choices = vec![
                        "Investigate a different area".to_string(),
                        "Ask for more information".to_string(),
                        "Take a moment to think".to_string(),
                    ];
                    choices.extend(generic_choices.into_iter().take(3 - choices.len()));
                },
                3 => {
                    // We have exactly 3 choices, do nothing
                },
                _ => {
                    // If we have more than 3 choices, truncate to 3
                    choices.truncate(3);
                },
            }

            // Debug print for choices
            println!("Choices after processing:");
            for (i, choice) in choices.iter().enumerate() {
                println!("  {}. {}", i + 1, choice);
            }

            let response = StoryResponse {
                story_segment,
                choices,
            };

            (StatusCode::OK, Json(response)).into_response()
        },
        Err(e) => {
            eprintln!("Error generating story segment: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": "Failed to generate story segment",
                "details": e.to_string()
            }))).into_response()
        }
    }
}

fn extract_choices(story_segment: &str) -> Vec<String> {
    let mut choices = Vec::new();
    let mut in_choice = false;
    let mut current_choice = String::new();

    for line in story_segment.lines() {
        if line.contains("**") && (line.contains(":") || line.to_lowercase().contains("option")) {
            // Start of a new choice
            if in_choice {
                choices.push(current_choice.trim().to_string());
                current_choice.clear();
            }
            in_choice = true;
            current_choice.push_str(line);
        } else if in_choice {
            // Continuation of the current choice
            current_choice.push(' ');
            current_choice.push_str(line.trim());
        }
    }

    // Add the last choice if there is one
    if in_choice {
        choices.push(current_choice.trim().to_string());
    }

    choices
}