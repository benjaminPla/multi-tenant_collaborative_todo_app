import react from '@vitejs/plugin-react'
import { defineConfig, type UserConfig } from 'vite'

export default defineConfig(({ command }): UserConfig => {
  if (command === 'build') {
    return { plugins: [react()] }
  }

  const target = process.env.BACKEND_URL
  if (!target) {
    throw new Error('BACKEND_URL is required for the dev server')
  }

  return {
    plugins: [react()],
    server: {
      proxy: {
        '/api': { rewrite: (path) => path.replace(/^\/api/, ''), target },
      },
    },
  }
})
