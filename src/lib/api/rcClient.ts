import { invoke } from '@tauri-apps/api/core';
import type { Remote, FileItem, Job, CopyOptions, RcloneInfo } from '$lib/types';

export async function listRemotes(): Promise<Remote[]> {
  return await invoke('list_remotes');
}

export async function listDir(remote: string, path: string): Promise<FileItem[]> {
  return await invoke('list_dir', { remote, path });
}

export async function copyItems(
  fromRemote: string,
  fromPaths: string[],
  toRemote: string,
  toPath: string,
  options: CopyOptions
): Promise<string> {
  return await invoke('copy_items', { fromRemote, fromPaths, toRemote, toPath, options });
}

export async function moveItems(
  fromRemote: string,
  fromPaths: string[],
  toRemote: string,
  toPath: string
): Promise<string> {
  return await invoke('move_items', { fromRemote, fromPaths, toRemote, toPath });
}

export async function deleteItems(remote: string, paths: string[]): Promise<void> {
  await invoke('delete_items', { remote, paths });
}

export async function getJobs(): Promise<Job[]> {
  return await invoke('get_jobs');
}

export async function getJob(jobId: string): Promise<Job | null> {
  return await invoke('get_job', { jobId });
}

export async function configList(): Promise<Remote[]> {
  return await invoke('config_list');
}

export async function configCreate(
  name: string,
  provider: string,
  params: Record<string, string>
): Promise<void> {
  await invoke('config_create', { name, provider, params });
}

export async function configReconnect(name: string): Promise<void> {
  await invoke('config_reconnect', { name });
}

export async function configDelete(name: string): Promise<void> {
  await invoke('config_delete', { name });
}

export async function checkRcloneVersion(): Promise<RcloneInfo> {
  return await invoke('check_rclone_version');
}

export async function installRclone(): Promise<string> {
  return await invoke('install_rclone');
}

export async function updateRclone(): Promise<string> {
  return await invoke('update_rclone');
}
