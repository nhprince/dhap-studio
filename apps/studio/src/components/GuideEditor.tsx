import React from 'react';
import { GuideStep } from '../types';
import { Clock, ChevronUp, ChevronDown, Trash2 } from 'lucide-react';

interface GuideEditorProps {
  step: GuideStep;
  totalSteps: number;
  onUpdateStep: (updated: GuideStep) => void;
  onMoveUp: () => void;
  onMoveDown: () => void;
  onDelete: () => void;
}

export const GuideEditor: React.FC<GuideEditorProps> = ({
  step,
  totalSteps,
  onUpdateStep,
  onMoveUp,
  onMoveDown,
  onDelete,
}) => {
  return (
    <div style={{ flex: 1, display: 'flex', flexDirection: 'column', height: '100%', overflow: 'hidden' }}>
      <div className="panel-header">
        <div className="panel-title">
          <span className="step-number-tag">Step {step.order}</span>
          <span>Edit Step Content</span>
        </div>
        <div style={{ display: 'flex', alignItems: 'center', gap: '0.4rem' }}>
          <button
            className="btn btn-ghost"
            style={{ padding: '0.35rem' }}
            disabled={step.order <= 1}
            onClick={onMoveUp}
            title="Move Step Up"
          >
            <ChevronUp size={16} />
          </button>
          <button
            className="btn btn-ghost"
            style={{ padding: '0.35rem' }}
            disabled={step.order >= totalSteps}
            onClick={onMoveDown}
            title="Move Step Down"
          >
            <ChevronDown size={16} />
          </button>
          <button
            className="btn btn-danger"
            style={{ padding: '0.35rem 0.6rem', fontSize: '0.8rem' }}
            onClick={onDelete}
            title="Delete this step"
          >
            <Trash2 size={14} />
          </button>
        </div>
      </div>

      <div className="panel-content" style={{ display: 'flex', flexDirection: 'column', gap: '1.25rem' }}>
        <div>
          <label style={{ display: 'block', fontSize: '0.825rem', fontWeight: 600, color: 'var(--text-muted)', marginBottom: '0.4rem' }}>
            Step Title
          </label>
          <input
            type="text"
            value={step.title}
            onChange={(e) => onUpdateStep({ ...step, title: e.target.value })}
            placeholder="e.g. Access Organization Overview"
            style={{ width: '100%', fontSize: '1rem', fontWeight: 600 }}
          />
        </div>

        <div>
          <label style={{ display: 'block', fontSize: '0.825rem', fontWeight: 600, color: 'var(--text-muted)', marginBottom: '0.4rem' }}>
            Imperative Instructions
          </label>
          <textarea
            rows={4}
            value={step.instruction}
            onChange={(e) => onUpdateStep({ ...step, instruction: e.target.value })}
            placeholder="Describe the action clearly in imperative voice (e.g. Click the blue 'Create' button...)"
            style={{ width: '100%', resize: 'vertical' }}
          />
        </div>

        <div>
          <label style={{ display: 'block', fontSize: '0.825rem', fontWeight: 600, color: 'var(--text-muted)', marginBottom: '0.4rem' }}>
            Expected Result (Optional)
          </label>
          <input
            type="text"
            value={step.expected_result || ''}
            onChange={(e) => onUpdateStep({ ...step, expected_result: e.target.value || undefined })}
            placeholder="e.g. The configuration panel expands showing options"
            style={{ width: '100%' }}
          />
        </div>

        <div>
          <label style={{ display: 'block', fontSize: '0.825rem', fontWeight: 600, color: 'var(--text-muted)', marginBottom: '0.4rem' }}>
            Notes or Warnings (Optional)
          </label>
          <input
            type="text"
            value={step.notes || ''}
            onChange={(e) => onUpdateStep({ ...step, notes: e.target.value || undefined })}
            placeholder="e.g. Never share this token over unencrypted communication"
            style={{ width: '100%' }}
          />
        </div>

        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', padding: '0.75rem', background: 'var(--bg-card)', borderRadius: 'var(--radius-sm)', border: '1px solid var(--border)' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', fontSize: '0.85rem', color: 'var(--text-muted)' }}>
            <Clock size={15} />
            <span>Timestamp: {(step.timestamp_ms / 1000).toFixed(1)}s</span>
          </div>

          <label style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', cursor: 'pointer', fontSize: '0.85rem' }}>
            <input
              type="checkbox"
              checked={step.is_reviewed}
              onChange={(e) => onUpdateStep({ ...step, is_reviewed: e.target.checked })}
            />
            <span style={{ color: step.is_reviewed ? 'var(--emerald)' : 'var(--text-muted)', fontWeight: 600 }}>
              {step.is_reviewed ? '✓ Reviewed' : 'Mark as Reviewed'}
            </span>
          </label>
        </div>
      </div>
    </div>
  );
};
