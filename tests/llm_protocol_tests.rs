use primordia::{
    create_llm_client_from_env, AnthropicLlmClient, GeminiLlmClient, LlmClient, LlmProtocol,
    OpenAiLlmClient,
};

#[tokio::test]
async fn test_openai_chat_protocol_offline_fallback() {
    let client = OpenAiLlmClient::new().with_protocol(LlmProtocol::OpenAiChat);
    let res = client
        .generate_json("系统法则", "实体演化")
        .await;
    assert!(res.is_ok());
    let val = res.unwrap();
    assert!(val.get("updated_state").is_some());
}

#[tokio::test]
async fn test_openai_responses_protocol_offline_fallback() {
    let client = OpenAiLlmClient::new().with_protocol(LlmProtocol::OpenAiResponses);
    let res = client
        .generate_json("宏观天道", "纪元相变")
        .await;
    assert!(res.is_ok());
    let val = res.unwrap();
    assert!(val.get("new_atmosphere").is_some());
}

#[tokio::test]
async fn test_anthropic_messages_protocol_offline_fallback() {
    let client = AnthropicLlmClient::new();
    let res = client
        .generate_json("系统法则", "两实体共生")
        .await;
    assert!(res.is_ok());
    let val = res.unwrap();
    assert_eq!(val["outcome_type"], "ASSEMBLAGE_SYMBIOSIS");
}

#[tokio::test]
async fn test_gemini_generate_content_protocol_offline_fallback() {
    let client = GeminiLlmClient::new();
    let res = client
        .generate_json("系统法则", "实体演化")
        .await;
    assert!(res.is_ok());
    let val = res.unwrap();
    assert!(val.get("updated_state").is_some());
}

#[tokio::test]
async fn test_create_llm_client_from_env_default() {
    let client = create_llm_client_from_env();
    let res = client.generate_json("系统法则", "实体测试").await;
    assert!(res.is_ok());
}
