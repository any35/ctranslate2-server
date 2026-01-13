use ctranslate2_server::api::openai::{ChatCompletionRequest, ChatCompletionResponse};
use serde_json::json;

#[test]
fn deserialize_chat_completion_request() {
    let json = json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {"role": "system", "content": "You are a helpful assistant."},
            {"role": "user", "content": "Hello!"}
        ]
    });

    let request: ChatCompletionRequest = serde_json::from_value(json).unwrap();
    assert_eq!(request.model, "gpt-3.5-turbo");
    assert_eq!(request.messages.len(), 2);
    assert_eq!(request.messages[0].role, "system");
    assert_eq!(request.messages[0].content, "You are a helpful assistant.");
}

#[test]
fn serialize_chat_completion_response() {
    let response = ChatCompletionResponse {
        id: "chatcmpl-123".into(),
        object: "chat.completion".into(),
        created: 1677652288,
        model: "gpt-3.5-turbo".into(),
        choices: vec![],
        usage: None,
    };

    let json = serde_json::to_value(&response).unwrap();
    assert_eq!(json["id"], "chatcmpl-123");
    assert_eq!(json["object"], "chat.completion");
}
