# AGENTS.md - 《原初》（Primordia: Meta）智能体开发与协作规范
> **AI Agent Guidelines, Architectural Constraints & Development Standards (Bilingual / 中英双语)**

---

## 🧭 1. 项目定位与核心宪章 (Project Mission & Charter)

**《原初》（Primordia: Meta）** 是一个完全基于**大语言模型（LLM-Native）**与 **Rust** 构建的自进化、自生长元世界基座。  
*Primordia: Meta* is an autonomous, self-evolving meta-world foundation built purely on **LLMs as the physics & cognitive substrate** using **Rust**.

任何在此项目中协作的 AI 智能体（Agents）必须严格遵守以下**不可违背的核心公理与工程约束**。  
All AI agents contributing to this repository must strictly adhere to the inviolable axioms and constraints defined below.

---

## 🚫 2. 绝对不可违背的架构戒律 (Inviolable Architectural Constraints)

### ❌ 禁令一：严禁硬编码游戏业务逻辑与分类系统 (NO Hardcoded Game Logic & Taxonomy)
* **[禁止]** 严禁在代码中写死职业（战士/法师）、生物种属（人类/哥布林）、合成配方表（木头+石头=斧头）或固定技能树。
* **[禁止]** 严禁将山川、草木、走兽划分为不同的派生类或结构体。
* **[必须]** 万物皆平等继承并遵从统一的 `Entity` 数据模型。所有的相变、合成、演化与交互结果，**必须完全由 LLM 基于第一性原理实时推理生成**。

### ❌ 禁令二：严禁破坏“语义即物理”原则 (Semantics as Physics)
* **[禁止]** 严禁引入传统的数值硬碰撞、HP/MP 减法战斗公式或固化状态机。
* **[必须]** 物理反应、魔法相变、生命诞生均视为 LLM 上下文图谱中的语义演进与因果判定。

### ❌ 禁令三：严禁限制玩家寄宿自由度 (Universal Mind Inhabitation)
* **[必须]** 玩家的意识在底层必须是一团无拘无束的“注意力算子（Attention Kernel）”。
* **[必须]** 玩家必须能够随时寄宿到世界上任意一个实体（一缕风、一团火、一座山、一只生灵），并以自然语言意图驱动其行为。

---

## 🦀 3. 技术栈与 Rust 编码规范 (Rust Engineering Standards)

### 3.1 依赖与系统库约束
* **Rust 版本**：2021 Edition 及以上。
* **异步运行时**：统一使用 `tokio` (full)。
* **网络与 TLS**：统一使用 `reqwest` + `rustls-tls`（`default-features = false`）。**严禁引入依赖系统底层 `openssl-sys / libssl-dev` 的依赖项**，确保在任何无 C/C++ OpenSSL 库的环境中均能即开即编译。
* **序列化**：统一使用 `serde` 与 `serde_json`。

### 3.2 错误处理与安全性
* 严禁在核心演化循环与运行时接口中随意使用 `.unwrap()` 导致服务崩溃。必须使用 `Result<T, String>` 或自定义 Error 传播错误。
* 保持 Rust 借用检查（Borrow Checker）的最佳实践，修改实体状态时避免长周期的可变借用冲突。

---

## 🌐 4. 中英双语强制规范 (Bilingual Requirement)

为了保持项目的国际化与极客气质，本项目严格实行**中英双语规范**：

1. **文档规范**：所有位于 `docs/` 及根目录下的 `.md` 文档，必须保持中英双语（对照或并列）。
2. **Prompt 模板规范**：位于 `docs/PROMPTS_AND_LOOPS.md` 中的所有 Prompt 模板必须包含中英双语版本。
3. **控制台输出与日志规范**：终端输出与世界编年史（`Chronicle`）记录必须包含中英文对照标识，例如：
   ```rust
   println!("[纪元 {} | Epoch {}] <{}> {}", tick, tick, event_type, detail);
   ```

---

## 📁 5. 项目目录结构约定 (Repository Layout)

```text
primordia/
├── AGENTS.md                  # [本文件] 智能体开发约束与协作规范
├── Cargo.toml                 # Rust 项目配置文件 (严格使用 rustls)
├── README.md                  # 中英双语项目总览与快速开始
├── docs/                      # 中英双语核心设计与规范文档
│   ├── VISION_AND_AXIOMS.md   # 核心哲学与四大元公理
│   ├── ARCHITECTURE.md        # 系统三层流水线架构规范
│   ├── DATA_SCHEMA.md         # 统一实体与编年史 Schema
│   └── PROMPTS_AND_LOOPS.md   # 核心 Prompt 模板与驱动协议
└── src/                       # Rust 核心引擎源码
    ├── lib.rs                 # 库根导出
    ├── entity.rs              # 统一灵元实体模型 (Entity, Spatial)
    ├── llm.rs                 # 异步 LLM 客户端与离线回退推演器
    ├── world.rs               # PrimordiaWorld 调度器 (演化/化生/寄宿/编年史)
    └── main.rs                # 运行演示入口
```

---

## 🧪 6. 验证与测试指引 (Verification & Testing)

在完成任何代码修改或功能扩展后，AI 智能体必须在本地执行并验证通过以下命令：

```bash
# 1. 语法与类型检查
cargo check

# 2. 编译并运行演示验证
cargo run
```

确保控制台完整打印出 4 个核心步骤（创世、寄宿、化生、自演化纪元），且无编译警告与运行时 Panic。
