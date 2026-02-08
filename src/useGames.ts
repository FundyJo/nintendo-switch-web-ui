import { useEffect, useState } from 'react';
import state from './state';
import { getGames, launchGame } from './tauri-api';

// Check if we're running in Tauri
const isTauri = () => {
  return typeof window !== 'undefined' && '__TAURI__' in window;
};

export function useGames() {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!isTauri()) {
      // Running in web browser, use default games
      return;
    }

    // Running in Tauri, load games from emulators
    const loadGames = async () => {
      setIsLoading(true);
      setError(null);
      
      try {
        const games = await getGames();
        
        if (games.length > 0) {
          // Convert backend games to frontend tiles
          const tiles = games.map(game => ({
            img: game.icon_path ? `asset://localhost/${game.icon_path}` : '/vite.svg',
            title: game.title,
            romPath: game.rom_path,
            titleId: game.title_id,
          }));
          
          state.tiles = tiles;
        }
        // If no games found, keep default tiles
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load games');
        console.error('Error loading games:', err);
      } finally {
        setIsLoading(false);
      }
    };

    loadGames();
  }, []);

  return { isLoading, error, isTauri: isTauri() };
}

export async function handleLaunchGame(gameIndex: number) {
  if (!isTauri()) {
    console.log('Cannot launch games in web version');
    return;
  }

  const game = state.tiles[gameIndex];
  if (!game?.romPath) {
    console.error('No ROM path for this game');
    return;
  }

  // Get emulator path from state or use default
  // User will need to set this in settings
  const emulatorPath = state.emulatorPath || detectEmulatorPath();
  
  if (!emulatorPath) {
    console.error('No emulator configured');
    alert('Please configure an emulator path in settings');
    return;
  }

  try {
    await launchGame(emulatorPath, game.romPath);
  } catch (error) {
    console.error('Failed to launch game:', error);
    alert('Failed to launch game: ' + error);
  }
}

// Try to detect emulator path based on platform
function detectEmulatorPath(): string {
  // This is a placeholder - user should configure this
  // We could add auto-detection logic here
  return '';
}
