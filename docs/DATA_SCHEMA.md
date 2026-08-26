# 《原初》数据模型与实体图谱规范 (Data Schema)

> **中英双语数据规范 / Bilingual Data Specification**

在《原初》（Primordia: Meta）中，数据结构追求**极简、万物统一与纯语义化**。  
In *Primordia: Meta*, data structures are designed to be **minimalist, ontologically unified, and purely semantic**.

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
      "description": "本质定义 / Intrinsic Nature (e.g., '吸纳地脉晨露的青色顽石 / Primordial stone absorbing astral dew')"
    },
    "spatial": {
      "type": "object",
      "properties": {
        "x": { "type": "number" },
        "y": { "type": "number" },
        "z": { "type": "number" },
        "scale": { "type": "number", "description": "空间尺度 / Scale" },
        "mobility": { "type": "string", "enum": ["static", "floating", "dynamic", "ethereal"] }
      },
      "required": ["x", "y", "z", "scale", "mobility"]
    },
    "traits": {
      "type": "array",
      "items": { "type": "string" },
      "description": "自生特征标签 / Emergent Traits (e.g., ['致密/Dense', '微光共振/Luminous Resonance'])"
    },
    "current_state": {
      "type": "string",
      "description": "当前物理与心智状态 / Current Physical & Mental State Description"
    },
    "memory_stream": {
      "type": "array",
      "items": { "type": "string" },
      "description": "时序历史记忆 / Temporal Memory Stream"
    },
    "born_at_tick": {
      "type": "integer",
      "description": "诞生所在纪元周期 / Epoch Tick of Genesis"
    }
  },
  "required": ["id", "name", "essence", "spatial", "traits", "current_state", "memory_stream", "born_at_tick"]
}
```

---

## 2. 实体生动示例 (Concrete Examples)

### 示例 A：原初矿石实体（低频代谢）/ Primordial Stone Entity (Low-Frequency Metabolism)
```json
{
  "id": "ent_001",
  "name": "鸣雷青岩 / Resonant Thunder-Stone",
  "essence": "坐落在山巅、被多次天雷击中的青黑岩块 / Obsidian cliff boulder struck repeatedly by astral lightning",
  "spatial": { "x": 12.5, "y": 45.0, "z": 8.0, "scale": 3.0, "mobility": "static" },
  "traits": ["导电结晶 / Conductive Crystal", "表皮焦黑 / Charred Surface", "低频震颤 / Low-Frequency Hum"],
  "current_state": "静置在峭壁，内部电荷正在雨水中缓慢流转 / Resting on the precipice; internal charge circulating through raindrops",
  "memory_stream": [
    "在第 0 纪元由天地雷击与巨石凝结诞生 / Formed at Epoch 0 through astral lightning impact",
    "曾有一只光蝶在其表面停歇，留下荧光粉末 / A light-moth rested on its surface, leaving luminescent dust"
  ],
  "born_at_tick": 0
}
```

### 示例 B：化生出的原初生灵（高频动态）/ Spontaneously Morphed Being (High-Frequency Dynamic)
```json
{
  "id": "ent_002",
  "name": "雷露灵息 / Thunder-Dew Wisp",
  "essence": "由鸣雷青岩的电涌与晨露水汽交融化生出的游荡光团 / Wandering luminescent wisp born of thunder surges and morning vapor",
  "spatial": { "x": 13.0, "y": 46.2, "z": 9.5, "scale": 0.4, "mobility": "floating" },
  "traits": ["半透明流体 / Semi-Transparent Fluid", "带电跳跃 / Electrified Arc", "好奇心强 / Curious Instinct"],
  "current_state": "正好奇地绕着母体岩石盘旋，发出轻微哔拨声 / Circling the mother rock curiously with faint crackling sounds",
  "memory_stream": [
    "于第 34 纪元由鸣雷青岩与暴雨水汽化生 / Born at Epoch 34 from the fusion of thunder rock and rain",
    "学会了借着山风滑翔 / Learned to glide along mountain currents"
  ],
  "born_at_tick": 34
}
```

---

## 3. 世界编年史结构 (World Chronicle Schema)

```json
{
  "tick": 34,
  "event_type": "COLLISION_MORPHOGENESIS",
  "detail": "天地化生：鸣雷青岩在晨雨中孕育出原初光团【雷露灵息】。 / Genesis: Thunder-Stone and rain vapor give rise to Thunder-Dew Wisp.",
  "timestamp": 1771995600
}
```
