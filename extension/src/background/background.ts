// Activate speed reader when extension icon is clicked
chrome.action.onClicked.addListener(async (tab) => {
  if (!tab.id) return

  try {
    // Get selected text
    const results = await chrome.scripting.executeScript({
      target: { tabId: tab.id },
      func: () => window.getSelection()?.toString() || '',
    })

    const text = results[0]?.result?.trim()

    if (text) {
      // Send message to content script
      await chrome.tabs.sendMessage(tab.id, {
        type: 'ACTIVATE_SPEED_READER',
        text: text,
      })
    } else {
      await chrome.scripting.executeScript({
        target: { tabId: tab.id },
        func: () => alert('Please select some text first!'),
      })
    }
  } catch (err) {
    console.error('Failed to activate speed reader:', err)
  }
})

// Handle keyboard shortcut
chrome.commands.onCommand.addListener(async (command) => {
  if (command === 'activate-speed-reader') {
    const [tab] = await chrome.tabs.query({ active: true, currentWindow: true })
    if (!tab.id) return

    try {
      // Get selected text
      const results = await chrome.scripting.executeScript({
        target: { tabId: tab.id },
        func: () => window.getSelection()?.toString() || '',
      })

      const text = results[0]?.result?.trim()

      if (text) {
        // Send message to content script
        await chrome.tabs.sendMessage(tab.id, {
          type: 'ACTIVATE_SPEED_READER',
          text: text,
        })
      } else {
        await chrome.scripting.executeScript({
          target: { tabId: tab.id },
          func: () => alert('Please select some text first!'),
        })
      }
    } catch (err) {
      console.error('Failed to activate speed reader:', err)
    }
  }
})

// Handle context menu
chrome.runtime.onInstalled.addListener(() => {
  chrome.contextMenus.create({
    id: 'libri-speed-reader',
    title: 'Speed Read Selected Text',
    contexts: ['selection'],
  })
})

chrome.contextMenus.onClicked.addListener(async (info, tab) => {
  if (info.menuItemId === 'libri-speed-reader' && tab?.id) {
    try {
      const text = info.selectionText?.trim()
      if (text) {
        // Send message to content script
        await chrome.tabs.sendMessage(tab.id, {
          type: 'ACTIVATE_SPEED_READER',
          text: text,
        })
      }
    } catch (err) {
      console.error('Failed to activate speed reader:', err)
    }
  }
})
