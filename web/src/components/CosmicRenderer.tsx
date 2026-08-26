import React, { useEffect, useRef } from 'react';
import * as THREE from 'three';
import { Entity } from '../types';

interface CosmicRendererProps {
  entities: Entity[];
  selectedEntityId: string | null;
  onSelectEntity: (entity: Entity) => void;
  cosmicAtmosphere: string;
}

export const CosmicRenderer: React.FC<CosmicRendererProps> = ({
  entities,
  selectedEntityId,
  onSelectEntity,
  cosmicAtmosphere,
}) => {
  const mountRef = useRef<HTMLDivElement>(null);
  const sceneRef = useRef<THREE.Scene | null>(null);
  const rendererRef = useRef<THREE.WebGLRenderer | null>(null);
  const cameraRef = useRef<THREE.PerspectiveCamera | null>(null);
  const entityMeshesRef = useRef<Map<string, THREE.Group>>(new Map());
  const assemblageLinesRef = useRef<THREE.LineSegments | null>(null);
  const isDraggingRef = useRef(false);
  const prevMousePosRef = useRef({ x: 0, y: 0 });
  const rotationRef = useRef({ x: 0.2, y: 0 });

  useEffect(() => {
    if (!mountRef.current) return;

    const width = mountRef.current.clientWidth;
    const height = mountRef.current.clientHeight;

    // 1. Scene & Camera
    const scene = new THREE.Scene();
    scene.fog = new THREE.FogExp2(0x07090e, 0.003);
    sceneRef.current = scene;

    const camera = new THREE.PerspectiveCamera(50, width / height, 0.1, 2000);
    camera.position.set(0, 40, 160);
    camera.lookAt(0, 0, 0);
    cameraRef.current = camera;

    // 2. WebGL Renderer
    const renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
    renderer.setSize(width, height);
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    mountRef.current.appendChild(renderer.domElement);
    rendererRef.current = renderer;

    // 3. Ambient Stardust Particle Field (2000 stars)
    const starGeo = new THREE.BufferGeometry();
    const starCount = 1800;
    const starPositions = new Float32Array(starCount * 3);
    const starColors = new Float32Array(starCount * 3);

    for (let i = 0; i < starCount * 3; i += 3) {
      starPositions[i] = (Math.random() - 0.5) * 800;
      starPositions[i + 1] = (Math.random() - 0.5) * 800;
      starPositions[i + 2] = (Math.random() - 0.5) * 800;

      starColors[i] = 0.3 + Math.random() * 0.7;
      starColors[i + 1] = 0.5 + Math.random() * 0.5;
      starColors[i + 2] = 0.8 + Math.random() * 0.2;
    }

    starGeo.setAttribute('position', new THREE.BufferAttribute(starPositions, 3));
    starGeo.setAttribute('color', new THREE.BufferAttribute(starColors, 3));

    const starMat = new THREE.PointsMaterial({
      size: 1.8,
      vertexColors: true,
      transparent: true,
      opacity: 0.6,
    });
    const starField = new THREE.Points(starGeo, starMat);
    scene.add(starField);

    // 4. Lights
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.8);
    scene.add(ambientLight);

    const pointLight = new THREE.PointLight(0x38bdf8, 2, 400);
    pointLight.position.set(50, 100, 50);
    scene.add(pointLight);

    // 5. Mouse Interaction for 3D Orbiting
    const handleMouseDown = (e: MouseEvent) => {
      isDraggingRef.current = true;
      prevMousePosRef.current = { x: e.clientX, y: e.clientY };
    };

    const handleMouseMove = (e: MouseEvent) => {
      if (!isDraggingRef.current) return;
      const dx = e.clientX - prevMousePosRef.current.x;
      const dy = e.clientY - prevMousePosRef.current.y;

      rotationRef.current.y += dx * 0.005;
      rotationRef.current.x += dy * 0.005;
      rotationRef.current.x = Math.max(-Math.PI / 3, Math.min(Math.PI / 3, rotationRef.current.x));

      prevMousePosRef.current = { x: e.clientX, y: e.clientY };
    };

    const handleMouseUp = () => {
      isDraggingRef.current = false;
    };

    const handleWheel = (e: WheelEvent) => {
      e.preventDefault();
      if (!cameraRef.current) return;
      cameraRef.current.position.z = Math.max(40, Math.min(400, cameraRef.current.position.z + e.deltaY * 0.15));
    };

    const dom = renderer.domElement;
    dom.addEventListener('mousedown', handleMouseDown);
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', handleMouseUp);
    dom.addEventListener('wheel', handleWheel, { passive: false });

    // 6. Raycaster for clicking entities
    const raycaster = new THREE.Raycaster();
    const mouse = new THREE.Vector2();

    const handleClick = (e: MouseEvent) => {
      const rect = dom.getBoundingClientRect();
      mouse.x = ((e.clientX - rect.left) / rect.width) * 2 - 1;
      mouse.y = -((e.clientY - rect.top) / rect.height) * 2 + 1;

      if (!cameraRef.current || !sceneRef.current) return;
      raycaster.setFromCamera(mouse, cameraRef.current);

      const hitList: THREE.Object3D[] = [];
      entityMeshesRef.current.forEach((mesh) => {
        hitList.push(mesh.children[0]); // sphere mesh
      });

      const intersects = raycaster.intersectObjects(hitList);
      if (intersects.length > 0) {
        const hitSphere = intersects[0].object;
        const entId = hitSphere.userData.entityId;
        const ent = entities.find((item) => item.id === entId);
        if (ent) onSelectEntity(ent);
      }
    };
    dom.addEventListener('click', handleClick);

    // 7. Animation Loop
    let animationFrameId: number;
    let clock = new THREE.Clock();

    const animate = () => {
      animationFrameId = requestAnimationFrame(animate);
      const elapsed = clock.getElapsedTime();

      // Gentle auto-rotation when not dragging
      if (!isDraggingRef.current) {
        rotationRef.current.y += 0.0015;
      }

      scene.rotation.y = rotationRef.current.y;
      scene.rotation.x = rotationRef.current.x;

      // Pulse entities and aura rings
      entityMeshesRef.current.forEach((group) => {
        const sphere = group.children[0] as THREE.Mesh;
        const ring = group.children[1] as THREE.Mesh;
        if (sphere) {
          const s = 1 + Math.sin(elapsed * 2 + sphere.userData.pulseOffset) * 0.08;
          sphere.scale.set(s, s, s);
        }
        if (ring) {
          ring.rotation.z += 0.015;
        }
      });

      renderer.render(scene, camera);
    };
    animate();

    // 8. Resize listener
    const handleResize = () => {
      if (!mountRef.current || !rendererRef.current || !cameraRef.current) return;
      const w = mountRef.current.clientWidth;
      const h = mountRef.current.clientHeight;
      cameraRef.current.aspect = w / h;
      cameraRef.current.updateProjectionMatrix();
      rendererRef.current.setSize(w, h);
    };
    window.addEventListener('resize', handleResize);

    return () => {
      cancelAnimationFrame(animationFrameId);
      window.removeEventListener('resize', handleResize);
      dom.removeEventListener('mousedown', handleMouseDown);
      window.removeEventListener('mousemove', handleMouseMove);
      window.removeEventListener('mouseup', handleMouseUp);
      dom.removeEventListener('wheel', handleWheel);
      dom.removeEventListener('click', handleClick);
      renderer.dispose();
      if (mountRef.current && dom.parentNode === mountRef.current) {
        mountRef.current.removeChild(dom);
      }
    };
  }, []);

  // Update entity 3D nodes & Assemblage energy cords whenever entities change
  useEffect(() => {
    const scene = sceneRef.current;
    if (!scene) return;

    // Clear old entity meshes
    entityMeshesRef.current.forEach((mesh) => scene.remove(mesh));
    entityMeshesRef.current.clear();

    if (assemblageLinesRef.current) {
      scene.remove(assemblageLinesRef.current);
      assemblageLinesRef.current = null;
    }

    const domainAngleMap = new Map<string, number>();
    const domainRadiusMap = new Map<string, number>();
    let domainIdx = 0;

    // Position entities in topological domain clusters in 3D
    const entityPositions = new Map<string, THREE.Vector3>();

    entities.forEach((ent, idx) => {
      const domain = ent.spatial.domain || '无垠虚空';
      if (!domainAngleMap.has(domain)) {
        domainAngleMap.set(domain, domainIdx * (Math.PI * 0.7));
        domainRadiusMap.set(domain, 35 + (domainIdx % 2) * 20);
        domainIdx++;
      }

      const baseAngle = domainAngleMap.get(domain)!;
      const baseRadius = domainRadiusMap.get(domain)!;
      const offsetAngle = baseAngle + (idx * 0.6);
      const x = Math.cos(offsetAngle) * baseRadius + (Math.sin(idx * 2) * 8);
      const y = Math.sin(offsetAngle * 0.8) * 18 + (Math.cos(idx) * 6);
      const z = Math.sin(offsetAngle) * baseRadius;
      const pos = new THREE.Vector3(x, y, z);
      entityPositions.set(ent.id, pos);

      // Node Group
      const group = new THREE.Group();
      group.position.copy(pos);

      // Color mapping by Lifecycle Phase (成住坏空)
      let color = 0x4ade80; // Flourishing - Emerald Green
      const phase = (ent.lifecycle || '').toLowerCase();
      if (phase.includes('genesis') || phase.includes('成')) color = 0x38bdf8; // Cyan
      else if (phase.includes('decay') || phase.includes('坏')) color = 0xfbbf24; // Amber
      else if (phase.includes('dissol') || phase.includes('空')) color = 0xf43f5e; // Rose Red

      if (ent.id === selectedEntityId) {
        color = 0xc084fc; // Selected - Vibrant Violet
      }

      // 1. Core Sphere
      const sphereGeo = new THREE.SphereGeometry(3.2 * (ent.cohesion || 1.0), 24, 24);
      const sphereMat = new THREE.MeshStandardMaterial({
        color,
        emissive: color,
        emissiveIntensity: ent.id === selectedEntityId ? 1.2 : 0.6,
        roughness: 0.2,
        metalness: 0.5,
      });
      const sphereMesh = new THREE.Mesh(sphereGeo, sphereMat);
      sphereMesh.userData = { entityId: ent.id, pulseOffset: idx * 0.8 };
      group.add(sphereMesh);

      // 2. Halo Energy Ring
      const ringGeo = new THREE.RingGeometry(4.2, 4.8, 32);
      const ringMat = new THREE.MeshBasicMaterial({
        color,
        side: THREE.DoubleSide,
        transparent: true,
        opacity: 0.4,
      });
      const ringMesh = new THREE.Mesh(ringGeo, ringMat);
      ringMesh.rotation.x = Math.PI / 2;
      group.add(ringMesh);

      // 3. Awakened Consciousness Crown (觉知降临光冕)
      if (ent.active_inhabitants && ent.active_inhabitants.length > 0) {
        const crownGeo = new THREE.TorusGeometry(3.6, 0.4, 16, 48);
        const crownMat = new THREE.MeshStandardMaterial({
          color: 0xc084fc,
          emissive: 0xc084fc,
          emissiveIntensity: 1.5,
          roughness: 0.1,
          metalness: 0.8,
        });
        const crownMesh = new THREE.Mesh(crownGeo, crownMat);
        crownMesh.position.set(0, 3.8, 0);
        crownMesh.rotation.x = Math.PI / 3;
        group.add(crownMesh);
      }

      // 4. Text Billboard Sprite for Entity Name
      const canvas = document.createElement('canvas');
      canvas.width = 256;
      canvas.height = 64;
      const ctx = canvas.getContext('2d');
      if (ctx) {
        ctx.fillStyle = 'rgba(7, 9, 14, 0.7)';
        ctx.roundRect ? ctx.roundRect(0, 0, 256, 64, 12) : ctx.fillRect(0, 0, 256, 64);
        ctx.fill();
        ctx.strokeStyle = ent.id === selectedEntityId ? '#c084fc' : '#38bdf8';
        ctx.lineWidth = 3;
        ctx.stroke();
        ctx.fillStyle = '#ffffff';
        ctx.font = 'bold 22px sans-serif';
        ctx.textAlign = 'center';
        const inhabitTag = ent.active_inhabitants && ent.active_inhabitants.length > 0 ? ' ✨' : '';
        ctx.fillText(`${ent.name.slice(0, 8)}${inhabitTag}`, 128, 40);
      }
      const texture = new THREE.CanvasTexture(canvas);
      const spriteMat = new THREE.SpriteMaterial({ map: texture, transparent: true });
      const sprite = new THREE.Sprite(spriteMat);
      sprite.position.set(0, 7.5, 0);
      sprite.scale.set(16, 4, 1);
      group.add(sprite);

      scene.add(group);
      entityMeshesRef.current.set(ent.id, group);
    });

    // Draw Assemblage Symbiosis Resonance Cords (德勒兹共生能量线)
    const linePositions: number[] = [];
    const drawnPairs = new Set<string>();

    entities.forEach((ent) => {
      const posA = entityPositions.get(ent.id);
      if (!posA) return;

      (ent.assemblages || []).forEach((partnerId) => {
        const pairKey = [ent.id, partnerId].sort().join('-');
        if (drawnPairs.has(pairKey)) return;
        drawnPairs.add(pairKey);

        const posB = entityPositions.get(partnerId);
        if (posB) {
          linePositions.push(posA.x, posA.y, posA.z);
          linePositions.push(posB.x, posB.y, posB.z);
        }
      });
    });

    if (linePositions.length > 0) {
      const lineGeo = new THREE.BufferGeometry();
      lineGeo.setAttribute('position', new THREE.Float32BufferAttribute(linePositions, 3));
      const lineMat = new THREE.LineBasicMaterial({
        color: 0x38bdf8,
        transparent: true,
        opacity: 0.7,
        linewidth: 2,
      });
      const lines = new THREE.LineSegments(lineGeo, lineMat);
      scene.add(lines);
      assemblageLinesRef.current = lines;
    }
  }, [entities, selectedEntityId]);

  return (
    <div className="relative w-full h-full min-h-[460px] bg-slate-950 rounded-xl overflow-hidden border border-slate-800">
      <div ref={mountRef} className="w-full h-full cursor-grab active:cursor-grabbing" />
      
      {/* 3D 悬浮指示徽章 */}
      <div className="absolute top-4 left-4 bg-slate-900/80 backdrop-blur-md border border-slate-700/60 rounded-lg px-3 py-1.5 text-xs flex items-center gap-2 text-slate-300 pointer-events-none">
        <span className="w-2 h-2 rounded-full bg-emerald-400 animate-ping" />
        <span>3D 拓扑星象场域 (按住拖拽视角 / 滚轮缩放 / 点击选中灵元)</span>
      </div>

      <div className="absolute bottom-4 right-4 bg-slate-900/80 backdrop-blur-md border border-slate-700/60 rounded-lg px-3 py-1.5 text-xs text-slate-400 pointer-events-none">
        当前场域灵元总数: <span className="text-emerald-400 font-bold">{entities.length}</span>
      </div>
    </div>
  );
};
