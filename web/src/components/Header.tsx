import React, { useRef } from 'react';
import { WorldStatus } from '../types';
import {
  Sparkles,
  Activity,
  Globe,
  Layers,
  Play,
  Pause,
  Download,
  Upload,
  RotateCcw,
  Cpu,
} from 'lucide-react';

interface HeaderProps {
  status: WorldStatus | null;
  isHeartbeatRunning: boolean;
  onToggleHeartbeat: () => void;
  onOpenMythos: () => void;
  onOpenTrace: () => void;
  onOpenHarness: () => void;
  onExportSnapshot: () => void;
  onRestoreSnapshot: (json: string) => void;
  onResetWorld: () => void;
}

export const Header: React.FC<HeaderProps> = ({
  status,
  isHeartbeatRunning,
  onToggleHeartbeat,
  onOpenMythos,
  onOpenTrace,
  onOpenHarness,
  onExportSnapshot,
  onRestoreSnapshot,
  onResetWorld,
}) => {
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = (event) => {
      const content = event.target?.result as string;
      if (content) {
        onRestoreSnapshot(content);
      }
    };
    reader.readAsText(file);
    if (fileInputRef.current) fileInputRef.current.value = '';
  };

  return (
    <header className="bg-slate-900/90 backdrop-blur-md border-b border-slate-800 px-6 py-3 flex flex-wrap justify-between items-center sticky top-0 z-50 gap-2">
      <div className="flex items-center gap-3">
        <span className="text-2xl filter drop-shadow-[0_0_8px_rgba(74,222,128,0.6)]">🌌</span>
        <div>
          <h1 className="text-lg font-bold bg-gradient-to-r from-white via-sky-200 to-emerald-400 bg-clip-text text-transparent">
            《原初》（Primordia: Meta）
          </h1>
        </div>
        <span className="text-xs text-slate-400 border border-slate-700 px-2 py-0.5 rounded-full font-mono">
          LLM-Native · Three.js 3D
        </span>
      </div>

      <div className="flex items-center gap-2.5 text-xs flex-wrap">
        {/* 宇宙自演化自治心跳开关 */}
        <button
          onClick={onToggleHeartbeat}
          className={`flex items-center gap-1.5 px-3 py-1.5 rounded-full font-medium transition-all border cursor-pointer ${
            isHeartbeatRunning
              ? 'bg-emerald-950/80 text-emerald-300 border-emerald-500 shadow-sm shadow-emerald-900/50'
              : 'bg-slate-950 text-slate-400 border-slate-800 hover:border-slate-700'
          }`}
        >
          {isHeartbeatRunning ? (
            <>
              <span className="w-2 h-2 rounded-full bg-emerald-400 animate-ping" />
              <Pause className="w-3.5 h-3.5 text-emerald-400" />
              <span>宇宙心跳 (5s)</span>
            </>
          ) : (
            <>
              <Play className="w-3.5 h-3.5 text-slate-400" />
              <span>启动自演化</span>
            </>
          )}
        </button>

        <div className="flex items-center gap-1.5 bg-slate-950 border border-slate-800 px-2.5 py-1 rounded-full">
          <Activity className="w-3.5 h-3.5 text-emerald-400" />
          <span className="text-slate-400">纪元:</span>
          <strong className="text-emerald-400 font-mono">{status?.tick_count ?? 0}</strong>
        </div>

        <div className="flex items-center gap-1.5 bg-slate-950 border border-slate-800 px-2.5 py-1 rounded-full">
          <Globe className="w-3.5 h-3.5 text-sky-400" />
          <span className="text-slate-400">灵元:</span>
          <strong className="text-sky-400 font-mono">{status?.total_entities ?? 0}</strong>
        </div>

        {/* 仿真与断言驾驭台 */}
        <button
          onClick={onOpenHarness}
          className="flex items-center gap-1.5 bg-sky-950/70 hover:bg-sky-900/80 text-sky-300 border border-sky-700/60 px-2.5 py-1 rounded-lg font-medium transition-all shadow-sm shadow-sky-900/30 cursor-pointer"
        >
          <Cpu className="w-3.5 h-3.5 text-sky-400" />
          <span>仿真驾驭台</span>
        </button>

        <button
          onClick={onOpenMythos}
          className="flex items-center gap-1.5 bg-purple-950/60 hover:bg-purple-900/80 text-purple-300 border border-purple-700/60 px-2.5 py-1 rounded-lg font-medium transition-all shadow-sm shadow-purple-900/30 cursor-pointer"
        >
          <Sparkles className="w-3.5 h-3.5 text-purple-400" />
          <span>史诗</span>
        </button>

        <button
          onClick={onOpenTrace}
          className="flex items-center gap-1.5 bg-slate-800 hover:bg-slate-700 text-slate-300 border border-slate-600 px-2.5 py-1 rounded-lg font-medium transition-all cursor-pointer"
        >
          <Layers className="w-3.5 h-3.5 text-slate-400" />
          <span>因果追踪</span>
        </button>

        {/* 快照导出与载入 */}
        <button
          onClick={onExportSnapshot}
          title="导出当前宇宙全景快照 JSON"
          className="flex items-center gap-1 bg-slate-950 hover:bg-slate-800 text-slate-300 border border-slate-800 px-2.5 py-1 rounded-lg transition-colors cursor-pointer"
        >
          <Download className="w-3 h-3 text-sky-400" />
          <span>快照</span>
        </button>

        <label
          title="载入平行宇宙快照 JSON"
          className="flex items-center gap-1 bg-slate-950 hover:bg-slate-800 text-slate-300 border border-slate-800 px-2.5 py-1 rounded-lg transition-colors cursor-pointer"
        >
          <Upload className="w-3 h-3 text-emerald-400" />
          <span>载入</span>
          <input
            ref={fileInputRef}
            type="file"
            accept=".json"
            onChange={handleFileUpload}
            className="hidden"
          />
        </label>

        {/* 重置世界 */}
        <button
          onClick={() => {
            if (confirm('确认重置宇宙回归虚空鸿蒙初辟之态？当前所有演化记录将被清空。')) {
              onResetWorld();
            }
          }}
          title="重置宇宙回归虚空鸿蒙"
          className="flex items-center gap-1 bg-rose-950/40 hover:bg-rose-900/60 text-rose-300 border border-rose-900/50 px-2 py-1 rounded-lg transition-colors cursor-pointer"
        >
          <RotateCcw className="w-3 h-3 text-rose-400" />
          <span>重置</span>
        </button>
      </div>
    </header>
  );
};
