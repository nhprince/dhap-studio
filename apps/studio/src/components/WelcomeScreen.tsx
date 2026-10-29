import React from 'react';
import { Video, Upload, Sparkles, Image, ShieldCheck, HardDrive } from 'lucide-react';

interface WelcomeScreenProps {
  onLoadSample: () => void;
  onImportMedia: () => void;
  onNewEmpty: () => void;
}

export const WelcomeScreen: React.FC<WelcomeScreenProps> = ({
  onLoadSample,
  onImportMedia,
  onNewEmpty,
}) => {
  return (
    <div style={{
      display: 'flex',
      flexDirection: 'column',
      alignItems: 'center',
      justifyContent: 'center',
      height: '100%',
      padding: '2rem',
      background: 'radial-gradient(circle at 50% 30%, rgba(225, 29, 72, 0.08) 0%, transparent 60%)'
    }}>
      <div style={{ textAlign: 'center', maxWidth: 640, marginBottom: '2.5rem' }}>
        <div style={{
          display: 'inline-flex',
          alignItems: 'center',
          justifyContent: 'center',
          width: 56,
          height: 56,
          background: 'var(--accent)',
          borderRadius: 14,
          fontSize: '2rem',
          color: '#fff',
          fontWeight: 800,
          marginBottom: '1rem',
          boxShadow: '0 8px 24px var(--accent-glow)'
        }}>
          ধ
        </div>
        <h1 style={{ fontSize: '2.4rem', fontWeight: 800, marginBottom: '0.75rem', letterSpacing: '-0.03em' }}>
          Welcome to Dhap Studio <span style={{ color: 'var(--accent)' }}>ধাপ</span>
        </h1>
        <p style={{ fontSize: '1.15rem', color: 'var(--text-muted)', lineHeight: 1.5 }}>
          Record once. Create the video and guide together. Transform workflow recordings into polished, illustrated documentation.
        </p>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(240px, 1fr))', gap: '1.25rem', width: '100%', maxWidth: 780, marginBottom: '2.5rem' }}>
        <button
          onClick={onImportMedia}
          style={{
            background: 'var(--bg-card)',
            border: '1px solid var(--border)',
            borderRadius: 'var(--radius-lg)',
            padding: '1.75rem 1.5rem',
            textAlign: 'left',
            cursor: 'pointer',
            transition: 'all 0.2s',
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.borderColor = 'var(--accent)';
            e.currentTarget.style.transform = 'translateY(-2px)';
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.borderColor = 'var(--border)';
            e.currentTarget.style.transform = 'none';
          }}
        >
          <div style={{ width: 44, height: 44, borderRadius: 10, background: 'rgba(225, 29, 72, 0.15)', color: 'var(--accent)', display: 'flex', alignItems: 'center', justifyContent: 'center', marginBottom: '1rem' }}>
            <Upload size={24} />
          </div>
          <h3 style={{ fontSize: '1.1rem', marginBottom: '0.4rem' }}>Import Recording</h3>
          <p style={{ fontSize: '0.85rem', color: 'var(--text-muted)' }}>Load an MP4, WebM, or MOV to extract timeline keyframes automatically.</p>
        </button>

        <button
          onClick={onLoadSample}
          style={{
            background: 'var(--bg-card)',
            border: '1px solid var(--accent)',
            borderRadius: 'var(--radius-lg)',
            padding: '1.75rem 1.5rem',
            textAlign: 'left',
            cursor: 'pointer',
            transition: 'all 0.2s',
            boxShadow: '0 4px 14px var(--accent-glow)'
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.transform = 'translateY(-2px)';
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.transform = 'none';
          }}
        >
          <div style={{ width: 44, height: 44, borderRadius: 10, background: 'var(--accent)', color: '#fff', display: 'flex', alignItems: 'center', justifyContent: 'center', marginBottom: '1rem' }}>
            <Sparkles size={24} />
          </div>
          <h3 style={{ fontSize: '1.1rem', marginBottom: '0.4rem' }}>Open Sample Project</h3>
          <p style={{ fontSize: '0.85rem', color: 'var(--text-muted)' }}>Explore a pre-built illustrated walkthrough with screenshots, badges, and redaction masks.</p>
        </button>

        <button
          onClick={onNewEmpty}
          style={{
            background: 'var(--bg-card)',
            border: '1px solid var(--border)',
            borderRadius: 'var(--radius-lg)',
            padding: '1.75rem 1.5rem',
            textAlign: 'left',
            cursor: 'pointer',
            transition: 'all 0.2s',
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.borderColor = 'var(--border-focus)';
            e.currentTarget.style.transform = 'translateY(-2px)';
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.borderColor = 'var(--border)';
            e.currentTarget.style.transform = 'none';
          }}
        >
          <div style={{ width: 44, height: 44, borderRadius: 10, background: 'rgba(56, 189, 248, 0.15)', color: 'var(--blue)', display: 'flex', alignItems: 'center', justifyContent: 'center', marginBottom: '1rem' }}>
            <Image size={24} />
          </div>
          <h3 style={{ fontSize: '1.1rem', marginBottom: '0.4rem' }}>Create Empty Guide</h3>
          <p style={{ fontSize: '0.85rem', color: 'var(--text-muted)' }}>Start from scratch and paste screenshots or write step-by-step instructions directly.</p>
        </button>
      </div>

      <div style={{ display: 'flex', gap: '2rem', color: 'var(--text-dim)', fontSize: '0.85rem' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '0.4rem' }}>
          <HardDrive size={16} />
          <span>100% Local-First & Offline</span>
        </div>
        <div style={{ display: 'flex', alignItems: 'center', gap: '0.4rem' }}>
          <ShieldCheck size={16} />
          <span>True Privacy Redaction</span>
        </div>
        <div style={{ display: 'flex', alignItems: 'center', gap: '0.4rem' }}>
          <Video size={16} />
          <span>No Accounts or Paywalls</span>
        </div>
      </div>
    </div>
  );
};
