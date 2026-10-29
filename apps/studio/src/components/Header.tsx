import React from 'react';
import { ViewMode } from '../types';
import { BookOpen, Film, Eye, Download, Moon, Sun, Save, FolderOpen, Plus } from 'lucide-react';

interface HeaderProps {
  projectTitle: string;
  viewMode: ViewMode;
  setViewMode: (mode: ViewMode) => void;
  onOpenExport: () => void;
  onNewProject: () => void;
  onSaveProject: () => void;
  onImportClick: () => void;
  theme: string;
  setTheme: (t: string) => void;
  saveStatus: string;
}

export const Header: React.FC<HeaderProps> = ({
  projectTitle,
  viewMode,
  setViewMode,
  onOpenExport,
  onNewProject,
  onSaveProject,
  onImportClick,
  theme,
  setTheme,
  saveStatus,
}) => {
  return (
    <header className="studio-header">
      <div className="header-brand">
        <div className="brand-badge">ধ</div>
        <span>Dhap Studio</span>
        <span className="brand-tag">ধাপ</span>
        <span style={{ color: 'var(--text-dim)', margin: '0 0.5rem' }}>|</span>
        <span style={{ fontSize: '0.9rem', color: 'var(--text-muted)', fontWeight: 500, maxWidth: 220, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
          {projectTitle}
        </span>
      </div>

      <div className="header-tabs">
        <button
          className={`tab-btn ${viewMode === 'guide' ? 'active' : ''}`}
          onClick={() => setViewMode('guide')}
          title="Guide Mode — Document & step-by-step instructions"
        >
          <BookOpen size={16} />
          <span>Guide</span>
        </button>
        <button
          className={`tab-btn ${viewMode === 'timeline' ? 'active' : ''}`}
          onClick={() => setViewMode('timeline')}
          title="Timeline Mode — Source video & keyframes"
        >
          <Film size={16} />
          <span>Timeline</span>
        </button>
        <button
          className={`tab-btn ${viewMode === 'preview' ? 'active' : ''}`}
          onClick={() => setViewMode('preview')}
          title="Preview Mode — Rendered documentation output"
        >
          <Eye size={16} />
          <span>Preview</span>
        </button>
      </div>

      <div className="header-actions">
        <span style={{ fontSize: '0.78rem', color: 'var(--text-dim)', marginRight: '0.25rem' }}>
          {saveStatus}
        </span>

        <button className="btn btn-ghost" onClick={onNewProject} title="Create New Project">
          <Plus size={16} />
          <span>New</span>
        </button>

        <button className="btn btn-ghost" onClick={onImportClick} title="Import Media">
          <FolderOpen size={16} />
          <span>Import</span>
        </button>

        <button className="btn btn-secondary" onClick={onSaveProject} title="Save .dhap Bundle">
          <Save size={16} />
          <span>Save</span>
        </button>

        <button className="btn btn-primary" onClick={onOpenExport} title="Export Documentation">
          <Download size={16} />
          <span>Export</span>
        </button>

        <button
          className="btn btn-ghost"
          style={{ padding: '0.5rem' }}
          onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
          title="Toggle Light/Dark Theme"
          aria-label="Toggle Theme"
        >
          {theme === 'dark' ? <Sun size={18} /> : <Moon size={18} />}
        </button>
      </div>
    </header>
  );
};
