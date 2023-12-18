import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

export default defineConfig(async () => ({
	plugins: [react()],
	clearScreen: false,
	server: {
		port: 7591,
		strictPort: true,
	},
}));
