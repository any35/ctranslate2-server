use crate::{model::ModelError, state::AppState};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatCompletionMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatCompletionMessage>,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub max_tokens: Option<u32>,
    /// Extension: Target language code (e.g. "fra_Latn")
    pub target_lang: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatCompletionChoice {
    pub index: u32,
    pub message: ChatCompletionMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: Option<Usage>,
}

pub enum ApiError {
    BadRequest(String),
    InternalServerError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}

pub async fn chat_completions(
    State(state): State<AppState>,
    Json(request): Json<ChatCompletionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // specific logic: take the last user message as prompt
    let prompt = request
        .messages
        .last()
        .map(|m| m.content.clone())
        .ok_or_else(|| ApiError::BadRequest("No messages provided".to_string()))?;

    // Model resolution is now handled by ModelManager (including aliases and defaults)
    // We pass the requested model name directly.
    let results = state
        .model_manager
        .generate(&request.model, vec![prompt], request.target_lang)
        .await
        .map_err(|e| match e {
            ModelError::NotFound { .. } | ModelError::ConfigNotFound { .. } => {
                ApiError::BadRequest(format!("Model error: {}", e))
            }
            _ => ApiError::InternalServerError(format!("Inference failed: {}", e)),
        })?;

    let response_text = results.first().cloned().unwrap_or_default();

    let response = ChatCompletionResponse {
        id: "chatcmpl-123".to_string(), // TODO: UUID
        object: "chat.completion".to_string(),
        created: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        model: request.model.clone(),
        choices: vec![ChatCompletionChoice {
            index: 0,
            message: ChatCompletionMessage {
                role: "assistant".to_string(),
                content: response_text,
            },
            finish_reason: Some("stop".to_string()),
        }],
        usage: None,
    };

    Ok(Json(response))
}
