# 《原初》数据模型与实体图谱规范 (Data Schema)

> **中英双语数据规范 / Bilingual Data Specification**

在《原初》（Primordia: Meta）中，数据结构追求**极简、万物统一与纯语义化**。融入面向客体本体论（OOO）的退隐内核与德勒兹装配共生结构。  
In *Primordia: Meta*, data structures are designed to be **minimalist, ontologically unified, and purely semantic**, integrating OOO Withdrawn Real Cores and Deleuzian Assemblages.

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
    "born_at_tick": {
      "type": "integer",
      "description": "诞生所在纪元周期 / Epoch Tick of Genesis"
    }
  },
  "required": ["id", "name", "essence", "withdrawn_core", "spatial", "traits", "current_state", "memory_stream", "assemblages", "born_at_tick"]
}
```

---

## 2. 实体生动示例 (Concrete Examples)

### 示例 A：原初矿石实体 / Primordial Stone Entity
```json
{
  "id": "ent_001",
  "name": "鸣雷青岩 / Resonant Thunder-Stone",
  "essence": "坐落在山巅、被多次天雷击中的青黑岩块 / Obsidian cliff boulder struck repeatedly by astral lightning",
  "withdrawn_core": "蕴含未曾显露的深渊雷火寂灭暗核 / Inscrutable dormant core resisting total exhaustion",
  "spatial": {
    "x": 12.5, "y": 45.0, "z": 8.0, "scale": 3.0, "mobility": "static",
    "domain": "悬天绝壁 / Celestial Cliff Precipice",
    "resonance_field": "高频微电流脉动 / High-Frequency Galvanic Hum"
  },
  "traits": ["导电结晶 / Conductive Crystal", "表皮焦黑 / Charred Surface", "低频震颤 / Low-Frequency Hum"],
  "current_state": "静置在峭壁，内部电荷正在雨水中缓慢流转 / Resting on the precipice; internal charge circulating through raindrops",
  "memory_stream": [
    "在第 0 纪元由天地雷击与巨石凝结诞生 / Formed at Epoch 0 through astral lightning impact",
    "与【ent_002】缔结为共生装配体：彼此交换感知与光晕"
  ],
  "assemblages": ["ent_002"],
  "born_at_tick": 0
}
```

---

## 3. 世界快照与编年史规范 (World Snapshot & Chronicle)

```json
{
  "name": "原初宇宙 0 号 / Universe-Zero",
  "tick_count": 34,
  "cosmic_atmosphere": "太初辉光纪元：星辰灵尘与地热熵流谐振 / Era of Primordial Radiance",
  "entities": { "...": { "..." : "..." } },
  "chronicle": [
    {
      "tick": 34,
      "event_type": "COSMIC_LAW_SHIFT",
      "detail": "宏观天道气象发生纪元相变：太初辉光纪元 / Cosmic law phase shift: Era of Primordial Radiance",
      "timestamp": 1771995600
    }
  ]
}
```
