/* ~~/extension/src/content/inject.ts */

// imports
import { createMessage } from '../utils/message'

if (Reflect.get(window, '__LIBRI_DICTATION__') === true) {
  if (document.visibilityState === 'hidden' && document.readyState === 'complete') {
    document.addEventListener(
      'visibilitychange',
      () => {
        if (document.visibilityState === 'visible') {
          register_leptos()
        }
      },
      {
        once: true,
      },
    )
  } else {
    register_leptos()
  }
}

function register_leptos() {
  console.log(
    '🚧%c%s%c%s',
    'background-color: #ff7f00; color: #fff; border-radius: 3px; padding: 1px 4px',
    'libri-dictation',
    '',
    ' is in early development! Please report any bugs to https://github.com/aekasitt/libri/issues',
  )
  window.postMessage(createMessage([]))
}
