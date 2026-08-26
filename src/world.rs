use crate::entity::Entity;
use crate::llm::{create_llm_client_from_env, LlmClient};
use crate::operator::{
    CausalExecutor, CosmicLawOperator, MindInhabitationContext, MindInhabitationOperator,
    MorphogenesisContext, MorphogenesisOperator, SelfEvolutionOperator,
};
use crate::trace::CausalityTracer;
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
    pub tracer: CausalityTracer,
}

/// 《原初》世界统筹中枢 (Primordia World Facade)
pub struct PrimordiaWorld {
    pub name: String,
    pub tick_count: u64,
    /// 宏观天道气象与法则场域 (Evolving Cosmic Atmosphere & Meta-Law)
    pub cosmic_atmosphere: String,
    pub entities: HashMap<String, Entity>,
    pub chronicle: Vec<ChronicleEvent>,
    /// 全生命周期因果链路追踪器 (Causality & Lineage Tracer)
    pub tracer: CausalityTracer,
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
            tracer: CausalityTracer::new(),
            llm: create_llm_client_from_env(),
        }
    }

    pub fn with_llm(name: &str, llm: Arc<dyn LlmClient>) -> Self {
        Self {
            name: name.to_string(),
            tick_count: 0,
            cosmic_atmosphere: "原初鸿蒙初辟，虚空中星尘与暗能量自发涌动 / Primordial Void Genesis, swirling stardust and dark currents".to_string(),
            entities: HashMap::new(),
            chronicle: Vec::new(),
            tracer: CausalityTracer::new(),
            llm,
        }
    }

    pub fn llm(&self) -> &dyn LlmClient {
        self.llm.as_ref()
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

    /// 单实体自生长演化 (Autonomous Self-Evolution - Strategy Delegation)
    pub async fn evolve_entity(&mut self, ent_id: &str) -> Result<Value, String> {
        CausalExecutor::execute(self, &SelfEvolutionOperator, &ent_id.to_string()).await
    }

    /// 两实体碰撞、相变与共生 (Collision, Morphogenesis & Symbiosis - Strategy Delegation)
    pub async fn collide(&mut self, id_a: &str, id_b: &str) -> Result<Value, String> {
        CausalExecutor::execute(
            self,
            &MorphogenesisOperator,
            &MorphogenesisContext {
                id_a: id_a.to_string(),
                id_b: id_b.to_string(),
            },
        )
        .await
    }

    /// 玩家意识寄宿与意志注入 (Mind Inhabitation & Agency - Strategy Delegation)
    pub async fn inhabit_and_act(&mut self, ent_id: &str, player_intent: &str) -> Result<Value, String> {
        CausalExecutor::execute(
            self,
            &MindInhabitationOperator,
            &MindInhabitationContext {
                ent_id: ent_id.to_string(),
                player_intent: player_intent.to_string(),
            },
        )
        .await
    }

    /// 宏观天道气象与纪元法则演化 (Cosmic Macro-Law Evolution - Strategy Delegation)
    pub async fn evolve_cosmic_law(&mut self) -> Result<String, String> {
        CausalExecutor::execute(self, &CosmicLawOperator, &()).await
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
            tracer: self.tracer.clone(),
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
            tracer: snapshot.tracer,
            llm,
        })
    }

    /// 从快照文件恢复世界 (Load Snapshot from File)
    pub fn load_snapshot(path: &str, llm: Arc<dyn LlmClient>) -> Result<Self, String> {
        let json_str = fs::read_to_string(path).map_err(|e| format!("Failed to read snapshot file: {}", e))?;
        Self::import_snapshot_json(&json_str, llm)
    }
}
