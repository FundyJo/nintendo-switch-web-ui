import react from '@vitejs/plugin-react-swc';
import { defineConfig } from 'vite';

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [react()],
	// For Tauri, we don't need the GitHub Pages base path
	clearScreen: false,
	server: {
		port: 5173,
		strictPort: true,
	},
	envPrefix: ['VITE_', 'TAURI_'],
});
