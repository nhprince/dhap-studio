import { Project } from '../types';

export const createSampleProject = (): Project => {
  // Generate sample SVG base64 screenshot data URLs for an instant working demo
  const sampleFrame1Svg = `data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" width="800" height="450" viewBox="0 0 800 450">
    <rect width="800" height="450" fill="%230f172a"/>
    <rect width="800" height="40" fill="%231e293b"/>
    <circle cx="20" cy="20" r="6" fill="%23ef4444"/>
    <circle cx="38" cy="20" r="6" fill="%23f59e0b"/>
    <circle cx="56" cy="20" r="6" fill="%2310b981"/>
    <text x="80" y="25" fill="%2394a3b8" font-family="sans-serif" font-size="12">Cloud Console — Dashboard Overview</text>
    <rect x="40" y="70" width="220" height="150" rx="8" fill="%231e293b"/>
    <rect x="280" y="70" width="480" height="150" rx="8" fill="%231e293b"/>
    <text x="60" y="105" fill="%23f8fafc" font-family="sans-serif" font-weight="bold" font-size="16">Organizations</text>
    <text x="60" y="135" fill="%2394a3b8" font-family="sans-serif" font-size="13">Active Projects: 12</text>
    <text x="300" y="105" fill="%23f8fafc" font-family="sans-serif" font-weight="bold" font-size="16">Recent Deployments</text>
    <text x="300" y="135" fill="%2310b981" font-family="sans-serif" font-size="13">✓ Production (v0.1.0) — Deployed 2h ago</text>
  </svg>`;

  const sampleFrame2Svg = `data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" width="800" height="450" viewBox="0 0 800 450">
    <rect width="800" height="450" fill="%230f172a"/>
    <rect width="800" height="40" fill="%231e293b"/>
    <circle cx="20" cy="20" r="6" fill="%23ef4444"/>
    <circle cx="38" cy="20" r="6" fill="%23f59e0b"/>
    <circle cx="56" cy="20" r="6" fill="%2310b981"/>
    <text x="80" y="25" fill="%2394a3b8" font-family="sans-serif" font-size="12">Security Settings — Secret Tokens</text>
    <rect x="60" y="80" width="680" height="280" rx="8" fill="%231e293b"/>
    <text x="90" y="120" fill="%23f8fafc" font-family="sans-serif" font-weight="bold" font-size="16">API Access Tokens</text>
    <rect x="90" y="145" width="400" height="38" rx="6" fill="%230f172a" stroke="%23334155"/>
    <text x="105" y="169" fill="%23e2e8f0" font-family="monospace" font-size="13">sk_live_51Mza992384918234812391</text>
    <rect x="505" y="145" width="100" height="38" rx="6" fill="%23e11d48"/>
    <text x="535" y="169" fill="%23ffffff" font-family="sans-serif" font-weight="bold" font-size="13">Copy</text>
  </svg>`;

  return {
    id: 'proj_sample_001',
    title: 'Deploying Production Services with Dhap',
    description: 'A verified step-by-step workflow for configuring security tokens and deploying documentation.',
    schema_version: 1,
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
    sources: [
      {
        id: 'media_sample_01',
        file_name: 'deployment_workflow.mp4',
        relative_path: 'media/deployment_workflow.mp4',
        duration_ms: 45000,
        width: 1920,
        height: 1080,
        frame_rate: 30.0,
        format: 'mp4',
      },
    ],
    sections: [
      {
        id: 'sec_01',
        title: 'Initial Dashboard Setup',
        order: 1,
      },
      {
        id: 'sec_02',
        title: 'Security & Access Keys',
        order: 2,
      },
    ],
    steps: [
      {
        id: 'step_sample_1',
        order: 1,
        title: 'Access Organization Overview',
        instruction: 'Open the cloud management console and confirm that all active cluster services report healthy status.',
        expected_result: 'The overview dashboard displays green status indicators across all deployment targets.',
        notes: 'If any cluster reports degraded status, resolve connectivity before proceeding.',
        timestamp_ms: 4500,
        frame_data_url: sampleFrame1Svg,
        is_reviewed: true,
        annotations: [
          {
            id: 'anno_1',
            type: 'number_badge',
            x: 60,
            y: 95,
            radius: 14,
            number: 1,
            color: '#e11d48',
          },
          {
            id: 'anno_2',
            type: 'rectangle',
            x: 40,
            y: 70,
            width: 220,
            height: 150,
            stroke_width: 2,
            color: '#38bdf8',
          },
        ],
      },
      {
        id: 'step_sample_2',
        order: 2,
        title: 'Generate and Redact API Access Token',
        instruction: 'Navigate to Security Settings and generate a deployment access token. Apply privacy redaction before exporting.',
        expected_result: 'A new 256-bit API secret token is generated and immediately masked in exported documentation.',
        notes: 'The live secret token is permanently blacked out on export to prevent credential leakage.',
        timestamp_ms: 18200,
        frame_data_url: sampleFrame2Svg,
        is_reviewed: true,
        annotations: [
          {
            id: 'anno_3',
            type: 'redaction',
            x: 90,
            y: 145,
            width: 400,
            height: 38,
            redaction_type: 'blackout',
          },
          {
            id: 'anno_4',
            type: 'number_badge',
            x: 520,
            y: 160,
            radius: 14,
            number: 2,
            color: '#e11d48',
          },
        ],
      },
    ],
    export_profile: {
      include_timestamps: true,
      include_toc: true,
      safe_redaction_mode: true,
      theme: 'auto',
    },
  };
};
