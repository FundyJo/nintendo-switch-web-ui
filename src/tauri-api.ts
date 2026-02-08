import { invoke } from '@tauri-apps/api/core';

export interface Game {
  title: string;
  icon_path: string;
  rom_path: string;
  title_id: string | null;
}

export async function getGames(): Promise<Game[]> {
  try {
    const games = await invoke<Game[]>('get_games');
    return games;
  } catch (error) {
    console.error('Failed to get games:', error);
    return [];
  }
}

export async function scanDirectory(directory: string): Promise<Game[]> {
  try {
    const games = await invoke<Game[]>('scan_directory', { directory });
    return games;
  } catch (error) {
    console.error('Failed to scan directory:', error);
    return [];
  }
}

export async function launchGame(emulatorPath: string, romPath: string): Promise<void> {
  try {
    await invoke('launch_game', { emulatorPath, romPath });
  } catch (error) {
    console.error('Failed to launch game:', error);
    throw error;
  }
}
