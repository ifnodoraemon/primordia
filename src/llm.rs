use async_trait::async_trait;
use serde_json::Value;
use std::env;

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn generate_json(&self, system_prompt: &str, user_prompt: &str) -> Result<Value, String>;
}

/// 基于 Reqwest 的 OpenAI 兼容异步 LLM 客户端
pub struct OpenAiLlmClient {
    api_key: Option<String>,
    api_base: String,
    model: String,
    client: reqwest::Client,
}

impl OpenAiLlmClient {
    pub fn new() -> Self {
        let api_key = env::var("OPENAI_API_KEY").ok();
        let api_base = env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
        let model = env::var("LLM_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string());

        Self {
            api_key,
            api_base,
            model,
            client: reqwest::Client::new(),
        }
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
        if let Some(ref key) = self.api_key {
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
                .map_err(|e| format!("HTTP Request failed: {}", e))?;

            if !res.status().is_success() {
                return Err(format!("API returned error status: {}", res.status()));
            }

            let resp_json: Value = res.json().await.map_err(|e| format!("Failed to parse JSON response: {}", e))?;
            if let Some(content) = resp_json["choices"][0]["message"]["content"].as_str() {
                let parsed: Value = serde_json::from_str(content)
                    .map_err(|e| format!("Failed to parse model content as JSON: {}", e))?;
                return Ok(parsed);
            }
        }

        // 离线/未配置 Key 时的模拟回退推演
        Ok(serde_json::json!({
            "updated_state": "表面渗透出微弱的虹彩辉光，内部物质开始自发分层旋转，呼吸着虚空中的微光",
            "new_traits": ["虹彩辉光", "内部自旋"],
            "new_memory": "感受到了时光在周围留下的涟漪",
            "narrative": "两股原初能量在虚空中交汇，激荡出微弱的共鸣波纹",
            "outcome_type": "MUTUAL_CHANGE",
            "action_result": "意识与物质发生谐振，形态产生轻微位移与流变",
            "subject_new_state": "内部核心泛起明亮光晕，以共鸣姿态朝向目标",
            "environmental_ripple": "周围三丈内的气流化作细微的旋转涡流"
        }))
    }
}
