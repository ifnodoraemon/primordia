use serde::{Deserialize, Serialize};

/// 实体空间位置与移动性描述
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spatial {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub scale: f64,
    pub mobility: String, // "static", "floating", "dynamic", "ethereal"
}

impl Default for Spatial {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            scale: 1.0,
            mobility: "dynamic".to_string(),
        }
    }
}

/// 灵元实体核心结构（万物统一平等的数据载体）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub essence: String,
    pub spatial: Spatial,
    pub traits: Vec<String>,
    pub current_state: String,
    pub memory_stream: Vec<String>,
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
            name,
            essence,
            spatial: Spatial::default(),
            traits,
            current_state: state,
            memory_stream: vec![format!("诞生于纪元第 {} 周期", born_tick)],
            born_at_tick: born_tick,
        }
    }

    pub fn record_memory(&mut self, memory: String) {
        self.memory_stream.push(memory);
    }
}
