// memora — AI that knows everything you've seen, said, or heard
// https://github.com/shrirampai3000/memora

# Memora Vision & Principles

Memora is designed to be your private, local digital memory.

## Core Mandates

1. **Local-First & Private**
   Your computer screen reflects your personal and professional life. Data captured by Memora must never leave your machine unless you explicitly choose to send specific prompt context to an external LLM.

2. **Screen-First Context**
   Memora focuses on visual and textual screen context: what you read, wrote, clicked, and viewed. Accessibility APIs provide structured UI trees, supplemented by OCR for bitmap applications.

3. **No Paywalls & No SaaS Telemetry**
   Memora is an open local application. Features are unlocked for all users without entitlement gates, cloud subscriptions, or tracking telemetry.

4. **Resource Efficiency**
   Background capture must consume minimal CPU and battery. Continuous per-frame processing uses optimized pipelines, event-driven triggers, and bounded buffers.

5. **Extensible AI Interfaces**
   Memora exposes a clean local REST API and Model Context Protocol (MCP) servers so that AI agents and tools (Claude, Ollama, local models) can query your screen context securely.
