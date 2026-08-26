# 《原初》系统架构与正交分层规范 (Architecture & Layering)

> **中英双语技术规范 / Bilingual Technical Specification**

本文档阐述《原初》（Primordia: Meta）纯 LLM 原生元世界基座的系统架构设计原则、正交分层规范与数据流转机制。  
This document specifies the architectural principles, orthogonal layering standards, and data flow mechanisms of the LLM-native *Primordia: Meta* foundation.

---

## 1. 架构哲学：体、法、意、象 四层正交模型 (The 4-Layer Orthogonal Model)

为了**彻底杜绝传统游戏引擎中类层次爆炸与硬编码的过度设计（Over-engineering），同时避免前期原型代码面条式的混乱堆积（Spaghetti Anti-pattern）**，《原初》依据东方“体/法/意/象”哲学与现代本体论，建立了一套严格单向流动的四层正交架构：

```
┌─────────────────────────────────────────────────────────────┐
│  第 3 层：感官表象与多模态层 (Phenomenal & Multimodal Layer)  │
│  [哲学：象 / 呈现] Three.js 3D 拓扑星象、中英双语神话史诗、SSE│
└──────────────────────────────▲──────────────────────────────┘
                               │ (只读投影 / Read & Project)
┌──────────────────────────────┴──────────────────────────────┐
│  第 2 层：自由觉知与感知视界 (Consciousness & Horizon Layer)  │
│  [哲学：意 / 觉知] Attention Kernel、SensoryHorizon 局部视界 │
└──────────────────────────────┬──────────────────────────────┘
                               │ (驱动与干预 / Drive & Collapse)
┌──────────────────────────────▼──────────────────────────────┐
│  第 1 层：语义因果与法则层 (Semantic Causality & Laws Layer) │
│  [哲学：法 / 演化] CausalOperator 策略矩阵、CausalExecutor  │
└──────────────────────────────┬──────────────────────────────┘
                               │ (原子状态提交 / Commit Transition)
┌──────────────────────────────▼──────────────────────────────┐
│  第 0 层：元本体图谱层 (Ontological Graph & Ledger Layer)   │
│  [哲学：体 / 基质] 统一 Entity 模型、成住坏空生命周期、编年史│
└─────────────────────────────────────────────────────────────┘
```

---

## 2. 各分层职责与边界约束 (Layer Responsibilities & Boundaries)

### 【第 0 层】元本体图谱层 (Ontological Graph & Ledger Layer) —— “体”（Being）
* **定位**：世界的纯粹数据基质（Substrate），提供客观存在与时序事实的存储。
* **核心组件**：
  - `Entity`：统一实体模型（含 `essence`、`withdrawn_core`、`spatial` 拓扑域与 `assemblages`、`lifecycle` 成住坏空与 `cohesion` 存在凝聚度）。
  - `Chronicle`：不可篡改的世界编年史时序事件流水与广播通道（`broadcast::Sender<ChronicleEvent>`）。
  - `Snapshot`：世界全量状态的无损 JSON 序列化与热加载。
* **⛔ 边界禁令**：**严禁在本层中调用 LLM、编写 Prompt 或硬编码业务物理规则**。它仅是纯粹的数据容器与局部状态变异算子。

### 【第 1 层】语义因果与法则层 (Semantic Causality & Laws Layer) —— “法”（Becoming）
* **定位**：世界的演化法则与相变中枢，大语言模型（LLM）在此充当物理法则与因果裁决核心。
* **核心设计模式**：**策略模式（Strategy Pattern）与流水线模式（Pipeline Pattern）**。
* **核心组件**：
  - `CausalOperator<Context, Output>`：因果算子策略特征。
    - `SelfEvolutionOperator`：实体对抗熵增的自生长、衰退、逆熵晶化或消解归墟。
    - `MorphogenesisOperator`：两实体相遇时的混沌碰撞、形态相融、天地化生或共生装配。
    - `MindInhabitationOperator`：自由觉知注入宿主后的意图物理化与因果波纹转译。
    - `AutonomousAgencyOperator`：实体基于局部感知视界自发萌发的心智意志行动。
    - `DomainResonanceOperator`：拓扑场域内多实体的集体共鸣与灵潮相变。
    - `CosmicLawOperator`：宇宙天道气象与法则常数的纪元跃迁。
  - `CausalExecutor`：标准因果流水线（Prompt 打包 ──► 多协议 LLM 调用 ──► 变异提交 ──► 编年史记录 ──► CausalSpan 链路追踪）。

### 【第 2 层】自由觉知与意图层 (Consciousness & Agency Layer) —— “意”（Mind）
* **定位**：脱嵌式注意力与自由意志中枢，践行泛心论与具身生成哲学。
* **核心组件**：
  - **觉知算子（Attention Kernel）**：玩家或自治意识脱离具体肉身，自由锚定并寄宿入世界任意实体。
  - **感知引擎（`PerceptionEngine`）与局部视界（`SensoryHorizon`）**：严格遵循 OOO 哲学，向实体装配其邻近客体的感官表象、共生装配体脉络、场域共鸣与编年史片段，绝不泄露客体退隐内核。

### 【第 3 层】感官表象与呈现层 (Phenomenal & Multimodal Layer) —— “象”（Phenomenon）
* **定位**：感官界面的多模态外显层。
* **核心组件**：
  - **Three.js 3D 渲染引擎（`CosmicRenderer`）**：在 WebGL 中以 3D 拓扑星象图渲染万物灵元，动态呈现生命周期辉光、凝聚度脉冲、德勒兹共生能量线与空间轨道。
  - **纪元神话史诗引擎（`MythosEngine`）**：实时从编年史中提炼中英双语纪元神话诗篇与宇宙基调。
  - **实时长连接（Server-Sent Events / SSE）**：基于 `GET /api/events/stream` 零延迟推送宇宙因果动态。
  - **原生命令行交互终端（`PrimordiaRepl`）**：支持富文本状态检视与调试。

---

## 3. 防范“过度设计”与“混乱堆积”的准则 (Engineering Balance Matrix)

| 潜在风险 | 传统误区 (What to Avoid) | 《原初》的正交克制原则 (Our Standard) |
| :--- | :--- | :--- |
| **过度设计 (Over-Engineering)** | 引入数十种 Component 的传统 ECS、抽象工厂层层嵌套、生硬的数值战斗公式与硬编码状态机。 | **万物皆 `Entity`**。无需 ECS 分解，自然语言描述即是一切特征与状态；所有物理逻辑全部收敛由 LLM 第一性原理实时推理。 |
| **混乱堆积 (Spaghetti Chaos)** | 将网络请求、Prompt 拼接、实体修改、终端打印全揉在一个数千行的 God Object 或循环中。 | **严格单向因果流与策略流水线**：觉知（意） ──► 策略裁决（法） ──► 原子状态提交（体） ──► 3D/史诗投影（象）。模块间边界清晰、高内聚低耦合。 |

---

## 4. 全生命周期因果链路追踪 (Causality Tracing & Harness)

为保证元世界因果关系的透明性与可重放性，系统配备了：
1. **`CausalityTracer`**：记录每次推演的 `CausalSpan`（操作算子、输入 Prompt、模型响应、受影响实体、状态变异明细与耗时）。
2. **`SimulationHarness`**：基于命令模式（Command Pattern）的自动化仿真剧本驾驭台，支持跨纪元演化、意图注入、碰撞化生与声明式因果断言测试。
