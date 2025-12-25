// Background service worker for Aynary browser extension

const NATIVE_HOST = "com.aynary.dictionary";

// Create context menu item when extension is installed
chrome.runtime.onInstalled.addListener(() => {
  chrome.contextMenus.create({
    id: "define-with-aynary",
    title: "Define with Aynary",
    contexts: ["selection"]
  });
});

// Handle context menu clicks
chrome.contextMenus.onClicked.addListener((info, tab) => {
  if (info.menuItemId === "define-with-aynary" && info.selectionText) {
    const word = info.selectionText.trim().split(/\s+/)[0].replace(/[^\w]/g, '');
    if (word) {
      sendToNativeHost({ action: "lookup", word: word });
    }
  }
});

// Send message to native messaging host
function sendToNativeHost(message) {
  const port = chrome.runtime.connectNative(NATIVE_HOST);
  
  port.postMessage(message);
  
  port.onMessage.addListener((response) => {
    console.log("Response from native host:", response);
  });
  
  port.onDisconnect.addListener(() => {
    if (chrome.runtime.lastError) {
      console.error("Native messaging error:", chrome.runtime.lastError.message);
      // Fallback: open dictionary app or show error
      chrome.tabs.create({ url: `aynary://lookup/${encodeURIComponent(message.word)}` });
    }
  });
}

