# 《原初》（Primordia: Meta）
> **一个完全基于大语言模型（LLM-Native）与 Rust 构建的自进化、自生长元世界基座**  
> **An LLM-Native, Self-Evolving & Autonomous Meta-World Foundation Built with Rust**

---

## 🌌 项目愿景 (Vision)

**[中文]**  
传统游戏依赖人工预设的规则树、技能表、剧情脚本与固化模型；《原初》（Primordia）采用纯粹的第一性原理与现代本体论哲学：  
**“不预设任何现成的人物、生物、职业或剧本，仅确立最底层的元公理。LLM 即是物理引擎、演化法则、万物心智与造物主。万物皆有灵性，自发演化、自发化生；玩家以纯粹自由意识融入万物，与世界共同呼吸与生长。”**

**[English]**  
Traditional game engines rely on handcrafted rule trees, fixed skill graphs, scripted narratives, and pre-baked models. *Primordia: Meta* takes a radical First-Principles approach grounded in modern flat ontology:  
**"Zero pre-defined characters, creatures, classes, or scripts. Only the most primitive meta-axioms are laid down. The LLM serves as the physics engine, evolutionary law, universal cognition, and creator. Everything possesses animacy, evolving and morphing spontaneously; players inhabit anything as pure, unbounded consciousness, co-evolving and breathing alongside the cosmos."**

---

## 🏛️ 不可动摇的 4 大底层元公理 (The 4 Fundamental Axioms)

1. **万物平等与灵元公理 (Universal Animacy & Equality)**
   * **[中]** 世界上没有死物与活物的绝对界限。一块顽石、一束流光、一泓清泉、一只未知生灵，底层皆为平等的 **实体节点（`Entity`）**，具备退隐内核与感官界面。
   * **[EN]** No rigid dichotomy between living organisms and inanimate objects. All existences share the same unified `Entity` node structure, complete with a Withdrawn Core and Sensual Interface.
2. **抵抗熵增与自生长公理 (Autopoiesis & Self-Evolution)**
   * **[中]** 每一个实体在时间流逝中，均由 LLM 自主推演其形态蜕变、内部心智萌芽与结构变异。
   * **[EN]** Every entity autonomously undergoes morphological metamorphosis, cognitive awakening, and structural mutation over time (Autopoiesis).
3. **碰撞化生与装配因果公理 (Emergent Morphogenesis & Assemblages)**
   * **[中]** 实体之间的相遇无需预设合成表，由 LLM 动态裁决相变、生命化生或结成德勒兹共生装配体。
   * **[EN]** Collisions require no recipes. The LLM dynamically adjudicates phase shifts, spontaneous morphogenesis, or Deleuzian symbiotic assemblages.
4. **自由觉知与融入公理 (Universal Mind Inhabitation)**
   * **[中]** 玩家并非固定的具象化身，而是一团自由流动的**“觉知意识算子（Attention Kernel）”**，可随时寄宿于任何实体中以自然语言驱动其意志。
   * **[EN]** The player is not an anchored avatar, but a free-floating **Conscious Attention Kernel** capable of inhabiting any entity across the cosmos.

---

## 🏗️ 正交系统分层 (The 4-Layer Orthogonal Architecture)

依据东方“体/法/意/象”哲学与现代本体论，系统建立了一套严格单向流动的四层正交架构，杜绝过度设计与混乱堆积：

```
┌─────────────────────────────────────────────────────────────┐
│  第 3 层：感官表象与多模态层 (Phenomenal & Multimodal Layer)  │
│  [象 / 呈现] 客体感官界面、自然语言史诗、多模态投影         │
└──────────────────────────────▲──────────────────────────────┘
                               │ (只读投影 / Read & Project)
┌──────────────────────────────┴──────────────────────────────┐
│  第 2 层：自由觉知与意图层 (Consciousness & Agency Layer)    │
│  [意 / 觉知] 玩家/自治意识算子、自由意志注入、注意力焦点     │
└──────────────────────────────┬──────────────────────────────┘
                               │ (驱动与干预 / Drive & Collapse)
┌──────────────────────────────▼──────────────────────────────┐
│  第 1 层：语义因果与法则层 (Semantic Causality & Laws Layer) │
│  [法 / 演化] LLM 即物理、自生长算子、相变装配、天道相变     │
└──────────────────────────────┬──────────────────────────────┘
                               │ (原子状态提交 / Commit Transition)
┌──────────────────────────────▼──────────────────────────────┐
│  第 0 层：元本体图谱层 (Ontological Graph & Ledger Layer)   │
│  [体 / 基质] 极简实体图谱 (Entity)、关系拓扑、编年史时序库  │
└─────────────────────────────────────────────────────────────┘
```

---

## 📂 文档导航 (Documentation Index)

* 📜 [核心哲学与公理体系 / Philosophy & Axioms (docs/VISION_AND_AXIOMS.md)](docs/VISION_AND_AXIOMS.md)
* 📐 [系统架构与正交分层 / Architecture & Layering (docs/ARCHITECTURE.md)](docs/ARCHITECTURE.md)
* 🗄️ [实体图谱与数据规范 / Data Schema (docs/DATA_SCHEMA.md)](docs/DATA_SCHEMA.md)
* 🧠 [核心 Prompt 链与驱动循环 / Prompts & Loops (docs/PROMPTS_AND_LOOPS.md)](docs/PROMPTS_AND_LOOPS.md)

---

## 🚀 极速运行与验证 (Quick Start)

### 运行 Rust 核心引擎 Demo / Run Rust Core Demo:
```bash
cargo run
```

### 运行完整测试套件 / Run Test Suite:
```bash
cargo test
```

### 配置真实大模型 API / Configure Real LLM API:
支持任意兼容 OpenAI 接口的模型（如 GPT-4o, DeepSeek, Qwen, Ollama, vLLM）：
```bash
export OPENAI_API_KEY="your-api-key"
export OPENAI_BASE_URL="https://api.openai.com/v1" # 或本地端点 / Or local endpoint
export LLM_MODEL="gpt-4o-mini"
cargo run
```
