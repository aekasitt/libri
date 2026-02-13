/* ~~/extension/src/content/content.ts */

// Dynamic import of WASM module
let wasmInitialized = false
let speedReaderModule = null

// Create root element with Shadow DOM for style isolation
function setupDOM(): ShadowRoot {
  // Create host element if it doesn't exist
  let host = document.getElementById('libri-speed-reader-host')
  if (!host) {
    host = document.createElement('div')
    host.id = 'libri-speed-reader-host'
    document.body.appendChild(host)
  }

  // Attach shadow root (if not already attached)
  let shadowRoot = host.shadowRoot
  if (!shadowRoot) {
    shadowRoot = host.attachShadow({ mode: 'open' })

    // Create the actual root element inside shadow DOM
    const root = document.createElement('div')
    root.id = 'libri-speed-reader-root'
    shadowRoot.appendChild(root)

    // Inject styles into shadow DOM (not document.head!)
    const indexStyle = document.createElement('link')
    indexStyle.rel = 'stylesheet'
    indexStyle.href = chrome.runtime.getURL('assets/dictation-web/index.css')
    shadowRoot.appendChild(indexStyle)

    // Optional: uno.css if it exists
    const unoStyle = document.createElement('link')
    unoStyle.rel = 'stylesheet'
    unoStyle.href = chrome.runtime.getURL('assets/dictation-web/uno.css')
    shadowRoot.appendChild(unoStyle)
  }

  return shadowRoot
}

async function initializeWasm() {
  if (wasmInitialized) {
    return
  }

  try {
    // Setup Shadow DOM and expose it globally for WASM
    const shadowRoot = setupDOM()
    ;(window as any).__LIBRI_SHADOW_ROOT__ = shadowRoot

    // Get URLs for WASM files
    const wasmUrl = chrome.runtime.getURL('assets/dictation-web/libri_bg.wasm')
    const moduleUrl = chrome.runtime.getURL('assets/dictation-web/libri.js')

    // Dynamically import the module
    speedReaderModule = await import(/* @vite-ignore */ moduleUrl)

    // Initialize WASM using modern syntax (pass object instead of string)
    // This will call main() which mounts the Leptos app
    await speedReaderModule.default({ module_or_path: wasmUrl })

    wasmInitialized = true
  } catch (error) {
    console.error('[Libri] Failed to initialize WASM:', error)
    throw error
  }
}

/**
 * Initialize WebAssembly which will setup DOM
 */
initializeWasm().catch((err) => {
  console.error('[Libri] Initialization failed:', err)
})

/**
 * Listen for activation from background script
 */
chrome.runtime.onMessage.addListener((message, _sender, sendResponse) => {
  if (message.type === 'ACTIVATE_SPEED_READER') {
    handleActivation(message.text)
      .then(() => {
        sendResponse({ success: true })
      })
      .catch((err) => {
        console.error('[Libri] Activation failed:', err)
        sendResponse({ success: false, error: err.message })
      })
    return true // Keep channel open for async response
  }
})

/**
 * Ensure WebAssembly module is initialized then trigger the Dictation modal
 * embedded with Leptos application listening to the selected texts
 * @param text {string}
 */
async function handleActivation(text: string) {
  if (!wasmInitialized) {
    await initializeWasm()
  }
  window.postMessage({ text: text, type: 'LIBRI_TEXT' }, '*')
}
