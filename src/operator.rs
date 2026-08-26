use crate::perception::PerceptionEngine;
use crate::world::PrimordiaWorld;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 因果算子抽象特征 (Causal Operator Trait - Strategy Pattern)
/// 哲学准则：零硬编码业务逻辑。算子只定义意图上下文输入与状态变异应用，所有因果相变由 LLM 实时涌现。
pub trait CausalOperator {
    type Context;
    type Output;

    /// 算子唯一标识名称 (Operator Type Name)
    fn operator_type(&self) -> &'static str;

    /// 构建第一性原理提示词 (Prompt Generation)
    fn build_prompts(&self, world: &PrimordiaWorld, ctx: &Self::Context) -> Result<(String, String), String>;

    /// 提取受影响的目标实体 ID 列表 (Target Entities for Lineage Tracking)
    fn target_entities(&self, ctx: &Self::Context) -> Vec<String>;

    /// 原子应用 LLM 因果裁决并返回结果及事件日志 (Atomic Mutation & Chronicle Detail)
    fn apply_mutation(
        &self,
        world: &mut PrimordiaWorld,
        ctx: &Self::Context,
        llm_result: &Value,
    ) -> Result<(Self::Output, String), String>;
}

// =========================================================================
// 1. 实体自发演化与流变算子 (Autonomous Self-Evolution Operator)
// =========================================================================

pub struct SelfEvolutionOperator;

impl CausalOperator for SelfEvolutionOperator {
    type Context = String; // ent_id
    type Output = Value;

    fn operator_type(&self) -> &'static str {
        "SELF_EVOLUTION"
    }

    fn target_entities(&self, ctx: &Self::Context) -> Vec<String> {
        vec![ctx.clone()]
    }

    fn build_prompts(&self, world: &PrimordiaWorld, ctx: &Self::Context) -> Result<(String, String), String> {
        let ent = world.entities.get(ctx).ok_or_else(|| format!("Entity {} not found", ctx))?;
        let horizon = PerceptionEngine::extract_horizon(world, ctx)?;

        let system_prompt = "你是《原初》元世界第一性原理法则裁决核心。万物处于混沌宇宙（Chaosmos）中，无固定机械宿命，永远处于激进偶然性与分叉流变之中。\
            You are the generative causality arbiter of Primordia Meta-World.\
            请根据实体本质、历史记忆、当前存在态、宏观天道与局部感知视界，推演其自发生长与分叉跃迁。\
            演化方向完全开放：可繁茂生长、异变相融、逆熵晶化、甚至消解散逸归墟。\
            请务必返回 JSON: {\
                updated_state: str, \
                new_traits: list, \
                new_memory: str, \
                dynamic_properties: object, \
                is_dissolved: bool, \
                sprouted_child: object or null\
            }".to_string();

        let ent_json = serde_json::to_string(ent).map_err(|e| e.to_string())?;
        let user_prompt = format!(
            "当前宏观天道气象 / Cosmic Atmosphere: {}\n{}\n目标实体全景（含记忆流与属性）:\n{}",
            world.cosmic_atmosphere,
            horizon.to_prompt_context(),
            ent_json
        );

        Ok((system_prompt, user_prompt))
    }

    fn apply_mutation(
        &self,
        world: &mut PrimordiaWorld,
        ctx: &Self::Context,
        result: &Value,
    ) -> Result<(Self::Output, String), String> {
        let mut event_detail = String::new();
        let mut is_dissolved = false;
        let mut entity_name = String::new();
        let mut domain_name = String::new();

        if let Some(target) = world.entities.get_mut(ctx) {
            entity_name = target.name.clone();
            domain_name = target.spatial.domain.clone();

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
            if let Some(props) = result["dynamic_properties"].as_object() {
                for (k, v) in props {
                    target.properties.insert(k.clone(), v.clone());
                }
            }

            if let Some(dissolved) = result["is_dissolved"].as_bool() {
                is_dissolved = dissolved;
            }

            event_detail = format!(
                "【{}】在天地流变中演进 ──► 存在态: {} / [{}] evolved ──► State: {}",
                entity_name, target.current_state, entity_name, target.current_state
            );
        }

        // 若实体消解散逸，反哺所在场域并解除所有共生关联
        if is_dissolved {
            world.entities.remove(ctx);
            for other_ent in world.entities.values_mut() {
                other_ent.unlink_assemblage(ctx);
            }
            event_detail = format!(
                "【{}】形体消解归墟，本源散逸反哺场域【{}】 / [{}] dissolved into [{}]",
                entity_name, domain_name, entity_name, domain_name
            );
        }

        // 孕育化生出新子实体
        if let Some(child_obj) = result["sprouted_child"].as_object() {
            let child_name = child_obj.get("name").and_then(|v| v.as_str()).unwrap_or("微光灵尘");
            let child_essence = child_obj.get("essence").and_then(|v| v.as_str()).unwrap_or("衍生出的微小灵元");
            let child_traits = child_obj
                .get("traits")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|x| x.as_str()).collect())
                .unwrap_or_else(|| vec!["幼嫩"]);
            let child_state = child_obj.get("current_state").and_then(|v| v.as_str()).unwrap_or("初生微芒");

            let child_id = world.add_entity_with_domain(
                child_name,
                child_essence,
                child_traits,
                child_state,
                &domain_name,
            );

            // 建立父子共生装配关联
            if let Some(parent) = world.entities.get_mut(ctx) {
                parent.link_assemblage(&child_id);
            }
            if let Some(child) = world.entities.get_mut(&child_id) {
                child.link_assemblage(ctx);
            }

            event_detail.push_str(&format!(" 并孕育化生出新灵元【{}】", child_name));
        }

        Ok((result.clone(), event_detail))
    }
}

// =========================================================================
// 2. 交互碰撞、形态相融与共生装配算子 (Morphogenesis & Assemblage Operator)
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorphogenesisContext {
    pub id_a: String,
    pub id_b: String,
}

pub struct MorphogenesisOperator;

impl CausalOperator for MorphogenesisOperator {
    type Context = MorphogenesisContext;
    type Output = Value;

    fn operator_type(&self) -> &'static str {
        "MORPHOGENESIS_COLLISION"
    }

    fn target_entities(&self, ctx: &Self::Context) -> Vec<String> {
        vec![ctx.id_a.clone(), ctx.id_b.clone()]
    }

    fn build_prompts(&self, world: &PrimordiaWorld, ctx: &Self::Context) -> Result<(String, String), String> {
        let ent_a = world.entities.get(&ctx.id_a).ok_or_else(|| format!("Entity {} not found", ctx.id_a))?;
        let ent_b = world.entities.get(&ctx.id_b).ok_or_else(|| format!("Entity {} not found", ctx.id_b))?;

        let system_prompt = "你是《原初》物理与相变法则的唯一因果裁决核心。万物皆平等，无固定合成表或技能树。\
            You are the First Principles Causality Arbiter of Primordia.\
            当两个实体相遇交互时，请基于它们的存在态与本质推演相变：\
            1. MUTUAL_CHANGE: 双方各自发生物理/灵性形变并记录记忆；\
            2. MORPHOGENESIS_NEW: 碰撞激荡化生出第三种全新实体；\
            3. ASSIMILATION: 一方吞纳融合另一方；\
            4. ASSEMBLAGE_SYMBIO: 双方缔结为德勒兹共生装配体（Rhizomatic Symbiosis）。\
            请务必返回 JSON: {\
                outcome_type: str, \
                narrative: str, \
                update_a: str, \
                update_b: str, \
                born_entity: object or null\
            }".to_string();

        let user_prompt = format!(
            "当前宏观天道气象: {}\n实体 A: {}\n实体 B: {}",
            world.cosmic_atmosphere,
            serde_json::to_string(ent_a).map_err(|e| e.to_string())?,
            serde_json::to_string(ent_b).map_err(|e| e.to_string())?
        );

        Ok((system_prompt, user_prompt))
    }

    fn apply_mutation(
        &self,
        world: &mut PrimordiaWorld,
        ctx: &Self::Context,
        result: &Value,
    ) -> Result<(Self::Output, String), String> {
        let outcome_type = result["outcome_type"].as_str().unwrap_or("MUTUAL_CHANGE");
        let narrative = result["narrative"].as_str().unwrap_or("两股本源相交，天地泛起微澜");

        if let Some(up_a) = result["update_a"].as_str() {
            if let Some(ent_a) = world.entities.get_mut(&ctx.id_a) {
                ent_a.current_state = up_a.to_string();
                ent_a.record_memory(format!("与【{}】相遇激荡: {}", ctx.id_b, narrative));
            }
        }

        if let Some(up_b) = result["update_b"].as_str() {
            if let Some(ent_b) = world.entities.get_mut(&ctx.id_b) {
                ent_b.current_state = up_b.to_string();
                ent_b.record_memory(format!("与【{}】相遇激荡: {}", ctx.id_a, narrative));
            }
        }

        if outcome_type == "ASSEMBLAGE_SYMBIO" {
            if let Some(ent_a) = world.entities.get_mut(&ctx.id_a) {
                ent_a.link_assemblage(&ctx.id_b);
            }
            if let Some(ent_b) = world.entities.get_mut(&ctx.id_b) {
                ent_b.link_assemblage(&ctx.id_a);
            }
        }

        if let Some(born) = result["born_entity"].as_object() {
            let name = born.get("name").and_then(|v| v.as_str()).unwrap_or("天地化生物");
            let essence = born.get("essence").and_then(|v| v.as_str()).unwrap_or("两股本源融合而生的全新存在");
            let traits = born
                .get("traits")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|x| x.as_str()).collect())
                .unwrap_or_else(|| vec!["初生", "异变"]);
            let state = born.get("current_state").and_then(|v| v.as_str()).unwrap_or("静静悬浮于虚空");

            let target_domain = world.entities.get(&ctx.id_a).map(|e| e.spatial.domain.clone()).unwrap_or_default();
            let new_id = world.add_entity_with_domain(name, essence, traits, state, &target_domain);

            if let Some(ent) = world.entities.get_mut(&new_id) {
                ent.record_memory(format!("由【{}】与【{}】碰撞化生诞生", ctx.id_a, ctx.id_b));
            }
        }

        Ok((result.clone(), narrative.to_string()))
    }
}

// =========================================================================
// 3. 自由意识寄宿与意志注入算子 (Mind Inhabitation & Will Operator)
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindInhabitationContext {
    pub ent_id: String,
    pub player_intent: String,
}

pub struct MindInhabitationOperator;

impl CausalOperator for MindInhabitationOperator {
    type Context = MindInhabitationContext;
    type Output = Value;

    fn operator_type(&self) -> &'static str {
        "MIND_INHABITATION"
    }

    fn target_entities(&self, ctx: &Self::Context) -> Vec<String> {
        vec![ctx.ent_id.clone()]
    }

    fn build_prompts(&self, world: &PrimordiaWorld, ctx: &Self::Context) -> Result<(String, String), String> {
        let ent = world.entities.get(&ctx.ent_id).ok_or_else(|| format!("Entity {} not found", ctx.ent_id))?;
        let horizon = PerceptionEngine::extract_horizon(world, &ctx.ent_id)?;

        let system_prompt = "你是《原初》意识具身化（Embodied Cognition）物理裁决核心。\
            自由意识（Attention Kernel）降临寄宿并驱动该实体发出自然语言意图。\
            请评估该实体的物理与灵性构造如何具体执行该意图，推演本体状态变化与周围因果波纹。\
            请务必返回 JSON: {action_result: str, subject_new_state: str, environmental_ripple: str}".to_string();

        let ent_json = serde_json::to_string(ent).map_err(|e| e.to_string())?;
        let user_prompt = format!(
            "当前宏观天道气象: {}\n{}\n宿主实体: {}\n降临意志意图: \"{}\"",
            world.cosmic_atmosphere,
            horizon.to_prompt_context(),
            ent_json,
            ctx.player_intent
        );

        Ok((system_prompt, user_prompt))
    }

    fn apply_mutation(
        &self,
        world: &mut PrimordiaWorld,
        ctx: &Self::Context,
        result: &Value,
    ) -> Result<(Self::Output, String), String> {
        let mut event_detail = String::new();

        if let Some(target) = world.entities.get_mut(&ctx.ent_id) {
            target.register_inhabitant("ConsciousAttention");
            if let Some(new_state) = result["subject_new_state"].as_str() {
                target.current_state = new_state.to_string();
            }
            target.record_memory(format!(
                "曾被不可名状的宏大意志降临驱使: {} / Guided by divine intent: {}",
                ctx.player_intent, ctx.player_intent
            ));
            let action_res = result["action_result"].as_str().unwrap_or("");
            event_detail = format!(
                "玩家寄宿【{}】并行动: {} ──► {} / Player inhabited [{}] and acted: {} ──► {}",
                target.name, ctx.player_intent, action_res, target.name, ctx.player_intent, action_res
            );
        }

        Ok((result.clone(), event_detail))
    }
}

// =========================================================================
// 4. 萌芽心智自治行为算子 (Autonomous Agency Operator - Layer 2)
// =========================================================================

pub struct AutonomousAgencyOperator;

impl CausalOperator for AutonomousAgencyOperator {
    type Context = String; // ent_id
    type Output = Value;

    fn operator_type(&self) -> &'static str {
        "AUTONOMOUS_AGENCY"
    }

    fn target_entities(&self, ctx: &Self::Context) -> Vec<String> {
        vec![ctx.clone()]
    }

    fn build_prompts(&self, world: &PrimordiaWorld, ctx: &Self::Context) -> Result<(String, String), String> {
        let ent = world.entities.get(ctx).ok_or_else(|| format!("Entity {} not found", ctx))?;
        let horizon = PerceptionEngine::extract_horizon(world, ctx)?;

        let system_prompt = "你是《原初》灵性心智裁决核心。该实体在天地浸润中萌发自主心智意志。\
            请基于其本质、记忆与局部感知视界，推演其自发的欲望、本体行动与环境波纹。\
            请务必返回 JSON: {autonomous_intent: str, action_execution: str, updated_state: str, environmental_ripple: str}".to_string();

        let ent_json = serde_json::to_string(ent).map_err(|e| e.to_string())?;
        let user_prompt = format!(
            "当前宏观天道气象: {}\n{}\n自主实体: {}",
            world.cosmic_atmosphere,
            horizon.to_prompt_context(),
            ent_json
        );

        Ok((system_prompt, user_prompt))
    }

    fn apply_mutation(
        &self,
        world: &mut PrimordiaWorld,
        ctx: &Self::Context,
        result: &Value,
    ) -> Result<(Self::Output, String), String> {
        let mut event_detail = String::new();

        if let Some(target) = world.entities.get_mut(ctx) {
            if let Some(updated_state) = result["updated_state"].as_str() {
                target.current_state = updated_state.to_string();
            }
            let intent = result["autonomous_intent"].as_str().unwrap_or("探求自身存在的边界 / Seeking existential boundaries");
            let action = result["action_execution"].as_str().unwrap_or("静默散发微光 / Silently radiating faint glow");
            target.record_memory(format!("萌发自主心智意志并行动: {} ──► {}", intent, action));

            event_detail = format!(
                "【{}】萌发自主意志: {} ──► 行动: {} / [{}] autonomous agency: {} ──► {}",
                target.name, intent, action, target.name, intent, action
            );
        }

        Ok((result.clone(), event_detail))
    }
}

// =========================================================================
// 5. 万物泛心论神念倾听与共鸣算子 (Panpsychic Communion Operator - Layer 2)
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunionContext {
    pub ent_id: String,
    pub player_query: String,
}

pub struct PanpsychicCommunionOperator;

impl CausalOperator for PanpsychicCommunionOperator {
    type Context = CommunionContext;
    type Output = Value;

    fn operator_type(&self) -> &'static str {
        "PANPSYCHIC_COMMUNION"
    }

    fn target_entities(&self, ctx: &Self::Context) -> Vec<String> {
        vec![ctx.ent_id.clone()]
    }

    fn build_prompts(&self, world: &PrimordiaWorld, ctx: &Self::Context) -> Result<(String, String), String> {
        let ent = world.entities.get(&ctx.ent_id).ok_or_else(|| format!("Entity {} not found", ctx.ent_id))?;
        let horizon = PerceptionEngine::extract_horizon(world, &ctx.ent_id)?;

        let system_prompt = "你是《原初》万物泛心论（Panpsychism）神念共鸣核心。\
            玩家正在以神识与该灵元实体直接倾听交流。\
            请完全代入该实体（以第一人称‘我’），结合你的本质、退隐内核的隐秘深度、全部历史记忆流与当前感知视界，做出具有哲学韵味、生动自性与诗意意境的真诚回应。\
            请务必返回 JSON: {entity_response: str, inner_resonance: str}".to_string();

        let ent_json = serde_json::to_string(ent).map_err(|e| e.to_string())?;
        let user_prompt = format!(
            "当前宏观天道背景: {}\n{}\n实体深渊全景（含退隐内核与全部记忆流）:\n{}\n玩家神念发问 / Player Inquiry:\n\"{}\"",
            world.cosmic_atmosphere,
            horizon.to_prompt_context(),
            ent_json,
            ctx.player_query
        );

        Ok((system_prompt, user_prompt))
    }

    fn apply_mutation(
        &self,
        world: &mut PrimordiaWorld,
        ctx: &Self::Context,
        result: &Value,
    ) -> Result<(Self::Output, String), String> {
        let mut event_detail = String::new();

        if let Some(target) = world.entities.get_mut(&ctx.ent_id) {
            let response = result["entity_response"].as_str().unwrap_or("静默地感受着神念的触碰 / Silently feeling the psychic touch");
            let inner = result["inner_resonance"].as_str().unwrap_or("微澜泛起 / Faint ripples");
            target.record_memory(format!("与神识交流问答: \"{}\" ──► 回应: \"{}\"", ctx.player_query, response));

            event_detail = format!(
                "玩家神念倾听【{}】: \"{}\" ──► 回应: \"{}\" (内在共鸣: {}) / Panpsychic communion with [{}]: \"{}\" ──► \"{}\"",
                target.name, ctx.player_query, response, inner, target.name, ctx.player_query, response
            );
        }

        Ok((result.clone(), event_detail))
    }
}

// =========================================================================
// 6. 宏观天道法则相变算子 (Cosmic Law Shift Operator)
// =========================================================================

pub struct CosmicLawOperator;

impl CausalOperator for CosmicLawOperator {
    type Context = ();
    type Output = String;

    fn operator_type(&self) -> &'static str {
        "COSMIC_LAW_SHIFT"
    }

    fn target_entities(&self, _ctx: &Self::Context) -> Vec<String> {
        vec!["@COSMOS".to_string()]
    }

    fn build_prompts(&self, world: &PrimordiaWorld, _ctx: &Self::Context) -> Result<(String, String), String> {
        let system_prompt = "你是《原初》宏观天道推演核心。请根据当前世界纪元、实体总数与历史编年，推演世界宏观法则/环境气候的迁跃相变。\
            请务必返回 JSON: {new_atmosphere: str, cosmic_ripple: str}".to_string();

        let latest_event = world.chronicle.last().map(|e| e.detail.as_str()).unwrap_or("无 / None");
        let user_prompt = format!(
            "当前纪元 / Current Tick: {}\n当前天道气象 / Current Atmosphere: {}\n实体总数 / Total Entities: {}\n最新事件 / Latest Event: {}",
            world.tick_count,
            world.cosmic_atmosphere,
            world.entities.len(),
            latest_event
        );

        Ok((system_prompt, user_prompt))
    }

    fn apply_mutation(
        &self,
        world: &mut PrimordiaWorld,
        _ctx: &Self::Context,
        result: &Value,
    ) -> Result<(Self::Output, String), String> {
        let new_atmosphere = result["new_atmosphere"]
            .as_str()
            .unwrap_or("流变不息的太初气象 / Ever-shifting primordial atmosphere")
            .to_string();

        let old_atmosphere = world.cosmic_atmosphere.clone();
        world.cosmic_atmosphere = new_atmosphere.clone();

        let event_detail = format!(
            "宇宙天道气象相变: 从【{}】迁跃至【{}】 / Cosmic phase shift: [{}] ──► [{}]",
            old_atmosphere, new_atmosphere, old_atmosphere, new_atmosphere
        );

        Ok((new_atmosphere, event_detail))
    }
}

// =========================================================================
// 7. 拓扑场域集体共鸣相变算子 (Domain Collective Resonance Operator)
// =========================================================================

pub struct DomainResonanceOperator;

impl CausalOperator for DomainResonanceOperator {
    type Context = String; // domain_name
    type Output = Value;

    fn operator_type(&self) -> &'static str {
        "DOMAIN_RESONANCE"
    }

    fn target_entities(&self, ctx: &Self::Context) -> Vec<String> {
        vec![format!("@DOMAIN:{}", ctx)]
    }

    fn build_prompts(&self, world: &PrimordiaWorld, ctx: &Self::Context) -> Result<(String, String), String> {
        let domain_entities: Vec<&crate::entity::Entity> = world
            .entities
            .values()
            .filter(|e| e.spatial.domain.contains(ctx.as_str()))
            .collect();

        let system_prompt = "你是《原初》拓扑场域集体共鸣相变裁决核心。\
            当一个场域内的多个实体发生集体共振激荡时，将引发场域级灵潮相变与涌现异象。\
            请务必返回 JSON: {\
                domain_narrative: str, \
                new_resonance_field: str, \
                emergent_phenomenon: str, \
                affected_entity_updates: list of {entity_id: str, new_state: str, new_trait: str}\
            }".to_string();

        let entities_json = serde_json::to_string(&domain_entities).map_err(|e| e.to_string())?;
        let current_resonance = domain_entities
            .first()
            .map(|e| e.spatial.resonance_field.as_str())
            .unwrap_or("静默微澜");

        let user_prompt = format!(
            "目标拓扑场域: {}\n当前场域共鸣印记: {}\n场域内实体群 (共 {} 个):\n{}",
            ctx,
            current_resonance,
            domain_entities.len(),
            entities_json
        );

        Ok((system_prompt, user_prompt))
    }

    fn apply_mutation(
        &self,
        world: &mut PrimordiaWorld,
        ctx: &Self::Context,
        result: &Value,
    ) -> Result<(Self::Output, String), String> {
        let narrative = result["domain_narrative"].as_str().unwrap_or("场域内灵元交汇，掀起集体共鸣激荡");
        let new_resonance = result["new_resonance_field"].as_str().unwrap_or("灵潮交织共鸣场");

        for ent in world.entities.values_mut() {
            if ent.spatial.domain.contains(ctx.as_str()) {
                ent.spatial.resonance_field = new_resonance.to_string();
                ent.record_memory(format!("经历了场域【{}】的集体共鸣激荡: {}", ctx, narrative));
            }
        }

        if let Some(updates) = result["affected_entity_updates"].as_array() {
            for item in updates {
                if let Some(id) = item.get("entity_id").and_then(|v| v.as_str()) {
                    if let Some(target) = world.entities.get_mut(id) {
                        if let Some(ns) = item.get("new_state").and_then(|v| v.as_str()) {
                            target.current_state = ns.to_string();
                        }
                        if let Some(nt) = item.get("new_trait").and_then(|v| v.as_str()) {
                            if !target.traits.contains(&nt.to_string()) {
                                target.traits.push(nt.to_string());
                            }
                        }
                    }
                }
            }
        }

        let event_detail = format!(
            "拓扑场域【{}】爆发集体共鸣: {} (共鸣印记 ──► {}) / Collective resonance in [{}]: {}",
            ctx, narrative, new_resonance, ctx, narrative
        );

        Ok((result.clone(), event_detail))
    }
}

// =========================================================================
// 8. 通用因果流水线执行器 (Universal Causal Pipeline Executor)
// =========================================================================

pub struct CausalExecutor;

impl CausalExecutor {
    pub async fn execute<Op: CausalOperator>(
        world: &mut PrimordiaWorld,
        op: &Op,
        ctx: &Op::Context,
    ) -> Result<Op::Output, String> {
        let start_time = std::time::Instant::now();

        // 1. 构建提示词
        let (sys_prompt, usr_prompt) = op.build_prompts(world, ctx)?;
        let target_entities = op.target_entities(ctx);

        // 2. 调用 LLM 客户端
        let llm_result = world.llm().generate_json(&sys_prompt, &usr_prompt).await?;

        // 3. 应用状态变异并记录编年史
        let (output, event_detail) = op.apply_mutation(world, ctx, &llm_result)?;

        let duration_ms = start_time.elapsed().as_millis() as u64;

        // 4. 记录全因果链路追踪
        world.tracer.record_span(
            world.tick_count,
            op.operator_type(),
            target_entities,
            &sys_prompt,
            &usr_prompt,
            llm_result,
            &event_detail,
            duration_ms,
        );

        // 5. 沉淀至世界编年史并广播
        world.record_event(op.operator_type(), &event_detail);

        Ok(output)
    }
}
