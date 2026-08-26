use crate::entity::Entity;
use crate::llm::{LlmClient, OpenAiLlmClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronicleEvent {
    pub tick: u64,
    pub event_type: String,
    pub detail: String,
    pub timestamp: u64,
}

pub struct PrimordiaWorld {
    pub name: String,
    pub tick_count: u64,
    pub entities: HashMap<String, Entity>,
    pub chronicle: Vec<ChronicleEvent>,
    llm: Arc<dyn LlmClient>,
}

impl PrimordiaWorld {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tick_count: 0,
            entities: HashMap::new(),
            chronicle: Vec::new(),
            llm: Arc::new(OpenAiLlmClient::new()),
        }
    }

    pub fn with_llm(name: &str, llm: Arc<dyn LlmClient>) -> Self {
        Self {
            name: name.to_string(),
            tick_count: 0,
            entities: HashMap::new(),
            chronicle: Vec::new(),
            llm,
        }
    }

    pub fn add_entity(&mut self, name: &str, essence: &str, traits: Vec<&str>, state: &str) -> String {
        let id = format!("ent_{:03}", self.entities.len() + 1);
        let traits_vec: Vec<String> = traits.into_iter().map(|s| s.to_string()).collect();
        let entity = Entity::new(
            id.clone(),
            name.to_string(),
            essence.to_string(),
            traits_vec,
            state.to_string(),
            self.tick_count,
        );

        self.entities.insert(id.clone(), entity);
        self.record_event(
            "ENTITY_GENESIS",
            &format!("实体【{}】({}) 在世界中凝结诞生。 / Genesis of entity [{}] ({}).", name, essence, name, essence),
        );
        id
    }

    pub fn record_event(&mut self, event_type: &str, detail: &str) {
        let event = ChronicleEvent {
            tick: self.tick_count,
            event_type: event_type.to_string(),
            detail: detail.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };
        println!("[纪元 {} | Epoch {}] <{}> {}", self.tick_count, self.tick_count, event_type, detail);
        self.chronicle.push(event);
    }

    /// 单实体自生长演化 / Autonomous Self-Evolution
    pub async fn evolve_entity(&mut self, ent_id: &str) -> Result<Value, String> {
        let ent = match self.entities.get(ent_id) {
            Some(e) => e.clone(),
            None => return Err(format!("Entity {} not found", ent_id)),
        };

        let system_prompt = "你是《原初》元世界法则裁决核心。万物皆有灵性，自发演变。\
            You are the generative causality arbiter of Primordia Meta-World.\
            请根据实体状态推演其在时间流逝中的自生长、自变异或心智萌芽。\
            请务必返回 JSON: {updated_state: str, new_traits: list, new_memory: str, sprouted_child: object or null}";
        let ent_json = serde_json::to_string(&ent).map_err(|e| e.to_string())?;
        let user_prompt = format!("目标实体 / Target Entity：{}", ent_json);

        let result = self.llm.generate_json(system_prompt, &user_prompt).await?;

        let mut event_msg = None;
        if let Some(target) = self.entities.get_mut(ent_id) {
            if let Some(updated_state) = result["updated_state"].as_str() {
                target.current_state = updated_state.to_string();
            }
            if let Some(new_traits) = result["new_traits"].as_array() {
                for t in new_traits {
                    if let Some(trait_str) = t.as_str() {
                        if !target.traits.contains(&trait_str.to_string()) {
                            target.traits.push(trait_str.to_string());
                        }
                    }
                }
            }
            if let Some(new_mem) = result["new_memory"].as_str() {
                target.record_memory(new_mem.to_string());
            }
            event_msg = Some(format!(
                "【{}】发生自演化：{} / [{}] evolved: {}",
                target.name, target.current_state, target.name, target.current_state
            ));
        }

        if let Some(msg) = event_msg {
            self.record_event("SELF_EVOLVE", &msg);
        }

        // 检查是否孕育出新实体 / Check if new child sprouted
        if result["sprouted_child"].is_object() {
            let child = &result["sprouted_child"];
            let name = child["name"].as_str().unwrap_or("新生灵元 / Sprouted Animus");
            let essence = child["essence"].as_str().unwrap_or("演化分裂出的新存在 / Emerging existence");
            let state = child["current_state"].as_str().unwrap_or("");
            let traits_arr: Vec<&str> = child["traits"]
                .as_array()
                .map(|a| a.iter().filter_map(|v| v.as_str()).collect())
                .unwrap_or_default();
            self.add_entity(name, essence, traits_arr, state);
        }

        Ok(result)
    }

    /// 两实体碰撞与化生 / Collision & Morphogenesis
    pub async fn collide(&mut self, id_a: &str, id_b: &str) -> Result<Value, String> {
        let ent_a = match self.entities.get(id_a) {
            Some(e) => e.clone(),
            None => return Err(format!("Entity {} not found", id_a)),
        };
        let ent_b = match self.entities.get(id_b) {
            Some(e) => e.clone(),
            None => return Err(format!("Entity {} not found", id_b)),
        };

        let system_prompt = "你是《原初》元世界法则裁决核心。两实体发生交互碰撞与交融。\
            You are the generative causality arbiter of Primordia Meta-World.\
            基于双方本质裁决其相变结果：是互相改变、同化、还是天地化生诞生第三种全新存在？\
            请务必返回 JSON: {narrative: str, outcome_type: str, born_entity: object or null, update_a: str, update_b: str}";
        let ent_a_json = serde_json::to_string(&ent_a).map_err(|e| e.to_string())?;
        let ent_b_json = serde_json::to_string(&ent_b).map_err(|e| e.to_string())?;
        let user_prompt = format!(
            "实体 A / Entity A: {}\n实体 B / Entity B: {}",
            ent_a_json, ent_b_json
        );

        let result = self.llm.generate_json(system_prompt, &user_prompt).await?;
        let narrative = result["narrative"].as_str().unwrap_or("两实体发生了碰撞交互。 / Entities collided.").to_string();
        self.record_event("COLLISION_MORPHOGENESIS", &narrative);

        if result["born_entity"].is_object() {
            let born = &result["born_entity"];
            let name = born["name"].as_str().unwrap_or("化生之灵 / Spontaneous Animus");
            let essence = born["essence"].as_str().unwrap_or("化生存在 / Morphic being");
            let state = born["current_state"].as_str().unwrap_or("刚从两者的激荡中化生而出 / Newly manifested");
            let traits_arr: Vec<&str> = born["traits"]
                .as_array()
                .map(|a| a.iter().filter_map(|v| v.as_str()).collect())
                .unwrap_or_default();
            self.add_entity(name, essence, traits_arr, state);
        }

        Ok(result)
    }

    /// 玩家意识寄宿与意志注入 / Mind Inhabitation & Agency
    pub async fn inhabit_and_act(&mut self, ent_id: &str, player_intent: &str) -> Result<Value, String> {
        let ent = match self.entities.get(ent_id) {
            Some(e) => e.clone(),
            None => return Err(format!("Entity {} not found", ent_id)),
        };

        let system_prompt = "你是《原初》元世界法则裁决核心。玩家作为原初自由意志，寄宿于该实体并发出行动意图。\
            You are the generative causality arbiter of Primordia Meta-World.\
            请评估该实体的物理/灵性本质如何响应此意图，推导其自身状态变化与对周围环境的波纹。\
            请务必返回 JSON: {action_result: str, subject_new_state: str, environmental_ripple: str}";
        let ent_json = serde_json::to_string(&ent).map_err(|e| e.to_string())?;
        let user_prompt = format!(
            "寄宿实体 / Inhabited Entity: {}\n玩家自由意志意图 / Player Intent: '{}'",
            ent_json,
            player_intent
        );

        let result = self.llm.generate_json(system_prompt, &user_prompt).await?;

        let mut event_msg = None;
        if let Some(target) = self.entities.get_mut(ent_id) {
            if let Some(new_state) = result["subject_new_state"].as_str() {
                target.current_state = new_state.to_string();
            }
            target.record_memory(format!("曾被不可名状的宏大意志降临驱使: {} / Guided by divine intent: {}", player_intent, player_intent));
            let action_res = result["action_result"].as_str().unwrap_or("");
            event_msg = Some(format!(
                "玩家寄宿【{}】并行动: {} ──► {} / Player inhabited [{}] and acted: {} ──► {}",
                target.name, player_intent, action_res, target.name, player_intent, action_res
            ));
        }

        if let Some(msg) = event_msg {
            self.record_event("MIND_INHABITATION", &msg);
        }

        Ok(result)
    }

    /// 推进一个世界纪元周期 / Advance World Epoch Tick
    pub async fn tick(&mut self) -> Result<(), String> {
        self.tick_count += 1;
        println!("\n===== 🌌 进入世界纪元 第 {} 周期 / Entering World Epoch {} =====", self.tick_count, self.tick_count);
        let ids: Vec<String> = self.entities.keys().cloned().collect();
        for id in ids {
            self.evolve_entity(&id).await?;
        }
        Ok(())
    }
}
