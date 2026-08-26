use async_trait::async_trait;
use primordia::{Entity, LlmClient, PrimordiaWorld};
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

    ent.record_memory("感受到了轻风拂过".to_string());
    assert_eq!(ent.memory_stream.len(), 2);
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
async fn test_world_epoch_tick() {
    let mut world = PrimordiaWorld::new("自演化测试 / Evolution Test");
    world.add_entity("古木 / Ancient Tree", "木灵 / Wood Animus", vec!["茂盛"], "舒展枝叶");

    let tick_res = world.tick().await;
    assert!(tick_res.is_ok());
    assert_eq!(world.tick_count, 1);
}
