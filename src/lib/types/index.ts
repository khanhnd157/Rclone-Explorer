export interface Remote {
  name: string;
  provider: string;
  status: 'Connected' | 'Error' | 'Needs reconnect';
  lastUsed?: string;
}

export interface FileItem {
  name: string;
  path: string;
  size: number;
  modified: string;
  is_dir: boolean;
  mime_type?: string;
}

export interface Job {
  id: string;
  operation: string;
  source: string;
  destination: string;
  status: 'running' | 'completed' | 'failed' | 'paused';
  progress: number;
  speed: string;
  eta?: string;
  started: string;
  finished?: string;
}

export interface PanelState {
  remote: string;
  path: string;
  files: FileItem[];
  selectedFiles: Set<string>;
  history: string[];
  historyIndex: number;
}

export interface TabState {
  id: string;
  name: string;
  leftPanel: PanelState;
  rightPanel: PanelState;
  activePanel: 'left' | 'right';
}

export interface CopyOptions {
  overwrite: boolean;
  skip_existing: boolean;
}

export interface ProviderInfo {
  id: string;
  name: string;
  icon: string;
  description: string;
  requiresOAuth: boolean;
}

export interface RcloneInfo {
  version: string | null;
  path: string;
  installed: boolean;
}
