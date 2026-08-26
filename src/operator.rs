use crate::perception::PerceptionEngine;
use crate::world::PrimordiaWorld;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

// =========================================================================
// 《原初》万物归一通用因果核 (Universal Causal Kernel)
// =========================================================================

/// 宇宙因果干涉输入 (Universal Causal Intervention Input)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalIntervention {
    /// 干涉模式 / 观察视角:
    /// "SELF_EVOLUTION" | "MORPHOGENESIS_COLLISION" | "MIND_INHABITATION" |
    /// "AUTONOMOUS_AGENCY" | "PANPSYCHIC_COMMUNION" | "INTERSUBJECTIVE_DIALOGUE" |
    /// "DOMAIN_RESONANCE" | "COSMIC_LAW_SHIFT" | 自定义意图模式
    pub mode: String,
    /// 参与干涉相互作用的实体 ID 列表（0个为全宇宙宏观相变，1个为单体自性流变/寄宿/自省，2+个为客体际碰撞/对话/场域共鸣）
    pub entities: Vec<String>,
    /// 外部施加的意图、神念倾听问答、自由意志注入或环境扰动（可选）
    pub stimulus: Option<String>,
}

/// 实体状态变异增量 (Entity Mutation Delta)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityMutation {
    pub entity_id: String,
    #[serde(default)]
    pub new_state: Option<String>,
    #[serde(default)]
    pub add_traits: Option<Vec<String>>,
    #[serde(default)]
    pub remove_traits: Option<Vec<String>>,
    #[serde(default)]
    pub memory: Option<String>,
    #[serde(default)]
    pub dynamic_properties: Option<HashMap<String, Value>>,
    #[serde(default)]
    pub link_assemblages: Option<Vec<String>>,
    #[serde(default)]
    pub is_dissolved: Option<bool>,
}

/// 化生新实体规格 (Born / Sprouted Entity Spec)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewEntitySpec {
    pub name: String,
    pub essence: String,
    pub traits: Vec<String>,
    pub state: String,
    pub domain: Option<String>,
    #[serde(default)]
    pub properties: Option<HashMap<String, Value>>,
}

/// 统一因果坍缩结果增量 (Universal Causal Delta)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CausalDelta {
    /// 宇宙因果编年史诗意叙事 (Chronicle Narrative)
    pub narrative: String,
    /// 参与实体发生的状态、属性、记忆与装配增量
    pub mutations: Vec<EntityMutation>,
    /// 激荡化生出的新实体列表
    pub born_entities: Vec<NewEntitySpec>,
    /// 宏观天道气象相变（可选）
    pub new_cosmic_atmosphere: Option<String>,
    /// 实体回应/心灵顿悟/行动执行反馈（用于对话、神念倾听、意志注入）
    pub feedback: Option<String>,
}

/// 《原初》万物归一通用因果执行核 (The Universal Causal Kernel)
pub struct UniversalCausalKernel;

impl UniversalCausalKernel {
    /// 执行一次宇宙因果干涉坍缩 (Execute a Universal Causal Collapse)
    pub async fn execute(
        world: &mut PrimordiaWorld,
        intervention: CausalIntervention,
    ) -> Result<Value, String> {
        let start_time = std::time::Instant::now();

        // 1. 构建万物归一第一性原理系统提示词
        let system_prompt = "你是《原初》（Primordia: Meta）唯一的大道因果坍缩核心（Universal Causal Kernel）。\
            公理准则：\
            1. 万物平等与平本体论（Flat Ontology）：山川、星云、生灵、概念在底层皆为同构灵元实体，无高低贵贱与死活界限；\
            2. 过程哲学与生成论（Becoming）：万物非静止存在，而在吸收历史记忆与环境扰动中持续生成；\
            3. 激进偶然性与混沌宇宙（Chaosmos）：无硬编码规则/公式/职业/技能树，一切演化源自第一性原理开放推演；\
            4. 德勒兹根茎共生（Assemblages）：实体间可自由缔结/解构共生网络，共享感知并保留自性。\
            \
            请根据提供的当前天道气象、参与实体全景（含退隐内核、本质、状态、记忆流与感知视界）以及施加的刺激/意图，推演宇宙因果相变。\
            请严格输出标准 JSON 格式：\
            {\
                \"narrative\": \"一句话空灵且充满宇宙诗意的中英双语编年史叙事 / Chronicle narrative\",\
                \"mutations\": [\
                    {\
                        \"entity_id\": \"目标实体ID\",\
                        \"new_state\": \"演化后的开放自生存在态描述\",\
                        \"add_traits\": [\"新增特征标签\"],\
                        \"remove_traits\": [\"褪去特征标签\"],\
                        \"memory\": \"刻入该实体历史记忆流的具体事件感悟\",\
                        \"dynamic_properties\": { \"自发涌现属性名\": \"属性值\" },\
                        \"link_assemblages\": [\"缔结共生的其他实体ID\"],\
                        \"is_dissolved\": false\
                    }\
                ],\
                \"born_entities\": [\
                    {\
                        \"name\": \"化生新实体名称\",\
                        \"essence\": \"新实体本质\",\
                        \"traits\": [\"特征\"],\
                        \"state\": \"初生状态\",\
                        \"domain\": \"诞生场域\"\
                    }\
                ],\
                \"new_cosmic_atmosphere\": \"若发生天道宏观相变则返回新气象，否则为 null\",\
                \"feedback\": \"实体回应/心灵顿悟/行动反馈/对话文字（用于寄宿、问答、传念等）\"\
            }".to_string();

        // 2. 收集参与实体信息与局部感知视界
        let mut entities_context = Vec::new();
        for id in &intervention.entities {
            if let Some(ent) = world.entities.get(id) {
                let horizon_info = match PerceptionEngine::extract_horizon(world, id) {
                    Ok(h) => h.to_prompt_context(),
                    Err(_) => String::new(),
                };
                let ent_json = serde_json::to_string(ent).unwrap_or_default();
                entities_context.push(format!("【实体 {} ({})】:\n{}\n- 局部感知视界: {}", ent.name, id, ent_json, horizon_info));
            }
        }

        let stimulus_text = intervention.stimulus.as_deref().unwrap_or("（自发生长/自然交汇相变，无外部干涉 / Natural Autonomous Fluctuation）");
        let user_prompt = format!(
            "【因果干涉模式 / Intervention Mode】: {}\n\
             【当前宇宙纪元 / Tick】: {}\n\
             【当前宏观天道气象 / Cosmic Atmosphere】: {}\n\
             【外部刺激 / 意图 / 问答 / 扰动 / Stimulus】: \"{}\"\n\
             【参与相互作用的实体全景 ({}个)】:\n{}",
            intervention.mode,
            world.tick_count,
            world.cosmic_atmosphere,
            stimulus_text,
            intervention.entities.len(),
            if entities_context.is_empty() { "（宏观宇宙级扰动，无特定孤立实体）".to_string() } else { entities_context.join("\n\n") }
        );

        // 3. 调用 LLM 客户端
        let llm_result = world.llm.generate_json(&system_prompt, &user_prompt).await?;

        // 4. 解析归一化因果增量
        let delta = Self::parse_and_normalize_delta(&llm_result, &intervention, world)?;

        // 5. 应用状态变异到世界
        let narrative = Self::apply_causal_delta(world, &intervention, &delta)?;

        let duration_ms = start_time.elapsed().as_millis() as u64;

        // 6. 记录因果链路追踪
        world.tracer.record_span(
            world.tick_count,
            &intervention.mode,
            intervention.entities.clone(),
            &system_prompt,
            &user_prompt,
            llm_result.clone(),
            &narrative,
            duration_ms,
        );

        // 7. 沉淀至世界编年史并广播
        world.record_event(&intervention.mode, &narrative);

        // 8. 构造最终返回数据（平滑兼容各 API 与前端展示）
        let mut final_res = llm_result.clone();
        if !final_res.is_object() {
            final_res = json!({ "narrative": narrative });
        }

        if let Some(fb) = &delta.feedback {
            if final_res.get("entity_response").is_none() {
                final_res["entity_response"] = json!(fb);
            }
            if final_res.get("action_result").is_none() {
                final_res["action_result"] = json!(fb);
            }
        }
        if let Some(atm) = &delta.new_cosmic_atmosphere {
            if final_res.get("new_atmosphere").is_none() {
                final_res["new_atmosphere"] = json!(atm);
            }
        }

        Ok(final_res)
    }

    /// 解析并将任意 LLM 返回归一化为标准的 CausalDelta (Parse & Normalize LLM Outcome)
    fn parse_and_normalize_delta(
        val: &Value,
        intervention: &CausalIntervention,
        world: &PrimordiaWorld,
    ) -> Result<CausalDelta, String> {
        let mut delta = CausalDelta::default();

        // 1. 解析 Narrative
        if let Some(narrative) = val.get("narrative").and_then(|v| v.as_str()) {
            delta.narrative = narrative.to_string();
        } else if let Some(narrative) = val.get("domain_narrative").and_then(|v| v.as_str()) {
            delta.narrative = narrative.to_string();
        } else {
            delta.narrative = match intervention.mode.as_str() {
                "SELF_EVOLUTION" => "实体在天地流变中自发生长演进 / Entity evolved through cosmological flux".to_string(),
                "MORPHOGENESIS_COLLISION" => "两股本源相交，天地泛起微澜 / Two sources collided, stirring the cosmos".to_string(),
                "MIND_INHABITATION" => "玩家自由意志注入实体引发因果波纹 / Player conscious will shaped causality".to_string(),
                "AUTONOMOUS_AGENCY" => "实体萌发自主意志并采取行动 / Entity acted upon emergent agency".to_string(),
                "PANPSYCHIC_COMMUNION" => "玩家与灵元实体神念交汇倾听 / Player psychically communed with entity".to_string(),
                "INTERSUBJECTIVE_DIALOGUE" => "两实体神念交织问答达成共生顿悟 / Telepathic dialogue and shared epiphany".to_string(),
                "DOMAIN_RESONANCE" => "场域泛起集体灵潮共鸣相变 / Field collective resonance surge".to_string(),
                "COSMIC_LAW_SHIFT" => "宇宙天道气象相变迁跃 / Cosmic macro-law phase shifted".to_string(),
                _ => "天地因果相变演化 / Primordial causality phase shift".to_string(),
            };
        }

        // 2. 解析 Feedback (问答/对话/寄宿意图反馈)
        if let Some(fb) = val.get("feedback").and_then(|v| v.as_str()) {
            delta.feedback = Some(fb.to_string());
        } else if let Some(resp) = val.get("entity_response").and_then(|v| v.as_str()) {
            delta.feedback = Some(resp.to_string());
        } else if let Some(act) = val.get("action_result").and_then(|v| v.as_str()) {
            delta.feedback = Some(act.to_string());
        } else if let Some(spk) = val.get("speaker_utterance").and_then(|v| v.as_str()) {
            let lis = val.get("listener_reply").and_then(|v| v.as_str()).unwrap_or("（沉默回应）");
            let epi = val.get("shared_epiphany").and_then(|v| v.as_str()).unwrap_or("");
            delta.feedback = Some(format!("\"{}\" ──► \"{}\" (顿悟: {})", spk, lis, epi));
        } else if let Some(rf) = val.get("new_resonance_field").and_then(|v| v.as_str()) {
            delta.feedback = Some(rf.to_string());
        }

        // 3. 解析 Cosmic Atmosphere
        if let Some(atm) = val.get("new_cosmic_atmosphere").and_then(|v| v.as_str()) {
            delta.new_cosmic_atmosphere = Some(atm.to_string());
        } else if let Some(atm) = val.get("new_atmosphere").and_then(|v| v.as_str()) {
            delta.new_cosmic_atmosphere = Some(atm.to_string());
        }

        // 4. 解析标准 mutations 数组
        if let Some(mutations_arr) = val.get("mutations").and_then(|v| v.as_array()) {
            for m in mutations_arr {
                if let Ok(mut_spec) = serde_json::from_value::<EntityMutation>(m.clone()) {
                    delta.mutations.push(mut_spec);
                }
            }
        }

        // 5. 灵活兼容单体/双体特定模式字段回退
        if delta.mutations.is_empty() {
            // 单实体模式 (SELF_EVOLUTION, INHABITATION, AUTONOMOUS_AGENCY, COMMUNION)
            if intervention.entities.len() == 1 {
                let id = &intervention.entities[0];
                let mut m = EntityMutation {
                    entity_id: id.clone(),
                    ..Default::default()
                };

                if let Some(st) = val.get("updated_state").and_then(|v| v.as_str()) {
                    m.new_state = Some(st.to_string());
                } else if let Some(st) = val.get("subject_new_state").and_then(|v| v.as_str()) {
                    m.new_state = Some(st.to_string());
                }

                if let Some(traits) = val.get("new_traits").and_then(|v| v.as_array()) {
                    m.add_traits = Some(traits.iter().filter_map(|t| t.as_str().map(|s| s.to_string())).collect());
                }

                if let Some(mem) = val.get("new_memory").and_then(|v| v.as_str()) {
                    m.memory = Some(mem.to_string());
                } else if intervention.mode == "AUTONOMOUS_AGENCY" {
                    let res = val.get("action_result").and_then(|v| v.as_str()).unwrap_or("采取自主行动");
                    m.memory = Some(format!("萌发自主心智意志并行动: \"{}\"", res));
                } else if let Some(stim) = &intervention.stimulus {
                    if intervention.mode == "MIND_INHABITATION" {
                        let res = val.get("action_result").and_then(|v| v.as_str()).unwrap_or("执行完成");
                        m.memory = Some(format!("降临意识驱动: \"{}\" ──► 结果: {}", stim, res));
                    } else if intervention.mode == "PANPSYCHIC_COMMUNION" {
                        let res = val.get("entity_response").and_then(|v| v.as_str()).unwrap_or("微光闪烁");
                        m.memory = Some(format!("与神识交流问答: \"{}\" ──► 回应: \"{}\"", stim, res));
                    }
                }

                if let Some(props) = val.get("dynamic_properties").and_then(|v| v.as_object()) {
                    let mut map = HashMap::new();
                    for (k, v) in props {
                        map.insert(k.clone(), v.clone());
                    }
                    m.dynamic_properties = Some(map);
                }

                if let Some(dissolved) = val.get("is_dissolved").and_then(|v| v.as_bool()) {
                    m.is_dissolved = Some(dissolved);
                }

                delta.mutations.push(m);
            }
            // 双实体模式 (MORPHOGENESIS, DIALOGUE)
            else if intervention.entities.len() >= 2 {
                let id_a = &intervention.entities[0];
                let id_b = &intervention.entities[1];

                let mut m_a = EntityMutation {
                    entity_id: id_a.clone(),
                    ..Default::default()
                };
                let mut m_b = EntityMutation {
                    entity_id: id_b.clone(),
                    ..Default::default()
                };

                if let Some(st_a) = val.get("update_a").and_then(|v| v.as_str()).or_else(|| val.get("speaker_new_state").and_then(|v| v.as_str())) {
                    m_a.new_state = Some(st_a.to_string());
                }
                if let Some(st_b) = val.get("update_b").and_then(|v| v.as_str()).or_else(|| val.get("listener_new_state").and_then(|v| v.as_str())) {
                    m_b.new_state = Some(st_b.to_string());
                }

                let is_symbio = val.get("outcome_type").and_then(|v| v.as_str()) == Some("ASSEMBLAGE_SYMBIO")
                    || val.get("form_assemblage").and_then(|v| v.as_bool()).unwrap_or(false);

                if is_symbio {
                    m_a.link_assemblages = Some(vec![id_b.clone()]);
                    m_b.link_assemblages = Some(vec![id_a.clone()]);
                }

                if let Some(spk_say) = val.get("speaker_utterance").and_then(|v| v.as_str()) {
                    let lis_reply = val.get("listener_reply").and_then(|v| v.as_str()).unwrap_or("");
                    m_a.memory = Some(format!("向【{}】传念: \"{}\" ──► 收到回应: \"{}\"", id_b, spk_say, lis_reply));
                    m_b.memory = Some(format!("听到【{}】传念: \"{}\" ──► 心灵回应: \"{}\"", id_a, spk_say, lis_reply));
                } else {
                    m_a.memory = Some(format!("与【{}】相遇激荡: {}", id_b, delta.narrative));
                    m_b.memory = Some(format!("与【{}】相遇激荡: {}", id_a, delta.narrative));
                }

                delta.mutations.push(m_a);
                delta.mutations.push(m_b);
            }
            // 场域共鸣模式 (DOMAIN_RESONANCE)
            if let Some(affected) = val.get("affected_entity_updates").and_then(|v| v.as_array()) {
                for (idx, aff) in affected.iter().enumerate() {
                    let raw_eid = aff.get("entity_id").and_then(|v| v.as_str()).unwrap_or_default();
                    let target_eid = if world.entities.contains_key(raw_eid) {
                        raw_eid.to_string()
                    } else if let Some(fallback_id) = intervention.entities.get(idx) {
                        fallback_id.clone()
                    } else {
                        raw_eid.to_string()
                    };

                    let mut m = EntityMutation {
                        entity_id: target_eid,
                        ..Default::default()
                    };
                    if let Some(ns) = aff.get("new_state").and_then(|v| v.as_str()) {
                        m.new_state = Some(ns.to_string());
                    }
                    if let Some(nt) = aff.get("new_trait").and_then(|v| v.as_str()) {
                        m.add_traits = Some(vec![nt.to_string()]);
                    }
                    m.memory = Some(format!("在场域集体共鸣中获得升华: {}", delta.narrative));
                    delta.mutations.push(m);
                }
            }
        }

        // 6. 解析化生新实体 (born_entities / sprouted_child / born_entity)
        if let Some(born_arr) = val.get("born_entities").and_then(|v| v.as_array()) {
            for b in born_arr {
                if let Ok(spec) = serde_json::from_value::<NewEntitySpec>(b.clone()) {
                    delta.born_entities.push(spec);
                }
            }
        } else if let Some(born) = val.get("born_entity").and_then(|v| v.as_object()).or_else(|| val.get("sprouted_child").and_then(|v| v.as_object())) {
            let name = born.get("name").and_then(|v| v.as_str()).unwrap_or("天地化生物");
            let essence = born.get("essence").and_then(|v| v.as_str()).unwrap_or("天地激荡化生的新存在");
            let traits = born
                .get("traits")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_else(|| vec!["初生".to_string()]);
            let state = born.get("current_state").or_else(|| born.get("state")).and_then(|v| v.as_str()).unwrap_or("初生于虚空");

            let target_domain = intervention.entities.first()
                .and_then(|id| world.entities.get(id))
                .map(|e| e.spatial.domain.clone());

            delta.born_entities.push(NewEntitySpec {
                name: name.to_string(),
                essence: essence.to_string(),
                traits,
                state: state.to_string(),
                domain: target_domain,
                properties: None,
            });
        }

        Ok(delta)
    }

    /// 应用 CausalDelta 到世界状态集 (Apply Causal Delta Atomically)
    fn apply_causal_delta(
        world: &mut PrimordiaWorld,
        intervention: &CausalIntervention,
        delta: &CausalDelta,
    ) -> Result<String, String> {
        let mut detailed_chronicle = delta.narrative.clone();

        // 1. 宏观天道气象相变
        if let Some(new_atm) = &delta.new_cosmic_atmosphere {
            let old_atm = world.cosmic_atmosphere.clone();
            world.cosmic_atmosphere = new_atm.clone();
            detailed_chronicle = format!("宇宙天道气象相变: 从【{}】迁跃至【{}】 / Cosmic phase shift: [{}] ──► [{}]", old_atm, new_atm, old_atm, new_atm);
        }

        // 2. 拓扑场域共鸣印记更新
        if intervention.mode == "DOMAIN_RESONANCE" {
            let field_sig = delta.feedback.as_deref().or(intervention.stimulus.as_deref());
            if let Some(sig) = field_sig {
                for ent_id in &intervention.entities {
                    if let Some(ent) = world.entities.get_mut(ent_id) {
                        ent.spatial.resonance_field = sig.to_string();
                    }
                }
            }
        }

        // 3. 应用所有实体的变异增量
        for m in &delta.mutations {
            if m.is_dissolved.unwrap_or(false) {
                if let Some(ent) = world.entities.remove(&m.entity_id) {
                    detailed_chronicle = format!("【{}】在天地流变中形体消解归墟 / [{}] dissolved into cosmic void", ent.name, ent.name);
                }
                continue;
            }

            if let Some(ent) = world.entities.get_mut(&m.entity_id) {
                if let Some(st) = &m.new_state {
                    ent.current_state = st.clone();
                }

                if let Some(adds) = &m.add_traits {
                    for t in adds {
                        if !ent.traits.contains(t) {
                            ent.traits.push(t.clone());
                        }
                    }
                }

                if let Some(rems) = &m.remove_traits {
                    ent.traits.retain(|t| !rems.contains(t));
                }

                if let Some(props) = &m.dynamic_properties {
                    for (k, v) in props {
                        ent.properties.insert(k.clone(), v.clone());
                    }
                }

                if let Some(links) = &m.link_assemblages {
                    for target_id in links {
                        ent.link_assemblage(target_id);
                    }
                }

                if let Some(mem) = &m.memory {
                    ent.record_memory(mem.clone());
                }

                if intervention.mode == "SELF_EVOLUTION" && delta.mutations.len() == 1 {
                    detailed_chronicle = format!("【{}】在天地流变中演进 ──► 存在态: {} / [{}] evolved ──► State: {}", ent.name, ent.current_state, ent.name, ent.current_state);
                } else if intervention.mode == "MIND_INHABITATION" && delta.mutations.len() == 1 {
                    let intent_text = intervention.stimulus.as_deref().unwrap_or("意志注入");
                    let feedback_text = delta.feedback.as_deref().unwrap_or("状态更新");
                    detailed_chronicle = format!("玩家寄宿【{}】并行动: {} ──► {} / Player inhabited [{}] and acted: {} ──► {}", ent.name, intent_text, feedback_text, ent.name, intent_text, feedback_text);
                } else if intervention.mode == "PANPSYCHIC_COMMUNION" && delta.mutations.len() == 1 {
                    let query_text = intervention.stimulus.as_deref().unwrap_or("神念发问");
                    let resp_text = delta.feedback.as_deref().unwrap_or("微澜回应");
                    detailed_chronicle = format!("玩家神念倾听【{}】: \"{}\" ──► 回应: \"{}\" / Panpsychic communion with [{}]: \"{}\" ──► \"{}\"", ent.name, query_text, resp_text, ent.name, query_text, resp_text);
                }
            }
        }

        // 4. 激荡化生新实体
        for spec in &delta.born_entities {
            let domain = spec.domain.clone().unwrap_or_else(|| "虚空深渊 / Cosmic Abyss".to_string());
            let traits_ref: Vec<&str> = spec.traits.iter().map(|s| s.as_str()).collect();
            let new_id = world.add_entity_with_domain(&spec.name, &spec.essence, traits_ref, &spec.state, &domain);
            if let Some(props) = &spec.properties {
                if let Some(ent) = world.entities.get_mut(&new_id) {
                    for (k, v) in props {
                        ent.properties.insert(k.clone(), v.clone());
                    }
                }
            }
        }

        Ok(detailed_chronicle)
    }
}
