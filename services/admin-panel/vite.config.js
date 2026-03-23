import vue from '@vitejs/plugin-vue'
import path from 'path'
import { defineConfig } from 'vite'
import vuetify from 'vite-plugin-vuetify'

export default defineConfig({
    plugins: [vue(), vuetify({ autoImport: true })],
    resolve: {
        alias: {
            '@': path.resolve(__dirname, './src'),
        },
    },
    server: {
        port: 8080,
        host: '0.0.0.0',
    },
    build: {
        outDir: 'dist',
        sourcemap: false,
        minify: 'terser',
    },
})
