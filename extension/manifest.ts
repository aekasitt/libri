/* ~~/libri/manifest.ts */

import { defineManifest } from '@crxjs/vite-plugin'
import { version } from './package.json'
import icons from './utils/icon'

export default defineManifest({
  manifest_version: 3,
  name: 'Libri Speed Reader',
  version,
  description: 'A speed reading tool that displays selected text word-by-word at customizable speeds.',
  homepage_url: 'https://github.com/aekasitt/libri',
  permissions: ['storage', 'activeTab', 'scripting', 'contextMenus'],
  content_scripts: [
    {
      js: ['content/content.ts'],
      matches: ['<all_urls>'],
      run_at: 'document_end',
      match_about_blank: true,
    },
  ],
  background: {
    service_worker: 'background/background.ts',
    type: 'module',
  },
  content_security_policy: {
    extension_pages:
      'script-src \'self\' \'wasm-unsafe-eval\'; style-src \'self\' \'unsafe-inline\'; default-src \'self\';',
  },
  web_accessible_resources: [
    {
      resources: [
        'assets/dictation-web/*'
      ],
      matches: ['<all_urls>']
    }
  ],
  action: {
    default_icon: icons.normal,
    default_title: 'Libri Speed Reader',
  },
  commands: {
    'activate-speed-reader': {
      suggested_key: {
        default: 'Ctrl+Shift+L',
        mac: 'Command+Shift+L'
      },
      description: 'Activate speed reader on selected text'
    }
  },
  icons: icons.normal,
})
