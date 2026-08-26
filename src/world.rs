use crate::entity::Entity;
use crate::llm::{LlmClient, OpenAiLlmClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronicleEvent {
    pub tick: u64,
    pub event_type: String,
    pub detail: String,
    pub timestamp: u64,
}

/// 世界数据快照（用于持久化与序列化）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub name: String,
    pub tick_count: u64,
    pub cosmic_atmosphere: String,
    pub entities: HashMap<String, Entity>,
    pub chronicle: Vec<ChronicleEvent>,
}

pub struct PrimordiaWorld {
    pub name: String,
    pub tick_count: u64,
    /// 宏观天道气象与法则场域 (Evolving Cosmic Atmosphere & Meta-Law)
    pub cosmic_atmosphere: String,
    pub entities: HashMap<String, Entity>,
    pub chronicle: Vec<ChronicleEvent>,
    llm: Arc<dyn LlmClient>,
}

impl PrimordiaWorld {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tick_count: 0,
            cosmic_atmosphere: "原初鸿蒙初辟，虚空中星尘与暗能量自发涌动 / Primordial Void Genesis, swirling stardust and dark currents".to_string(),
            entities: HashMap::new(),
            chronicle: Vec::new(),
            llm: Arc::new(OpenAiLlmClient::new()),
        }
    }

    pub fn with_llm(name: &str, llm: Arc<dyn LlmClient>) -> Self {
        Self {
            name: name.to_string(),
            tick_count: 0,
            cosmic_atmosphere: "原初鸿蒙初辟，虚空中星尘与暗能量自发涌动 / Primordial Void Genesis, swirling stardust and dark currents".to_string(),
            entities: HashMap::new(),
            chronicle: Vec::new(),
            llm,
        }
    }

    pub fn add_entity(&mut self, name: &str, essence: &str, traits: Vec<&str>, state: &str) -> String {
        self.add_entity_with_domain(name, essence, traits, state, "原初灵虚界 / Primordial Ethereal Domain")
    }

    pub fn add_entity_with_domain(
        &mut self,
        name: &str,
        essence: &str,
        traits: Vec<&str>,
        state: &str,
        domain: &str,
    ) -> String {
        let id = format!("ent_{:03}", self.entities.len() + 1);
        let traits_vec: Vec<String> = traits.into_iter().map(|s| s.to_string()).collect();
        let mut entity = Entity::new(
            id.clone(),
            name.to_string(),
            essence.to_string(),
            traits_vec,
            state.to_string(),
            self.tick_count,
        );
        entity.spatial.domain = domain.to_string();

        self.entities.insert(id.clone(), entity);
        self.record_event(
            "ENTITY_GENESIS",
            &format!("实体【{}】({}) 在【{}】凝结诞生。 / Genesis of [{}] ({}) in [{}].", name, essence, domain, name, essence, domain),
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

    /// 建立德勒兹共生装配体 (Form Deleuzian Assemblage)
    pub fn form_assemblage(&mut self, id_a: &str, id_b: &str, bond_narrative: &str) -> Result<(), String> {
        if !self.entities.contains_key(id_a) || !self.entities.contains_key(id_b) {
            return Err("One or both entities not found".to_string());
        }

        if let Some(ent_a) = self.entities.get_mut(id_a) {
            ent_a.link_assemblage(id_b);
            ent_a.record_memory(format!("与【{}】缔结为共生装配体: {}", id_b, bond_narrative));
        }

        if let Some(ent_b) = self.entities.get_mut(id_b) {
            ent_b.link_assemblage(id_a);
            ent_b.record_memory(format!("与【{}】缔结为共生装配体: {}", id_a, bond_narrative));
        }

        let name_a = self.entities.get(id_a).map(|e| e.name.clone()).unwrap_or_default();
        let name_b = self.entities.get(id_b).map(|e| e.name.clone()).unwrap_or_default();

        self.record_event(
            "ASSEMBLAGE_FORMED",
            &format!("【{}】与【{}】结成共生装配网络：{} / Assemblage formed between [{}] and [{}]: {}", name_a, name_b, bond_narrative, name_a, name_b, bond_narrative),
        );

        Ok(())
    }

    /// 宏观天道气象与纪元法则演化 (Cosmic Macro-Law Evolution)
    pub async fn evolve_cosmic_law(&mut self) -> Result<String, String> {
        let system_prompt = "你是《原初》宏观天道推演核心。请根据当前世界纪元、实体总数与历史编年，推演世界宏观法则/环境气候的迁跃相变。\
            You are the Cosmic Arbiter of Primordia. Reason through the macro-law / atmospheric phase shift of the universe.\
            请务必返回 JSON: {new_atmosphere: str, cosmic_ripple: str}";

        let context_summary = format!(
            "当前纪元 / Current Tick: {}\n当前天道气象 / Current Atmosphere: {}\n实体总数 / Total Entities: {}\n最新事件 / Latest Event: {}",
            self.tick_count,
            self.cosmic_atmosphere,
            self.entities.len(),
            self.chronicle.last().map(|e| e.detail.as_str()).unwrap_or("无")
        );

        let result = self.llm.generate_json(system_prompt, &context_summary).await?;
        let new_atmo = result["new_atmosphere"].as_str().unwrap_or(&self.cosmic_atmosphere).to_string();
        let ripple = result["cosmic_ripple"].as_str().unwrap_or("天道气象微微流变 / Cosmic atmosphere gently ripples").to_string();

        self.cosmic_atmosphere = new_atmo.clone();
        self.record_event(
            "COSMIC_LAW_SHIFT",
            &format!("宏观天道气象发生纪元相变：{} ──► 波纹: {} / Cosmic law phase shift: {} ──► Ripple: {}", new_atmo, ripple, new_atmo, ripple),
        );

        Ok(new_atmo)
    }

    /// 单实体自生长演化 (Autonomous Self-Evolution)
    pub async fn evolve_entity(&mut self, ent_id: &str) -> Result<Value, String> {
        let ent = match self.entities.get(ent_id) {
            Some(e) => e.clone(),
            None => return Err(format!("Entity {} not found", ent_id)),
        };

        let system_prompt = "你是《原初》元世界法则裁决核心。万物皆有灵性，自发演变。\
            You are the generative causality arbiter of Primordia Meta-World.\
            请根据实体内在状态与当前宏观天道气象，推演其自生长、自变异或心智萌芽。\
            请务必返回 JSON: {updated_state: str, new_traits: list, new_memory: str, sprouted_child: object or null}";

        let ent_json = serde_json::to_string(&ent).map_err(|e| e.to_string())?;
        let user_prompt = format!(
            "当前宏观天道气象 / Cosmic Atmosphere: {}\n目标实体 / Target Entity: {}",
            self.cosmic_atmosphere, ent_json
        );

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

    /// 两实体碰撞、相变与共生 (Collision, Morphogenesis & Symbiosis)
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
            基于双方本质、感官界面与宏观天道裁决相变结果：互相改变(MUTUAL_CHANGE)、共生装配(ASSEMBLAGE_SYMBIOSIS)、或天地化生(MORPHOGENESIS_NEW)。\
            请务必返回 JSON: {narrative: str, outcome_type: str, born_entity: object or null, update_a: str, update_b: str}";

        let user_prompt = format!(
            "当前天道气象 / Cosmic Atmosphere: {}\n实体 A 感官界面 / Entity A Sensual Interface: {}\n实体 A 详情: {}\n实体 B 感官界面 / Entity B Sensual Interface: {}\n实体 B 详情: {}",
            self.cosmic_atmosphere,
            ent_a.sensory_manifestation(),
            serde_json::to_string(&ent_a).map_err(|e| e.to_string())?,
            ent_b.sensory_manifestation(),
            serde_json::to_string(&ent_b).map_err(|e| e.to_string())?
        );

        let result = self.llm.generate_json(system_prompt, &user_prompt).await?;
        let narrative = result["narrative"].as_str().unwrap_or("两实体发生了碰撞交互。 / Entities collided.").to_string();
        let outcome_type = result["outcome_type"].as_str().unwrap_or("MUTUAL_CHANGE");

        self.record_event("COLLISION_MORPHOGENESIS", &format!("[{}] {}", outcome_type, narrative));

        if outcome_type == "ASSEMBLAGE_SYMBIOSIS" {
            let _ = self.form_assemblage(id_a, id_b, &narrative);
        }

        if let Some(target_a) = self.entities.get_mut(id_a) {
            if let Some(up_a) = result["update_a"].as_str() {
                target_a.current_state = up_a.to_string();
            }
        }

        if let Some(target_b) = self.entities.get_mut(id_b) {
            if let Some(up_b) = result["update_b"].as_str() {
                target_b.current_state = up_b.to_string();
            }
        }

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

    /// 玩家意识寄宿与意志注入 (Mind Inhabitation & Agency)
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
            "当前天道气象 / Cosmic Atmosphere: {}\n寄宿实体 / Inhabited Entity: {}\n玩家自由意志意图 / Player Intent: '{}'",
            self.cosmic_atmosphere,
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

    /// 推进一个世界纪元周期 (Advance World Epoch Tick)
    pub async fn tick(&mut self) -> Result<(), String> {
        self.tick_count += 1;
        println!("\n===== 🌌 进入世界纪元 第 {} 周期 / Entering World Epoch {} =====", self.tick_count, self.tick_count);
        let ids: Vec<String> = self.entities.keys().cloned().collect();
        for id in ids {
            self.evolve_entity(&id).await?;
        }
        Ok(())
    }

    /// 导出世界状态快照 JSON (Export Snapshot JSON)
    pub fn export_snapshot_json(&self) -> Result<String, String> {
        let snapshot = WorldSnapshot {
            name: self.name.clone(),
            tick_count: self.tick_count,
            cosmic_atmosphere: self.cosmic_atmosphere.clone(),
            entities: self.entities.clone(),
            chronicle: self.chronicle.clone(),
        };
        serde_json::to_string_pretty(&snapshot).map_err(|e| e.to_string())
    }

    /// 保存世界快照至文件 (Save Snapshot to File)
    pub fn save_snapshot(&self, path: &str) -> Result<(), String> {
        let json = self.export_snapshot_json()?;
        fs::write(path, json).map_err(|e| format!("Failed to write snapshot file: {}", e))
    }

    /// 从快照 JSON 恢复世界 (Import Snapshot from JSON)
    pub fn import_snapshot_json(json_str: &str, llm: Arc<dyn LlmClient>) -> Result<Self, String> {
        let snapshot: WorldSnapshot = serde_json::from_str(json_str)
            .map_err(|e| format!("Failed to parse world snapshot: {}", e))?;
        Ok(Self {
            name: snapshot.name,
            tick_count: snapshot.tick_count,
            cosmic_atmosphere: snapshot.cosmic_atmosphere,
            entities: snapshot.entities,
            chronicle: snapshot.chronicle,
            llm,
        })
    }

    /// 从快照文件恢复世界 (Load Snapshot from File)
    pub fn load_snapshot(path: &str, llm: Arc<dyn LlmClient>) -> Result<Self, String> {
        let json_str = fs::read_to_string(path).map_err(|e| format!("Failed to read snapshot file: {}", e))?;
        Self::import_snapshot_json(&json_str, llm)
    }
}
