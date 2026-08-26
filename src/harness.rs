use crate::llm::LlmClient;
use crate::world::PrimordiaWorld;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;

/// 创世实体规格配置 (Genesis Entity Spec)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisSpec {
    pub name: String,
    pub essence: String,
    pub traits: Vec<String>,
    pub state: String,
    pub domain: String,
}

/// 步骤执行结果状态 (Step Execution Outcome)
pub enum StepOutcome {
    ActionSuccess,
    AssertionPassed(String),
    AssertionFailed(String),
}

/// Harness 仿真驱动单步动作 (Harness Command Pattern)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HarnessStep {
    /// 玩家意识寄宿并行动
    InhabitAndAct {
        entity_name_or_id: String,
        intent: String,
    },
    /// 实体自发萌发心智意志并行动
    ActAutonomously {
        entity_name_or_id: String,
    },
    /// 两实体相撞/化生
    Collide {
        entity_a: String,
        entity_b: String,
    },
    /// 宏观天道气象相变
    ShiftCosmicLaw,
    /// 触发场域集体共鸣与灵潮相变
    TriggerDomainResonance { domain_name: String },
    /// 推进自演化纪元
    TickEpoch { count: u64 },
    /// 断言：世界实体总数检查
    AssertEntityCount { expected: usize },
    /// 断言：实体具备某特征
    AssertEntityHasTrait {
        entity_name_substr: String,
        trait_substr: String,
    },
    /// 断言：编年史包含某类型事件
    AssertChronicleContains { event_type: String },
}

impl HarnessStep {
    /// 执行单个命令或断言 (Command Execution)
    pub async fn execute(&self, world: &mut PrimordiaWorld) -> Result<StepOutcome, String> {
        match self {
            HarnessStep::InhabitAndAct { entity_name_or_id, intent } => {
                let target_id = find_entity_id(world, entity_name_or_id)
                    .ok_or_else(|| format!("Entity '{}' not found for InhabitAndAct", entity_name_or_id))?;
                world.inhabit_and_act(&target_id, intent).await?;
                Ok(StepOutcome::ActionSuccess)
            }
            HarnessStep::ActAutonomously { entity_name_or_id } => {
                let target_id = find_entity_id(world, entity_name_or_id)
                    .ok_or_else(|| format!("Entity '{}' not found for ActAutonomously", entity_name_or_id))?;
                world.act_autonomously(&target_id).await?;
                Ok(StepOutcome::ActionSuccess)
            }
            HarnessStep::Collide { entity_a, entity_b } => {
                let id_a = find_entity_id(world, entity_a)
                    .ok_or_else(|| format!("Entity A '{}' not found for Collide", entity_a))?;
                let id_b = find_entity_id(world, entity_b)
                    .ok_or_else(|| format!("Entity B '{}' not found for Collide", entity_b))?;
                world.collide(&id_a, &id_b).await?;
                Ok(StepOutcome::ActionSuccess)
            }
            HarnessStep::ShiftCosmicLaw => {
                world.evolve_cosmic_law().await?;
                Ok(StepOutcome::ActionSuccess)
            }
            HarnessStep::TriggerDomainResonance { domain_name } => {
                world.trigger_domain_resonance(domain_name).await?;
                Ok(StepOutcome::ActionSuccess)
            }
            HarnessStep::TickEpoch { count } => {
                for _ in 0..*count {
                    world.tick().await?;
                }
                Ok(StepOutcome::ActionSuccess)
            }
            HarnessStep::AssertEntityCount { expected } => {
                let actual = world.entities.len();
                if actual == *expected {
                    Ok(StepOutcome::AssertionPassed(format!("Entity count is {} as expected.", actual)))
                } else {
                    Ok(StepOutcome::AssertionFailed(format!("Expected entity count {}, found {}", expected, actual)))
                }
            }
            HarnessStep::AssertEntityHasTrait { entity_name_substr, trait_substr } => {
                let matched = world.entities.values().find(|e| e.name.contains(entity_name_substr));
                match matched {
                    Some(ent) => {
                        let has_trait = ent.traits.iter().any(|t| t.contains(trait_substr));
                        if has_trait {
                            Ok(StepOutcome::AssertionPassed(format!("Entity [{}] possesses trait '{}'.", ent.name, trait_substr)))
                        } else {
                            Ok(StepOutcome::AssertionFailed(format!("Entity [{}] does not have trait '{}'", ent.name, trait_substr)))
                        }
                    }
                    None => {
                        Ok(StepOutcome::AssertionFailed(format!("Entity with name substr '{}' not found for trait assertion", entity_name_substr)))
                    }
                }
            }
            HarnessStep::AssertChronicleContains { event_type } => {
                let has_event = world.chronicle.iter().any(|e| &e.event_type == event_type);
                if has_event {
                    Ok(StepOutcome::AssertionPassed(format!("Chronicle contains event type '{}'.", event_type)))
                } else {
                    Ok(StepOutcome::AssertionFailed(format!("Chronicle does not contain event type '{}'", event_type)))
                }
            }
        }
    }
}

/// 辅助函数：根据名称子串或 ID 精准定位实体
fn find_entity_id(world: &PrimordiaWorld, name_or_id: &str) -> Option<String> {
    if world.entities.contains_key(name_or_id) {
        return Some(name_or_id.to_string());
    }
    for (id, ent) in &world.entities {
        if ent.name.contains(name_or_id) || id == name_or_id {
            return Some(id.clone());
        }
    }
    None
}

/// 仿真剧本定义 (Simulation Scenario)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub name: String,
    pub description: String,
    pub genesis_entities: Vec<GenesisSpec>,
    pub steps: Vec<HarnessStep>,
}

/// Harness 执行报告 (Simulation Execution Report)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarnessReport {
    pub scenario_name: String,
    pub success: bool,
    pub steps_executed: usize,
    pub assertions_passed: usize,
    pub assertions_failed: usize,
    pub failures: Vec<String>,
    pub total_entities: usize,
    pub total_chronicle_events: usize,
    pub duration_ms: u64,
    pub trace_summary: String,
}

/// 世界仿真与测试驾驭台 (World Simulation & Verification Harness)
pub struct SimulationHarness {
    pub world: PrimordiaWorld,
}

impl SimulationHarness {
    pub fn new(world_name: &str, llm: Arc<dyn LlmClient>) -> Self {
        Self {
            world: PrimordiaWorld::with_llm(world_name, llm),
        }
    }

    /// 执行完整仿真剧本 (Execute Scenario)
    pub async fn run_scenario(&mut self, scenario: Scenario) -> Result<HarnessReport, String> {
        let start_time = Instant::now();
        println!("\n╔══════════════════════════════════════════════════════════════════╗");
        println!("║ 🚀 启动仿真驾驭台 / Launching Simulation Harness                  ║");
        println!("║ 剧本 / Scenario: {:<47} ║", scenario.name);
        println!("╚══════════════════════════════════════════════════════════════════╝");
        println!("📖 描述 / Description: {}\n", scenario.description);

        // 1. 创世阶段 / Genesis Phase
        for spec in &scenario.genesis_entities {
            let traits_refs: Vec<&str> = spec.traits.iter().map(|s| s.as_str()).collect();
            self.world.add_entity_with_domain(
                &spec.name,
                &spec.essence,
                traits_refs,
                &spec.state,
                &spec.domain,
            );
        }

        let mut assertions_passed = 0;
        let mut assertions_failed = 0;
        let mut failures = Vec::new();
        let mut steps_executed = 0;

        // 2. 依次驱动各个仿真命令 (Command Pattern Execution)
        for step in scenario.steps {
            steps_executed += 1;
            match step.execute(&mut self.world).await? {
                StepOutcome::ActionSuccess => {}
                StepOutcome::AssertionPassed(msg) => {
                    assertions_passed += 1;
                    println!("  ✅ [Assertion Passed] {}", msg);
                }
                StepOutcome::AssertionFailed(msg) => {
                    assertions_failed += 1;
                    println!("  ❌ [Assertion FAILED] {}", msg);
                    failures.push(msg);
                }
            }
        }

        let duration_ms = start_time.elapsed().as_millis() as u64;
        let trace_summary = self.world.tracer.summary();
        let success = assertions_failed == 0;

        let report = HarnessReport {
            scenario_name: scenario.name,
            success,
            steps_executed,
            assertions_passed,
            assertions_failed,
            failures,
            total_entities: self.world.entities.len(),
            total_chronicle_events: self.world.chronicle.len(),
            duration_ms,
            trace_summary,
        };

        println!("\n📊 ═══ 驾驭台执行报告 / Harness Execution Report ═══");
        println!("  - 状态 / Status: {}", if success { "✅ SUCCESS" } else { "❌ FAILED" });
        println!("  - 执行步数 / Steps: {}", report.steps_executed);
        println!("  - 断言通过/失败 / Assertions: {} passed, {} failed", report.assertions_passed, report.assertions_failed);
        println!("  - 耗时 / Duration: {}ms", report.duration_ms);
        println!("  - 链路追踪 / Trace: {}", report.trace_summary);

        Ok(report)
    }

    /// 获取内置基准预设剧本列表 (Preset Benchmark Scenarios)
    pub fn preset_scenarios() -> Vec<Scenario> {
        vec![
            Scenario {
                name: "原初创世与双灵共生 / Genesis & Symbiosis".to_string(),
                description: "验证创世凝结、意识降临、碰撞相变与自演化生长 / Verify genesis, inhabitation, collision and autonomous growth".to_string(),
                genesis_entities: vec![
                    GenesisSpec {
                        name: "青峭古石".to_string(),
                        essence: "伫立绝壁的沉寂岩石".to_string(),
                        traits: vec!["坚固".to_string(), "微光".to_string()],
                        state: "沉睡于风雨之中".to_string(),
                        domain: "悬天绝壁".to_string(),
                    },
                    GenesisSpec {
                        name: "地脉游火".to_string(),
                        essence: "裂缝中跳跃的赤炎".to_string(),
                        traits: vec!["炽热".to_string(), "灵动".to_string()],
                        state: "吞吐火星".to_string(),
                        domain: "地底裂隙".to_string(),
                    },
                ],
                steps: vec![
                    HarnessStep::InhabitAndAct {
                        entity_name_or_id: "青峭古石".to_string(),
                        intent: "向地脉游火发出共鸣 ──► 青石释放引力波".to_string(),
                    },
                    HarnessStep::Collide {
                        entity_a: "青峭古石".to_string(),
                        entity_b: "地脉游火".to_string(),
                    },
                    HarnessStep::ShiftCosmicLaw,
                    HarnessStep::TickEpoch { count: 1 },
                    HarnessStep::AssertEntityCount { expected: 2 },
                    HarnessStep::AssertChronicleContains {
                        event_type: "MIND_INHABITATION".to_string(),
                    },
                    HarnessStep::AssertChronicleContains {
                        event_type: "COSMIC_LAW_SHIFT".to_string(),
                    },
                ],
            },
            Scenario {
                name: "场域共鸣与神话重铸 / Domain Resonance & Mythos".to_string(),
                description: "验证多实体场域集体共鸣灵潮与纪元神话提炼 / Verify field resonance and mythos distillation".to_string(),
                genesis_entities: vec![
                    GenesisSpec {
                        name: "雷鸣玄岩".to_string(),
                        essence: "沐浴万千雷霆的古岩".to_string(),
                        traits: vec!["导电".to_string(), "坚硬".to_string()],
                        state: "吸纳天地雷元".to_string(),
                        domain: "万仞雷池".to_string(),
                    },
                    GenesisSpec {
                        name: "紫霄游电".to_string(),
                        essence: "虚空跳跃的紫电之灵".to_string(),
                        traits: vec!["迅捷".to_string(), "破虚".to_string()],
                        state: "穿梭于雷池边缘".to_string(),
                        domain: "万仞雷池".to_string(),
                    },
                ],
                steps: vec![
                    HarnessStep::TriggerDomainResonance {
                        domain_name: "万仞雷池".to_string(),
                    },
                    HarnessStep::TickEpoch { count: 2 },
                    HarnessStep::AssertChronicleContains {
                        event_type: "DOMAIN_RESONANCE".to_string(),
                    },
                ],
            },
        ]
    }
}
