import {defineConfig} from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
    plugins: [react()],
    esbuild: {
        legalComments: "none",
        supported: {
            "top-level-await": true // browsers can handle top-level-await features
        }
    },
    clearScreen: false,
    server: {
        port: 1420,
        strictPort: true,
        watch: {
            ignored: ["**/src-*/**"]
        }
    },
    build: {
        outDir: "dist/wwwroot",
    }
});
