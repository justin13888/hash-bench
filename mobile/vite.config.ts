import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

// Tauri expects a fixed dev-server port and reads TAURI_DEV_HOST so a device or
// emulator on the LAN can reach the bundler during `tauri android dev`.
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig({
	plugins: [react(), tailwindcss()],
	// Tauri serves the built assets from a custom protocol, so use relative paths.
	base: "./",
	// Prevent Vite from clearing Rust compiler output in the terminal.
	clearScreen: false,
	server: {
		port: 1420,
		strictPort: true,
		host: host || false,
		hmr: host
			? {
					protocol: "ws",
					host,
					port: 1421,
				}
			: undefined,
		watch: {
			// Don't watch the Rust core — it's rebuilt by the Tauri CLI.
			ignored: ["**/src-tauri/**"],
		},
	},
});
