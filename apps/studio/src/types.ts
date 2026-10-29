export type TimestampMs = number;

export type RedactionType = 
  | { type: 'blackout' }
  | { type: 'pixelate'; block_size: number }
  | { type: 'blur'; radius: number };

export interface Annotation {
  id: string;
  type: 'rectangle' | 'number_badge' | 'arrow' | 'highlight' | 'redaction';
  x: number;
  y: number;
  width?: number;
  height?: number;
  radius?: number;
  number?: number;
  color?: string;
  stroke_width?: number;
  redaction_type?: 'blackout' | 'pixelate' | 'blur';
  label?: string;
}

export interface GuideStep {
  id: string;
  order: number;
  title: string;
  instruction: string;
  expected_result?: string;
  notes?: string;
  frame_id?: string;
  frame_data_url?: string; // base64 or relative path
  timestamp_ms: TimestampMs;
  annotations: Annotation[];
  is_reviewed: boolean;
}

export interface GuideSection {
  id: string;
  title: string;
  order: number;
}

export interface SourceMedia {
  id: string;
  file_name: string;
  relative_path: string;
  duration_ms: TimestampMs;
  width: number;
  height: number;
  frame_rate: number;
  format: string;
}

export interface ExportProfile {
  title_override?: string;
  include_timestamps: boolean;
  include_toc: boolean;
  safe_redaction_mode: boolean;
  theme: string;
}

export interface Project {
  id: string;
  title: string;
  description?: string;
  schema_version: number;
  created_at: string;
  updated_at: string;
  sources: SourceMedia[];
  sections: GuideSection[];
  steps: GuideStep[];
  export_profile: ExportProfile;
}

export type ViewMode = 'guide' | 'timeline' | 'preview';
