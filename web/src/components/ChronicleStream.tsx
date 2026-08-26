import React from 'react';
import { ChronicleEvent } from '../types';
import { BookOpen } from 'lucide-react';

interface ChronicleStreamProps {
  events: ChronicleEvent[];
}

export const ChronicleStream: React.FC<ChronicleStreamProps> = ({ events }) => {
  return (
    <div className="bg-slate-900/90 border border-slate-800 rounded-xl p-4 flex flex-col gap-3 h-full">
      <div className="flex items-center justify-between pb-2 border-b border-slate-800 text-sm font-semibold text-slate-200">
        <div className="flex items-center gap-2">
          <BookOpen className="w-4 h-4 text-amber-400" />
          <span>宇宙编年史流 / World Chronicle</span>
        </div>
        <span className="text-xs text-slate-500 font-mono">Total: {events.length}</span>
      </div>

      <div className="flex-1 overflow-y-auto space-y-2.5 pr-1 max-h-[520px]">
        {events.length === 0 ? (
          <div className="text-center text-slate-500 text-xs py-8">宇宙初始，暂无因果记录……</div>
        ) : (
          events.map((e, idx) => {
            let borderCol = 'border-l-emerald-500';
            if (e.event_type === 'MIND_INHABITATION') borderCol = 'border-l-purple-500';
            else if (e.event_type === 'COLLISION_MORPHOGENESIS') borderCol = 'border-l-amber-500';
            else if (e.event_type === 'DOMAIN_RESONANCE') borderCol = 'border-l-sky-500';
            else if (e.event_type === 'ENTITY_DISSOLUTION') borderCol = 'border-l-rose-500';
            else if (e.event_type === 'COSMIC_LAW_SHIFT') borderCol = 'border-l-indigo-500';

            return (
              <div
                key={idx}
                className={`bg-slate-950/80 border-l-4 ${borderCol} border border-slate-800/80 rounded-r-lg p-2.5 text-xs transition-all hover:bg-slate-900`}
              >
                <div className="flex justify-between items-center text-[10px] text-slate-400 mb-1 font-mono">
                  <span className="text-slate-300 font-semibold">
                    [Epoch {e.tick}] &lt;{e.event_type}&gt;
                  </span>
                  <span>{new Date(e.timestamp * 1000).toLocaleTimeString()}</span>
                </div>
                <div className="text-slate-200 leading-relaxed text-[11px] break-words">
                  {e.detail}
                </div>
              </div>
            );
          })
        )}
      </div>
    </div>
  );
};
