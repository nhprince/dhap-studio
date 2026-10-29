import React, { useState } from 'react';
import { GuideStep, SourceMedia } from '../types';
import { Play, Pause, Plus, Flag, Film } from 'lucide-react';

interface TimelineViewProps {
  sourceMedia?: SourceMedia;
  steps: GuideStep[];
  selectedStepId: string;
  onSelectStep: (id: string) => void;
  onAddStepAtTimestamp: (ts: number) => void;
}

export const TimelineView: React.FC<TimelineViewProps> = ({
  sourceMedia,
  steps,
  selectedStepId,
  onSelectStep,
  onAddStepAtTimestamp,
}) => {
  const [currentTimeMs, setCurrentTimeMs] = useState(4500);
  const [isPlaying, setIsPlaying] = useState(false);

  const durationMs = sourceMedia?.duration_ms || 45000;
  const currentSec = (currentTimeMs / 1000).toFixed(1);
  const totalSec = (durationMs / 1000).toFixed(1);

  const handleSeek = (e: React.ChangeEvent<HTMLInputElement>) => {
    setCurrentTimeMs(Number(e.target.value));
  };

  const handleAddMarker = () => {
    onAddStepAtTimestamp(currentTimeMs);
  };

  return (
    <div style={{ flex: 1, display: 'flex', flexDirection: 'column', height: '100%', background: 'var(--bg-app)', padding: '1.5rem', gap: '1.5rem', overflowY: 'auto' }}>
      {/* Video Simulation / Monitor */}
      <div style={{
        background: '#090d16',
        borderRadius: 'var(--radius-lg)',
        border: '1px solid var(--border)',
        height: '380px',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        position: 'relative',
        overflow: 'hidden',
        boxShadow: 'var(--shadow-lg)'
      }}>
        <div style={{ position: 'absolute', top: 16, left: 20, display: 'flex', alignItems: 'center', gap: '0.5rem', color: 'var(--text-muted)', fontSize: '0.85rem' }}>
          <Film size={16} />
          <span>{sourceMedia?.file_name || 'Workflow Recording Source'}</span>
        </div>

        <div style={{ textAlign: 'center' }}>
          <div style={{ fontSize: '3rem', fontWeight: 800, color: 'var(--text-main)', letterSpacing: '0.05em', fontFamily: 'var(--font-mono)' }}>
            {Math.floor(currentTimeMs / 60000)}:{String(Math.floor((currentTimeMs % 60000) / 1000)).padStart(2, '0')}.{String(Math.floor((currentTimeMs % 1000) / 100))}
          </div>
          <div style={{ color: 'var(--text-dim)', fontSize: '0.9rem', marginTop: '0.5rem' }}>
            Current Playhead Position ({currentSec}s / {totalSec}s)
          </div>
        </div>

        {/* Video Overlay Controls */}
        <div style={{ position: 'absolute', bottom: 16, display: 'flex', alignItems: 'center', gap: '1rem' }}>
          <button
            className="btn btn-primary"
            style={{ borderRadius: '50%', width: 44, height: 44, padding: 0 }}
            onClick={() => setIsPlaying(!isPlaying)}
            title={isPlaying ? 'Pause' : 'Play'}
          >
            {isPlaying ? <Pause size={18} /> : <Play size={18} style={{ marginLeft: 2 }} />}
          </button>

          <button
            className="btn btn-secondary"
            onClick={handleAddMarker}
            title="Create a new guide step linked to this exact timestamp"
          >
            <Plus size={16} />
            <span>Mark Step at {currentSec}s</span>
          </button>
        </div>
      </div>

      {/* Interactive Timeline Track */}
      <div style={{ background: 'var(--bg-card)', padding: '1.25rem', borderRadius: 'var(--radius-md)', border: '1px solid var(--border)' }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '0.75rem', fontSize: '0.85rem' }}>
          <span style={{ fontWeight: 600, color: 'var(--text-muted)' }}>Workflow Timeline & Markers</span>
          <span style={{ fontFamily: 'var(--font-mono)', color: 'var(--text-dim)' }}>
            {steps.length} Steps linked to timestamps
          </span>
        </div>

        {/* Scrub Bar */}
        <div style={{ position: 'relative', width: '100%', marginBottom: '1.5rem' }}>
          <input
            type="range"
            min={0}
            max={durationMs}
            value={currentTimeMs}
            onChange={handleSeek}
            style={{ width: '100%', accentColor: 'var(--accent)', cursor: 'pointer' }}
          />

          {/* Step Markers placed along track */}
          <div style={{ position: 'relative', height: '24px', marginTop: '0.25rem' }}>
            {steps.map((s) => {
              const pct = (s.timestamp_ms / durationMs) * 100;
              const isSelected = s.id === selectedStepId;
              return (
                <div
                  key={s.id}
                  onClick={() => {
                    setCurrentTimeMs(s.timestamp_ms);
                    onSelectStep(s.id);
                  }}
                  style={{
                    position: 'absolute',
                    left: `${pct}%`,
                    transform: 'translateX(-50%)',
                    cursor: 'pointer',
                    display: 'flex',
                    flexDirection: 'column',
                    alignItems: 'center',
                  }}
                  title={`Step ${s.order}: ${s.title} (${(s.timestamp_ms / 1000).toFixed(1)}s)`}
                >
                  <Flag
                    size={14}
                    fill={isSelected ? 'var(--accent)' : 'var(--blue)'}
                    color={isSelected ? 'var(--accent)' : 'var(--blue)'}
                  />
                  <span style={{ fontSize: '0.7rem', fontWeight: 700, color: isSelected ? 'var(--accent)' : 'var(--text-muted)' }}>
                    {s.order}
                  </span>
                </div>
              );
            })}
          </div>
        </div>

        {/* Keyframe Step Badges Grid */}
        <div style={{ display: 'flex', gap: '0.75rem', overflowX: 'auto', paddingBottom: '0.5rem' }}>
          {steps.map((s) => {
            const isSelected = s.id === selectedStepId;
            return (
              <div
                key={s.id}
                onClick={() => {
                  setCurrentTimeMs(s.timestamp_ms);
                  onSelectStep(s.id);
                }}
                style={{
                  minWidth: 180,
                  background: isSelected ? 'rgba(225, 29, 72, 0.12)' : 'var(--bg-app)',
                  border: `1px solid ${isSelected ? 'var(--accent)' : 'var(--border)'}`,
                  borderRadius: 'var(--radius-sm)',
                  padding: '0.75rem',
                  cursor: 'pointer',
                  transition: 'all 0.15s'
                }}
              >
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '0.25rem' }}>
                  <span className="step-number-tag">Step {s.order}</span>
                  <span style={{ fontSize: '0.75rem', color: 'var(--text-dim)', fontFamily: 'var(--font-mono)' }}>
                    {(s.timestamp_ms / 1000).toFixed(1)}s
                  </span>
                </div>
                <div style={{ fontSize: '0.85rem', fontWeight: 600, whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>
                  {s.title}
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
};
