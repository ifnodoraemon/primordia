use crate::world::PrimordiaWorld;
use serde::{Deserialize, Serialize};

/// 实体在特定时刻捕获的局部感知视界 (Local Sensory Horizon)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensoryHorizon {
    pub host_id: String,
    pub domain_name: String,
    pub domain_resonance: String,
    /// 邻近实体的感官表象列表（遵循 OOO 哲学，隐藏退隐内核）
    pub neighbor_manifestations: Vec<String>,
    /// 共生装配体伙伴的感官表象
    pub symbiont_manifestations: Vec<String>,
    /// 本域内最近发生的宏观波纹与事件
    pub local_chronicle_snippets: Vec<String>,
}

impl SensoryHorizon {
    /// 格式化为注入 LLM 上下文的结构化文本
    pub fn to_prompt_context(&self) -> String {
        let neighbors = if self.neighbor_manifestations.is_empty() {
            "无其他明显实体波纹 / No distinct nearby entities".to_string()
        } else {
            self.neighbor_manifestations.join("\n- ")
        };

        let symbionts = if self.symbiont_manifestations.is_empty() {
            "无共生装配关联 / No linked symbiotic assemblages".to_string()
        } else {
            self.symbiont_manifestations.join("\n- ")
        };

        format!(
            "所属场域 / Domain: 【{}】(共鸣波: {})\n邻近实体感知表象 / Neighboring Entities:\n- {}\n共生装配伙伴 / Symbiont Partners:\n- {}",
            self.domain_name, self.domain_resonance, neighbors, symbionts
        )
    }
}

/// 局部感知视界构建引擎 (Perception Horizon Engine)
pub struct PerceptionEngine;

impl PerceptionEngine {
    /// 为指定实体提取其局部环境感知视界 (Extract Sensory Horizon for Entity)
    pub fn extract_horizon(world: &PrimordiaWorld, ent_id: &str) -> Result<SensoryHorizon, String> {
        let host = world
            .entities
            .get(ent_id)
            .ok_or_else(|| format!("Host entity {} not found", ent_id))?;

        let host_domain = &host.spatial.domain;
        let mut neighbor_manifestations = Vec::new();
        let mut symbiont_manifestations = Vec::new();

        // 收集同域实体或装配共生实体的感官表象（只暴露 sensory_manifestation）
        for (id, other) in &world.entities {
            if id == ent_id {
                continue;
            }

            // 检查是否为共生装配体
            if host.assemblages.contains(id) {
                symbiont_manifestations.push(format!("[共生伙伴 / Symbiont] {}", other.sensory_manifestation()));
            } else if &other.spatial.domain == host_domain {
                // 同拓扑域邻近实体
                neighbor_manifestations.push(other.sensory_manifestation());
            }
        }

        // 提取本域相关的最近 2 条编年史事件
        let local_chronicle_snippets: Vec<String> = world
            .chronicle
            .iter()
            .rev()
            .filter(|e| e.detail.contains(&host.name) || e.detail.contains(host_domain))
            .take(2)
            .map(|e| format!("[Tick {}] {}", e.tick, e.detail))
            .collect();

        Ok(SensoryHorizon {
            host_id: ent_id.to_string(),
            domain_name: host_domain.clone(),
            domain_resonance: host.spatial.resonance_field.clone(),
            neighbor_manifestations,
            symbiont_manifestations,
            local_chronicle_snippets,
        })
    }
}
