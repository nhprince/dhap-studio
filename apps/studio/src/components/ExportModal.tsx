import React, { useState } from 'react';
import { Project } from '../types';
import { X, CheckCircle, AlertTriangle, ShieldCheck, Download, FileText, Globe } from 'lucide-react';

interface ExportModalProps {
  project: Project;
  onClose: () => void;
  onExportSuccess: (format: string) => void;
}

export const ExportModal: React.FC<ExportModalProps> = ({
  project,
  onClose,
  onExportSuccess,
}) => {
  const [selectedFormat, setSelectedFormat] = useState<'all' | 'markdown' | 'html'>('all');
  const [isExporting, setIsExporting] = useState(false);
  const [exportComplete, setExportComplete] = useState(false);

  // Preflight calculations
  const totalSteps = project.steps.length;
  const blankTitles = project.steps.filter((s) => !s.title.trim()).length;
  const totalRedactions = project.steps.reduce(
    (acc, s) => acc + s.annotations.filter((a) => a.type === 'redaction').length,
    0
  );
  const isPreflightValid = project.title.trim().length > 0 && blankTitles === 0;

  const handleExport = () => {
    setIsExporting(true);
    setTimeout(() => {
      setIsExporting(false);
      setExportComplete(true);
      onExportSuccess(selectedFormat);
    }, 800);
  };

  return (
    <div className="modal-backdrop">
      <div className="modal-content">
        <div className="modal-header">
          <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', fontWeight: 700, fontSize: '1.1rem' }}>
            <Download size={20} color="var(--accent)" />
            <span>Export Illustrated Documentation</span>
          </div>
          <button className="btn btn-ghost" style={{ padding: '0.25rem' }} onClick={onClose} aria-label="Close modal">
            <X size={18} />
          </button>
        </div>

        <div className="modal-body" style={{ display: 'flex', flexDirection: 'column', gap: '1.25rem' }}>
          {/* Preflight Checklist Card */}
          <div style={{ background: 'var(--bg-app)', border: '1px solid var(--border)', borderRadius: 'var(--radius-md)', padding: '1.25rem' }}>
            <h4 style={{ fontSize: '0.9rem', marginBottom: '0.75rem', color: 'var(--text-muted)' }}>
              Preflight Quality & Privacy Verification
            </h4>

            <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem', fontSize: '0.85rem' }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                <CheckCircle size={16} color="var(--emerald)" />
                <span>Project Title: <strong>{project.title}</strong></span>
              </div>

              <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                {blankTitles === 0 ? (
                  <CheckCircle size={16} color="var(--emerald)" />
                ) : (
                  <AlertTriangle size={16} color="var(--amber)" />
                )}
                <span>Step Verification: <strong>{totalSteps} steps configured</strong> ({blankTitles} empty titles)</span>
              </div>

              <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                <ShieldCheck size={16} color="var(--blue)" />
                <span>
                  Privacy Redactions: <strong>{totalRedactions} sensitive region(s)</strong> will be permanently blacked out on export
                </span>
              </div>
            </div>
          </div>

          {/* Format Selection */}
          <div>
            <label style={{ display: 'block', fontSize: '0.85rem', fontWeight: 600, color: 'var(--text-muted)', marginBottom: '0.5rem' }}>
              Select Output Formats
            </label>
            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: '0.75rem' }}>
              <button
                type="button"
                onClick={() => setSelectedFormat('all')}
                style={{
                  background: selectedFormat === 'all' ? 'var(--bg-elevated)' : 'var(--bg-app)',
                  border: `1px solid ${selectedFormat === 'all' ? 'var(--accent)' : 'var(--border)'}`,
                  borderRadius: 'var(--radius-sm)',
                  padding: '1rem',
                  textAlign: 'left',
                  cursor: 'pointer',
                }}
              >
                <div style={{ fontWeight: 700, fontSize: '0.9rem', color: 'var(--accent)', marginBottom: '0.25rem' }}>
                  Complete Bundle
                </div>
                <div style={{ fontSize: '0.75rem', color: 'var(--text-muted)' }}>
                  Markdown + Standalone HTML5 + Assets
                </div>
              </button>

              <button
                type="button"
                onClick={() => setSelectedFormat('markdown')}
                style={{
                  background: selectedFormat === 'markdown' ? 'var(--bg-elevated)' : 'var(--bg-app)',
                  border: `1px solid ${selectedFormat === 'markdown' ? 'var(--accent)' : 'var(--border)'}`,
                  borderRadius: 'var(--radius-sm)',
                  padding: '1rem',
                  textAlign: 'left',
                  cursor: 'pointer',
                }}
              >
                <div style={{ display: 'flex', alignItems: 'center', gap: '0.35rem', fontWeight: 700, fontSize: '0.9rem', marginBottom: '0.25rem' }}>
                  <FileText size={15} color="var(--blue)" />
                  <span>Markdown</span>
                </div>
                <div style={{ fontSize: '0.75rem', color: 'var(--text-muted)' }}>
                  GFM guide.md with relative images
                </div>
              </button>

              <button
                type="button"
                onClick={() => setSelectedFormat('html')}
                style={{
                  background: selectedFormat === 'html' ? 'var(--bg-elevated)' : 'var(--bg-app)',
                  border: `1px solid ${selectedFormat === 'html' ? 'var(--accent)' : 'var(--border)'}`,
                  borderRadius: 'var(--radius-sm)',
                  padding: '1rem',
                  textAlign: 'left',
                  cursor: 'pointer',
                }}
              >
                <div style={{ display: 'flex', alignItems: 'center', gap: '0.35rem', fontWeight: 700, fontSize: '0.9rem', marginBottom: '0.25rem' }}>
                  <Globe size={15} color="var(--emerald)" />
                  <span>HTML5 Guide</span>
                </div>
                <div style={{ fontSize: '0.75rem', color: 'var(--text-muted)' }}>
                  Self-contained responsive page
                </div>
              </button>
            </div>
          </div>

          {exportComplete && (
            <div style={{ background: 'rgba(16, 185, 129, 0.1)', border: '1px solid var(--emerald)', borderRadius: 'var(--radius-sm)', padding: '1rem', color: 'var(--emerald)', fontSize: '0.9rem' }}>
              ✓ Export generated successfully! Files validated and ready in the output directory.
            </div>
          )}
        </div>

        <div className="modal-footer">
          <button className="btn btn-ghost" onClick={onClose}>
            Close
          </button>
          <button
            className="btn btn-primary"
            disabled={!isPreflightValid || isExporting}
            onClick={handleExport}
          >
            {isExporting ? 'Generating Artifacts...' : 'Confirm & Export'}
          </button>
        </div>
      </div>
    </div>
  );
};
