use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

/// 因果推演单步追踪 Span (Causal Step Trace Span)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalSpan {
    pub span_id: String,
    pub tick: u64,
    pub operator: String, // "SELF_EVOLVE", "COLLIDE", "MIND_INHABIT", "COSMIC_LAW"
    pub target_entities: Vec<String>,
    pub system_prompt: String,
    pub user_prompt: String,
    pub llm_response: Option<Value>,
    pub mutations_summary: String,
    pub duration_ms: u64,
    pub timestamp: u64,
}

/// 因果链路追踪器 (Causality & Lineage Tracer)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CausalityTracer {
    pub spans: Vec<CausalSpan>,
    pub total_llm_calls: usize,
    pub total_latency_ms: u64,
}

impl CausalityTracer {
    pub fn new() -> Self {
        Self::default()
    }

    /// 记录一个已完成的因果推演 Span
    #[allow(clippy::too_many_arguments)]
    pub fn record_span(
        &mut self,
        tick: u64,
        operator: &str,
        target_entities: Vec<String>,
        system_prompt: &str,
        user_prompt: &str,
        llm_response: Value,
        mutations_summary: &str,
        duration_ms: u64,
    ) -> String {
        let span_id = format!("span_{:04}", self.spans.len() + 1);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let span = CausalSpan {
            span_id: span_id.clone(),
            tick,
            operator: operator.to_string(),
            target_entities,
            system_prompt: system_prompt.to_string(),
            user_prompt: user_prompt.to_string(),
            llm_response: Some(llm_response),
            mutations_summary: mutations_summary.to_string(),
            duration_ms,
            timestamp,
        };

        self.total_llm_calls += 1;
        self.total_latency_ms += duration_ms;
        self.spans.push(span);

        span_id
    }

    /// 导出追踪记录为格式化 JSON
    pub fn export_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self).map_err(|e| e.to_string())
    }

    /// 导出追踪记录为 JSONL (按行流式，每行一个 CausalSpan)
    pub fn export_jsonl(&self) -> Result<String, String> {
        let mut lines = Vec::new();
        for span in &self.spans {
            let line = serde_json::to_string(span).map_err(|e| e.to_string())?;
            lines.push(line);
        }
        Ok(lines.join("\n"))
    }

    /// 获取因果链路追踪摘要报告
    pub fn summary(&self) -> String {
        let avg_latency = if self.total_llm_calls > 0 {
            self.total_latency_ms / self.total_llm_calls as u64
        } else {
            0
        };

        format!(
            "[Trace Summary] Spans: {} | LLM Invocations: {} | Total Latency: {}ms | Avg Latency: {}ms",
            self.spans.len(),
            self.total_llm_calls,
            self.total_latency_ms,
            avg_latency
        )
    }
}
