import { proxy } from 'valtio';

const state = proxy({
	selectedTitle: null as null | number,
});

export default state;

// Empty tiles array - games should be loaded from emulators in Tauri mode
export const tiles: { img: string; title: string }[] = [];
