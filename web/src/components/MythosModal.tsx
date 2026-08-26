import React from 'react';
import { MythosChapter } from '../types';
import { Sparkles, X } from 'lucide-react';

interface MythosModalProps {
  mythos: MythosChapter | null;
  isOpen: boolean;
  onClose: () => void;
}

export const MythosModal: React.FC<MythosModalProps> = ({ mythos, isOpen, onClose }) => {
  if (!isOpen || !mythos) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm p-4">
      <div className="bg-slate-900 border border-purple-800 rounded-2xl max-w-xl w-full p-6 shadow-2xl shadow-purple-950/60 relative animate-in fade-in zoom-in-95 duration-200">
        <button
          onClick={onClose}
          className="absolute top-4 right-4 text-slate-400 hover:text-white p-1 rounded-lg hover:bg-slate-800 transition-colors"
        >
          <X className="w-5 h-5" />
        </button>

        <div className="flex items-center gap-2 text-purple-400 text-sm font-semibold mb-3">
          <Sparkles className="w-5 h-5" />
          <span>《原初》纪元史诗篇章 / Epoch Mythos Chapter</span>
        </div>

        <div className="bg-gradient-to-r from-purple-950/40 to-slate-950/80 border border-purple-900/60 rounded-xl p-5 mb-4">
          <h3 className="text-lg font-bold text-purple-200 mb-1">{mythos.title}</h3>
          <div className="text-xs text-purple-400/80 font-mono mb-4">
            宇宙基调 / Tone: {mythos.world_tone} · 纪元跨度: Tick {mythos.epoch_range[0]} ~ {mythos.epoch_range[1]}
          </div>

          <div className="text-sm text-slate-100 italic leading-relaxed whitespace-pre-line border-l-2 border-purple-500 pl-4">
            {mythos.poetic_epic}
          </div>
        </div>

        <div className="flex justify-end">
          <button
            onClick={onClose}
            className="bg-purple-700 hover:bg-purple-600 text-white text-xs px-4 py-2 rounded-lg font-medium transition-colors"
          >
            知悉神话 / Acknowledge
          </button>
        </div>
      </div>
    </div>
  );
};
