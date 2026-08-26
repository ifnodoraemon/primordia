use crate::world::PrimordiaWorld;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

pub type SharedWorld = Arc<Mutex<PrimordiaWorld>>;

#[derive(Debug, Deserialize)]
pub struct InhabitPayload {
    pub entity_id: String,
    pub intent: String,
}

#[derive(Debug, Deserialize)]
pub struct ActPayload {
    pub entity_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CollidePayload {
    pub entity_a: String,
    pub entity_b: String,
}

#[derive(Debug, Deserialize)]
pub struct ResonatePayload {
    pub domain_name: String,
}

#[derive(Debug, Deserialize)]
pub struct TickPayload {
    pub count: Option<u64>,
}

use tower_http::services::ServeDir;

/// 启动 Primordia Web 服务器
pub async fn start_web_server(world: SharedWorld, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let serve_dir = ServeDir::new("web/dist").fallback(get(index_handler));

    let app = Router::new()
        .route("/api/world/status", get(get_world_status))
        .route("/api/entities", get(get_entities))
        .route("/api/entities/:id", get(get_entity_detail))
        .route("/api/inhabit", post(inhabit_entity))
        .route("/api/act", post(act_autonomously))
        .route("/api/collide", post(collide_entities))
        .route("/api/resonate", post(trigger_resonance))
        .route("/api/tick", post(tick_world))
        .route("/api/shift_law", post(shift_cosmic_law))
        .route("/api/mythos", get(get_mythos))
        .route("/api/trace", get(get_trace))
        .route("/api/snapshot", get(get_snapshot))
        .fallback_service(serve_dir)
        .layer(CorsLayer::permissive())
        .with_state(world);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("=================================================================");
    println!("  🌌 《原初》（Primordia: Meta）Web 客户端与服务已启动！");
    println!("  🌐 访问地址 / Web Client URL: http://localhost:{}", port);
    println!("  🔗 API 端点 / REST APIs: http://localhost:{}/api/world/status", port);
    println!("=================================================================\n");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// -------------------------------------------------------------------------
// REST API Handlers
// -------------------------------------------------------------------------

async fn get_world_status(State(world): State<SharedWorld>) -> impl IntoResponse {
    let w = world.lock().await;
    Json(json!({
        "name": w.name,
        "tick_count": w.tick_count,
        "cosmic_atmosphere": w.cosmic_atmosphere,
        "total_entities": w.entities.len(),
        "chronicle_count": w.chronicle.len(),
        "recent_chronicle": w.chronicle.iter().rev().take(15).collect::<Vec<_>>()
    }))
}

async fn get_entities(State(world): State<SharedWorld>) -> impl IntoResponse {
    let w = world.lock().await;
    let entities: Vec<_> = w.entities.values().cloned().collect();
    Json(entities)
}

async fn get_entity_detail(
    State(world): State<SharedWorld>,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let w = world.lock().await;
    if let Some(ent) = w.entities.get(&id) {
        Ok(Json(json!(ent)))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn inhabit_entity(
    State(world): State<SharedWorld>,
    Json(payload): Json<InhabitPayload>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let mut w = world.lock().await;
    let res = w
        .inhabit_and_act(&payload.entity_id, &payload.intent)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok(Json(res))
}

async fn act_autonomously(
    State(world): State<SharedWorld>,
    Json(payload): Json<ActPayload>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let mut w = world.lock().await;
    let res = w
        .act_autonomously(&payload.entity_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok(Json(res))
}

async fn collide_entities(
    State(world): State<SharedWorld>,
    Json(payload): Json<CollidePayload>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let mut w = world.lock().await;
    let res = w
        .collide(&payload.entity_a, &payload.entity_b)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok(Json(res))
}

async fn trigger_resonance(
    State(world): State<SharedWorld>,
    Json(payload): Json<ResonatePayload>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let mut w = world.lock().await;
    let res = w
        .trigger_domain_resonance(&payload.domain_name)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok(Json(res))
}

async fn tick_world(
    State(world): State<SharedWorld>,
    Json(payload): Json<TickPayload>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let mut w = world.lock().await;
    let count = payload.count.unwrap_or(1);
    for _ in 0..count {
        w.tick()
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    }
    Ok(Json(json!({ "status": "ok", "current_tick": w.tick_count })))
}

async fn shift_cosmic_law(State(world): State<SharedWorld>) -> Result<Json<Value>, (StatusCode, String)> {
    let mut w = world.lock().await;
    let res = w
        .evolve_cosmic_law()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok(Json(json!({ "new_atmosphere": res })))
}

async fn get_mythos(State(world): State<SharedWorld>) -> Result<Json<Value>, (StatusCode, String)> {
    let w = world.lock().await;
    let mythos = w
        .distill_mythos()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok(Json(json!(mythos)))
}

async fn get_trace(State(world): State<SharedWorld>) -> impl IntoResponse {
    let w = world.lock().await;
    Json(json!({
        "summary": w.tracer.summary(),
        "spans": w.tracer.spans
    }))
}

async fn get_snapshot(State(world): State<SharedWorld>) -> Result<Json<Value>, (StatusCode, String)> {
    let w = world.lock().await;
    let json_str = w
        .export_snapshot_json()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    let val: Value = serde_json::from_str(&json_str)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(val))
}

// -------------------------------------------------------------------------
// Single-Page Embedded Web UI Client
// -------------------------------------------------------------------------

async fn index_handler() -> Html<&'static str> {
    Html(r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>《原初》（Primordia: Meta）- LLM-Native 元世界控制中枢</title>
    <style>
        :root {
            --bg-deep: #07090e;
            --bg-card: #0e131f;
            --bg-card-hover: #151d30;
            --primary: #4ade80;
            --primary-glow: rgba(74, 222, 128, 0.25);
            --accent-cyan: #38bdf8;
            --accent-purple: #c084fc;
            --accent-amber: #fbbf24;
            --accent-rose: #f43f5e;
            --text-main: #f1f5f9;
            --text-muted: #94a3b8;
            --border-dim: #1e293b;
            --border-bright: #334155;
        }

        * { box-sizing: border-box; margin: 0; padding: 0; }
        body {
            background-color: var(--bg-deep);
            color: var(--text-main);
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "PingFang SC", "Hiragino Sans GB", "Microsoft YaHei", sans-serif;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            overflow-x: hidden;
        }

        /* 顶部导航栏 */
        header {
            background: rgba(14, 19, 31, 0.85);
            backdrop-filter: blur(12px);
            border-bottom: 1px solid var(--border-dim);
            padding: 12px 24px;
            display: flex;
            justify-content: space-between;
            align-items: center;
            position: sticky;
            top: 0;
            z-index: 100;
        }
        .brand {
            display: flex;
            align-items: center;
            gap: 12px;
        }
        .logo-icon {
            font-size: 24px;
            filter: drop-shadow(0 0 8px var(--primary));
        }
        .brand h1 {
            font-size: 18px;
            font-weight: 700;
            letter-spacing: 0.5px;
            background: linear-gradient(135deg, #fff, var(--accent-cyan));
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }
        .brand span {
            font-size: 12px;
            color: var(--text-muted);
            border: 1px solid var(--border-bright);
            padding: 2px 8px;
            border-radius: 12px;
        }
        .header-meta {
            display: flex;
            gap: 20px;
            font-size: 13px;
        }
        .meta-pill {
            display: flex;
            align-items: center;
            gap: 6px;
            background: var(--bg-card);
            border: 1px solid var(--border-dim);
            padding: 6px 14px;
            border-radius: 20px;
        }
        .meta-pill strong { color: var(--primary); }

        /* 宏观天道横幅 */
        .atmosphere-banner {
            background: linear-gradient(90deg, rgba(56, 189, 248, 0.1), rgba(192, 132, 252, 0.1), rgba(74, 222, 128, 0.1));
            border-bottom: 1px solid var(--border-dim);
            padding: 10px 24px;
            display: flex;
            align-items: center;
            gap: 10px;
            font-size: 13px;
        }
        .atmosphere-label {
            color: var(--accent-cyan);
            font-weight: bold;
            white-space: nowrap;
        }
        .atmosphere-content {
            color: var(--text-main);
            font-style: italic;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }

        /* 主体双栏/三栏布局 */
        main {
            display: grid;
            grid-template-columns: 360px 1fr 400px;
            gap: 16px;
            padding: 16px 24px;
            flex: 1;
        }

        @media (max-width: 1200px) {
            main { grid-template-columns: 1fr; }
        }

        .panel {
            background: var(--bg-card);
            border: 1px solid var(--border-dim);
            border-radius: 12px;
            display: flex;
            flex-direction: column;
            overflow: hidden;
        }
        .panel-header {
            padding: 14px 18px;
            border-bottom: 1px solid var(--border-dim);
            display: flex;
            justify-content: space-between;
            align-items: center;
            background: rgba(14, 19, 31, 0.5);
        }
        .panel-title {
            font-size: 14px;
            font-weight: 600;
            display: flex;
            align-items: center;
            gap: 8px;
        }
        .panel-body {
            padding: 16px;
            overflow-y: auto;
            flex: 1;
        }

        /* 操作控制台按钮组 */
        .controls-grid {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 8px;
            margin-bottom: 16px;
        }
        .btn {
            background: var(--bg-card-hover);
            color: var(--text-main);
            border: 1px solid var(--border-bright);
            padding: 10px 14px;
            border-radius: 8px;
            font-size: 13px;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 6px;
            transition: all 0.2s ease;
        }
        .btn:hover {
            background: #1e293b;
            border-color: var(--primary);
            box-shadow: 0 0 12px var(--primary-glow);
        }
        .btn-primary {
            background: linear-gradient(135deg, #10b981, #059669);
            border: none;
            color: #fff;
            font-weight: 600;
            grid-column: span 2;
        }
        .btn-primary:hover {
            background: linear-gradient(135deg, #059669, #047857);
            box-shadow: 0 0 16px rgba(16, 185, 129, 0.4);
        }

        /* 实体列表卡片 */
        .entity-card {
            background: var(--bg-deep);
            border: 1px solid var(--border-dim);
            border-radius: 8px;
            padding: 12px;
            margin-bottom: 10px;
            cursor: pointer;
            transition: all 0.2s ease;
            position: relative;
        }
        .entity-card:hover, .entity-card.selected {
            border-color: var(--accent-cyan);
            box-shadow: 0 0 12px rgba(56, 189, 248, 0.2);
            background: rgba(56, 189, 248, 0.05);
        }
        .entity-card-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 6px;
        }
        .entity-name {
            font-size: 14px;
            font-weight: 600;
            color: var(--text-main);
        }
        .phase-badge {
            font-size: 11px;
            padding: 2px 8px;
            border-radius: 10px;
            font-weight: bold;
        }
        .phase-genesis { background: rgba(56, 189, 248, 0.2); color: var(--accent-cyan); }
        .phase-flourishing { background: rgba(74, 222, 128, 0.2); color: var(--primary); }
        .phase-decay { background: rgba(251, 191, 36, 0.2); color: var(--accent-amber); }
        .phase-dissolution { background: rgba(244, 63, 94, 0.2); color: var(--accent-rose); }

        .entity-domain {
            font-size: 12px;
            color: var(--accent-purple);
            margin-bottom: 4px;
        }
        .entity-state {
            font-size: 12px;
            color: var(--text-muted);
            line-height: 1.4;
        }
        .traits-list {
            display: flex;
            flex-wrap: wrap;
            gap: 4px;
            margin-top: 8px;
        }
        .trait-tag {
            font-size: 10px;
            background: rgba(255, 255, 255, 0.06);
            border: 1px solid var(--border-dim);
            padding: 2px 6px;
            border-radius: 4px;
            color: var(--text-muted);
        }

        /* 觉知寄宿交互输入框 */
        .inhabit-box {
            margin-top: 14px;
            background: var(--bg-deep);
            border: 1px solid var(--border-dim);
            border-radius: 8px;
            padding: 12px;
        }
        .inhabit-box textarea {
            width: 100%;
            background: transparent;
            border: none;
            color: var(--text-main);
            font-size: 13px;
            resize: none;
            height: 60px;
            outline: none;
            font-family: inherit;
        }
        .inhabit-box button {
            margin-top: 8px;
            width: 100%;
        }

        /* 编年史时间线与追踪 */
        .chronicle-list {
            display: flex;
            flex-direction: column;
            gap: 10px;
        }
        .chronicle-item {
            background: var(--bg-deep);
            border-left: 3px solid var(--primary);
            border-radius: 0 6px 6px 0;
            padding: 10px 12px;
            font-size: 12px;
        }
        .chronicle-item.MIND_INHABITATION { border-left-color: var(--accent-purple); }
        .chronicle-item.COLLISION_MORPHOGENESIS { border-left-color: var(--accent-amber); }
        .chronicle-item.DOMAIN_RESONANCE { border-left-color: var(--accent-cyan); }
        .chronicle-item.ENTITY_DISSOLUTION { border-left-color: var(--accent-rose); }
        .chronicle-meta {
            display: flex;
            justify-content: space-between;
            color: var(--text-muted);
            font-size: 11px;
            margin-bottom: 4px;
        }

        /* 神话碑刻弹窗卡片 */
        .mythos-card {
            background: radial-gradient(circle at center, rgba(192, 132, 252, 0.15), transparent 80%), var(--bg-deep);
            border: 1px solid var(--accent-purple);
            border-radius: 8px;
            padding: 16px;
            margin-bottom: 16px;
        }
        .mythos-title {
            font-size: 15px;
            font-weight: 700;
            color: var(--accent-purple);
            margin-bottom: 8px;
            display: flex;
            align-items: center;
            gap: 6px;
        }
        .mythos-text {
            font-size: 13px;
            color: var(--text-main);
            line-height: 1.6;
            font-style: italic;
        }

        /* 状态加载动画 */
        .spinner {
            display: inline-block;
            width: 14px;
            height: 14px;
            border: 2px solid rgba(255,255,255,0.3);
            border-radius: 50%;
            border-top-color: #fff;
            animation: spin 0.8s ease-in-out infinite;
        }
        @keyframes spin { to { transform: rotate(360deg); } }
    </style>
</head>
<body>
    <header>
        <div class="brand">
            <span class="logo-icon">🌌</span>
            <div>
                <h1>《原初》（Primordia: Meta）</h1>
            </div>
            <span>LLM-Native Meta-World</span>
        </div>
        <div class="header-meta">
            <div class="meta-pill">纪元周期 / Epoch: <strong id="val-tick">0</strong></div>
            <div class="meta-pill">灵元实体 / Entities: <strong id="val-entities">0</strong></div>
            <div class="meta-pill">编年因果 / Chronicle: <strong id="val-chronicle">0</strong></div>
        </div>
    </header>

    <div class="atmosphere-banner">
        <span class="atmosphere-label">🌌 宏观天道气象 / Cosmic Atmosphere:</span>
        <span class="atmosphere-content" id="val-atmosphere">正在感知虚空因果……</span>
    </div>

    <main>
        <!-- 左栏：因果激荡操作台 & 自由寄宿 -->
        <section class="panel">
            <div class="panel-header">
                <span class="panel-title">⚡ 因果操作台 / Causality Deck</span>
            </div>
            <div class="panel-body">
                <div class="controls-grid">
                    <button class="btn btn-primary" onclick="triggerTick()">🌌 推进世界纪元 / Advance Epoch Tick</button>
                    <button class="btn" onclick="triggerShiftLaw()">⚡ 宏观天道相变 / Shift Law</button>
                    <button class="btn" onclick="fetchMythos()">📜 提炼纪元史诗 / Distill Mythos</button>
                    <button class="btn" onclick="triggerResonancePrompt()">🌊 激发场域共鸣 / Resonate</button>
                    <button class="btn" onclick="triggerCollidePrompt()">💥 触发碰撞相变 / Collide</button>
                </div>

                <div class="panel-title" style="margin: 16px 0 8px 0;">🧠 觉知寄宿与意志注入</div>
                <div class="inhabit-box">
                    <div style="font-size: 11px; color: var(--accent-cyan); margin-bottom: 6px;" id="selected-entity-label">
                        👉 请在右侧选择一个寄宿实体
                    </div>
                    <textarea id="intent-input" placeholder="输入您的自然语言自由意志（例如：将体内的星光向内核聚集，朝向虚空发出呼唤……）"></textarea>
                    <button class="btn btn-primary" onclick="submitInhabitation()">✨ 注入意识驱动实体 / Ground Agency</button>
                </div>
            </div>
        </section>

        <!-- 中栏：灵元实体星象图谱 -->
        <section class="panel">
            <div class="panel-header">
                <span class="panel-title">📜 灵元实体图谱 / Ontological Entities</span>
                <button class="btn" style="padding: 4px 10px; font-size: 11px;" onclick="refreshAll()">🔄 刷新 / Refresh</button>
            </div>
            <div class="panel-body" id="entities-container">
                <!-- 动态渲染实体卡片 -->
            </div>
        </section>

        <!-- 右栏：编年史流与神话史诗碑刻 -->
        <section class="panel">
            <div class="panel-header">
                <span class="panel-title">📜 宇宙编年史与史诗 / Mythos & Chronicle</span>
            </div>
            <div class="panel-body">
                <div id="mythos-container" style="display: none;" class="mythos-card">
                    <div class="mythos-title">📜 <span id="mythos-title-text">纪元篇章</span></div>
                    <div class="mythos-text" id="mythos-epic-text"></div>
                </div>

                <div class="chronicle-list" id="chronicle-container">
                    <!-- 动态渲染编年史事件 -->
                </div>
            </div>
        </section>
    </main>

    <script>
        let selectedEntityId = null;

        async function fetchStatus() {
            try {
                const res = await fetch('/api/world/status');
                const data = await res.json();
                document.getElementById('val-tick').innerText = data.tick_count;
                document.getElementById('val-entities').innerText = data.total_entities;
                document.getElementById('val-chronicle').innerText = data.chronicle_count;
                document.getElementById('val-atmosphere').innerText = data.cosmic_atmosphere;

                renderChronicle(data.recent_chronicle || []);
            } catch (e) {
                console.error("Failed to fetch world status:", e);
            }
        }

        async function fetchEntities() {
            try {
                const res = await fetch('/api/entities');
                const entities = await res.json();
                renderEntities(entities);
            } catch (e) {
                console.error("Failed to fetch entities:", e);
            }
        }

        function renderEntities(entities) {
            const container = document.getElementById('entities-container');
            container.innerHTML = '';

            entities.forEach(ent => {
                const card = document.createElement('div');
                card.className = `entity-card ${selectedEntityId === ent.id ? 'selected' : ''}`;
                card.onclick = () => selectEntity(ent);

                let phaseClass = 'phase-flourishing';
                let phaseText = ent.lifecycle || '住 / Flourishing';
                if (phaseText.includes('Genesis') || phaseText.includes('成')) phaseClass = 'phase-genesis';
                else if (phaseText.includes('Decay') || phaseText.includes('坏')) phaseClass = 'phase-decay';
                else if (phaseText.includes('Dissol') || phaseText.includes('空')) phaseClass = 'phase-dissolution';

                card.innerHTML = `
                    <div class="entity-card-header">
                        <span class="entity-name">${ent.name}</span>
                        <span class="phase-badge ${phaseClass}">${phaseText} [${ent.cohesion.toFixed(2)}]</span>
                    </div>
                    <div class="entity-domain">📍 场域 / Domain: ${ent.spatial.domain}</div>
                    <div class="entity-state">${ent.current_state}</div>
                    <div class="traits-list">
                        ${(ent.traits || []).map(t => `<span class="trait-tag">${t}</span>`).join('')}
                    </div>
                `;
                container.appendChild(card);
            });
        }

        function selectEntity(ent) {
            selectedEntityId = ent.id;
            document.getElementById('selected-entity-label').innerText = `👉 当前已寄宿: 【${ent.name}】(${ent.id})`;
            fetchEntities();
        }

        function renderChronicle(events) {
            const container = document.getElementById('chronicle-container');
            container.innerHTML = '';

            events.forEach(e => {
                const item = document.createElement('div');
                item.className = `chronicle-item ${e.event_type}`;
                item.innerHTML = `
                    <div class="chronicle-meta">
                        <span>[Epoch ${e.tick}] &lt;${e.event_type}&gt;</span>
                        <span>${new Date(e.timestamp * 1000).toLocaleTimeString()}</span>
                    </div>
                    <div>${e.detail}</div>
                `;
                container.appendChild(item);
            });
        }

        async function triggerTick() {
            await fetch('/api/tick', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ count: 1 })
            });
            refreshAll();
        }

        async function triggerShiftLaw() {
            await fetch('/api/shift_law', { method: 'POST' });
            refreshAll();
        }

        async function fetchMythos() {
            const res = await fetch('/api/mythos');
            const data = await res.json();
            const box = document.getElementById('mythos-container');
            document.getElementById('mythos-title-text').innerText = `${data.title} (${data.world_tone})`;
            document.getElementById('mythos-epic-text').innerText = data.poetic_epic;
            box.style.display = 'block';
        }

        async function submitInhabitation() {
            if (!selectedEntityId) {
                alert("请先在右侧点击选中一个要寄宿的灵元实体！");
                return;
            }
            const intent = document.getElementById('intent-input').value.trim();
            if (!intent) {
                alert("请输入意图！");
                return;
            }

            await fetch('/api/inhabit', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ entity_id: selectedEntityId, intent: intent })
            });
            document.getElementById('intent-input').value = '';
            refreshAll();
        }

        async function triggerResonancePrompt() {
            const domain = prompt("请输入要激发共鸣的拓扑场域名称（例如：悬天绝壁 / Celestial Cliff Precipice）：", "悬天绝壁 / Celestial Cliff Precipice");
            if (domain) {
                await fetch('/api/resonate', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ domain_name: domain })
                });
                refreshAll();
            }
        }

        async function triggerCollidePrompt() {
            const a = prompt("请输入实体 A 的 ID（例如 ent_001）：", "ent_001");
            const b = prompt("请输入实体 B 的 ID（例如 ent_002）：", "ent_002");
            if (a && b) {
                await fetch('/api/collide', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ entity_a: a, entity_b: b })
                });
                refreshAll();
            }
        }

        function refreshAll() {
            fetchStatus();
            fetchEntities();
        }

        // 初始化轮询刷新
        refreshAll();
        setInterval(fetchStatus, 3000);
        setInterval(fetchEntities, 4000);
    </script>
</body>
</html>"#)
}
