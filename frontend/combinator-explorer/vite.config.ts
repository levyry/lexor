import { defineConfig } from 'vite';
import preact from '@preact/preset-vite';
import wasm from 'vite-plugin-wasm';
import tailwindcss from '@tailwindcss/vite';
import { VitePWA } from 'vite-plugin-pwa';

export default defineConfig({
	plugins: [
		preact(),
		wasm(),
		tailwindcss(),
		VitePWA({
			registerType: 'autoUpdate',
			injectRegister: 'auto',
			workbox: {
				globPatterns: ['**/*.{js,css,html,ico,png,svg,wasm}']
			},
			manifest: {
				name: 'Combinator Explorer',
				short_name: 'Lexor',
				description: 'A visual SKI and lambda calculus evaluator and converted',
				theme_color: '#1e1e1e',
				background_color: '#1e1e1e',
				display: 'standalone',
				icons: [
					{
						src: 'pfp2.png',
						sizes: '192x192',
						type: 'image/png'
					},
					{
						src: 'pfp22.png',
						sizes: '512x512',
						type: 'image/png'
					}
				]
			}
		})
	],
	build: {
		target: 'esnext'
	}
});