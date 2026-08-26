import React, { useState } from 'react';
import { Entity } from '../types';
import {
  triggerTick,
  triggerShiftLaw,
  triggerDomainResonance,
  triggerCollision,
  triggerInhabitation,
  triggerAutonomousAct,
  triggerGenesisEntity,
  triggerDialogue,
} from '../api';
import { Zap, Compass, Flame, Waves, Sparkles, Send, Bot, PlusCircle, X, MessagesSquare } from 'lucide-react';

interface CausalityDeckProps {
  entities: Entity[];
  selectedEntity: Entity | null;
  onRefresh: () => void;
}

export const CausalityDeck: React.FC<CausalityDeckProps> = ({
  entities,
  selectedEntity,
  onRefresh,
}) => {
  const [intent, setIntent] = useState('');
  const [loadingAction, setLoadingAction] = useState<string | null>(null);

  // 化生新实体表单状态
  const [isGenesisOpen, setIsGenesisOpen] = useState(false);
  const [newName, setNewName] = useState('');
  const [newEssence, setNewEssence] = useState('');
  const [newTraits, setNewTraits] = useState('');
  const [newState, setNewState] = useState('');
  const [newDomain, setNewDomain] = useState('悬天绝壁 / Celestial Cliff Precipice');

  const handleAction = async (name: string, fn: () => Promise<any>) => {
    setLoadingAction(name);
    try {
      await fn();
      onRefresh();
    } catch (err: any) {
      alert(`执行失败: ${err.message}`);
    } finally {
      setLoadingAction(null);
    }
  };

  const handleInhabitSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!selectedEntity) {
      alert('请先在 3D 视图或列表中点击选中要寄宿的灵元实体！');
      return;
    }
    if (!intent.trim()) {
      alert('请输入您的自由意志意图！');
      return;
    }

    await handleAction('inhabit', () => triggerInhabitation(selectedEntity.id, intent.trim()));
    setIntent('');
  };

  const handleAutonomousAct = async () => {
    if (!selectedEntity) {
      alert('请先选中一个实体！');
      return;
    }
    await handleAction('act', () => triggerAutonomousAct(selectedEntity.id));
  };

  const handlePromptResonance = () => {
    const defaultDomain = selectedEntity?.spatial.domain || '悬天绝壁 / Celestial Cliff Precipice';
    const domain = prompt('请输入要激发集体共鸣的拓扑场域名称：', defaultDomain);
    if (domain) {
      handleAction('resonance', () => triggerDomainResonance(domain));
    }
  };

  const handlePromptCollision = () => {
    if (entities.length < 2) {
      alert('宇宙中需要至少两个实体才能触发碰撞相变！');
      return;
    }
    const defaultA = entities[0].id;
    const defaultB = entities[1].id;
    const idA = prompt('请输入实体 A 的 ID：', defaultA);
    const idB = prompt('请输入实体 B 的 ID：', defaultB);
    if (idA && idB) {
      handleAction('collide', () => triggerCollision(idA, idB));
    }
  };

  const handlePromptDialogue = () => {
    if (entities.length < 2) {
      alert('宇宙中需要至少两个实体才能进行客体际神念问答！');
      return;
    }
    const defaultA = selectedEntity ? selectedEntity.id : entities[0].id;
    const defaultB = entities.find((e) => e.id !== defaultA)?.id || entities[1].id;
    const idA = prompt('请输入发话实体 A 的 ID：', defaultA);
    const idB = prompt('请输入听话实体 B 的 ID：', defaultB);
    if (idA && idB) {
      handleAction('dialogue', () => triggerDialogue(idA, idB));
    }
  };

  const handleGenesisSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newName.trim() || !newEssence.trim()) {
      alert('请填写实体名称与本质！');
      return;
    }

    const traitsArr = newTraits.split(/[,，]/).map((t) => t.trim()).filter(Boolean);

    await handleAction('genesis', () =>
      triggerGenesisEntity({
        name: newName.trim(),
        essence: newEssence.trim(),
        traits: traitsArr,
        state: newState.trim() || '初生于虚空之中，静静呼吸 / Formed in void, breathing quietly',
        domain: newDomain.trim(),
      })
    );

    setNewName('');
    setNewEssence('');
    setNewTraits('');
    setNewState('');
    setIsGenesisOpen(false);
  };

  return (
    <div className="bg-slate-900/90 border border-slate-800 rounded-xl p-4 flex flex-col gap-4">
      <div className="flex items-center justify-between pb-2 border-b border-slate-800 text-sm font-semibold text-slate-200">
        <div className="flex items-center gap-2">
          <Zap className="w-4 h-4 text-emerald-400" />
          <span>⚡ 因果操作台 / Causality Deck</span>
        </div>
        <button
          onClick={() => setIsGenesisOpen(!isGenesisOpen)}
          className="flex items-center gap-1 text-xs text-emerald-400 hover:text-emerald-300 bg-emerald-950/60 border border-emerald-800 px-2 py-0.5 rounded cursor-pointer transition-colors"
        >
          <PlusCircle className="w-3.5 h-3.5" />
          <span>化生新灵元 / Genesis</span>
        </button>
      </div>

      {/* 化生新灵元弹窗/内联表单 */}
      {isGenesisOpen && (
        <form onSubmit={handleGenesisSubmit} className="bg-slate-950/90 border border-emerald-900/80 rounded-lg p-3 flex flex-col gap-2 animate-in fade-in duration-150 text-xs">
          <div className="flex justify-between items-center text-emerald-400 font-semibold mb-1">
            <span>🌱 虚空化生新灵元实体 / Spawn Animus Node</span>
            <button type="button" onClick={() => setIsGenesisOpen(false)} className="text-slate-400 hover:text-white">
              <X className="w-4 h-4" />
            </button>
          </div>

          <div className="grid grid-cols-2 gap-2">
            <input
              type="text"
              placeholder="实体名称（如：星海沉光）"
              value={newName}
              onChange={(e) => setNewName(e.target.value)}
              className="bg-slate-900 border border-slate-800 rounded px-2 py-1 text-slate-200 placeholder-slate-500 focus:outline-none focus:border-emerald-500"
              required
            />
            <input
              type="text"
              placeholder="本体本质（如：凝结的星辉微粒）"
              value={newEssence}
              onChange={(e) => setNewEssence(e.target.value)}
              className="bg-slate-900 border border-slate-800 rounded px-2 py-1 text-slate-200 placeholder-slate-500 focus:outline-none focus:border-emerald-500"
              required
            />
          </div>

          <input
            type="text"
            placeholder="本征特征（逗号分隔，如：致密, 发光, 微凉）"
            value={newTraits}
            onChange={(e) => setNewTraits(e.target.value)}
            className="bg-slate-900 border border-slate-800 rounded px-2 py-1 text-slate-200 placeholder-slate-500 focus:outline-none focus:border-emerald-500"
          />

          <div className="grid grid-cols-2 gap-2">
            <input
              type="text"
              placeholder="初始感官状态（如：静静悬浮）"
              value={newState}
              onChange={(e) => setNewState(e.target.value)}
              className="bg-slate-900 border border-slate-800 rounded px-2 py-1 text-slate-200 placeholder-slate-500 focus:outline-none focus:border-emerald-500"
            />
            <input
              type="text"
              placeholder="拓扑场域（如：太虚星海）"
              value={newDomain}
              onChange={(e) => setNewDomain(e.target.value)}
              className="bg-slate-900 border border-slate-800 rounded px-2 py-1 text-slate-200 placeholder-slate-500 focus:outline-none focus:border-emerald-500"
            />
          </div>

          <button
            type="submit"
            disabled={loadingAction !== null}
            className="bg-emerald-600 hover:bg-emerald-500 text-white font-medium py-1.5 rounded text-xs transition-colors cursor-pointer mt-1"
          >
            {loadingAction === 'genesis' ? '正在虚空化生中……' : '✨ 凝结并化生入宇宙 / Manifest Entity'}
          </button>
        </form>
      )}

      {/* 宏观与场域级因果按钮 */}
      <div className="grid grid-cols-2 gap-2">
        <button
          onClick={() => handleAction('tick', () => triggerTick(1))}
          disabled={loadingAction !== null}
          className="col-span-2 bg-gradient-to-r from-emerald-600 to-teal-600 hover:from-emerald-500 hover:to-teal-500 text-white font-medium py-2 px-3 rounded-lg text-xs flex items-center justify-center gap-2 transition-all shadow-md shadow-emerald-950/50 cursor-pointer disabled:opacity-50"
        >
          <Zap className="w-3.5 h-3.5" />
          <span>{loadingAction === 'tick' ? '推演纪元演化中……' : '推进世界纪元 / Advance Epoch'}</span>
        </button>

        <button
          onClick={() => handleAction('shift_law', triggerShiftLaw)}
          disabled={loadingAction !== null}
          className="bg-slate-800 hover:bg-slate-700 text-slate-200 py-2 px-3 rounded-lg text-xs flex items-center justify-center gap-1.5 border border-slate-700 transition-all cursor-pointer disabled:opacity-50"
        >
          <Compass className="w-3.5 h-3.5 text-sky-400" />
          <span>天道相变 / Shift Law</span>
        </button>

        <button
          onClick={handlePromptResonance}
          disabled={loadingAction !== null}
          className="bg-slate-800 hover:bg-slate-700 text-slate-200 py-2 px-3 rounded-lg text-xs flex items-center justify-center gap-1.5 border border-slate-700 transition-all cursor-pointer disabled:opacity-50"
        >
          <Waves className="w-3.5 h-3.5 text-cyan-400" />
          <span>场域共鸣 / Resonate</span>
        </button>

        <button
          onClick={handlePromptCollision}
          disabled={loadingAction !== null}
          className="bg-slate-800 hover:bg-slate-700 text-slate-200 py-2 px-3 rounded-lg text-xs flex items-center justify-center gap-1.5 border border-slate-700 transition-all cursor-pointer disabled:opacity-50"
        >
          <Flame className="w-3.5 h-3.5 text-amber-400" />
          <span>碰撞相变 / Collide</span>
        </button>

        <button
          onClick={handleAutonomousAct}
          disabled={loadingAction !== null || !selectedEntity}
          className="bg-slate-800 hover:bg-slate-700 text-slate-200 py-2 px-3 rounded-lg text-xs flex items-center justify-center gap-1.5 border border-slate-700 transition-all cursor-pointer disabled:opacity-50"
        >
          <Bot className="w-3.5 h-3.5 text-purple-400" />
          <span>自主心智 / Auto Act</span>
        </button>

        <button
          onClick={handlePromptDialogue}
          disabled={loadingAction !== null || entities.length < 2}
          className="bg-slate-800 hover:bg-slate-700 text-slate-200 py-2 px-3 rounded-lg text-xs flex items-center justify-center gap-1.5 border border-slate-700 transition-all cursor-pointer disabled:opacity-50"
        >
          <MessagesSquare className="w-3.5 h-3.5 text-cyan-400" />
          <span>神念交织 / Dialogue</span>
        </button>
      </div>

      {/* 自由觉知寄宿中枢 */}
      <form onSubmit={handleInhabitSubmit} className="bg-slate-950 border border-slate-800/80 rounded-lg p-3 flex flex-col gap-2">
        <div className="flex justify-between items-center text-xs">
          <span className="font-semibold text-slate-300 flex items-center gap-1.5">
            <Sparkles className="w-3.5 h-3.5 text-purple-400" />
            <span>觉知寄宿与意志注入</span>
          </span>
          {selectedEntity ? (
            <span className="text-purple-400 font-mono bg-purple-950/60 border border-purple-800/50 px-2 py-0.5 rounded text-[11px]">
              已锚定: {selectedEntity.name}
            </span>
          ) : (
            <span className="text-slate-500 text-[11px]">请在 3D 视图选中实体</span>
          )}
        </div>

        <textarea
          value={intent}
          onChange={(e) => setIntent(e.target.value)}
          placeholder={
            selectedEntity
              ? `以自然语言输入您作为【${selectedEntity.name}】的自由意图（如：聚拢星辰微光向外发出温润引力）……`
              : '请先在 3D 视图或下方列表中选中一个灵元实体以寄宿自由觉知……'
          }
          disabled={!selectedEntity || loadingAction !== null}
          rows={3}
          className="w-full bg-slate-900/80 border border-slate-800 rounded-md p-2 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-purple-500 transition-colors resize-none disabled:opacity-50"
        />

        <button
          type="submit"
          disabled={!selectedEntity || loadingAction !== null || !intent.trim()}
          className="bg-purple-600 hover:bg-purple-500 disabled:opacity-40 text-white font-medium py-1.5 px-3 rounded text-xs flex items-center justify-center gap-1.5 transition-all cursor-pointer"
        >
          <Send className="w-3 h-3" />
          <span>{loadingAction === 'inhabit' ? '意识投影中……' : '注入意志驱动实体 / Ground Agency'}</span>
        </button>
      </form>
    </div>
  );
};
