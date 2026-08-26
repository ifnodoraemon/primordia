use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::sync::Arc;

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn generate_json(&self, system_prompt: &str, user_prompt: &str) -> Result<Value, String>;
}

/// 支持的多模型接入协议类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LlmProtocol {
    /// 1. OpenAI 格式一：Chat Completions API (/v1/chat/completions)
    OpenAiChat,
    /// 2. OpenAI 格式二：Responses API (/v1/responses)
    OpenAiResponses,
    /// 3. Anthropic 格式：Claude Messages API (/v1/messages)
    AnthropicMessages,
    /// 4. Google 格式：Gemini GenerateContent API (/v1beta/models/{model}:generateContent)
    GeminiGenerateContent,
}

/// 辅助函数：从大模型文本输出中清理并提取合法 JSON
fn parse_json_from_llm_text(text: &str) -> Result<Value, String> {
    let trimmed = text.trim();
    let cleaned = if trimmed.starts_with("```json") && trimmed.ends_with("```") {
        &trimmed[7..trimmed.len() - 3].trim()
    } else if trimmed.starts_with("```") && trimmed.ends_with("```") {
        &trimmed[3..trimmed.len() - 3].trim()
    } else {
        trimmed
    };

    serde_json::from_str(cleaned).map_err(|e| {
        format!(
            "Failed to parse model content as JSON: {} | Raw output: {}",
            e, cleaned
        )
    })
}

/// 离线模拟第一性原理回退推演
fn mock_offline_reasoning(system_prompt: &str, user_prompt: &str) -> Value {
    if system_prompt.contains("宏观天道") || system_prompt.contains("Cosmic Law") || system_prompt.contains("Cosmic Arbiter") {
        return serde_json::json!({
            "new_atmosphere": "太初辉光纪元：星辰灵尘与地热熵流谐振，虚空中涌现微弱的引力拓扑网 / Era of Primordial Radiance: Stardust and thermal entropy resonate, weaving gravitational topology in the void",
            "cosmic_ripple": "天道法则发生微观跃迁，万物感官视界向外延伸三舍 / World laws shift subtly; entity perceptual horizons expand outward"
        });
    }

    if user_prompt.contains("共生") || user_prompt.contains("Symbiosis") {
        return serde_json::json!({
            "outcome_type": "ASSEMBLAGE_SYMBIOSIS",
            "narrative": "两股原初灵性未发生消解，而是编织成共生装配体，彼此交换感知与光晕 / The two entities entwine into a rhizomatic assemblage, exchanging perceptions and radiance without losing individuality",
            "update_a": "体内沉淀出与对方共振的脉络 / Forms internal resonance channels attuned to the partner",
            "update_b": "外层激荡出护持母体的温润光晕 / Radiates a warm halo protecting the host companion",
            "born_entity": null
        });
    }

    serde_json::json!({
        "updated_state": "表面渗透出微弱的虹彩辉光，内部物质开始自发分层旋转，呼吸着虚空中的微光",
        "new_traits": ["虹彩辉光 / Iridescent Sheen", "内部自旋 / Internal Spin"],
        "new_memory": "感受到了时光与宏观天道在周围留下的涟漪",
        "narrative": "两股原初能量在虚空中交汇，激荡出微弱的共鸣波纹 / Two primordial energies converge, rippling across the void",
        "outcome_type": "MUTUAL_CHANGE",
        "action_result": "意识与物质发生谐振，形态产生轻微位移与流变",
        "subject_new_state": "内部核心泛起明亮光晕，以共鸣姿态朝向目标",
        "environmental_ripple": "周围三丈内的气流化作细微的旋转涡流"
    })
}

// =========================================================================
// 1 & 2. OpenAI 客户端（支持 Chat Completions 与 Responses 两种协议格式）
// =========================================================================

pub struct OpenAiLlmClient {
    api_key: Option<String>,
    api_base: String,
    model: String,
    protocol: LlmProtocol,
    client: reqwest::Client,
}

impl OpenAiLlmClient {
    pub fn new() -> Self {
        let api_key = env::var("OPENAI_API_KEY").ok();
        let api_base = env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
        let model = env::var("LLM_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string());
        let protocol_str = env::var("OPENAI_PROTOCOL").unwrap_or_else(|_| "chat".to_string());
        let protocol = if protocol_str.to_lowercase() == "responses" {
            LlmProtocol::OpenAiResponses
        } else {
            LlmProtocol::OpenAiChat
        };

        Self {
            api_key,
            api_base,
            model,
            protocol,
            client: reqwest::Client::new(),
        }
    }

    pub fn with_protocol(mut self, protocol: LlmProtocol) -> Self {
        self.protocol = protocol;
        self
    }
}

impl Default for OpenAiLlmClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LlmClient for OpenAiLlmClient {
    async fn generate_json(&self, system_prompt: &str, user_prompt: &str) -> Result<Value, String> {
        let key = match self.api_key {
            Some(ref k) => k,
            None => return Ok(mock_offline_reasoning(system_prompt, user_prompt)),
        };

        match self.protocol {
            // OpenAI 格式一：/v1/chat/completions
            LlmProtocol::OpenAiChat => {
                let url = format!("{}/chat/completions", self.api_base);
                let body = serde_json::json!({
                    "model": self.model,
                    "messages": [
                        {"role": "system", "content": system_prompt},
                        {"role": "user", "content": user_prompt}
                    ],
                    "response_format": {"type": "json_object"}
                });

                let res = self.client
                    .post(&url)
                    .bearer_auth(key)
                    .json(&body)
                    .send()
                    .await
                    .map_err(|e| format!("OpenAI Chat request failed: {}", e))?;

                if !res.status().is_success() {
                    return Err(format!("OpenAI API error status: {}", res.status()));
                }

                let resp_json: Value = res.json().await.map_err(|e| format!("Failed to parse response JSON: {}", e))?;
                if let Some(content) = resp_json["choices"][0]["message"]["content"].as_str() {
                    return parse_json_from_llm_text(content);
                }
                Err("Missing content in OpenAI Chat response choices".to_string())
            }

            // OpenAI 格式二：/v1/responses
            LlmProtocol::OpenAiResponses => {
                let url = format!("{}/responses", self.api_base);
                let body = serde_json::json!({
                    "model": self.model,
                    "input": [
                        {"role": "system", "content": system_prompt},
                        {"role": "user", "content": user_prompt}
                    ],
                    "response_format": {"type": "json_object"}
                });

                let res = self.client
                    .post(&url)
                    .bearer_auth(key)
                    .json(&body)
                    .send()
                    .await
                    .map_err(|e| format!("OpenAI Responses request failed: {}", e))?;

                if !res.status().is_success() {
                    return Err(format!("OpenAI Responses API error status: {}", res.status()));
                }

                let resp_json: Value = res.json().await.map_err(|e| format!("Failed to parse response JSON: {}", e))?;
                // 兼容 responses 规范中的 output_text 或 output[0].content[0].text
                if let Some(content) = resp_json["output_text"].as_str() {
                    return parse_json_from_llm_text(content);
                }
                if let Some(content) = resp_json["output"][0]["content"][0]["text"].as_str() {
                    return parse_json_from_llm_text(content);
                }
                if let Some(content) = resp_json["choices"][0]["message"]["content"].as_str() {
                    return parse_json_from_llm_text(content);
                }
                Err("Missing content in OpenAI Responses API response".to_string())
            }

            _ => Err("Invalid protocol for OpenAiLlmClient".to_string()),
        }
    }
}

// =========================================================================
// 3. Anthropic 客户端（Claude Messages API 协议格式）
// =========================================================================

pub struct AnthropicLlmClient {
    api_key: Option<String>,
    api_base: String,
    model: String,
    client: reqwest::Client,
}

impl AnthropicLlmClient {
    pub fn new() -> Self {
        let api_key = env::var("ANTHROPIC_API_KEY").ok();
        let api_base = env::var("ANTHROPIC_BASE_URL").unwrap_or_else(|_| "https://api.anthropic.com".to_string());
        let model = env::var("ANTHROPIC_MODEL").unwrap_or_else(|_| "claude-3-5-sonnet-20241022".to_string());

        Self {
            api_key,
            api_base,
            model,
            client: reqwest::Client::new(),
        }
    }
}

impl Default for AnthropicLlmClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LlmClient for AnthropicLlmClient {
    async fn generate_json(&self, system_prompt: &str, user_prompt: &str) -> Result<Value, String> {
        let key = match self.api_key {
            Some(ref k) => k,
            None => return Ok(mock_offline_reasoning(system_prompt, user_prompt)),
        };

        let url = format!("{}/v1/messages", self.api_base);
        let body = serde_json::json!({
            "model": self.model,
            "max_tokens": 4096,
            "system": format!("{}\n\nIMPORTANT: You must return raw JSON only with no conversational text.", system_prompt),
            "messages": [
                {"role": "user", "content": user_prompt}
            ]
        });

        let res = self.client
            .post(&url)
            .header("x-api-key", key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Anthropic request failed: {}", e))?;

        if !res.status().is_success() {
            return Err(format!("Anthropic API error status: {}", res.status()));
        }

        let resp_json: Value = res.json().await.map_err(|e| format!("Failed to parse response JSON: {}", e))?;
        if let Some(content) = resp_json["content"][0]["text"].as_str() {
            return parse_json_from_llm_text(content);
        }

        Err("Missing content[0].text in Anthropic response".to_string())
    }
}

// =========================================================================
// 4. Google 客户端（Gemini GenerateContent API 协议格式）
// =========================================================================

pub struct GeminiLlmClient {
    api_key: Option<String>,
    api_base: String,
    model: String,
    client: reqwest::Client,
}

impl GeminiLlmClient {
    pub fn new() -> Self {
        let api_key = env::var("GEMINI_API_KEY")
            .or_else(|_| env::var("GOOGLE_API_KEY"))
            .ok();
        let api_base = env::var("GEMINI_BASE_URL")
            .unwrap_or_else(|_| "https://generativelanguage.googleapis.com".to_string());
        let model = env::var("GEMINI_MODEL")
            .unwrap_or_else(|_| "gemini-1.5-flash".to_string());

        Self {
            api_key,
            api_base,
            model,
            client: reqwest::Client::new(),
        }
    }
}

impl Default for GeminiLlmClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LlmClient for GeminiLlmClient {
    async fn generate_json(&self, system_prompt: &str, user_prompt: &str) -> Result<Value, String> {
        let key = match self.api_key {
            Some(ref k) => k,
            None => return Ok(mock_offline_reasoning(system_prompt, user_prompt)),
        };

        let url = format!("{}/v1beta/models/{}:generateContent?key={}", self.api_base, self.model, key);
        let body = serde_json::json!({
            "contents": [
                {
                    "role": "user",
                    "parts": [{"text": user_prompt}]
                }
            ],
            "systemInstruction": {
                "parts": [{"text": system_prompt}]
            },
            "generationConfig": {
                "responseMimeType": "application/json"
            }
        });

        let res = self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Gemini request failed: {}", e))?;

        if !res.status().is_success() {
            return Err(format!("Gemini API error status: {}", res.status()));
        }

        let resp_json: Value = res.json().await.map_err(|e| format!("Failed to parse response JSON: {}", e))?;
        if let Some(content) = resp_json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
            return parse_json_from_llm_text(content);
        }

        Err("Missing candidates[0].content.parts[0].text in Gemini response".to_string())
    }
}

// =========================================================================
// 5. 通用大模型工厂函数 (Universal Factory: Auto-detect from Environment)
// =========================================================================

pub fn create_llm_client_from_env() -> Arc<dyn LlmClient> {
    let provider = env::var("LLM_PROVIDER")
        .unwrap_or_default()
        .to_lowercase();

    match provider.as_str() {
        "openai-responses" => Arc::new(OpenAiLlmClient::new().with_protocol(LlmProtocol::OpenAiResponses)),
        "anthropic" | "claude" => Arc::new(AnthropicLlmClient::new()),
        "gemini" | "google" => Arc::new(GeminiLlmClient::new()),
        "openai" | "chat" => Arc::new(OpenAiLlmClient::new()),
        _ => {
            // 依据环境变量自动探测优先协议
            if env::var("GEMINI_API_KEY").is_ok() || env::var("GOOGLE_API_KEY").is_ok() {
                Arc::new(GeminiLlmClient::new())
            } else if env::var("ANTHROPIC_API_KEY").is_ok() {
                Arc::new(AnthropicLlmClient::new())
            } else {
                Arc::new(OpenAiLlmClient::new())
            }
        }
    }
}
