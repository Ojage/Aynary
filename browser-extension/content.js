// Content script for Aynary browser extension
// This script runs in the context of web pages

// Listen for messages from background script
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  if (request.action === "getSelection") {
    const selection = window.getSelection().toString();
    sendResponse({ selection: selection });
  }
  return true;
});

