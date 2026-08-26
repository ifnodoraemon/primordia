export interface Spatial {
  x: number;
  y: number;
  z: number;
  scale: number;
  mobility: string;
  domain: string;
  resonance_field: string;
}

export type LifecyclePhase = 'Genesis' | 'Flourishing' | 'Decay' | 'Dissolution' | string;

export interface Entity {
  id: string;
  name: string;
  essence: string;
  withdrawn_core: string;
  spatial: Spatial;
  traits: string[];
  current_state: string;
  lifecycle: LifecyclePhase;
  cohesion: number;
  memory_stream: string[];
  assemblages: string[];
  active_inhabitants: string[];
  born_at_tick: number;
}

export interface ChronicleEvent {
  tick: number;
  event_type: string;
  detail: string;
  timestamp: number;
}

export interface WorldStatus {
  name: string;
  tick_count: number;
  cosmic_atmosphere: string;
  total_entities: number;
  chronicle_count: number;
  recent_chronicle: ChronicleEvent[];
}

export interface MythosChapter {
  epoch_range: [number, number];
  title: string;
  poetic_epic: string;
  world_tone: string;
}

export interface CausalSpan {
  span_id: string;
  tick: number;
  operator: string;
  target_entities: string[];
  system_prompt: string;
  user_prompt: string;
  llm_response: any;
  mutations_summary: string;
  duration_ms: number;
}

export interface TraceData {
  summary: string;
  spans: CausalSpan[];
}
