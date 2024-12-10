import {defineConfig} from "vite";
import react from "@vitejs/plugin-react";

const port = 3000;
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
        host: true,
        port: port,
        strictPort: true,
        hmr: {
            protocol: "ws",
            host: "localhost",
            port: port,
            clientPort: port,
            overlay: true
        },
        watch: {
            ignored: ["**/src-*/**"]
        }
    },
    build: {
        outDir: "target/wwwroot"
    }
});