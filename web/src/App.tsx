import React, { useEffect, useState, useCallback } from 'react';
import { Entity, MythosChapter, TraceData, WorldStatus } from './types';
import { fetchEntities, fetchMythos, fetchTrace, fetchWorldStatus } from './api';
import { Header } from './components/Header';
import { CosmicRenderer } from './components/CosmicRenderer';
import { CausalityDeck } from './components/CausalityDeck';
import { EntityInspector } from './components/EntityInspector';
import { ChronicleStream } from './components/ChronicleStream';
import { MythosModal } from './components/MythosModal';
import { TraceDrawer } from './components/TraceDrawer';

export const App: React.FC = () => {
  const [status, setStatus] = useState<WorldStatus | null>(null);
  const [entities, setEntities] = useState<Entity[]>([]);
  const [selectedEntity, setSelectedEntity] = useState<Entity | null>(null);
  const [mythos, setMythos] = useState<MythosChapter | null>(null);
  const [trace, setTrace] = useState<TraceData | null>(null);
  const [isMythosOpen, setIsMythosOpen] = useState(false);
  const [isTraceOpen, setIsTraceOpen] = useState(false);

  const refreshWorld = useCallback(async () => {
    try {
      const [statusData, entitiesData] = await Promise.all([
        fetchWorldStatus(),
        fetchEntities(),
      ]);
      setStatus(statusData);
      setEntities(entitiesData);

      // Keep selected entity updated
      if (selectedEntity) {
        const found = entitiesData.find((e) => e.id === selectedEntity.id);
        if (found) setSelectedEntity(found);
      }
    } catch (err) {
      console.error('Failed to sync world:', err);
    }
  }, [selectedEntity]);

  useEffect(() => {
    refreshWorld();

    // 建立与 Rust 后端的原生 Server-Sent Events (SSE) 实时长连接
    const eventSource = new EventSource('/api/events/stream');

    eventSource.addEventListener('chronicle', (e) => {
      try {
        const newEvent = JSON.parse(e.data);
        setStatus((prev) => {
          if (!prev) return prev;
          return {
            ...prev,
            chronicle_count: prev.chronicle_count + 1,
            recent_chronicle: [newEvent, ...prev.recent_chronicle.slice(0, 49)],
          };
        });
        refreshWorld();
      } catch (err) {
        console.error('Failed to parse SSE event:', err);
      }
    });

    eventSource.onerror = (err) => {
      console.warn('SSE connection error, fallback to heartbeat polling:', err);
    };

    const interval = setInterval(refreshWorld, 4000);
    return () => {
      clearInterval(interval);
      eventSource.close();
    };
  }, [refreshWorld]);

  const handleOpenMythos = async () => {
    try {
      const data = await fetchMythos();
      setMythos(data);
      setIsMythosOpen(true);
    } catch (e: any) {
      alert(`无法提炼史诗: ${e.message}`);
    }
  };

  const handleOpenTrace = async () => {
    try {
      const data = await fetchTrace();
      setTrace(data);
      setIsTraceOpen(true);
    } catch (e: any) {
      alert(`无法获取追踪: ${e.message}`);
    }
  };

  const handleSelectPartner = (partnerId: string) => {
    const partner = entities.find((e) => e.id === partnerId);
    if (partner) setSelectedEntity(partner);
  };

  return (
    <div className="min-h-screen bg-[#07090e] text-slate-100 flex flex-col font-sans">
      <Header
        status={status}
        onOpenMythos={handleOpenMythos}
        onOpenTrace={handleOpenTrace}
      />

      {/* 宏观天道气象横幅 */}
      <div className="bg-gradient-to-r from-sky-950/40 via-purple-950/30 to-emerald-950/40 border-b border-slate-800 px-6 py-2 flex items-center gap-3 text-xs">
        <span className="text-sky-400 font-semibold shrink-0">🌌 宏观天道气象 / Cosmic Atmosphere:</span>
        <span className="text-slate-300 italic truncate">
          {status?.cosmic_atmosphere || '正在感知虚空因果……'}
        </span>
      </div>

      {/* 主界面网格 */}
      <main className="flex-1 grid grid-cols-1 lg:grid-cols-12 gap-4 p-4 lg:p-6 max-w-[1920px] mx-auto w-full">
        {/* 左侧 & 中间区域 (8 列) */}
        <div className="lg:col-span-8 flex flex-col gap-4">
          {/* 3D 灵元星象图渲染引擎 */}
          <div className="h-[460px] w-full">
            <CosmicRenderer
              entities={entities}
              selectedEntityId={selectedEntity?.id || null}
              onSelectEntity={setSelectedEntity}
              cosmicAtmosphere={status?.cosmic_atmosphere || ''}
            />
          </div>

          {/* 下半部分：操作台与实体检视器 */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <CausalityDeck
              entities={entities}
              selectedEntity={selectedEntity}
              onRefresh={refreshWorld}
            />
            <EntityInspector
              entity={selectedEntity}
              onSelectPartner={handleSelectPartner}
            />
          </div>
        </div>

        {/* 右侧：实时因果编年史流 (4 列) */}
        <div className="lg:col-span-4 h-full min-h-[600px]">
          <ChronicleStream events={status?.recent_chronicle || []} />
        </div>
      </main>

      {/* 神话史诗弹窗 */}
      <MythosModal
        mythos={mythos}
        isOpen={isMythosOpen}
        onClose={() => setIsMythosOpen(false)}
      />

      {/* 因果链路追踪抽屉 */}
      <TraceDrawer
        trace={trace}
        isOpen={isTraceOpen}
        onClose={() => setIsTraceOpen(false)}
      />
    </div>
  );
};
