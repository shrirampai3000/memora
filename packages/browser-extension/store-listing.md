# Chrome Web Store Listing

## Name
MEMORA Browser Bridge

## Short Description (132 chars max)
Connects your browser to MEMORA for AI-powered web automation, data sync, and memory capture from authenticated pages.

## Detailed Description
MEMORA Browser Bridge connects your browser to the MEMORA desktop app, enabling AI-powered pipes to interact with web pages you're logged into.

HOW IT WORKS
This extension connects to MEMORA's local server (localhost:3030) via WebSocket. When a MEMORA pipe needs to read data from a web page, it sends a request through the local server to this extension, which executes it in the browser tab where you're already authenticated.

USE CASES
- Sync your ChatGPT and Claude conversation history into MEMORA memories
- Extract data from authenticated dashboards and internal tools
- Automate web workflows triggered by your screen activity

PRIVACY & SECURITY
- All communication stays on localhost â€” no data is sent to external servers
- The extension only activates when MEMORA's local server is running
- JavaScript execution is triggered only by your local MEMORA pipes
- No tracking, no analytics, no data collection
- Fully open source: https://github.com/MEMORA/MEMORA

REQUIREMENTS
- MEMORA desktop app (https://github.com/shrirampai3000/memora) running on the same machine
- Works with Chrome, Arc, Brave, Edge, and other Chromium browsers

## Category
Productivity

## Language
English

## Privacy Policy URL
https://github.com/shrirampai3000/memora/privacy

## Website
https://github.com/shrirampai3000/memora

## Support URL
https://github.com/MEMORA/MEMORA/issues
