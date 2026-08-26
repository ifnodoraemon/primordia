import React from 'react';
import { WorldStatus } from '../types';
import { Sparkles, Activity, Globe, BookOpen, Layers } from 'lucide-react';

interface HeaderProps {
  status: WorldStatus | null;
  onOpenMythos: () => void;
  onOpenTrace: () => void;
}

export const Header: React.FC<HeaderProps> = ({ status, onOpenMythos, onOpenTrace }) => {
  return (
    <header className="bg-slate-900/90 backdrop-blur-md border-b border-slate-800 px-6 py-3 flex flex-wrap justify-between items-center sticky top-0 z-50">
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

      <div className="flex items-center gap-4 text-xs">
        <div className="flex items-center gap-1.5 bg-slate-950 border border-slate-800 px-3 py-1.5 rounded-full">
          <Activity className="w-3.5 h-3.5 text-emerald-400" />
          <span className="text-slate-400">纪元 / Epoch:</span>
          <strong className="text-emerald-400 font-mono text-sm">{status?.tick_count ?? 0}</strong>
        </div>

        <div className="flex items-center gap-1.5 bg-slate-950 border border-slate-800 px-3 py-1.5 rounded-full">
          <Globe className="w-3.5 h-3.5 text-sky-400" />
          <span className="text-slate-400">灵元 / Entities:</span>
          <strong className="text-sky-400 font-mono text-sm">{status?.total_entities ?? 0}</strong>
        </div>

        <div className="flex items-center gap-1.5 bg-slate-950 border border-slate-800 px-3 py-1.5 rounded-full">
          <BookOpen className="w-3.5 h-3.5 text-amber-400" />
          <span className="text-slate-400">编年史 / Events:</span>
          <strong className="text-amber-400 font-mono text-sm">{status?.chronicle_count ?? 0}</strong>
        </div>

        <button
          onClick={onOpenMythos}
          className="flex items-center gap-1.5 bg-purple-950/60 hover:bg-purple-900/80 text-purple-300 border border-purple-700/60 px-3 py-1.5 rounded-lg font-medium transition-all shadow-sm shadow-purple-900/30 cursor-pointer"
        >
          <Sparkles className="w-3.5 h-3.5 text-purple-400" />
          <span>纪元史诗 / Mythos</span>
        </button>

        <button
          onClick={onOpenTrace}
          className="flex items-center gap-1.5 bg-slate-800 hover:bg-slate-700 text-slate-300 border border-slate-600 px-3 py-1.5 rounded-lg font-medium transition-all cursor-pointer"
        >
          <Layers className="w-3.5 h-3.5 text-slate-400" />
          <span>因果追踪 / Trace</span>
        </button>
      </div>
    </header>
  );
};
