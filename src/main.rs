use primordia::PrimordiaWorld;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=================================================================");
    println!("  🌌 《原初》（Primordia: Meta）- Rust 核心引擎启动");
    println!("  LLM-Native · Universal Animacy · Autonomous Meta-World Engine");
    println!("=================================================================\n");

    let mut world = PrimordiaWorld::new("原初宇宙 0 号 / Universe-Zero");
    println!("📜 [初始宏观天道 / Initial Cosmic Atmosphere]:\n   {}\n", world.cosmic_atmosphere);

    // 1. 创世：向虚空中注入两个无预设规则的原初存在（附带关系场域与退隐内核）
    println!("--- [步骤 1 / Step 1: 创世凝结原初实体 / Entity Genesis] ---");
    let stone_id = world.add_entity_with_domain(
        "青峭古石 / Resonant Thunder-Stone",
        "伫立在悬崖千万年的青黑岩块，饱吸星光与晨露 / Ancient cliff boulder absorbing astral dew",
        vec!["致密 / Dense", "微凉 / Cool", "沉寂 / Silent"],
        "静静卧在云雾缭绕的绝壁边缘 / Resting quietly along the mist-shrouded precipice",
        "悬天绝壁 / Celestial Cliff Precipice",
    );

    let fire_id = world.add_entity_with_domain(
        "地脉游火 / Subterranean Flame",
        "从地底裂缝中窜出的一缕跳跃赤炎 / Dancing crimson flame leaping from deep rifts",
        vec!["高热 / Radiant Heat", "流动 / Fluid", "极度活跃 / Hyperactive"],
        "在虚空中飘忽不定地舞动，吞吐着火星 / Flickering in the void, releasing glowing embers",
        "地底裂隙 / Abyssal Rift Domain",
    );

    // 2. 玩家意识寄宿到石头中，并注入自由意志
    println!("\n--- [步骤 2 / Step 2: 玩家意识融入【青峭古石】并行动 / Mind Inhabitation] ---");
    world.inhabit_and_act(
        &stone_id,
        "我尝试将体内沉淀的星光向内核聚集，向地脉游火发出微弱的共鸣引力。 / I focus internal starlight into the core, exerting a resonant gravitational pull toward the flame.",
    ).await?;

    // 3. 碰撞化生与德勒兹共生装配体测试
    println!("\n--- [步骤 3 / Step 3: 两实体发生碰撞、相变与共生装配 / Morphogenesis & Symbiosis] ---");
    world.collide(&stone_id, &fire_id).await?;

    // 4. 宏观天道气象纪元相变推演
    println!("\n--- [步骤 4 / Step 4: 宏观天道法则与纪元气象演化 / Cosmic Law Phase Shift] ---");
    world.evolve_cosmic_law().await?;

    // 5. 世界推进一个自演化纪元
    println!("\n--- [步骤 5 / Step 5: 世界推进自生长周期 / Autonomous World Evolution Tick] ---");
    world.tick().await?;

    // 6. 导出世界状态快照
    println!("\n--- [步骤 6 / Step 6: 生成世界状态快照 (Snapshot Persistence) ] ---");
    let snapshot_json = world.export_snapshot_json()?;
    println!("  💾 快照字节大小 / Snapshot Size: {} bytes", snapshot_json.len());

    println!("\n=================================================================");
    println!("  ✅ 原初世界运行测试完成 / Simulation Tick Complete");
    println!("  📊 当前实体总数 / Total Entities: {}", world.entities.len());
    println!("  📜 世界编年史记录条数 / Chronicle Records: {}", world.chronicle.len());
    println!("  🌌 最新天道气象 / Current Cosmic Atmosphere: {}", world.cosmic_atmosphere);
    println!("=================================================================");

    Ok(())
}
