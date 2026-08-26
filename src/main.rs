use primordia::{
    create_llm_client_from_env, GenesisSpec, HarnessStep, Scenario, SimulationHarness,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=================================================================");
    println!("  🌌 《原初》（Primordia: Meta）- Rust 核心引擎启动");
    println!("  LLM-Native · Universal Animacy · Multi-Provider Harness");
    println!("=================================================================\n");

    let llm = create_llm_client_from_env();
    let mut harness = SimulationHarness::new("原初宇宙 0 号 / Universe-Zero", llm);

    // 定义基准仿真剧本 (Benchmark Scenario)
    let benchmark_scenario = Scenario {
        name: "原初相变与天道相变基准剧本 / Primordia Genesis Benchmark".to_string(),
        description: "驱动创世、意识寄宿、碰撞共生、宏观天道跃迁及纪元演化 / Drive genesis, inhabitation, symbiosis, cosmic shift and autonomous evolution".to_string(),
        genesis_entities: vec![
            GenesisSpec {
                name: "青峭古石 / Resonant Thunder-Stone".to_string(),
                essence: "伫立在悬崖千万年的青黑岩块，饱吸星光与晨露 / Ancient cliff boulder absorbing astral dew".to_string(),
                traits: vec!["致密 / Dense".to_string(), "微凉 / Cool".to_string(), "沉寂 / Silent".to_string()],
                state: "静静卧在云雾缭绕的绝壁边缘 / Resting quietly along the mist-shrouded precipice".to_string(),
                domain: "悬天绝壁 / Celestial Cliff Precipice".to_string(),
            },
            GenesisSpec {
                name: "地脉游火 / Subterranean Flame".to_string(),
                essence: "从地底裂缝中窜出的一缕跳跃赤炎 / Dancing crimson flame leaping from deep rifts".to_string(),
                traits: vec!["高热 / Radiant Heat".to_string(), "流动 / Fluid".to_string(), "极度活跃 / Hyperactive".to_string()],
                state: "在虚空中飘忽不定地舞动，吞吐着火星 / Flickering in the void, releasing glowing embers".to_string(),
                domain: "地底裂隙 / Abyssal Rift Domain".to_string(),
            },
        ],
        steps: vec![
            HarnessStep::InhabitAndAct {
                entity_name_or_id: "青峭古石".to_string(),
                intent: "我尝试将体内沉淀的星光向内核聚集，向地脉游火发出微弱的共鸣引力。 / I focus internal starlight into the core, exerting a resonant gravitational pull toward the flame.".to_string(),
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
            HarnessStep::AssertChronicleContains {
                event_type: "SELF_EVOLVE".to_string(),
            },
        ],
    };

    // 使用 Harness 驱动剧本执行
    let report = harness.run_scenario(benchmark_scenario).await?;

    // 导出并打印链路追踪与快照
    let snapshot_json = harness.world.export_snapshot_json()?;
    println!("\n💾 [宇宙状态快照 / World Snapshot]: {} bytes", snapshot_json.len());

    println!("\n🔍 [因果链路追踪明细 / Causality Trace Spans]:");
    for span in &harness.world.tracer.spans {
        println!(
            "  • [{}] <{}> Entities: {:?} ({}ms) ──► {}",
            span.span_id, span.operator, span.target_entities, span.duration_ms, span.mutations_summary
        );
    }

    println!("\n=================================================================");
    println!("  ✅ 仿真剧本执行完成 / Harness Simulation Run Complete");
    println!("  📊 结果状态 / Status: {}", if report.success { "SUCCESS" } else { "FAILED" });
    println!("  📜 编年史总条数 / Chronicle Count: {}", harness.world.chronicle.len());
    println!("  🔭 追踪报告 / Trace Summary: {}", report.trace_summary);
    println!("=================================================================");

    Ok(())
}
