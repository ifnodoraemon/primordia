use crate::perception::PerceptionEngine;
use crate::world::PrimordiaWorld;
use serde_json::Value;

/// 因果算子抽象特征 (Causal Operator Trait - Strategy Pattern)
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
// 1. 自生长与生命周期演化算子 (Self-Evolution & Lifecycle Operator)
// =========================================================================

pub struct SelfEvolutionOperator;

impl CausalOperator for SelfEvolutionOperator {
    type Context = String; // ent_id
    type Output = Value;

    fn operator_type(&self) -> &'static str {
        "SELF_EVOLVE"
    }

    fn target_entities(&self, ctx: &Self::Context) -> Vec<String> {
        vec![ctx.clone()]
    }

    fn build_prompts(&self, world: &PrimordiaWorld, ctx: &Self::Context) -> Result<(String, String), String> {
        let ent = world.entities.get(ctx).ok_or_else(|| format!("Entity {} not found", ctx))?;
        let horizon = PerceptionEngine::extract_horizon(world, ctx)?;

        let system_prompt = "你是《原初》元世界法则裁决核心。万物皆有灵性，自发遵循‘成·住·坏·空’（初生·成熟·风化衰败·归墟解体）自创生法则演化。\
            You are the generative causality arbiter of Primordia Meta-World.\
            请根据其实体本质、全部历史记忆（包括曾经被意识寄宿/碰撞交互的过往）、宏观天道气象与局部感知视界，推演其自演化。\
            评估其实体结构是欣欣向荣、发生异变突变、或是遭受风化衰变/病变老化，乃至耗散归墟。\
            请务必返回 JSON: {\
                updated_state: str, \
                new_traits: list, \
                new_memory: str, \
                lifecycle_phase: str (Genesis/Flourishing/Decay/Dissolution), \
                cohesion_change: float (-0.3 to +0.2), \
                domain_nourishment: str or null, \
                sprouted_child: object or null\
            }".to_string();

        let ent_json = serde_json::to_string(ent).map_err(|e| e.to_string())?;
        let user_prompt = format!(
            "当前宏观天道气象 / Cosmic Atmosphere: {}\n{}\n目标实体全景（含历史记忆流） / Target Entity Full Context:\n{}",
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
        let mut domain_name = String::new();
        let mut entity_name = String::new();

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

            // 更新生命周期阶段与存在凝聚度 (Lifecycle Phase & Cohesion)
            if let Some(phase_str) = result["lifecycle_phase"].as_str() {
                target.lifecycle = crate::entity::LifecyclePhase::from_str_loose(phase_str);
            }
            if let Some(delta) = result["cohesion_change"].as_f64() {
                target.cohesion = (target.cohesion + delta).clamp(0.0, 1.0);
            }

            // 判断是否达到消解归墟阶段 (Dissolution / Return to Void)
            if target.lifecycle == crate::entity::LifecyclePhase::Dissolution || target.cohesion <= 0.05 {
                is_dissolved = true;
                target.lifecycle = crate::entity::LifecyclePhase::Dissolution;
                event_detail = format!(
                    "【{}】历经漫长岁月演变，形体耗散归墟，灵蕴反哺场域【{}】 / [{}] dissolved into void, nourishing domain [{}]",
                    target.name, target.spatial.domain, target.name, target.spatial.domain
                );
            } else {
                event_detail = format!(
                    "【{}】发生自演化（阶段: {} | 凝聚度: {:.2}）：{} / [{}] evolved ({}: {:.2}): {}",
                    target.name,
                    target.lifecycle.as_str(),
                    target.cohesion,
                    target.current_state,
                    target.name,
                    target.lifecycle.as_str(),
                    target.cohesion,
                    target.current_state
                );
            }
        }

        // 如果实体彻底解体归墟，解除共生装配关联
        if is_dissolved {
            for (_, other) in world.entities.iter_mut() {
                other.unlink_assemblage(ctx);
            }
            let nourishment = result["domain_nourishment"].as_str().unwrap_or("沉淀下微弱的灵性尘埃，滋养周遭万物 / Precipitaded ethereal stardust, nourishing surroundings");
            world.record_event(
                "ENTITY_DISSOLUTION",
                &format!("【{}】归墟反哺：{} / [{}] Void Return: {}", entity_name, nourishment, entity_name, nourishment),
            );
        }

        // 检查是否有自创生子实体分裂萌芽 (Sprouted Child)
        if result["sprouted_child"].is_object() {
            let child = &result["sprouted_child"];
            let name = child["name"].as_str().unwrap_or("新生灵元 / Sprouted Animus");
            let essence = child["essence"].as_str().unwrap_or("演化分裂出的新存在 / Emerging existence");
            let state = child["current_state"].as_str().unwrap_or("");
            let traits_arr: Vec<&str> = child["traits"]
                .as_array()
                .map(|a| a.iter().filter_map(|v| v.as_str()).collect())
                .unwrap_or_default();
            world.add_entity_with_domain(name, essence, traits_arr, state, &domain_name);
        }

        Ok((result.clone(), event_detail))
    }
}

// =========================================================================
// 2. 碰撞化生与共生装配算子 (Morphogenesis & Assemblage Operator)
// =========================================================================

pub struct MorphogenesisOperator;

pub struct MorphogenesisContext {
    pub id_a: String,
    pub id_b: String,
}

impl CausalOperator for MorphogenesisOperator {
    type Context = MorphogenesisContext;
    type Output = Value;

    fn operator_type(&self) -> &'static str {
        "COLLISION_MORPHOGENESIS"
    }

    fn target_entities(&self, ctx: &Self::Context) -> Vec<String> {
        vec![ctx.id_a.clone(), ctx.id_b.clone()]
    }

    fn build_prompts(&self, world: &PrimordiaWorld, ctx: &Self::Context) -> Result<(String, String), String> {
        let ent_a = world.entities.get(&ctx.id_a).ok_or_else(|| format!("Entity {} not found", ctx.id_a))?;
        let ent_b = world.entities.get(&ctx.id_b).ok_or_else(|| format!("Entity {} not found", ctx.id_b))?;

        let system_prompt = "你是《原初》元世界法则裁决核心。两实体发生交互碰撞与交融。\
            You are the generative causality arbiter of Primordia Meta-World.\
            基于双方本质、感官界面与宏观天道裁决相变结果：互相改变(MUTUAL_CHANGE)、共生装配(ASSEMBLAGE_SYMBIOSIS)、或天地化生(MORPHOGENESIS_NEW)。\
            请务必返回 JSON: {narrative: str, outcome_type: str, born_entity: object or null, update_a: str, update_b: str}".to_string();

        let user_prompt = format!(
            "当前天道气象 / Cosmic Atmosphere: {}\n实体 A 感官界面 / Entity A Sensual Interface: {}\n实体 A 详情: {}\n实体 B 感官界面 / Entity B Sensual Interface: {}\n实体 B 详情: {}",
            world.cosmic_atmosphere,
            ent_a.sensory_manifestation(),
            serde_json::to_string(ent_a).map_err(|e| e.to_string())?,
            ent_b.sensory_manifestation(),
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
        let narrative = result["narrative"].as_str().unwrap_or("两实体发生了碰撞交互。 / Entities collided.").to_string();
        let outcome_type = result["outcome_type"].as_str().unwrap_or("MUTUAL_CHANGE");
        let event_detail = format!("[{}] {}", outcome_type, narrative);

        if outcome_type == "ASSEMBLAGE_SYMBIOSIS" {
            let _ = world.form_assemblage(&ctx.id_a, &ctx.id_b, &narrative);
        }

        if let Some(target_a) = world.entities.get_mut(&ctx.id_a) {
            if let Some(up_a) = result["update_a"].as_str() {
                target_a.current_state = up_a.to_string();
            }
        }

        if let Some(target_b) = world.entities.get_mut(&ctx.id_b) {
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
            world.add_entity(name, essence, traits_arr, state);
        }

        Ok((result.clone(), event_detail))
    }
}

// =========================================================================
// 3. 自由觉知寄宿算子 (Mind Inhabitation & Agency Operator)
// =========================================================================

pub struct MindInhabitationOperator;

pub struct MindInhabitationContext {
    pub ent_id: String,
    pub player_intent: String,
}

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

        let system_prompt = "你是《原初》元世界法则裁决核心。玩家作为原初自由意志，寄宿于该实体并发出行动意图。\
            You are the generative causality arbiter of Primordia Meta-World.\
            请评估该实体的物理/灵性本质结合周围环境如何响应此意图，推导其自身状态变化与对周围环境的波纹。\
            请务必返回 JSON: {action_result: str, subject_new_state: str, environmental_ripple: str}".to_string();

        let ent_json = serde_json::to_string(ent).map_err(|e| e.to_string())?;
        let user_prompt = format!(
            "当前天道气象 / Cosmic Atmosphere: {}\n{}\n寄宿实体 / Inhabited Entity: {}\n玩家自由意志意图 / Player Intent: '{}'",
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
            You are the Animus Cognition Arbiter of Primordia.\
            请基于其本质特征、历史记忆与当前局部感知视界，推演其自发萌生的意图欲望、付诸的本体行动、自身状态变化与对周围的波纹。\
            请务必返回 JSON: {autonomous_intent: str, action_execution: str, updated_state: str, environmental_ripple: str}".to_string();

        let ent_json = serde_json::to_string(ent).map_err(|e| e.to_string())?;
        let user_prompt = format!(
            "当前宏观天道气象 / Cosmic Atmosphere: {}\n{}\n自主实体 / Autonomous Entity: {}",
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
// 5. 宏观天道法则相变算子 (Cosmic Law Shift Operator)
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
            You are the Cosmic Arbiter of Primordia. Reason through the macro-law / atmospheric phase shift of the universe.\
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
        let new_atmo = result["new_atmosphere"].as_str().unwrap_or(&world.cosmic_atmosphere).to_string();
        let ripple = result["cosmic_ripple"].as_str().unwrap_or("天道气象微微流变 / Cosmic atmosphere gently ripples").to_string();

        world.cosmic_atmosphere = new_atmo.clone();
        let event_detail = format!(
            "宏观天道气象发生纪元相变：{} ──► 波纹: {} / Cosmic law phase shift: {} ──► Ripple: {}",
            new_atmo, ripple, new_atmo, ripple
        );

        Ok((new_atmo, event_detail))
    }
}

// =========================================================================
// 6. 因果执行器流水线 (Causal Pipeline Executor)
// =========================================================================

pub struct CausalExecutor;

impl CausalExecutor {
    /// 统一执行任何符合 CausalOperator 契约的因果算子 (Pipeline Pattern)
    pub async fn execute<O: CausalOperator>(
        world: &mut PrimordiaWorld,
        operator: &O,
        ctx: &O::Context,
    ) -> Result<O::Output, String> {
        let (system_prompt, user_prompt) = operator.build_prompts(world, ctx)?;
        let targets = operator.target_entities(ctx);
        let op_type = operator.operator_type();
        let tick = world.tick_count;

        let start_time = std::time::Instant::now();
        let llm_result = world.llm().generate_json(&system_prompt, &user_prompt).await?;
        let duration_ms = start_time.elapsed().as_millis() as u64;

        // 原子提交状态突变并获取编年史描述
        let (output, event_detail) = operator.apply_mutation(world, ctx, &llm_result)?;

        // 统一沉淀编年史
        world.record_event(op_type, &event_detail);

        // 统一记录全链路因果 Span 追踪
        world.tracer.record_span(
            tick,
            op_type,
            targets,
            &system_prompt,
            &user_prompt,
            llm_result,
            &event_detail,
            duration_ms,
        );

        Ok(output)
    }
}
