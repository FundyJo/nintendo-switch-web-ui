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
		console.error('Failed to launch game:', err);
		alert(`Failed to launch game: ${err}`);
	}
}

// Backward compatibility with old tiles format
export const IMAGE_RES = 512;
export const IMAGE_RES_STR = `${IMAGE_RES}/${IMAGE_RES}`;

// Default example games (will be replaced when real games are loaded)
export const defaultGames: Game[] = [
	{ id: '1', title: 'The Legend of Zelda: Breath of the Wild', path: '', icon: `https://tinfoil.media/ti/01007EF00011E000/${IMAGE_RES_STR}`, emulator: 'yuzu' },
	{ id: '2', title: 'Super Smash Bros. Ultimate', path: '', icon: `https://tinfoil.media/ti/01006A800016E000/${IMAGE_RES_STR}`, emulator: 'yuzu' },
	{ id: '3', title: 'Super Mario Odyssey', path: '', icon: `https://tinfoil.media/ti/0100000000010000/${IMAGE_RES_STR}`, emulator: 'yuzu' },
	{ id: '4', title: 'Animal Crossing: New Horizons', path: '', icon: `https://tinfoil.media/ti/01006F8002326000/${IMAGE_RES_STR}`, emulator: 'yuzu' },
	{ id: '5', title: 'Mario Party Superstars', path: '', icon: `https://tinfoil.media/ti/01006FE013472000/${IMAGE_RES_STR}`, emulator: 'yuzu' },
	{ id: '6', title: 'Mario Kart 8 Deluxe', path: '', icon: `https://tinfoil.media/ti/0100152000022000/${IMAGE_RES_STR}`, emulator: 'yuzu' },
	{ id: '7', title: 'Donkey Kong Country: Tropical Freeze', path: '', icon: `https://tinfoil.media/ti/0100C1F0051B6000/${IMAGE_RES_STR}`, emulator: 'yuzu' },
	{ id: '8', title: 'Hades', path: '', icon: `https://tinfoil.media/ti/0100535012974000/${IMAGE_RES_STR}`, emulator: 'yuzu' },
	{ id: '9', title: 'Luigi\'s Mansion 3', path: '', icon: `https://tinfoil.media/ti/0100DCA0064A6000/${IMAGE_RES_STR}`, emulator: 'yuzu' },
	{ id: '10', title: 'Mario Strikers: Battle League', path: '', icon: `https://tinfoil.media/ti/010019401051C000/${IMAGE_RES_STR}`, emulator: 'yuzu' },
	{ id: '11', title: 'Pikmin 4', path: '', icon: `https://tinfoil.media/ti/0100B7C00933A000/${IMAGE_RES_STR}`, emulator: 'yuzu' },
	{ id: '12', title: 'Super Mario 3D World + Bowser\'s Fury', path: '', icon: `https://tinfoil.media/ti/010028600EBDA000/${IMAGE_RES_STR}`, emulator: 'yuzu' },
	{ id: '13', title: 'Kirby and the Forgotten Land', path: '', icon: `https://tinfoil.media/ti/01004D300C5AE000/${IMAGE_RES_STR}`, emulator: 'yuzu' },
	{ id: '14', title: 'Portal 2', path: '', icon: `https://tinfoil.media/ti/0100ABD01785C000/${IMAGE_RES_STR}`, emulator: 'yuzu' },
	{ id: '15', title: 'Pico Park', path: '', icon: `https://tinfoil.media/ti/01004EA00DF70000/${IMAGE_RES_STR}`, emulator: 'yuzu' },
	{ id: '16', title: 'Sea of Stars', path: '', icon: `https://tinfoil.media/ti/01008C0016544000/${IMAGE_RES_STR}`, emulator: 'yuzu' },
	{ id: '17', title: 'Super Mario 3D All-Stars', path: '', icon: `https://tinfoil.media/ti/010049900F546000/${IMAGE_RES_STR}`, emulator: 'yuzu' },
];
