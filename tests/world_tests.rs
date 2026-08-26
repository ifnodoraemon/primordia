use async_trait::async_trait;
use primordia::{Entity, LlmClient, PerceptionEngine, PrimordiaWorld};
use serde_json::Value;
use std::sync::Arc;

struct MockLlmClient {
    return_val: Value,
}

#[async_trait]
impl LlmClient for MockLlmClient {
    async fn generate_json(&self, _system_prompt: &str, _user_prompt: &str) -> Result<Value, String> {
        Ok(self.return_val.clone())
    }
}

#[tokio::test]
async fn test_entity_creation_and_memory() {
    let mut ent = Entity::new(
        "ent_001".to_string(),
        "星尘石 / Stardust Stone".to_string(),
        "蕴含原初星光的晶石 / Crystal containing starlight".to_string(),
        vec!["发光 / Glowing".to_string()],
        "静止 / Idle".to_string(),
        0,
    );

    assert_eq!(ent.id, "ent_001");
    assert_eq!(ent.name, "星尘石 / Stardust Stone");
    assert_eq!(ent.traits.len(), 1);
    assert_eq!(ent.memory_stream.len(), 1);
    assert!(ent.withdrawn_core.contains("星尘石"));

    ent.record_memory("感受到了轻风拂过".to_string());
    assert_eq!(ent.memory_stream.len(), 2);

    let sensory = ent.sensory_manifestation();
    assert!(sensory.contains("星尘石"));
}

#[tokio::test]
async fn test_world_genesis_and_inhabitation() {
    let mock_resp = serde_json::json!({
        "action_result": "共鸣发生，引力扩散",
        "subject_new_state": "星光激荡",
        "environmental_ripple": "周边微风旋转"
    });

    let mock_llm = Arc::new(MockLlmClient { return_val: mock_resp });
    let mut world = PrimordiaWorld::with_llm("测试世界 / Test World", mock_llm);

    let id = world.add_entity("青石 / Stone", "顽石 / Stone", vec!["坚固 / Hard"], "静止 / Static");
    assert_eq!(world.entities.len(), 1);
    assert_eq!(world.chronicle.len(), 1);

    let res = world.inhabit_and_act(&id, "发出共鸣").await;
    assert!(res.is_ok());

    let target = world.entities.get(&id).unwrap();
    assert_eq!(target.current_state, "星光激荡");
    assert_eq!(target.memory_stream.len(), 2);
}

#[tokio::test]
async fn test_perception_engine_and_autonomous_agency() {
    let mock_resp = serde_json::json!({
        "autonomous_intent": "向共生伙伴传递地热能量",
        "action_execution": "内核激荡出赤红脉冲",
        "updated_state": "表层泛起高温纹路",
        "environmental_ripple": "方圆三丈空气升温"
    });

    let mock_llm = Arc::new(MockLlmClient { return_val: mock_resp });
    let mut world = PrimordiaWorld::with_llm("心智与感知世界", mock_llm);

    let id_a = world.add_entity_with_domain("地脉游火", "跳跃赤炎", vec!["高热"], "活跃", "地火深渊");
    let id_b = world.add_entity_with_domain("熔岩晶石", "吸收地热的黑曜晶体", vec!["致密"], "沉寂", "地火深渊");
    let _ = world.form_assemblage(&id_a, &id_b, "熔岩与地火缔结共生");

    // 验证感知视界提取
    let horizon = PerceptionEngine::extract_horizon(&world, &id_a);
    assert!(horizon.is_ok());
    let h = horizon.unwrap();
    assert_eq!(h.domain_name, "地火深渊");
    assert_eq!(h.symbiont_manifestations.len(), 1);

    // 验证实体自主心智行为
    let act_res = world.act_autonomously(&id_a).await;
    assert!(act_res.is_ok());

    let target = world.entities.get(&id_a).unwrap();
    assert_eq!(target.current_state, "表层泛起高温纹路");
    assert!(target.memory_stream.last().unwrap().contains("萌发自主心智意志"));
}

#[tokio::test]
async fn test_world_morphogenesis_and_sprouting() {
    let mock_resp = serde_json::json!({
        "outcome_type": "MORPHOGENESIS_NEW",
        "narrative": "两股原初灵息碰撞，诞化出灵露。",
        "update_a": "能量释放",
        "update_b": "形态微变",
        "born_entity": {
            "name": "星露灵 / Star Dew Wisp",
            "essence": "灵息汇聚之体",
            "traits": ["晶莹", "轻盈"],
            "current_state": "初生"
        }
    });

    let mock_llm = Arc::new(MockLlmClient { return_val: mock_resp });
    let mut world = PrimordiaWorld::with_llm("化生世界 / Morphogenesis World", mock_llm);

    let id_a = world.add_entity("岩石 / Rock", "玄武岩 / Basalt", vec!["坚硬"], "沉寂");
    let id_b = world.add_entity("泉水 / Spring", "清泉 / Spring", vec!["清澈"], "流淌");

    let res = world.collide(&id_a, &id_b).await;
    assert!(res.is_ok());

    assert_eq!(world.entities.len(), 3);
    assert!(world.entities.values().any(|e| e.name == "星露灵 / Star Dew Wisp"));
}

#[tokio::test]
async fn test_world_assemblage_and_cosmic_law() {
    let mock_resp = serde_json::json!({
        "new_atmosphere": "太初晶化纪元：虚空中析出微细晶体尘埃",
        "cosmic_ripple": "引力常数发生微弱振荡"
    });

    let mock_llm = Arc::new(MockLlmClient { return_val: mock_resp });
    let mut world = PrimordiaWorld::with_llm("装配与天道世界 / Assemblage World", mock_llm.clone());

    let id_a = world.add_entity("风 / Wind", "气旋 / Vortex", vec!["疾驰"], "流动");
    let id_b = world.add_entity("云 / Cloud", "雾气 / Mist", vec!["漂浮"], "聚拢");

    let form_res = world.form_assemblage(&id_a, &id_b, "风云交汇，化为风云装配体");
    assert!(form_res.is_ok());

    let ent_a = world.entities.get(&id_a).unwrap();
    assert!(ent_a.assemblages.contains(&id_b.to_string()));

    let law_res = world.evolve_cosmic_law().await;
    assert!(law_res.is_ok());
    assert_eq!(world.cosmic_atmosphere, "太初晶化纪元：虚空中析出微细晶体尘埃");

    // Snapshot Roundtrip Test
    let snapshot_json = world.export_snapshot_json().unwrap();
    let restored_world = PrimordiaWorld::import_snapshot_json(&snapshot_json, mock_llm).unwrap();
    assert_eq!(restored_world.entities.len(), 2);
    assert_eq!(restored_world.cosmic_atmosphere, world.cosmic_atmosphere);
}

#[tokio::test]
async fn test_world_epoch_tick() {
    let mut world = PrimordiaWorld::new("自演化测试 / Evolution Test");
    world.add_entity("古木 / Ancient Tree", "木灵 / Wood Animus", vec!["茂盛"], "舒展枝叶");

    let tick_res = world.tick().await;
    assert!(tick_res.is_ok());
    assert_eq!(world.tick_count, 1);
}

#[tokio::test]
async fn test_entity_lifecycle_and_dissolution() {
    let mock_resp = serde_json::json!({
        "updated_state": "形体在漫长岁月风化中崩解为尘埃",
        "new_traits": ["风化粉尘"],
        "new_memory": "形体消散，灵蕴回归大地",
        "lifecycle_phase": "Dissolution",
        "cohesion_change": -0.95,
        "domain_nourishment": "散落为十里沃土与灵气微尘"
    });

    let mock_llm = Arc::new(MockLlmClient { return_val: mock_resp });
    let mut world = PrimordiaWorld::with_llm("归墟测试世界", mock_llm);

    let id = world.add_entity_with_domain("暮年古岩", "风化中的巨石", vec!["残破"], "布满裂痕", "无尽荒原");
    let res = world.evolve_entity(&id).await;
    assert!(res.is_ok());

    let target = world.entities.get(&id).unwrap();
    assert_eq!(target.lifecycle, primordia::LifecyclePhase::Dissolution);
    assert!((target.cohesion - 0.05).abs() < 1e-5);
    assert!(world.chronicle.iter().any(|e| e.event_type == "ENTITY_DISSOLUTION"));
}
