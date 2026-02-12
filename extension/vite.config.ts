/* ~~/extension/vite.config.ts */

// imports
import { UserConfig, defineConfig } from 'vite'
import { crx } from '@crxjs/vite-plugin'
import manifest from './manifest'
import { viteStaticCopy as copy } from 'vite-plugin-static-copy'

export default defineConfig(({ mode }) => {
  const config: UserConfig = {
    plugins: [
      copy({
        targets: [
          {
            src: '../core/dist/**/*',
            dest: 'assets/dictation-web',
          },
        ],
      }),
      crx({ manifest }),
    ],
  }
  if (mode === 'development') {
    config.build!.minify = false
  }
  return config
})
