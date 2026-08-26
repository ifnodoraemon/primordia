import React, { useState, useEffect } from 'react';
import { fetchHarnessScenarios, runHarnessScenario } from '../api';
import { Play, CheckCircle2, XCircle, Terminal, X, RefreshCw, Cpu, Layers } from 'lucide-react';

interface HarnessModalProps {
  isOpen: boolean;
  onClose: () => void;
  onRefreshWorld: () => void;
}

export const HarnessModal: React.FC<HarnessModalProps> = ({ isOpen, onClose, onRefreshWorld }) => {
  const [scenarios, setScenarios] = useState<any[]>([]);
  const [selectedIdx, setSelectedIdx] = useState(0);
  const [isRunning, setIsRunning] = useState(false);
  const [report, setReport] = useState<any | null>(null);

  useEffect(() => {
    if (isOpen) {
      fetchHarnessScenarios()
        .then((data) => setScenarios(data))
        .catch((err) => console.error('Failed to load scenarios:', err));
    }
  }, [isOpen]);

  if (!isOpen) return null;

  const currentScenario = scenarios[selectedIdx];

  const handleRun = async () => {
    if (!currentScenario) return;
    setIsRunning(true);
    setReport(null);
    try {
      const res = await runHarnessScenario(currentScenario);
      setReport(res);
      onRefreshWorld();
    } catch (e: any) {
      alert(`剧本执行失败: ${e.message}`);
    } finally {
      setIsRunning(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/75 backdrop-blur-md p-4">
      <div className="bg-slate-900 border border-sky-800/80 rounded-2xl max-w-3xl w-full p-6 shadow-2xl shadow-sky-950/70 relative animate-in fade-in zoom-in-95 duration-200 flex flex-col max-h-[90vh]">
        {/* Header */}
        <div className="flex justify-between items-center pb-4 border-b border-slate-800">
          <div className="flex items-center gap-2.5">
            <div className="p-2 bg-sky-950/80 border border-sky-700/60 rounded-xl text-sky-400">
              <Cpu className="w-5 h-5" />
            </div>
            <div>
              <h2 className="text-base font-bold text-slate-100 flex items-center gap-2">
                <span>仿真与自动化断言驾驭台</span>
                <span className="text-xs font-normal text-sky-400 font-mono border border-sky-800 px-2 py-0.5 rounded-full">
                  Simulation Harness
                </span>
              </h2>
              <p className="text-xs text-slate-400">
                以剧本（Scenario）与命令模式驱动端到端创世、干涉、相变并执行不可违背断言
              </p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="text-slate-400 hover:text-white p-1.5 rounded-lg hover:bg-slate-800 transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Content */}
        <div className="grid grid-cols-1 md:grid-cols-12 gap-4 my-4 flex-1 overflow-y-auto pr-1">
          {/* 剧本列表 */}
          <div className="md:col-span-5 flex flex-col gap-2">
            <span className="text-xs font-semibold text-slate-400 uppercase tracking-wider">
              基准预设剧本 / Scenarios
            </span>
            <div className="flex flex-col gap-1.5">
              {scenarios.map((sc, idx) => (
                <button
                  key={idx}
                  onClick={() => {
                    setSelectedIdx(idx);
                    setReport(null);
                  }}
                  className={`text-left p-3 rounded-xl border text-xs transition-all cursor-pointer ${
                    selectedIdx === idx
                      ? 'bg-sky-950/80 border-sky-500 text-sky-200 shadow-md shadow-sky-950/50'
                      : 'bg-slate-950/60 border-slate-800 text-slate-400 hover:border-slate-700 hover:text-slate-200'
                  }`}
                >
                  <div className="font-semibold">{sc.name}</div>
                  <div className="text-[11px] text-slate-500 mt-1 line-clamp-2">{sc.description}</div>
                </button>
              ))}
            </div>
          </div>

          {/* 剧本详情与执行报告 */}
          <div className="md:col-span-7 flex flex-col gap-3">
            {currentScenario && (
              <div className="bg-slate-950 border border-slate-800 rounded-xl p-4 flex flex-col gap-3">
                <div className="flex justify-between items-center">
                  <span className="text-xs font-semibold text-slate-300">剧本步骤 ({currentScenario.steps?.length ?? 0} 步)</span>
                  <button
                    onClick={handleRun}
                    disabled={isRunning}
                    className="flex items-center gap-1.5 bg-gradient-to-r from-sky-600 to-emerald-600 hover:from-sky-500 hover:to-emerald-500 text-white text-xs px-4 py-1.5 rounded-lg font-medium shadow-md shadow-sky-950/50 transition-all disabled:opacity-50 cursor-pointer"
                  >
                    {isRunning ? (
                      <>
                        <RefreshCw className="w-3.5 h-3.5 animate-spin" />
                        <span>推演中……</span>
                      </>
                    ) : (
                      <>
                        <Play className="w-3.5 h-3.5" />
                        <span>启动驾驭台推演</span>
                      </>
                    )}
                  </button>
                </div>

                {/* 创世实体一览 */}
                <div className="flex flex-col gap-1.5 text-xs bg-slate-900/60 border border-slate-800/80 rounded-lg p-2.5">
                  <span className="text-slate-400 text-[11px]">创世实体 / Genesis Entities:</span>
                  <div className="flex flex-wrap gap-1.5">
                    {currentScenario.genesis_entities?.map((ent: any, i: number) => (
                      <span key={i} className="bg-sky-950/80 border border-sky-800 text-sky-300 px-2 py-0.5 rounded text-[11px]">
                        【{ent.name}】 ({ent.essence})
                      </span>
                    ))}
                  </div>
                </div>

                {/* 执行报告 */}
                {report && (
                  <div className={`border rounded-xl p-3 text-xs flex flex-col gap-2 ${
                    report.success ? 'bg-emerald-950/30 border-emerald-800/60 text-emerald-200' : 'bg-rose-950/30 border-rose-800/60 text-rose-200'
                  }`}>
                    <div className="flex items-center justify-between font-bold">
                      <div className="flex items-center gap-1.5">
                        {report.success ? (
                          <>
                            <CheckCircle2 className="w-4 h-4 text-emerald-400" />
                            <span>断言全部通过 / ALL ASSERTIONS PASSED</span>
                          </>
                        ) : (
                          <>
                            <XCircle className="w-4 h-4 text-rose-400" />
                            <span>断言检查未通过 / ASSERTIONS FAILED</span>
                          </>
                        )}
                      </div>
                      <span className="font-mono text-[11px] text-slate-400">{report.duration_ms}ms</span>
                    </div>

                    <div className="grid grid-cols-3 gap-2 text-[11px] bg-black/40 rounded-lg p-2 font-mono">
                      <div>执行步数: {report.steps_executed}</div>
                      <div className="text-emerald-400">通过断言: {report.assertions_passed}</div>
                      <div className={report.assertions_failed > 0 ? 'text-rose-400' : 'text-slate-400'}>
                        失败断言: {report.assertions_failed}
                      </div>
                    </div>

                    <div className="text-[11px] text-slate-400 flex items-center gap-1">
                      <Layers className="w-3 h-3 text-sky-400" />
                      <span>{report.trace_summary}</span>
                    </div>
                  </div>
                )}
              </div>
            )}
          </div>
        </div>

        {/* Footer */}
        <div className="flex justify-end pt-3 border-t border-slate-800">
          <button
            onClick={onClose}
            className="bg-slate-800 hover:bg-slate-700 text-slate-300 text-xs px-4 py-2 rounded-lg font-medium transition-colors cursor-pointer"
          >
            完成 / Close
          </button>
        </div>
      </div>
    </div>
  );
};
