# 《原初》系统架构与正交分层规范 (Architecture & Layering)

> **中英双语技术规范 / Bilingual Technical Specification**

本文档阐述《原初》（Primordia: Meta）纯 LLM 原生元世界基座的系统架构设计原则、正交分层规范与“万物归一”因果内核机制。  
This document specifies the architectural principles, orthogonal layering standards, and the Singularity Universal Causal Kernel of the LLM-native *Primordia: Meta* foundation.

---

## 1. 架构哲学：体、法、意、象 四层正交模型 (The 4-Layer Orthogonal Model)

为了**彻底杜绝传统引擎中分类学爆炸与硬编码的过度设计（Over-engineering），同时避免前期原型代码面条式的混乱堆积（Spaghetti Anti-pattern）**，《原初》依据东方“体/法/意/象”哲学与现代本体论，建立了一套严格单向流动的四层正交架构：

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
                               │ (驱动与干预 / Drive & Intervene)
┌──────────────────────────────▼──────────────────────────────┐
│  第 1 层：万物归一通用因果核 (Universal Causal Kernel Layer) │
│  [哲学：法 / 演化] UniversalCausalKernel, CausalIntervention│
└──────────────────────────────┬──────────────────────────────┘
                               │ (原子状态提交 / Commit Transition)
┌──────────────────────────────▼──────────────────────────────┐
│  第 0 层：元本体图谱层 (Ontological Graph & Ledger Layer)   │
│  [哲学：体 / 基质] 统一 Entity 模型、开放状态/属性、编年史流水│
└─────────────────────────────────────────────────────────────┘
```

---

## 2. 各分层职责与边界约束 (Layer Responsibilities & Boundaries)

### 【第 0 层】元本体图谱层 (Ontological Graph & Ledger Layer) —— “体”（Being）
* **定位**：世界的纯粹数据基质（Substrate），提供客观存在与时序事实的存储。
* **核心组件**：
  - `Entity`：统一实体模型（含 `essence`、`withdrawn_core`、`spatial` 拓扑域与 `assemblages`、开放自然语言 `current_state` 与 `properties: HashMap<String, Value>` 动态语义属性池）。
  - `Chronicle`：不可篡改的世界编年史时序事件流水与广播通道（`broadcast::Sender<ChronicleEvent>`）。
  - `Snapshot`：世界全量状态的无损 JSON 序列化与热加载。
* **⛔ 边界禁令**：**严禁在本层中调用 LLM、编写 Prompt 或硬编码业务物理规则**。它仅是纯粹的数据容器与状态集合。

### 【第 1 层】万物归一通用因果核 (Universal Causal Kernel Layer) —— “法”（Becoming）
* **定位**：世界的终极演化法则与相变中枢，大语言模型（LLM）在此充当物理法则与因果裁决核心。
* **核心设计**：**归一化因果坍缩（Singularity Causal Collapse）**。
* **核心组件**：
  - `CausalIntervention`：统一因果干涉输入（包含干涉模式 `mode`、参与实体集 `entities`、外部刺激/意图 `stimulus`）。
  - `UniversalCausalKernel`：万物归一通用因果算子，将自演化、碰撞化生、意识寄宿、神念倾听、客体际对话、场域共鸣与天道相变统摄为单一因果坍缩管线。
  - `CausalDelta`：统一状态变异增量（含 `narrative` 叙事、`mutations` 实体状态/属性/装配增量、`born_entities` 新化生物、`new_cosmic_atmosphere` 天道气象相变、`feedback` 心灵反馈）。

### 【第 2 层】自由觉知与意图层 (Consciousness & Agency Layer) —— “意”（Mind）
* **定位**：脱嵌式注意力与自由意志中枢，践行泛心论与具身生成哲学。
* **核心组件**：
  - **觉知算子（Attention Kernel）**：玩家或自治意识脱离具体肉身，自由锚定并寄宿入世界任意实体。
  - **感知引擎（`PerceptionEngine`）与局部视界（`SensoryHorizon`）**：严格遵循 OOO 哲学，向实体装配其邻近客体的感官表象、共生装配体脉络、场域共鸣与编年史片段，绝不泄露客体退隐内核。

### 【第 3 层】感官表象与呈现层 (Phenomenal & Multimodal Layer) —— “象”（Phenomenon）
* **定位**：感官界面的多模态外显层。
* **核心组件**：
  - **Three.js 3D 渲染引擎（`CosmicRenderer`）**：在 WebGL 中以 3D 拓扑星象图渲染万物灵元，动态呈现状态光晕、德勒兹共生能量线与空间轨道。
  - **纪元神话史诗引擎（`MythosEngine`）**：实时从编年史中提炼中英双语纪元神话诗篇与宇宙基调。
  - **实时长连接（Server-Sent Events / SSE）**：基于 `GET /api/events/stream` 零延迟推送宇宙因果动态。
  - **原生命令行交互终端（`PrimordiaRepl`）**：支持富文本状态检视与调试。

---

## 3. 防范“过度设计”与“混乱堆积”的准则 (Engineering Balance Matrix)

| 潜在风险 | 传统误区 (What to Avoid) | 《原初》的正交克制原则 (Our Standard) |
| :--- | :--- | :--- |
| **规则硬编码** | 为不同物体写继承子类、战斗公式、合成表 | **万物皆同构 `Entity`，演化 100% 委托 `UniversalCausalKernel` 推理** |
| **算子类膨胀** | 为每种交互写一个单独的类与定制字段 | **统摄为单一套 `CausalIntervention ──► UniversalCausalKernel ──► CausalDelta` 流水线** |
| **全局状态污染** | 在 LLM 提示词中塞入世界所有数据 | **通过 `SensoryHorizon` 仅提取局部视界，严格维护客体退隐性（OOO）** |
| **异步长借用死锁** | 在跨 `.await` 期间持有 `world` 的长周期借用 | **分阶段提取不可变上下文 ──► 异步 LLM ──► 短周期加锁原子提交状态变更** |
