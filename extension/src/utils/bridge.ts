/* ~~/extension/src/utils/bridge.ts */

/**
 *
 */
export function createPortMessanger(port: chrome.runtime.Port) {
  const listeners: Array<(message: any, port: chrome.runtime.Port) => void> = []
  const disconnectListeners: Array<() => void> = []

  let connected = true
  port.onDisconnect.addListener(() => {
    console.log(`${port.name} port disconnected.`)
    connected = false
    listeners.splice(0, listeners.length)
    port.onMessage.removeListener(onMessage)
    disconnectListeners.forEach((fn) => fn()) // FIXME: should not return value
    disconnectListeners.splice(0, disconnectListeners.length)
  })

  // FIXME: any -> typed
  function onMessage(message: any, port: chrome.runtime.Port) {
    listeners.forEach((fn) => fn(message, port)) // FIXME: should not return value
  }
  port.onMessage.addListener(onMessage)

  return {
    postPortMessage: (message: any) => {
      if (!connected) return
      port.postMessage(message)
    },
    onPortMessage: (handler: (message: any, port: chrome.runtime.Port) => void) => {
      if (!connected) return

      listeners.push(handler)
    },
    onDisconnect: (handler: () => void) => {
      if (!connected) return

      disconnectListeners.push(handler)
    },
  }
}
