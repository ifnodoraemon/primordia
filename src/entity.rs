use serde::{Deserialize, Serialize};

/// 实体空间位置与关系拓扑场描述 (Relational & Spatial Presence)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spatial {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub scale: f64,
    pub mobility: String, // "static", "floating", "dynamic", "ethereal"
    pub domain: String,   // 所属拓扑场域 / Relational Domain (e.g., "云崖绝壁 / Cliff Precipice")
    pub resonance_field: String, // 场域共鸣波 / Resonance Field
}

impl Default for Spatial {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            scale: 1.0,
            mobility: "dynamic".to_string(),
            domain: "无垠虚空 / Boundless Void".to_string(),
            resonance_field: "静默微澜 / Silent Ripple".to_string(),
        }
    }
}

/// 灵元实体核心结构（万物统一平等的存在载体 / Universal Ontological Entity）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub essence: String,
    /// OOO哲学：退隐内核（不可被外部穷尽的内在深度 / Withdrawn Real Core）
    pub withdrawn_core: String,
    pub spatial: Spatial,
    pub traits: Vec<String>,
    pub current_state: String,
    pub memory_stream: Vec<String>,
    /// 德勒兹哲学：共生装配体关系列表 (Assemblages & Rhizomatic Symbiosis)
    pub assemblages: Vec<String>,
    pub born_at_tick: u64,
}

impl Entity {
    pub fn new(
        id: String,
        name: String,
        essence: String,
        traits: Vec<String>,
        state: String,
        born_tick: u64,
    ) -> Self {
        Self {
            id,
            name: name.clone(),
            essence: essence.clone(),
            withdrawn_core: format!("蕴含【{}】({}) 未曾显露的深渊本质 / Inscrutable primordial depth of [{}] ({})", name, essence, name, essence),
            spatial: Spatial::default(),
            traits,
            current_state: state,
            memory_stream: vec![format!("诞生于纪元第 {} 周期 / Formed at Epoch {}", born_tick, born_tick)],
            assemblages: Vec::new(),
            born_at_tick: born_tick,
        }
    }

    pub fn with_details(
        id: String,
        name: String,
        essence: String,
        withdrawn_core: String,
        spatial: Spatial,
        traits: Vec<String>,
        state: String,
        born_tick: u64,
    ) -> Self {
        Self {
            id,
            name,
            essence,
            withdrawn_core,
            spatial,
            traits,
            current_state: state,
            memory_stream: vec![format!("诞生于纪元第 {} 周期 / Formed at Epoch {}", born_tick, born_tick)],
            assemblages: Vec::new(),
            born_at_tick: born_tick,
        }
    }

    pub fn record_memory(&mut self, memory: String) {
        self.memory_stream.push(memory);
    }

    /// 建立共生装配关系 (Attach to an Assemblage)
    pub fn link_assemblage(&mut self, other_id: &str) {
        if !self.assemblages.contains(&other_id.to_string()) {
            self.assemblages.push(other_id.to_string());
        }
    }

    /// 解除装配关系 (Deterritorialize Assemblage)
    pub fn unlink_assemblage(&mut self, other_id: &str) {
        self.assemblages.retain(|id| id != other_id);
    }

    /// 对外辐射的表象界面（面向客体交互时的感官界面 / Sensual Interface）
    pub fn sensory_manifestation(&self) -> String {
        format!(
            "【{}】(特征: {}; 场域: {}; 状态: {})",
            self.name,
            self.traits.join(", "),
            self.spatial.domain,
            self.current_state
        )
    }
}
