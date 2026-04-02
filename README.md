# System Info App (Tauri Basics)

This project is a beginner-friendly Tauri app that shows system information and teaches the core concepts you need to build desktop apps with Tauri.

It is designed as a learning project focused on:

- IPC (frontend to backend communication)
- Events (sending and listening to app events)
- Window management (creating and controlling windows)
- Basic Rust command handling in a Tauri backend

## What You Learn

By working through this project, you will learn how to:

- Build a Tauri app using a Svelte frontend and Rust backend
- Call Rust commands from the frontend using Tauri IPC
- Emit and listen to events between frontend and backend
- Manage windows in Tauri (open, close, update window behavior)
- Structure a small real-world desktop app project

## Project Overview

This app displays system information such as RAM and other machine details through UI components in Svelte, while Rust handles native-level system access and app behavior.

Frontend highlights:

- Svelte routes and reusable UI components
- Pages for system data and RAM details
- Event and IPC interactions with the Tauri backend

Backend highlights:

- Rust modules for system information retrieval
- Tauri command registration and invocation
- Window-related app logic
- Capability configuration for secure access

## Tech Stack

- Tauri
- Rust
- SvelteKit
- TypeScript
- Vite
- Deno (primary package/runtime workflow)

## Getting Started

### 1. Install dependencies

This project uses Deno by default. You can also run it with Node or Bun.

- Deno: deno install
- Node: npm install
- Bun: bun install

### 2. Run the app in development

- Deno: deno task tauri dev
- Node: npm run tauri -- dev
- Bun: bun run tauri dev

This starts the Svelte frontend and launches the Tauri desktop app.

### 3. Build the app

- Deno: deno task tauri build
- Node: npm run tauri -- build
- Bun: bun run tauri build

## Folder Notes

- src/: Svelte frontend (routes, components, styles)
- src-tauri/src/: Rust backend source
- src-tauri/capabilities/: Tauri capabilities and permissions
- src-tauri/tauri.conf.json: Tauri app configuration

## Why This Project

This project is meant to teach the basics of Tauri in a practical way. Instead of only reading theory, you will implement and connect frontend and backend pieces directly, which makes core ideas like IPC, events, and window management easier to understand.
