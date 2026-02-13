/* ~~/extension/dictation.ts */

// imports
import { createPortMessanger } from '../utils/bridge'
import { ConnectionName } from '../utils/constant'

const port = chrome.runtime.connect({ name: ConnectionName.Developer })
const { onPortMessage: fromBackground } = createPortMessanger(port)

let panel: chrome.dictation.panels.ExtensionPanel | null = null

fromBackground((message) => {
  if (message.payload.length === 1 && message.payload[0] === 'OpenDictationPanel') {
    if (panel) {
      return
    }
    chrome.dictation.panels.create('Libri', '', 'index.html', (newPanel) => {
      panel = newPanel
      // TODO pref: hidden and shown
      // panel.onHidden
      // panel.onShown
    })
  }
})
