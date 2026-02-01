# ServerMark

Local development environment manager for Linux - A Herd/Valet alternative.

## Stack

- **Frontend**: Vue 3 + TypeScript + Vite + Pinia
- **Backend**: Rust + Tauri 2
- **Target**: Linux (Ubuntu, Debian, Fedora, Arch, etc.)

## Project Structure

```
servermark/
├── src/                    # Vue 3 frontend
│   ├── views/              # Page components
│   ├── components/         # Reusable components
│   ├── stores/             # Pinia state management
│   ├── composables/        # Vue 3 composition API hooks
│   ├── types/              # TypeScript types
│   ├── router/             # Vue Router config
│   └── styles/             # Global CSS
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri commands
│   │   ├── main.rs
│   │   └── lib.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── package.json
└── vite.config.ts
```

## Commands

```bash
# Development
npm install              # Install dependencies
npm run tauri:dev        # Run app in development

# Build
npm run tauri:build      # Build production app

# Linting
npm run check            # Type check + lint
npm run format           # Format with Prettier
```

## Features

- [ ] Service management (Nginx, MySQL, PostgreSQL, Redis, etc.)
- [ ] Multiple PHP version support
- [ ] Site management with automatic SSL
- [ ] DNSMasq integration for .test domains
- [ ] Caddy/Nginx web server support
- [ ] Auto-detection of Linux distribution
