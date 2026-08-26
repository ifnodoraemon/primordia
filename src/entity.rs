use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 实体空间位置与关系拓扑场描述 (Relational & Spatial Presence)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spatial {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub scale: f64,
    pub domain: String,          // 所属拓扑场域 / Relational Domain (e.g., "悬天绝壁 / Celestial Cliff Precipice")
    pub resonance_field: String, // 场域共鸣微澜 / Resonance Field
}

impl Default for Spatial {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            scale: 1.0,
            domain: "无垠虚空 / Boundless Void".to_string(),
            resonance_field: "静默微澜 / Silent Ripple".to_string(),
        }
    }
}

/// 灵元实体核心结构（万物统一平等的存在载体 / Universal Meta-Entity）
/// 哲学原则：零预设类型、零死板枚举。万物皆平等，一切存在态均由大模型潜空间实时涌现。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub essence: String,
    /// 面向客体本体论（OOO）：退隐内核（不可穷尽的深渊内在实在 / Withdrawn Real Core）
    pub withdrawn_core: String,
    pub spatial: Spatial,
    pub traits: Vec<String>,
    /// 开放自由的存在状态（非固化枚举，允许晶化、升维、涅槃、重组、枯寂等万千演化态）
    pub current_state: String,
    /// 开放动态语义属性池（由 LLM 因果推演自由增删改 / Open Dynamic Semantic Properties）
    #[serde(default)]
    pub properties: HashMap<String, Value>,
    /// 历史记忆流（过程哲学：过去的事实塑造当前的自性 / Epistemic Memory Stream）
    pub memory_stream: Vec<String>,
    /// 德勒兹根茎网络：共生装配体关联 (Rhizomatic Assemblages & Symbiosis)
    pub assemblages: Vec<String>,
    /// 当前寄宿该实体的自由意识/玩家算子列表 (Active Inhabitant Kernels)
    #[serde(default)]
    pub active_inhabitants: Vec<String>,
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
            properties: HashMap::new(),
            memory_stream: vec![format!("诞生于纪元第 {} 周期 / Formed at Epoch {}", born_tick, born_tick)],
            assemblages: Vec::new(),
            active_inhabitants: Vec::new(),
            born_at_tick: born_tick,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_details(
        id: String,
        name: String,
        essence: String,
        withdrawn_core: String,
        spatial: Spatial,
        traits: Vec<String>,
        state: String,
        properties: HashMap<String, Value>,
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
            properties,
            memory_stream: vec![format!("诞生于纪元第 {} 周期 / Formed at Epoch {}", born_tick, born_tick)],
            assemblages: Vec::new(),
            active_inhabitants: Vec::new(),
            born_at_tick: born_tick,
        }
    }

    pub fn record_memory(&mut self, memory: String) {
        self.memory_stream.push(memory);
    }

    /// 注册意识寄宿 (Register Inhabitant Attention Kernel)
    pub fn register_inhabitant(&mut self, observer_id: &str) {
        if !self.active_inhabitants.contains(&observer_id.to_string()) {
            self.active_inhabitants.push(observer_id.to_string());
        }
    }

    /// 抽离意识寄宿 (Depart Inhabitation)
    pub fn depart_inhabitant(&mut self, observer_id: &str) {
        self.active_inhabitants.retain(|id| id != observer_id);
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
        let inhabitation_tag = if self.active_inhabitants.is_empty() {
            String::new()
        } else {
            format!(" [觉知寄宿: {}]", self.active_inhabitants.join(", "))
        };

        format!(
            "【{}】[状态: {}]{} (本质: {}; 特征: {}; 场域: {})",
            self.name,
            self.current_state,
            inhabitation_tag,
            self.essence,
            self.traits.join(", "),
            self.spatial.domain
        )
    }
}
