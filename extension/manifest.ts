/* ~~/extension/manifest.ts */

// imports
import { defineManifest } from '@crxjs/vite-plugin'
import { version } from './package.json'
import icons from './src/utils/icon'

export default defineManifest({
  action: {
    default_icon: icons.normal,
    default_title: 'Libri Speed Reader',
  },
  background: {
    service_worker: 'src/background/background.ts',
    type: 'module',
  },
  commands: {
    'activate-speed-reader': {
      suggested_key: {
        default: 'Ctrl+Shift+L',
        mac: 'Command+Shift+L',
      },
      description: 'Activate speed reader on selected text',
    },
  },
  content_scripts: [
    {
      js: ['src/content/content.ts'],
      matches: ['<all_urls>'],
      run_at: 'document_end',
      match_about_blank: true,
    },
  ],
  content_security_policy: {
    extension_pages:
      "script-src 'self' 'wasm-unsafe-eval'; style-src 'self' 'unsafe-inline'; default-src 'self';",
  },
  description:
    'A speed reading tool that displays selected text word-by-word at customizable speeds.',
  homepage_url: 'https://github.com/aekasitt/libri',
  icons: icons.normal,
  manifest_version: 3,
  name: 'Libri Speed Reader',
  permissions: ['storage', 'activeTab', 'scripting', 'contextMenus'],
  version,
  web_accessible_resources: [
    {
      resources: ['assets/dictation-web/*'],
      matches: ['<all_urls>'],
    },
  ],
})
