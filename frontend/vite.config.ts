import {defineConfig} from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
    build: {
        outDir: '../backend/static'
    },
    plugins: [react()],
    server: {
        proxy: {
            '/api': 'http://localhost:8000',
            '/socket.io': {
                target: 'ws://localhost:8000',
                ws: true,
                rewriteWsOrigin: true,
            },
        },
    }
})
