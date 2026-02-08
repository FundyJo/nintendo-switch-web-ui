import { invoke } from '@tauri-apps/api/core';
import { proxy } from 'valtio';

export interface Game {
	id: string;
	title: string;
	path: string;
	icon: string | null;
	emulator: string;
}

const state = proxy({
	selectedTitle: null as null | number,
	games: [] as Game[],
	loading: false,
	error: null as string | null,
});

export default state;

// Game management functions
export async function scanGames(): Promise<void> {
	state.loading = true;
	state.error = null;
	try {
		const games = await invoke<Game[]>('scan_games');
		state.games = games;
	} catch (err) {
		state.error = err as string;
		console.error('Failed to scan games:', err);
	} finally {
		state.loading = false;
	}
}

export async function getGames(): Promise<void> {
	state.loading = true;
	state.error = null;
	try {
		const games = await invoke<Game[]>('get_games');
		state.games = games;
	} catch (err) {
		state.error = err as string;
		console.error('Failed to get games:', err);
	} finally {
		state.loading = false;
	}
}

export async function addGame(title: string, path: string, emulator: string): Promise<void> {
	state.loading = true;
	state.error = null;
	try {
		const game = await invoke<Game>('add_game', { title, path, emulator });
		state.games.push(game);
	} catch (err) {
		state.error = err as string;
		console.error('Failed to add game:', err);
	} finally {
		state.loading = false;
	}
}

export async function launchGame(game: Game): Promise<void> {
	try {
		await invoke('launch_game_cmd', { game });
	} catch (err) {
		const message = String(err ?? '');
		if (message.includes('Game already running')) {
			return;
		}
		console.error('Failed to launch game:', err);
		alert(`Failed to launch game: ${err}`);
	}
}

// Backward compatibility with old tiles format
export const IMAGE_RES = 512;
export const IMAGE_RES_STR = `${IMAGE_RES}/${IMAGE_RES}`;

// No default games - only load from emulators
export const defaultGames: Game[] = [];
