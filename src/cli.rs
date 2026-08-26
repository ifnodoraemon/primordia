use crate::world::PrimordiaWorld;
use std::io::{self, Write};

/// 原初交互式控制台分发器 (Interactive Primordia REPL)
pub struct PrimordiaRepl;

impl PrimordiaRepl {
    /// 运行交互式控制台循环 (Run Interactive Loop)
    pub async fn run(world: &mut PrimordiaWorld) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=================================================================");
        println!("  🌌 《原初》（Primordia: Meta）- 交互式觉知终端 (Interactive REPL)");
        println!("  输入 'help' 查看指令，输入 'exit' 退出");
        println!("=================================================================\n");

        let stdin = io::stdin();

        loop {
            print!("primordia [Epoch {}] ──► ", world.tick_count);
            io::stdout().flush()?;

            let mut input = String::new();
            if stdin.read_line(&mut input)? == 0 {
                break;
            }

            let trimmed = input.trim();
            if trimmed.is_empty() {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            let command = parts[0].to_lowercase();

            match command.as_str() {
                "exit" | "quit" | "q" => {
                    println!("🌌 觉知收回虚空，世界进入静默…… / Returning to the void.");
                    break;
                }
                "help" | "?" => {
                    Self::print_help();
                }
                "status" => {
                    println!("📊 宇宙状态 / Universe Status:");
                    println!("  - 纪元周期 / Epoch Tick: {}", world.tick_count);
                    println!("  - 实体总数 / Total Entities: {}", world.entities.len());
                    println!("  - 天道气象 / Cosmic Atmosphere: {}", world.cosmic_atmosphere);
                    println!("  - 编年史记录 / Chronicle Count: {}", world.chronicle.len());
                }
                "list" | "ls" => {
                    println!("📜 当前灵元实体列表 / Entities in Cosmos:");
                    for (id, ent) in &world.entities {
                        println!("  • [{}] {}", id, ent.sensory_manifestation());
                        if !ent.assemblages.is_empty() {
                            println!("    └── 🔗 共生装配 / Assemblages: {:?}", ent.assemblages);
                        }
                    }
                }
                "inspect" => {
                    if parts.len() < 2 {
                        println!("⚠️ 用法: inspect <entity_id 或 名称>");
                        continue;
                    }
                    let target_key = parts[1];
                    let matched = world.entities.iter().find(|(id, ent)| id.as_str() == target_key || ent.name.contains(target_key));
                    match matched {
                        Some((id, ent)) => {
                            println!("🔍 实体全景剖析 / Entity Inspection: [{}] {}", id, ent.name);
                            println!("  - 本质定义 / Essence: {}", ent.essence);
                            println!("  - 退隐内核 / Withdrawn Core (OOO): {}", ent.withdrawn_core);
                            println!("  - 拓扑场域 / Domain: {} (共鸣: {})", ent.spatial.domain, ent.spatial.resonance_field);
                            println!("  - 特征标签 / Traits: {:?}", ent.traits);
                            println!("  - 当前状态 / State: {}", ent.current_state);
                            println!("  - 记忆流 / Memory Stream ({}条):", ent.memory_stream.len());
                            for mem in &ent.memory_stream {
                                println!("    • {}", mem);
                            }
                        }
                        None => println!("❌ 未找到实体: {}", target_key),
                    }
                }
                "inhabit" => {
                    if parts.len() < 3 {
                        println!("⚠️ 用法: inhabit <entity_id 或 名称> <自由意志自然语言意图>");
                        continue;
                    }
                    let target_key = parts[1];
                    let intent = parts[2..].join(" ");
                    let target_id = world.entities.iter().find(|(id, ent)| id.as_str() == target_key || ent.name.contains(target_key)).map(|(id, _)| id.clone());
                    match target_id {
                        Some(id) => {
                            println!("🧠 意识正在锚定寄宿并注入意图……");
                            let _ = world.inhabit_and_act(&id, &intent).await;
                        }
                        None => println!("❌ 未找到实体: {}", target_key),
                    }
                }
                "act" => {
                    if parts.len() < 2 {
                        println!("⚠️ 用法: act <entity_id 或 名称>");
                        continue;
                    }
                    let target_key = parts[1];
                    let target_id = world.entities.iter().find(|(id, ent)| id.as_str() == target_key || ent.name.contains(target_key)).map(|(id, _)| id.clone());
                    match target_id {
                        Some(id) => {
                            println!("🌱 触发实体萌芽自主心智意志……");
                            let _ = world.act_autonomously(&id).await;
                        }
                        None => println!("❌ 未找到实体: {}", target_key),
                    }
                }
                "collide" => {
                    if parts.len() < 3 {
                        println!("⚠️ 用法: collide <entity_a> <entity_b>");
                        continue;
                    }
                    let key_a = parts[1];
                    let key_b = parts[2];
                    let id_a = world.entities.iter().find(|(id, ent)| id.as_str() == key_a || ent.name.contains(key_a)).map(|(id, _)| id.clone());
                    let id_b = world.entities.iter().find(|(id, ent)| id.as_str() == key_b || ent.name.contains(key_b)).map(|(id, _)| id.clone());
                    match (id_a, id_b) {
                        (Some(a), Some(b)) => {
                            println!("⚡ 激发两实体空间交汇相变……");
                            let _ = world.collide(&a, &b).await;
                        }
                        _ => println!("❌ 无法解析指定的两实体标识"),
                    }
                }
                "tick" => {
                    let count = if parts.len() > 1 {
                        parts[1].parse::<u64>().unwrap_or(1)
                    } else {
                        1
                    };
                    for _ in 0..count {
                        world.tick().await?;
                    }
                }
                "shift" => {
                    println!("🌌 正在推演宏观天道气象迁跃……");
                    let _ = world.evolve_cosmic_law().await;
                }
                "resonate" => {
                    if parts.len() < 2 {
                        println!("⚠️ 用法: resonate <拓扑场域名称>");
                        continue;
                    }
                    let domain_name = parts[1..].join(" ");
                    println!("🌊 正在激发场域【{}】的集体共鸣与灵潮相变……", domain_name);
                    let _ = world.trigger_domain_resonance(&domain_name).await;
                }
                "epic" | "mythos" => {
                    println!("📜 正在提炼宇宙纪元神话篇章……");
                    match world.distill_mythos().await {
                        Ok(chapter) => {
                            println!("\n╔══════════════════════════════════════════════════════════════════╗");
                            println!("║ 📜 《原初》纪元神话篇章 / Epoch Mythos Chapter                   ║");
                            println!("║ 篇目 / Title: {:<50} ║", chapter.title);
                            println!("║ 宇宙基调 / Tone: {:<47} ║", chapter.world_tone);
                            println!("╚══════════════════════════════════════════════════════════════════╝");
                            println!("📖 史诗吟诵 / Poetic Epic:\n{}\n", chapter.poetic_epic);
                        }
                        Err(e) => println!("❌ 提炼神话篇章失败: {}", e),
                    }
                }
                "trace" => {
                    println!("🔭 因果链路追踪摘要 / Trace Summary: {}", world.tracer.summary());
                    println!("📜 最近 5 个 CausalSpan 明细:");
                    for span in world.tracer.spans.iter().rev().take(5) {
                        println!("  • [{}] <{}> Targets: {:?} ({}ms) ──► {}", span.span_id, span.operator, span.target_entities, span.duration_ms, span.mutations_summary);
                    }
                }
                "save" => {
                    let path = if parts.len() > 1 { parts[1] } else { "world_snapshot.json" };
                    match world.save_snapshot(path) {
                        Ok(_) => println!("💾 世界快照已成功保存至: {}", path),
                        Err(e) => println!("❌ 保存快照失败: {}", e),
                    }
                }
                "load" => {
                    if parts.len() < 2 {
                        println!("⚠️ 用法: load <snapshot_path>");
                        continue;
                    }
                    let path = parts[1];
                    match std::fs::read_to_string(path) {
                        Ok(json_str) => {
                            match serde_json::from_str::<crate::world::WorldSnapshot>(&json_str) {
                                Ok(snapshot) => {
                                    world.name = snapshot.name;
                                    world.tick_count = snapshot.tick_count;
                                    world.cosmic_atmosphere = snapshot.cosmic_atmosphere;
                                    world.entities = snapshot.entities;
                                    world.chronicle = snapshot.chronicle;
                                    world.tracer = snapshot.tracer;
                                    println!("💾 成功从快照恢复世界状态: {}", path);
                                }
                                Err(e) => println!("❌ 快照数据反序列化失败: {}", e),
                            }
                        }
                        Err(e) => println!("❌ 读取快照文件失败: {}", e),
                    }
                }
                _ => {
                    println!("❓ 未知指令: '{}'。输入 'help' 查看所有可用指令。", command);
                }
            }
        }

        Ok(())
    }

    fn print_help() {
        println!("📖 《原初》交互指令列表 / Command Reference:");
        println!("  • status                 - 查看宇宙当前纪元、实体数、天道气象与编年史总数");
        println!("  • list / ls              - 列出宇宙中所有实体的感官表象与共生装配关系");
        println!("  • inspect <id/name>      - 深入剖析实体的退隐内核、本质、记忆流与完整状态");
        println!("  • inhabit <id> <intent>  - 自由意识寄宿入实体并注入自然语言意图");
        println!("  • act <id/name>          - 触发实体基于局部感知视界萌发自主心智意志并行动");
        println!("  • collide <id_a> <id_b>  - 触发两实体碰撞相变（互相改变/化生新生命/共生装配）");
        println!("  • tick [n]               - 推进 n 个世界演化纪元周期 (默认 1)");
        println!("  • shift                  - 触发宏观天道气象与宇宙常数演化相变");
        println!("  • trace                  - 查看因果追踪摘要与最近 Span 明细");
        println!("  • save [path]            - 保存宇宙全量状态快照至 JSON 文件");
        println!("  • exit / quit            - 退出交互式终端");
    }
}
