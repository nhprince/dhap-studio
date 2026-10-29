import React, { useState, useRef } from 'react';
import { Annotation } from '../types';
import { Square, Hash, ShieldAlert, Trash2, RotateCcw } from 'lucide-react';

interface AnnotationCanvasProps {
  frameUrl?: string;
  annotations: Annotation[];
  onUpdateAnnotations: (annos: Annotation[]) => void;
}

type ActiveTool = 'select' | 'badge' | 'rect' | 'redaction';

export const AnnotationCanvas: React.FC<AnnotationCanvasProps> = ({
  frameUrl,
  annotations,
  onUpdateAnnotations,
}) => {
  const [activeTool, setActiveTool] = useState<ActiveTool>('badge');
  const [isDrawing, setIsDrawing] = useState(false);
  const [startPos, setStartPos] = useState<{ x: number; y: number } | null>(null);
  const [currentRect, setCurrentRect] = useState<{ x: number; y: number; w: number; h: number } | null>(null);
  const [selectedAnnoId, setSelectedAnnoId] = useState<string | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  if (!frameUrl) {
    return (
      <div className="canvas-container" style={{ alignItems: 'center', justifyContent: 'center', color: 'var(--text-dim)' }}>
        <p>No screenshot attached to this step. Extract or paste a frame to add annotations.</p>
      </div>
    );
  }

  const handleMouseDown = (e: React.MouseEvent<SVGSVGElement>) => {
    const rect = e.currentTarget.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    if (activeTool === 'badge') {
      const nextNum = annotations.filter((a) => a.type === 'number_badge').length + 1;
      const newBadge: Annotation = {
        id: `anno_${Date.now()}`,
        type: 'number_badge',
        x: Math.round(x),
        y: Math.round(y),
        radius: 14,
        number: nextNum,
        color: '#e11d48',
      };
      onUpdateAnnotations([...annotations, newBadge]);
    } else if (activeTool === 'rect' || activeTool === 'redaction') {
      setIsDrawing(true);
      setStartPos({ x, y });
      setCurrentRect({ x, y, w: 0, h: 0 });
    }
  };

  const handleMouseMove = (e: React.MouseEvent<SVGSVGElement>) => {
    if (!isDrawing || !startPos) return;
    const rect = e.currentTarget.getBoundingClientRect();
    const curX = e.clientX - rect.left;
    const curY = e.clientY - rect.top;

    const x = Math.min(startPos.x, curX);
    const y = Math.min(startPos.y, curY);
    const w = Math.abs(curX - startPos.x);
    const h = Math.abs(curY - startPos.y);

    setCurrentRect({ x, y, w, h });
  };

  const handleMouseUp = () => {
    if (isDrawing && currentRect && currentRect.w > 10 && currentRect.h > 10) {
      if (activeTool === 'redaction') {
        const newRedaction: Annotation = {
          id: `anno_${Date.now()}`,
          type: 'redaction',
          x: Math.round(currentRect.x),
          y: Math.round(currentRect.y),
          width: Math.round(currentRect.w),
          height: Math.round(currentRect.h),
          redaction_type: 'blackout',
        };
        onUpdateAnnotations([...annotations, newRedaction]);
      } else if (activeTool === 'rect') {
        const newRect: Annotation = {
          id: `anno_${Date.now()}`,
          type: 'rectangle',
          x: Math.round(currentRect.x),
          y: Math.round(currentRect.y),
          width: Math.round(currentRect.w),
          height: Math.round(currentRect.h),
          color: '#38bdf8',
          stroke_width: 2,
        };
        onUpdateAnnotations([...annotations, newRect]);
      }
    }
    setIsDrawing(false);
    setStartPos(null);
    setCurrentRect(null);
  };

  const handleDeleteSelected = () => {
    if (!selectedAnnoId) return;
    onUpdateAnnotations(annotations.filter((a) => a.id !== selectedAnnoId));
    setSelectedAnnoId(null);
  };

  const handleClearAll = () => {
    onUpdateAnnotations([]);
    setSelectedAnnoId(null);
  };

  return (
    <div className="canvas-container">
      <div className="canvas-toolbar">
        <div className="canvas-tools">
          <button
            className={`tool-btn ${activeTool === 'badge' ? 'active' : ''}`}
            onClick={() => setActiveTool('badge')}
            title="Add Numbered Step Badge (Click on screenshot)"
          >
            <Hash size={14} />
            <span>Badge</span>
          </button>

          <button
            className={`tool-btn ${activeTool === 'rect' ? 'active' : ''}`}
            onClick={() => setActiveTool('rect')}
            title="Draw Focus Rectangle"
          >
            <Square size={14} />
            <span>Box</span>
          </button>

          <button
            className={`tool-btn ${activeTool === 'redaction' ? 'active' : ''}`}
            onClick={() => setActiveTool('redaction')}
            style={activeTool === 'redaction' ? { background: '#0f172a', borderColor: 'var(--amber)', color: 'var(--amber)' } : {}}
            title="Draw Solid Blackout Redaction Mask"
          >
            <ShieldAlert size={14} />
            <span>Blackout Redaction</span>
          </button>
        </div>

        <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
          {selectedAnnoId && (
            <button className="btn btn-danger" style={{ padding: '0.3rem 0.6rem', fontSize: '0.8rem' }} onClick={handleDeleteSelected}>
              <Trash2 size={13} />
              <span>Delete Element</span>
            </button>
          )}

          <button className="btn btn-ghost" style={{ padding: '0.3rem 0.6rem', fontSize: '0.8rem' }} onClick={handleClearAll} title="Clear all annotations on this frame">
            <RotateCcw size={13} />
            <span>Reset</span>
          </button>
        </div>
      </div>

      <div className="canvas-stage" ref={containerRef}>
        <div className="canvas-image-wrapper">
          <img src={frameUrl} alt="Step Frame" draggable={false} />

          <svg
            className="canvas-overlay-svg"
            onMouseDown={handleMouseDown}
            onMouseMove={handleMouseMove}
            onMouseUp={handleMouseUp}
          >
            {annotations.map((anno) => {
              const isSelected = anno.id === selectedAnnoId;

              if (anno.type === 'redaction') {
                return (
                  <g key={anno.id} onClick={(e) => { e.stopPropagation(); setSelectedAnnoId(anno.id); }} style={{ cursor: 'pointer' }}>
                    <rect
                      x={anno.x}
                      y={anno.y}
                      width={anno.width || 0}
                      height={anno.height || 0}
                      fill="#0b0f19"
                      stroke={isSelected ? '#f59e0b' : '#334155'}
                      strokeWidth={isSelected ? 2 : 1}
                      rx={3}
                    />
                    <text
                      x={anno.x + 8}
                      y={anno.y + 16}
                      fill="#64748b"
                      fontSize={10}
                      fontFamily="monospace"
                      fontWeight="bold"
                    >
                      [REDACTED]
                    </text>
                  </g>
                );
              }

              if (anno.type === 'rectangle') {
                return (
                  <rect
                    key={anno.id}
                    x={anno.x}
                    y={anno.y}
                    width={anno.width || 0}
                    height={anno.height || 0}
                    fill="none"
                    stroke={isSelected ? '#f43f5e' : (anno.color || '#38bdf8')}
                    strokeWidth={isSelected ? 3 : (anno.stroke_width || 2)}
                    rx={4}
                    onClick={(e) => { e.stopPropagation(); setSelectedAnnoId(anno.id); }}
                    style={{ cursor: 'pointer' }}
                  />
                );
              }

              if (anno.type === 'number_badge') {
                return (
                  <g
                    key={anno.id}
                    onClick={(e) => { e.stopPropagation(); setSelectedAnnoId(anno.id); }}
                    style={{ cursor: 'pointer' }}
                  >
                    <circle
                      cx={anno.x}
                      cy={anno.y}
                      r={anno.radius || 14}
                      fill={anno.color || '#e11d48'}
                      stroke={isSelected ? '#ffffff' : '#0f172a'}
                      strokeWidth={2}
                    />
                    <text
                      x={anno.x}
                      y={anno.y + 4}
                      fill="#ffffff"
                      fontSize={12}
                      fontWeight="bold"
                      textAnchor="middle"
                      fontFamily="sans-serif"
                    >
                      {anno.number || 1}
                    </text>
                  </g>
                );
              }

              return null;
            })}

            {/* Drawing Preview */}
            {isDrawing && currentRect && (
              <rect
                x={currentRect.x}
                y={currentRect.y}
                width={currentRect.w}
                height={currentRect.h}
                fill={activeTool === 'redaction' ? 'rgba(15, 23, 42, 0.85)' : 'none'}
                stroke={activeTool === 'redaction' ? '#f59e0b' : '#38bdf8'}
                strokeWidth={2}
                strokeDasharray="4 4"
              />
            )}
          </svg>
        </div>
      </div>
    </div>
  );
};
