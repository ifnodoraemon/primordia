import { Entity, MythosChapter, TraceData, WorldStatus } from './types';

const API_BASE = '/api';

export async function fetchWorldStatus(): Promise<WorldStatus> {
  const res = await fetch(`${API_BASE}/world/status`);
  if (!res.ok) throw new Error(`Failed to fetch status: ${res.statusText}`);
  return res.json();
}

export async function fetchEntities(): Promise<Entity[]> {
  const res = await fetch(`${API_BASE}/entities`);
  if (!res.ok) throw new Error(`Failed to fetch entities: ${res.statusText}`);
  return res.json();
}

export async function fetchEntityDetail(id: string): Promise<Entity> {
  const res = await fetch(`${API_BASE}/entities/${encodeURIComponent(id)}`);
  if (!res.ok) throw new Error(`Failed to fetch entity ${id}: ${res.statusText}`);
  return res.json();
}

export async function triggerInhabitation(entityId: string, intent: string): Promise<any> {
  const res = await fetch(`${API_BASE}/inhabit`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ entity_id: entityId, intent }),
  });
  if (!res.ok) throw new Error(`Inhabitation failed: ${await res.text()}`);
  return res.json();
}

export async function triggerAutonomousAct(entityId: string): Promise<any> {
  const res = await fetch(`${API_BASE}/act`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ entity_id: entityId }),
  });
  if (!res.ok) throw new Error(`Act failed: ${await res.text()}`);
  return res.json();
}

export async function triggerCollision(entityA: string, entityB: string): Promise<any> {
  const res = await fetch(`${API_BASE}/collide`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ entity_a: entityA, entity_b: entityB }),
  });
  if (!res.ok) throw new Error(`Collision failed: ${await res.text()}`);
  return res.json();
}

export async function triggerDomainResonance(domainName: string): Promise<any> {
  const res = await fetch(`${API_BASE}/resonate`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ domain_name: domainName }),
  });
  if (!res.ok) throw new Error(`Resonance failed: ${await res.text()}`);
  return res.json();
}

export async function triggerTick(count: number = 1): Promise<any> {
  const res = await fetch(`${API_BASE}/tick`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ count }),
  });
  if (!res.ok) throw new Error(`Tick failed: ${await res.text()}`);
  return res.json();
}

export async function triggerShiftLaw(): Promise<{ new_atmosphere: string }> {
  const res = await fetch(`${API_BASE}/shift_law`, { method: 'POST' });
  if (!res.ok) throw new Error(`Shift law failed: ${await res.text()}`);
  return res.json();
}

export async function fetchMythos(): Promise<MythosChapter> {
  const res = await fetch(`${API_BASE}/mythos`);
  if (!res.ok) throw new Error(`Fetch mythos failed: ${res.statusText}`);
  return res.json();
}

export async function fetchTrace(): Promise<TraceData> {
  const res = await fetch(`${API_BASE}/trace`);
  if (!res.ok) throw new Error(`Fetch trace failed: ${res.statusText}`);
  return res.json();
}
