# 《原初》系统架构与运行机制 (Architecture & Mechanics)

> **中英双语技术规范 / Bilingual Technical Specification**

本文档阐述《原初》（Primordia: Meta）纯 LLM 原生世界基座的技术分层、数据流动与核心调度机制。  
This document specifies the technical layering, data flow, and core scheduling mechanisms of the LLM-native *Primordia: Meta* foundation.

---

## 1. 总体架构分层 (Layered Architecture)

```
┌─────────────────────────────────────────────────────────────┐
│                 1. 交互与觉知层 (Interaction Layer)            │
│   • 自由视角 / 宏观天道视角 (Cosmic God View)                  │
│   • 实体寄宿模式 (Mind Inhabitation: Mountain/Flora/Beast)      │
│   • 自然语言意图输入器 (Natural Language Prompt Input)         │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│          2. LLM 原生世界调度中枢 (LLM Genesis Core)          │
│   ┌─────────────────────────────────────────────────────┐   │
│   │ A. 自生长心跳器 (Self-Evolution Ticker: evolve_entity)│   │
│   │    - 异步批量驱动实体自变异与进化 (Autonomous mutations)│   │
│   ├─────────────────────────────────────────────────────┤   │
│   │ B. 碰撞与化生引擎 (Morphogenesis Engine: collide)   │   │
│   │    - 实体交互、物理反应与生命化生 (Spontaneous genesis)│   │
│   ├─────────────────────────────────────────────────────┤   │
│   │ C. 玩家觉知转译器 (Mind Grounding: inhabit_and_act)  │   │
│   │    - 将玩家意图转译为实体行为与连锁因果 (Action ground)│   │
│   └─────────────────────────────────────────────────────┘   │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│            3. 原初实体图谱与时序库 (State & Ledger)           │
│   • 实体状态集 (JSON Entity Graph / HashMap<String, Entity>)│
│   • 空间索引 (Spatial KD-Tree / Multi-Resolution Grid)      │
│   • 世界编年史 (World Chronicle / Event Ledger)             │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│            4. 呈现与多模态生成层 (Generative Presentation)    │
│   • 实体自然语言状态与故事日志 (Story Logs & Semantic Inspect)│
│   • 动态 3D 体素 / 粒子表现映射 (Generative 3D / Particles)   │
│   • 动态氛围音效生成 (Procedural Generative Audio / Music)   │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. 三大核心运转流程 (The 3 Core Workflows)

### 流程一：自生长心跳（Self-Evolution Loop）
* **[中]**
  1. **采样**：世界调度器按时间步长，选择活跃范围内的实体。
  2. **构建上下文**：打包该实体的本质、属性、记忆及周边环境信息。
  3. **LLM 演化推演**：调用 `Prompts.EVOLVE_ENTITY`，LLM 推导其自发变异与萌芽。
  4. **状态写回**：更新实体状态，若产生分裂/孕育则向图谱注入新实体。
* **[EN]**
  1. **Sample**: The world scheduler selects active entities per tick.
  2. **Context Assembly**: Packs the entity's essence, traits, memory, and surroundings.
  3. **LLM Evolution**: Invokes `Prompts.EVOLVE_ENTITY` to deduce spontaneous mutations.
  4. **State Commit**: Updates entity state and instantiates offspring into the graph if sprouted.

### 流程二：碰撞与化生（Collision & Morphogenesis）
* **[中]**
  1. **邻近检测**：当两个实体在空间中靠近或发生接触。
  2. **构建相互作用 Context**：打包双方本质属性与动量。
  3. **LLM 裁决推演**：调用 `Prompts.RESOLVE_INTERACTION`，裁决相互影响、同化、或化生出第三种全新存在。
  4. **编年史记录**：将化生事件永久记录入世界编年史。
* **[EN]**
  1. **Proximity Detection**: Triggered when two entities intersect in spatial coordinates.
  2. **Interaction Context**: Bundles both entities' intrinsic essences and momenta.
  3. **LLM Adjudication**: Invokes `Prompts.RESOLVE_INTERACTION` to determine mutual change, assimilation, or spontaneous genesis of a new entity.
  4. **Chronicle Inscription**: Inscribes the morphogenesis event into the permanent ledger.

### 流程三：玩家觉知融入（Mind Inhabitation）
* **[中]**
  1. **选定寄宿体**：玩家将意念锚定在世界中的任意实体。
  2. **意图注入**：玩家输入自然语言自由意志指令。
  3. **LLM 因果推导**：以该实体为主体，推导其动作执行、自身负荷与周围环境的波纹。
  4. **波纹反馈**：更新被寄宿实体状态，并在记忆中铭刻被“宏大意志”驱使的痕迹。
* **[EN]**
  1. **Anchor**: Player anchors consciousness onto any target entity.
  2. **Intent Injection**: Player provides natural language free-will instructions.
  3. **LLM Grounding**: Reasons through physical/spiritual feasibility, subject feedback, and environmental ripples.
  4. **Ripple Feedback**: Updates the host entity and inscribes the memory of divine possession.
