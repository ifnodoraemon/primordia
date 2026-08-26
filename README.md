# 《原初》（Primordia: Meta）
> **一个完全基于大语言模型（LLM-Native）与 Rust 构建的自进化、自生长元世界基座**  
> **An LLM-Native, Self-Evolving & Autonomous Meta-World Foundation Built with Rust**

---

## 🌌 项目愿景 (Vision)

**[中文]**  
传统游戏依赖人工预设的规则树、技能表、剧情脚本与固化模型；《原初》（Primordia）采用纯粹的第一性原理与 Rust 高性能并发架构：  
**“不预设任何现成的人物、生物、职业或剧本，仅确立最底层的元公理。LLM 即是物理引擎、演化法则、万物心智与造物主。万物皆有灵性，自发演化、自发化生；玩家以纯粹自由意识融入万物，与世界共同呼吸与生长。”**

**[English]**  
Traditional game engines rely on handcrafted rule trees, fixed skill graphs, scripted narratives, and pre-baked models. *Primordia: Meta* takes a radical First-Principles approach powered by an LLM-native and high-concurrency Rust architecture:  
**"Zero pre-defined characters, creatures, classes, or scripts. Only the most primitive meta-axioms are laid down. The LLM serves as the physics engine, evolutionary law, universal cognition, and creator. Everything possesses animacy, evolving and morphing spontaneously; players inhabit anything as pure, unbounded consciousness, co-evolving and breathing alongside the cosmos."**

---

## 🏛️ 不可动摇的 4 大底层元公理 (The 4 Fundamental Axioms)

1. **万物平等与灵元公理 (Universal Animacy & Equality)**
   * **[中]** 世界上没有死物与活物的绝对界限。一块顽石、一束流光、一泓清泉、一只未知生灵，底层皆为平等的 **实体图谱节点（`Entity`）**。
   * **[EN]** No rigid dichotomy between living organisms and inanimate objects. A primordial stone, a beam of light, a spring, or an unknown beast are all equal **Entity Nodes** in the universal graph.
2. **抵抗熵增与自生长公理 (Autopoiesis & Self-Evolution)**
   * **[中]** 每一个实体在时间流逝中，均由 LLM 自主推演其形态蜕变、内部心智萌芽与结构变异。
   * **[EN]** Every entity autonomously undergoes morphological metamorphosis, cognitive awakening, and structural mutation over time, driven by LLM reasoning.
3. **碰撞化生与因果公理 (Emergent Morphogenesis & Causality)**
   * **[中]** 实体之间的相遇、碰撞与融合无需预设合成表，由 LLM 根据两者的本质属性动态裁决是否诞生新形态。
   * **[EN]** Collisions and encounters require no pre-baked crafting recipes. The LLM dynamically adjudicates causal outcomes and morphogenesis based on the fundamental nature of the interacting entities.
4. **自由觉知与融入公理 (Universal Mind Inhabitation)**
   * **[中]** 玩家并非固定的具象化身，而是一团自由流动的**“觉知意识”**。玩家可以随时寄宿于任何实体中（山川、草木、走兽、微风），以自然语言驱动其意志。
   * **[EN]** The player is not an anchored avatar, but a free-floating **Conscious Attention Kernel**. Players can seamlessly inhabit any entity (mountains, flora, beasts, breeze) and guide them via natural language intent.

---

## 🏗️ 架构与系统分层 (System Architecture)

```
┌─────────────────────────────────────────────────────────────┐
│             玩家自由觉知层 (Player Mind Inhabitation)         │
│   [Natural Language Intent] ──► [Inhabitation] ──► [Cosmic Will] │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│          Rust + LLM 原生世界核心 (PrimordiaWorld Engine)      │
│  ├─ 1. 自生长推演器 (Self-Evolution Ticker: evolve_entity)    │
│  ├─ 2. 交互碰撞裁决器 (Morphogenesis Engine: collide)         │
│  └─ 3. 玩家意志转译器 (Mind Grounding Engine: inhabit_and_act)│
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│               原初实体图谱 (Entity Graph & State)            │
│  [Entity Collection: HashMap<String, Entity>] + [Chronicle] │
└─────────────────────────────────────────────────────────────┘
```

---

## 📂 文档导航 (Documentation Index)

* 📜 [核心哲学与公理体系 / Philosophy & Axioms (docs/VISION_AND_AXIOMS.md)](docs/VISION_AND_AXIOMS.md)
* 📐 [系统架构与运行机制 / Architecture & Mechanics (docs/ARCHITECTURE.md)](docs/ARCHITECTURE.md)
* 🗄️ [实体图谱与数据规范 / Data Schema (docs/DATA_SCHEMA.md)](docs/DATA_SCHEMA.md)
* 🧠 [核心 Prompt 链与驱动循环 / Prompts & Loops (docs/PROMPTS_AND_LOOPS.md)](docs/PROMPTS_AND_LOOPS.md)

---

## 🚀 极速运行与验证 (Quick Start)

### 运行 Rust 核心引擎 Demo / Run Rust Core Demo:
```bash
cargo run
```

### 配置真实大模型 API / Configure Real LLM API:
支持任意兼容 OpenAI 接口的模型（如 GPT-4o, DeepSeek, Qwen, Ollama, vLLM）：
```bash
export OPENAI_API_KEY="your-api-key"
export OPENAI_BASE_URL="https://api.openai.com/v1" # 或本地端点 / Or local endpoint
export LLM_MODEL="gpt-4o-mini"
cargo run
```
