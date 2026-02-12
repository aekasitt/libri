// Simple approach: Import and initialize WASM directly in content script
// No page context injection, no context isolation issues

// Dynamic import of WASM module
let wasmInitialized = false;
let speedReaderModule: any = null;

// Create root element and inject styles FIRST
function setupDOM() {
    // Create root element if it doesn't exist
    let root = document.getElementById('libri-speed-reader-root');
    if (!root) {
        root = document.createElement('div');
        root.id = 'libri-speed-reader-root';
        document.body.appendChild(root);
    }
    
    // Inject styles
    if (!document.querySelector('link[href*="dictation-web/index.css"]')) {
        const indexStyle = document.createElement('link');
        indexStyle.rel = 'stylesheet';
        indexStyle.href = chrome.runtime.getURL('assets/dictation-web/index.css');
        document.head.appendChild(indexStyle);
    }
    
    if (!document.querySelector('link[href*="dictation-web/uno.css"]')) {
        const unoStyle = document.createElement('link');
        unoStyle.rel = 'stylesheet';
        unoStyle.href = chrome.runtime.getURL('assets/dictation-web/uno.css');
        document.head.appendChild(unoStyle);
    }
}

async function initializeWasm() {
    if (wasmInitialized) {
        return;
    }

    try {
        // Get URLs for WASM files
        const wasmUrl = chrome.runtime.getURL('assets/dictation-web/libri_bg.wasm');
        const moduleUrl = chrome.runtime.getURL('assets/dictation-web/libri.js');
        
        // Dynamically import the module
        speedReaderModule = await import(/* @vite-ignore */ moduleUrl);
        
        // Initialize WASM using modern syntax (pass object instead of string)
        // This will call main() which mounts the Leptos app
        await speedReaderModule.default({ module_or_path: wasmUrl });
        
        wasmInitialized = true;
        
    } catch (error) {
        console.error('[Libri] Failed to initialize WASM:', error);
        throw error;
    }
}

// Setup DOM first, then initialize WASM
setupDOM();
initializeWasm().catch(err => {
    console.error('[Libri] Initialization failed:', err);
});

// Listen for activation from background script
chrome.runtime.onMessage.addListener((message, _sender, sendResponse) => {
  if (message.type === 'ACTIVATE_SPEED_READER') {
    handleActivation(message.text).then(() => {
      sendResponse({ success: true });
    }).catch(err => {
      console.error('[Libri] Activation failed:', err);
      sendResponse({ success: false, error: err.message });
    });
      return true; // Keep channel open for async response
  }
});

async function handleActivation(text: string) {
    // Ensure WASM is initialized
    if (!wasmInitialized) {
        await initializeWasm();
    }
    
    // Trigger the speed reader with the text
    // The Leptos app is listening for this message
    window.postMessage({
        type: 'LIBRI_TEXT',
        text: text
    }, '*');
}
