import React, { useState } from 'react';
import { Entity } from '../types';
import { triggerCommunion } from '../api';
import { Shield, MapPin, Eye, Sparkles, Network, MessageSquare, Send } from 'lucide-react';

interface EntityInspectorProps {
  entity: Entity | null;
  onSelectPartner?: (partnerId: string) => void;
  onRefresh?: () => void;
}

export const EntityInspector: React.FC<EntityInspectorProps> = ({
  entity,
  onSelectPartner,
  onRefresh,
}) => {
  const [communeQuery, setCommuneQuery] = useState('');
  const [communeLoading, setCommuneLoading] = useState(false);
  const [lastCommuneReply, setLastCommuneReply] = useState<{ response: string; inner: string } | null>(null);

  if (!entity) {
    return (
      <div className="bg-slate-900/90 border border-slate-800 rounded-xl p-4 flex flex-col items-center justify-center text-center text-slate-500 h-64 text-xs">
        <Sparkles className="w-8 h-8 text-slate-700 mb-2 animate-pulse" />
        <p>在 3D 宇宙图谱或卡片列表中点击灵元实体</p>
        <p className="mt-1 text-slate-600">以深入剖析其面向对象本体论（OOO）退隐内核、神念对话与历史记忆</p>
      </div>
    );
  }

  const handleCommune = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!communeQuery.trim()) return;

    setCommuneLoading(true);
    try {
      const res = await triggerCommunion(entity.id, communeQuery.trim());
      setLastCommuneReply({ response: res.entity_response, inner: res.inner_resonance });
      setCommuneQuery('');
      if (onRefresh) onRefresh();
    } catch (err: any) {
      alert(`神念倾听失败: ${err.message}`);
    } finally {
      setCommuneLoading(false);
    }
  };

  let phaseColor = 'bg-emerald-950/60 text-emerald-300 border-emerald-800';
  const phase = (entity.lifecycle || '').toLowerCase();
  if (phase.includes('genesis') || phase.includes('成')) {
    phaseColor = 'bg-sky-950/60 text-sky-300 border-sky-800';
  } else if (phase.includes('decay') || phase.includes('坏')) {
    phaseColor = 'bg-amber-950/60 text-amber-300 border-amber-800';
  } else if (phase.includes('dissol') || phase.includes('空')) {
    phaseColor = 'bg-rose-950/60 text-rose-300 border-rose-800';
  }

  return (
    <div className="bg-slate-900/90 border border-slate-800 rounded-xl p-4 flex flex-col gap-3 text-xs overflow-y-auto max-h-[460px]">
      <div className="flex justify-between items-start pb-2 border-b border-slate-800">
        <div>
          <h2 className="text-sm font-bold text-slate-100 flex items-center gap-2">
            <span>{entity.name}</span>
            <span className="text-[10px] font-mono text-slate-400 bg-slate-800 px-1.5 py-0.5 rounded">
              {entity.id}
            </span>
            {entity.active_inhabitants && entity.active_inhabitants.length > 0 && (
              <span className="text-[10px] text-purple-400 bg-purple-950/80 border border-purple-800 px-1.5 py-0.5 rounded-full animate-pulse">
                ✨ 已觉醒
              </span>
            )}
          </h2>
          <p className="text-[11px] text-slate-400 mt-0.5 italic">{entity.essence}</p>
        </div>
        <div className={`border px-2 py-0.5 rounded-full font-mono text-[10px] ${phaseColor}`}>
          {entity.lifecycle} (凝聚度: {(entity.cohesion || 1.0).toFixed(2)})
        </div>
      </div>

      {/* 拓扑场域与共鸣 */}
      <div className="flex items-center gap-2 text-slate-300 bg-slate-950/80 p-2 rounded-lg border border-slate-800">
        <MapPin className="w-3.5 h-3.5 text-purple-400 shrink-0" />
        <div>
          <span className="text-slate-400">场域: </span>
          <span className="text-purple-300 font-medium">{entity.spatial.domain}</span>
          <span className="text-slate-500 ml-2 text-[10px] font-mono">[{entity.spatial.resonance_field}]</span>
        </div>
      </div>

      {/* OOO 退隐内核 */}
      <div className="bg-slate-950/90 border border-purple-950 p-2.5 rounded-lg flex flex-col gap-1">
        <div className="text-[11px] font-semibold text-purple-400 flex items-center gap-1.5">
          <Shield className="w-3 h-3 text-purple-400" />
          <span>退隐内核 / Withdrawn Core (OOO 不可穷尽本质):</span>
        </div>
        <p className="text-[11px] text-purple-200/90 font-mono leading-relaxed bg-purple-950/30 p-1.5 rounded">
          {entity.withdrawn_core}
        </p>
      </div>

      {/* 当前感官状态与特征 */}
      <div className="flex flex-col gap-1.5">
        <div className="text-slate-400 flex items-center gap-1">
          <Eye className="w-3 h-3 text-sky-400" />
          <span>感官表象状态:</span>
        </div>
        <p className="text-slate-200 bg-slate-950/60 p-2 rounded border border-slate-800 leading-relaxed">
          {entity.current_state}
        </p>
      </div>

      {/* 🗣️ 泛心论神念倾听问答 (Panpsychic Communion) */}
      <div className="bg-slate-950/90 border border-cyan-950 p-2.5 rounded-lg flex flex-col gap-2">
        <div className="text-[11px] font-semibold text-cyan-400 flex items-center gap-1.5">
          <MessageSquare className="w-3 h-3 text-cyan-400" />
          <span>神念倾听与问答 / Panpsychic Communion:</span>
        </div>

        {lastCommuneReply && (
          <div className="bg-cyan-950/30 border-l-2 border-cyan-500 p-2 rounded text-[11px] text-cyan-200 italic leading-relaxed">
            "{lastCommuneReply.response}"
            <div className="text-[10px] text-cyan-400/70 not-italic mt-1 font-mono">
              [内在共鸣: {lastCommuneReply.inner}]
            </div>
          </div>
        )}

        <form onSubmit={handleCommune} className="flex gap-1.5">
          <input
            type="text"
            value={communeQuery}
            onChange={(e) => setCommuneQuery(e.target.value)}
            placeholder={`以神念询问【${entity.name}】（例如：你在想什么？/ 感受到了什么？）`}
            disabled={communeLoading}
            className="flex-1 bg-slate-900 border border-slate-800 rounded px-2.5 py-1 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-cyan-500"
          />
          <button
            type="submit"
            disabled={communeLoading || !communeQuery.trim()}
            className="bg-cyan-700 hover:bg-cyan-600 disabled:opacity-40 text-white px-2.5 py-1 rounded text-xs flex items-center gap-1 cursor-pointer"
          >
            <Send className="w-3 h-3" />
            <span>{communeLoading ? '倾听中…' : '传念'}</span>
          </button>
        </form>
      </div>

      {/* 特征列表 */}
      <div>
        <span className="text-slate-400 text-[11px]">本征特征 / Traits:</span>
        <div className="flex flex-wrap gap-1.5 mt-1">
          {(entity.traits || []).map((t, idx) => (
            <span key={idx} className="bg-slate-800 border border-slate-700 text-slate-300 px-2 py-0.5 rounded text-[10px]">
              {t}
            </span>
          ))}
        </div>
      </div>

      {/* 德勒兹共生装配体伙伴 */}
      {entity.assemblages && entity.assemblages.length > 0 && (
        <div>
          <span className="text-cyan-400 text-[11px] flex items-center gap-1">
            <Network className="w-3 h-3" />
            <span>共生装配体关联 / Assemblages:</span>
          </span>
          <div className="flex flex-wrap gap-1.5 mt-1">
            {entity.assemblages.map((id) => (
              <button
                key={id}
                onClick={() => onSelectPartner && onSelectPartner(id)}
                className="bg-cyan-950/60 hover:bg-cyan-900/80 border border-cyan-800 text-cyan-300 px-2 py-0.5 rounded text-[10px] font-mono cursor-pointer transition-colors"
              >
                🔗 {id}
              </button>
            ))}
          </div>
        </div>
      )}

      {/* 历史记忆流 */}
      <div className="flex flex-col gap-1 mt-1">
        <span className="text-slate-400 text-[11px]">历史记忆流 / Memory Stream ({entity.memory_stream?.length || 0} 条):</span>
        <div className="max-h-24 overflow-y-auto space-y-1 bg-slate-950/90 p-2 rounded border border-slate-800 text-[10px] font-mono text-slate-400">
          {(entity.memory_stream || []).slice().reverse().map((mem, idx) => (
            <div key={idx} className="border-b border-slate-900 pb-0.5 last:border-none">
              • {mem}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
