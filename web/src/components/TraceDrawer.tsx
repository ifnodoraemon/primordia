import React from 'react';
import { TraceData } from '../types';
import { Layers, X, Clock, Zap } from 'lucide-react';

interface TraceDrawerProps {
  trace: TraceData | null;
  isOpen: boolean;
  onClose: () => void;
}

export const TraceDrawer: React.FC<TraceDrawerProps> = ({ trace, isOpen, onClose }) => {
  if (!isOpen || !trace) return null;

  return (
    <div className="fixed inset-y-0 right-0 z-50 w-full max-w-xl bg-slate-900 border-l border-slate-800 shadow-2xl p-6 flex flex-col gap-4 overflow-hidden animate-in slide-in-from-right duration-200">
      <div className="flex justify-between items-center pb-3 border-b border-slate-800">
        <div className="flex items-center gap-2 text-sm font-semibold text-slate-200">
          <Layers className="w-4 h-4 text-sky-400" />
          <span>全生命周期因果链路追踪 / Causality Tracer</span>
        </div>
        <button
          onClick={onClose}
          className="text-slate-400 hover:text-white p-1 rounded-lg hover:bg-slate-800 transition-colors"
        >
          <X className="w-5 h-5" />
        </button>
      </div>

      <div className="bg-slate-950 p-3 rounded-lg border border-slate-800 text-xs font-mono text-slate-300">
        {trace.summary}
      </div>

      <div className="flex-1 overflow-y-auto space-y-3 pr-1">
        {trace.spans.slice().reverse().map((span) => (
          <div key={span.span_id} className="bg-slate-950/90 border border-slate-800 rounded-lg p-3 text-xs flex flex-col gap-2">
            <div className="flex justify-between items-center text-[11px]">
              <span className="font-mono text-sky-400 font-semibold flex items-center gap-1">
                <Zap className="w-3 h-3" />
                {span.span_id} &lt;{span.operator}&gt;
              </span>
              <span className="text-slate-500 flex items-center gap-1 font-mono text-[10px]">
                <Clock className="w-3 h-3" />
                {span.duration_ms}ms · Tick {span.tick}
              </span>
            </div>

            <div className="text-[11px] text-purple-300 font-mono">
              波及实体 / Targets: {JSON.stringify(span.target_entities)}
            </div>

            <div className="text-slate-300 text-[11px] bg-slate-900/60 p-2 rounded border border-slate-800/80">
              {span.mutations_summary}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
