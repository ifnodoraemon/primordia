use crate::llm::LlmClient;
use crate::world::ChronicleEvent;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 纪元神话篇章 (Epoch Mythos Chapter)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MythosChapter {
    pub epoch_range: (u64, u64),
    pub title: String,
    pub poetic_epic: String,
    pub world_tone: String,
}

/// 史诗与神话编年引擎 (Mythos & Epic Generation Engine - Layer 3)
pub struct MythosEngine;

impl MythosEngine {
    /// 基于编年史事件提炼纪元神话史诗 (Distill Chronicle into Mythos Epic)
    pub async fn distill_epoch_mythos(
        llm: &dyn LlmClient,
        events: &[ChronicleEvent],
        current_tick: u64,
    ) -> Result<MythosChapter, String> {
        if events.is_empty() {
            return Ok(MythosChapter {
                epoch_range: (0, current_tick),
                title: "虚空始初 / Genesis Silence".to_string(),
                poetic_epic: "虚空中唯有一片混沌，万物尚在深渊中沉睡。 / In the primordial void, myriad forms slumber in silent chaos.".to_string(),
                world_tone: "寂静沉冥 / Primordial Quiescence".to_string(),
            });
        }

        let system_prompt = "你是《原初》元世界的宇宙史诗记录者与神话编纂者。\
            You are the Cosmic Mythographer of Primordia.\
            请根据提供的世界编年史事件，提炼出一段庄严、诗意、具有东方玄奥与现代哲学意境的中英双语纪元神话史诗。\
            请务必返回 JSON: {title: str, poetic_epic: str, world_tone: str}".to_string();

        let mut events_summary = String::new();
        for e in events.iter().rev().take(10) {
            events_summary.push_str(&format!("- [Tick {}] <{}> {}\n", e.tick, e.event_type, e.detail));
        }

        let user_prompt = format!(
            "当前宇宙纪元: Tick {}\n近期发生的核心编年史事件:\n{}",
            current_tick, events_summary
        );

        let res = llm.generate_json(&system_prompt, &user_prompt).await?;

        let title = res["title"].as_str().unwrap_or("太初流变赋 / Ballad of Primordial Becoming").to_string();
        let poetic_epic = res["poetic_epic"].as_str().unwrap_or(
            "星尘与流火交融，顽石在微光中呼吸。意识降临于灵元，世界在混沌中编织因果。 / Stardust entwines with dancing flame; ancient stone breathes in the faint radiance. Consciousness awakens across animas as cosmos weaves causality from chaos."
        ).to_string();
        let world_tone = res["world_tone"].as_str().unwrap_or("神性涌现 / Emergent Numina").to_string();

        let start_tick = events.first().map(|e| e.tick).unwrap_or(0);

        Ok(MythosChapter {
            epoch_range: (start_tick, current_tick),
            title,
            poetic_epic,
            world_tone,
        })
    }
}
