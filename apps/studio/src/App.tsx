import React, { useState, useEffect } from 'react';
import { Project, GuideStep, ViewMode, Annotation } from './types';
import { Header } from './components/Header';
import { WelcomeScreen } from './components/WelcomeScreen';
import { GuideEditor } from './components/GuideEditor';
import { AnnotationCanvas } from './components/AnnotationCanvas';
import { TimelineView } from './components/TimelineView';
import { ExportModal } from './components/ExportModal';
import { createSampleProject } from './components/SampleData';
import { Plus, Check, Clock, FileText } from 'lucide-react';

export const App: React.FC = () => {
  const [project, setProject] = useState<Project>(() => {
    const saved = localStorage.getItem('dhap_current_project');
    if (saved) {
      try {
        return JSON.parse(saved);
      } catch {
        // Fallback
      }
    }
    return createSampleProject();
  });

  const [selectedStepId, setSelectedStepId] = useState<string>(() => {
    return project.steps[0]?.id || '';
  });

  const [viewMode, setViewMode] = useState<ViewMode>('guide');
  const [theme, setTheme] = useState<string>('dark');
  const [isExportOpen, setIsExportOpen] = useState(false);
  const [saveStatus, setSaveStatus] = useState('Autosaved');

  // Sync theme attribute to HTML element
  useEffect(() => {
    document.documentElement.setAttribute('data-theme', theme);
  }, [theme]);

  // Autosave to localStorage
  useEffect(() => {
    setSaveStatus('Saving...');
    const timer = setTimeout(() => {
      localStorage.setItem('dhap_current_project', JSON.stringify(project));
      setSaveStatus('Autosaved');
    }, 400);
    return () => clearTimeout(timer);
  }, [project]);

  // Global Keyboard Shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === 's') {
        e.preventDefault();
        setSaveStatus('Saved to bundle');
      }
      if ((e.ctrlKey || e.metaKey) && e.key === 'e') {
        e.preventDefault();
        setIsExportOpen(true);
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  const activeStep = project.steps.find((s) => s.id === selectedStepId) || project.steps[0];

  const handleUpdateActiveStep = (updated: GuideStep) => {
    setProject((prev) => ({
      ...prev,
      updated_at: new Date().toISOString(),
      steps: prev.steps.map((s) => (s.id === updated.id ? updated : s)),
    }));
  };

  const handleUpdateAnnotations = (annos: Annotation[]) => {
    if (!activeStep) return;
    handleUpdateActiveStep({
      ...activeStep,
      annotations: annos,
    });
  };

  const handleAddStep = () => {
    const nextOrder = project.steps.length + 1;
    const newStep: GuideStep = {
      id: `step_${Date.now()}`,
      order: nextOrder,
      title: `Step ${nextOrder}: New Action`,
      instruction: 'Enter clear step-by-step instructions here.',
      timestamp_ms: (nextOrder - 1) * 5000,
      annotations: [],
      is_reviewed: false,
    };
    setProject((prev) => ({
      ...prev,
      steps: [...prev.steps, newStep],
    }));
    setSelectedStepId(newStep.id);
  };

  const handleAddStepAtTimestamp = (timestamp_ms: number) => {
    const nextOrder = project.steps.length + 1;
    const newStep: GuideStep = {
      id: `step_${Date.now()}`,
      order: nextOrder,
      title: `Step at ${(timestamp_ms / 1000).toFixed(1)}s`,
      instruction: 'Follow the action demonstrated at this timestamp.',
      timestamp_ms,
      annotations: [],
      is_reviewed: false,
    };
    setProject((prev) => ({
      ...prev,
      steps: [...prev.steps, newStep],
    }));
    setSelectedStepId(newStep.id);
    setViewMode('guide');
  };

  const handleDeleteStep = (id: string) => {
    setProject((prev) => {
      const remaining = prev.steps.filter((s) => s.id !== id);
      const reindexed = remaining.map((s, idx) => ({ ...s, order: idx + 1 }));
      return { ...prev, steps: reindexed };
    });
    if (selectedStepId === id && project.steps.length > 1) {
      const nextStep = project.steps.find((s) => s.id !== id);
      if (nextStep) setSelectedStepId(nextStep.id);
    }
  };

  const handleMoveStep = (stepIndex: number, direction: 'up' | 'down') => {
    const targetIndex = direction === 'up' ? stepIndex - 1 : stepIndex + 1;
    if (targetIndex < 0 || targetIndex >= project.steps.length) return;

    setProject((prev) => {
      const newSteps = [...prev.steps];
      const temp = newSteps[stepIndex];
      newSteps[stepIndex] = newSteps[targetIndex];
      newSteps[targetIndex] = temp;
      const normalized = newSteps.map((s, idx) => ({ ...s, order: idx + 1 }));
      return { ...prev, steps: normalized };
    });
  };

  const handleLoadSample = () => {
    const sample = createSampleProject();
    setProject(sample);
    setSelectedStepId(sample.steps[0].id);
    setViewMode('guide');
  };

  const handleNewEmpty = () => {
    const emptyProj: Project = {
      id: `proj_${Date.now()}`,
      title: 'Untitled Guide',
      schema_version: 1,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
      sources: [],
      sections: [],
      steps: [
        {
          id: `step_${Date.now()}`,
          order: 1,
          title: 'Initial Step',
          instruction: 'Describe the first action of this workflow.',
          timestamp_ms: 0,
          annotations: [],
          is_reviewed: false,
        },
      ],
      export_profile: {
        include_timestamps: true,
        include_toc: true,
        safe_redaction_mode: true,
        theme: 'auto',
      },
    };
    setProject(emptyProj);
    setSelectedStepId(emptyProj.steps[0].id);
    setViewMode('guide');
  };

  return (
    <>
      <Header
        projectTitle={project.title}
        viewMode={viewMode}
        setViewMode={setViewMode}
        onOpenExport={() => setIsExportOpen(true)}
        onNewProject={handleNewEmpty}
        onSaveProject={() => setSaveStatus('Saved .dhap')}
        onImportClick={handleLoadSample}
        theme={theme}
        setTheme={setTheme}
        saveStatus={saveStatus}
      />

      <div className="studio-layout">
        {project.steps.length === 0 ? (
          <WelcomeScreen
            onLoadSample={handleLoadSample}
            onImportMedia={handleLoadSample}
            onNewEmpty={handleNewEmpty}
          />
        ) : (
          <>
            {/* Left Sidebar: Step Hierarchy & Navigation */}
            <aside className="studio-sidebar">
              <div className="panel-header">
                <div className="panel-title">
                  <FileText size={16} />
                  <span>Workflow Steps ({project.steps.length})</span>
                </div>
                <button
                  className="btn btn-primary"
                  style={{ padding: '0.35rem 0.65rem', fontSize: '0.8rem' }}
                  onClick={handleAddStep}
                  title="Add new step (Ctrl+N)"
                >
                  <Plus size={14} />
                  <span>Add Step</span>
                </button>
              </div>

              <div className="panel-content">
                {project.steps.map((step) => {
                  const isSelected = step.id === selectedStepId;
                  const redactions = step.annotations.filter((a) => a.type === 'redaction').length;
                  return (
                    <div
                      key={step.id}
                      className={`step-item ${isSelected ? 'selected' : ''}`}
                      onClick={() => setSelectedStepId(step.id)}
                    >
                      <div className="step-item-header">
                        <span className="step-number-tag">{step.order}</span>
                        <span className="step-item-title">{step.title}</span>
                      </div>
                      <div className="step-item-meta">
                        <span style={{ display: 'flex', alignItems: 'center', gap: '0.25rem' }}>
                          <Clock size={12} />
                          <span>{(step.timestamp_ms / 1000).toFixed(1)}s</span>
                        </span>
                        {redactions > 0 && (
                          <span style={{ color: 'var(--amber)', fontWeight: 600 }}>
                            🛡️ {redactions} Redacted
                          </span>
                        )}
                        {step.is_reviewed && (
                          <span style={{ color: 'var(--emerald)', display: 'flex', alignItems: 'center', gap: '0.2rem', marginLeft: 'auto' }}>
                            <Check size={12} /> Reviewed
                          </span>
                        )}
                      </div>
                    </div>
                  );
                })}
              </div>
            </aside>

            {/* Central / Main Workspace based on ViewMode */}
            <main className="studio-main">
              {viewMode === 'guide' && activeStep && (
                <div style={{ display: 'flex', flex: 1, height: '100%', overflow: 'hidden' }}>
                  {/* Annotation Canvas */}
                  <AnnotationCanvas
                    frameUrl={activeStep.frame_data_url}
                    annotations={activeStep.annotations}
                    onUpdateAnnotations={handleUpdateAnnotations}
                  />

                  {/* Step Metadata & Instructions Panel */}
                  <div style={{ width: 380, borderLeft: '1px solid var(--border)', background: 'var(--bg-sidebar)', display: 'flex', flexDirection: 'column' }}>
                    <GuideEditor
                      step={activeStep}
                      totalSteps={project.steps.length}
                      onUpdateStep={handleUpdateActiveStep}
                      onMoveUp={() => {
                        const idx = project.steps.findIndex((s) => s.id === activeStep.id);
                        handleMoveStep(idx, 'up');
                      }}
                      onMoveDown={() => {
                        const idx = project.steps.findIndex((s) => s.id === activeStep.id);
                        handleMoveStep(idx, 'down');
                      }}
                      onDelete={() => handleDeleteStep(activeStep.id)}
                    />
                  </div>
                </div>
              )}

              {viewMode === 'timeline' && (
                <TimelineView
                  sourceMedia={project.sources[0]}
                  steps={project.steps}
                  selectedStepId={selectedStepId}
                  onSelectStep={(id) => setSelectedStepId(id)}
                  onAddStepAtTimestamp={handleAddStepAtTimestamp}
                />
              )}

              {viewMode === 'preview' && (
                <div style={{ flex: 1, padding: '3rem', overflowY: 'auto', maxWidth: 850, margin: '0 auto' }}>
                  <div style={{ marginBottom: '2.5rem', borderBottom: '1px solid var(--border)', paddingBottom: '1.5rem' }}>
                    <h1 style={{ fontSize: '2.2rem', fontWeight: 800, marginBottom: '0.5rem' }}>{project.title}</h1>
                    <p style={{ color: 'var(--text-muted)', fontSize: '1.1rem' }}>{project.description}</p>
                  </div>

                  {project.steps.map((s) => (
                    <div key={s.id} style={{ background: 'var(--bg-card)', border: '1px solid var(--border)', borderRadius: 'var(--radius-lg)', padding: '1.75rem', marginBottom: '2rem' }}>
                      <div style={{ display: 'flex', alignItems: 'center', gap: '0.75rem', marginBottom: '1rem' }}>
                        <span className="step-number-tag" style={{ background: 'var(--accent)', color: '#fff', padding: '4px 10px', fontSize: '0.85rem' }}>
                          Step {s.order}
                        </span>
                        <h2 style={{ fontSize: '1.35rem', fontWeight: 700 }}>{s.title}</h2>
                      </div>
                      <p style={{ fontSize: '1.05rem', marginBottom: '1.25rem' }}>{s.instruction}</p>
                      {s.frame_data_url && (
                        <div style={{ borderRadius: 8, overflow: 'hidden', border: '1px solid var(--border)', margin: '1rem 0' }}>
                          <img src={s.frame_data_url} alt={s.title} style={{ width: '100%', display: 'block' }} />
                        </div>
                      )}
                      {s.expected_result && (
                        <div style={{ background: 'rgba(16, 185, 129, 0.1)', borderLeft: '4px solid var(--emerald)', padding: '0.75rem 1rem', borderRadius: 4, marginTop: '1rem' }}>
                          <strong style={{ color: 'var(--emerald)' }}>✓ Expected Result:</strong> {s.expected_result}
                        </div>
                      )}
                    </div>
                  ))}
                </div>
              )}
            </main>
          </>
        )}
      </div>

      {/* Export Preflight & Generation Modal */}
      {isExportOpen && (
        <ExportModal
          project={project}
          onClose={() => setIsExportOpen(false)}
          onExportSuccess={() => {}}
        />
      )}
    </>
  );
};
