# 《原初》数据模型与实体图谱规范 (Data Schema)

> **中英双语数据规范 / Bilingual Data Specification**

在《原初》（Primordia: Meta）中，数据结构追求**极简、万物统一与纯语义化**。融入面向客体本体论（OOO）的退隐内核、成住坏空生命周期、德勒兹装配共生以及自由觉知算子寄宿列表。  
In *Primordia: Meta*, data structures are designed to be **minimalist, ontologically unified, and purely semantic**, integrating OOO Withdrawn Real Cores, Autopoietic Lifecycle Phases, Deleuzian Assemblages, and Consciousness Inhabitation Registries.

---

## 1. 灵元实体规范 (Entity Node Schema)

每一个存在（山川、草木、风雨、生灵）均遵从统一的 JSON / Rust 结构：  
Every existence adheres to a unified data structure:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "PrimordiaEntity",
  "type": "object",
  "properties": {
    "id": {
      "type": "string",
      "description": "唯一实体标识符 / Unique Entity ID (e.g., 'ent_001')"
    },
    "name": {
      "type": "string",
      "description": "自生名称 / Emergent Name"
    },
    "essence": {
      "type": "string",
      "description": "本质定义 / Intrinsic Nature"
    },
    "withdrawn_core": {
      "type": "string",
      "description": "OOO退隐内核（不可穷尽的深渊内在实在） / Withdrawn Real Core"
    },
    "spatial": {
      "type": "object",
      "properties": {
        "x": { "type": "number" },
        "y": { "type": "number" },
        "z": { "type": "number" },
        "scale": { "type": "number", "description": "空间尺度 / Scale" },
        "mobility": { "type": "string", "enum": ["static", "floating", "dynamic", "ethereal"] },
        "domain": { "type": "string", "description": "所属拓扑场域 / Relational Domain" },
        "resonance_field": { "type": "string", "description": "场域共鸣印记 / Resonance Field" }
      },
      "required": ["x", "y", "z", "scale", "mobility", "domain", "resonance_field"]
    },
    "traits": {
      "type": "array",
      "items": { "type": "string" },
      "description": "自生特征标签 / Emergent Traits"
    },
    "current_state": {
      "type": "string",
      "description": "当前物理与心智状态 / Current State Description"
    },
    "lifecycle": {
      "type": "string",
      "enum": ["Genesis", "Flourishing", "Decay", "Dissolution"],
      "description": "自创生生命周期阶段（成·住·坏·空） / Autopoietic Lifecycle Phase"
    },
    "cohesion": {
      "type": "number",
      "minimum": 0.0,
      "maximum": 1.0,
      "description": "存在结构凝聚度 / Ontological Cohesion"
    },
    "memory_stream": {
      "type": "array",
      "items": { "type": "string" },
      "description": "时序历史记忆 / Temporal Memory Stream"
    },
    "assemblages": {
      "type": "array",
      "items": { "type": "string" },
      "description": "德勒兹共生装配关联列表 / Linked Symbiotic Assemblage IDs"
    },
    "active_inhabitants": {
      "type": "array",
      "items": { "type": "string" },
      "description": "当前寄宿该实体的自由意志/玩家算子列表 / Active Inhabitants / Observers"
    },
    "born_at_tick": {
      "type": "integer",
      "description": "诞生所在纪元周期 / Epoch Tick of Genesis"
    }
  },
  "required": ["id", "name", "essence", "withdrawn_core", "spatial", "traits", "current_state", "lifecycle", "cohesion", "memory_stream", "assemblages", "active_inhabitants", "born_at_tick"]
}
```

---

## 2. 实体生动示例 (Concrete Examples)

### 示例 A：原初矿石实体 / Primordial Stone Entity
```json
{
  "id": "ent_001",
  "name": "青峭古石 / Resonant Thunder-Stone",
  "essence": "伫立在悬崖千万年的青黑岩块，饱吸星光与晨露 / Ancient cliff boulder absorbing astral dew",
  "withdrawn_core": "蕴含未曾显露的深渊雷火寂灭暗核 / Inscrutable dormant core resisting total exhaustion",
  "spatial": {
    "x": 12.5, "y": 45.0, "z": 8.0, "scale": 3.0, "mobility": "static",
    "domain": "悬天绝壁 / Celestial Cliff Precipice",
    "resonance_field": "高频微电流脉动 / High-Frequency Galvanic Hum"
  },
  "traits": ["致密 / Dense", "微凉 / Cool", "沉寂 / Silent", "星光吸附 / Astral Absorption"],
  "current_state": "静静卧在云雾缭绕的绝壁边缘，正在雨水中缓慢流转电荷 / Resting on the precipice; internal charge circulating through mist",
  "lifecycle": "Flourishing",
  "cohesion": 0.98,
  "memory_stream": [
    "在第 0 纪元由天地星尘与巨石凝结诞生 / Formed at Epoch 0 through astral dust condensation",
    "曾被宏大意志降临注入共鸣引力 / Inhabited by conscious mind to exert resonant gravity",
    "与【ent_002】缔结为共生装配体：彼此交换感知与光晕"
  ],
  "assemblages": ["ent_002"],
  "active_inhabitants": ["Player_Conscious_Alpha"],
  "born_at_tick": 0
}
```

---

## 3. 世界全景快照规范 (World Snapshot Schema)

```json
{
  "name": "原初宇宙 0 号 / Universe-Zero",
  "tick_count": 42,
  "cosmic_atmosphere": "太初流变纪元：星辰灵尘与地热熵流谐振 / Era of Primordial Becoming: Astral stardust resonates with thermal entropy",
  "entities": {
    "ent_001": { /* Entity Schema */ }
  },
  "chronicle": [
    {
      "tick": 42,
      "event_type": "DOMAIN_RESONANCE",
      "detail": "场域【悬天绝壁】爆发集体共鸣激荡……",
      "timestamp": 1724659200
    }
  ],
  "tracer": {
    "spans": [
      {
        "span_id": "span_0001",
        "tick": 42,
        "operator": "DOMAIN_RESONANCE",
        "target_entities": ["@DOMAIN:悬天绝壁"],
        "mutations_summary": "场域共鸣场更新为: 万仞雷光场",
        "duration_ms": 42
      }
    ]
  }
}
```
