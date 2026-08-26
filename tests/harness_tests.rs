use async_trait::async_trait;
use primordia::{
    GenesisSpec, HarnessStep, LlmClient, Scenario, SimulationHarness,
};
use serde_json::Value;
use std::sync::Arc;

struct MockHarnessLlm;

#[async_trait]
impl LlmClient for MockHarnessLlm {
    async fn generate_json(&self, system_prompt: &str, user_prompt: &str) -> Result<Value, String> {
        if system_prompt.contains("宏观天道") || system_prompt.contains("Cosmic Arbiter") {
            return Ok(serde_json::json!({
                "new_atmosphere": "太初星云纪元：灵元重力场扩散",
                "cosmic_ripple": "虚空引力常数微颤"
            }));
        }

        if user_prompt.contains("共鸣") {
            return Ok(serde_json::json!({
                "action_result": "青石释放沉寂已久的引力波",
                "subject_new_state": "表层泛起星光共振",
                "environmental_ripple": "火星向青石聚拢"
            }));
        }

        Ok(serde_json::json!({
            "outcome_type": "ASSEMBLAGE_SYMBIOSIS",
            "narrative": "青石与游火交织，形成石火共生体",
            "update_a": "体内熔铸火纹",
            "update_b": "焰心凝结青石辉光",
            "born_entity": null,
            "updated_state": "自发旋转吸收虚空灵气",
            "new_traits": ["灵气自旋 / Spontaneous Spin"],
            "new_memory": "吸纳了星云纪元的灵气"
        }))
    }
}

#[tokio::test]
async fn test_harness_driven_scenario_execution() {
    let mock_llm = Arc::new(MockHarnessLlm);
    let mut harness = SimulationHarness::new("Harness 仿真测试世界", mock_llm);

    let scenario = Scenario {
        name: "原初相变与共生基准剧本 / Genesis & Symbiosis Benchmark".to_string(),
        description: "验证创世、意识寄宿、碰撞共生、天道迁跃与断言检查 / Verify genesis, inhabitation, symbiosis, cosmic shift and assertions".to_string(),
        genesis_entities: vec![
            GenesisSpec {
                name: "青峭古石".to_string(),
                essence: "伫立绝壁的沉寂岩石".to_string(),
                traits: vec!["致密".to_string(), "微凉".to_string()],
                state: "静卧于云雾中".to_string(),
                domain: "悬天绝壁".to_string(),
            },
            GenesisSpec {
                name: "地脉游火".to_string(),
                essence: "裂缝中跳跃的赤炎".to_string(),
                traits: vec!["高热".to_string(), "活跃".to_string()],
                state: "吞吐火星".to_string(),
                domain: "地底裂隙".to_string(),
            },
        ],
        steps: vec![
            HarnessStep::InhabitAndAct {
                entity_name_or_id: "青峭古石".to_string(),
                intent: "向地脉游火发出共鸣".to_string(),
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
    };

    let report = harness.run_scenario(scenario).await;
    assert!(report.is_ok());

    let r = report.unwrap();
    assert!(r.success);
    assert_eq!(r.assertions_failed, 0);
    assert_eq!(r.assertions_passed, 3);
    assert_eq!(r.steps_executed, 7);

    // 验证 Trace 链路追踪
    let tracer = &harness.world.tracer;
    assert_eq!(tracer.spans.len(), 5); // inhabit(1) + collide(1) + cosmic_law(1) + tick->2 entities evolve(2) = 5
    assert_eq!(tracer.total_llm_calls, 5);

    let json_trace = tracer.export_json();
    assert!(json_trace.is_ok());

    let jsonl_trace = tracer.export_jsonl();
    assert!(jsonl_trace.is_ok());
    assert_eq!(jsonl_trace.unwrap().lines().count(), 5);
}
