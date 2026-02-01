# ServerMark

Local development environment manager for Linux - A Herd/Valet alternative.

![ServerMark](https://img.shields.io/badge/Platform-Linux-blue)
![Tauri](https://img.shields.io/badge/Tauri-2.x-orange)
![Vue](https://img.shields.io/badge/Vue-3.x-green)

## Overview

ServerMark is a desktop application for managing local development environments on Linux. It provides an easy-to-use graphical interface for managing services, PHP versions, and development sites.

## Features

- **Service Management** - Start/stop MySQL, PostgreSQL, Redis, Mailpit, etc.
- **Container Support** - Run services in Docker/Podman containers
- **Multiple PHP Versions** - Install and switch between PHP versions
- **Site Management** - Configure local development sites with custom domains
- **Laravel Integration** - Detect and manage Laravel versions per project
- **SSL Certificates** - Automatic HTTPS for local sites
- **System Tray** - Quick access from the system tray

## Tech Stack

- **Frontend**: Vue 3 + TypeScript + Vite + Pinia
- **Backend**: Rust + Tauri 2
- **Target**: Linux (Ubuntu, Debian, Fedora, Arch, etc.)

## Requirements

- Linux (Ubuntu 20.04+, Debian 11+, Fedora 38+, Arch)
- Docker or Podman (for containerized services)
- Rust 1.77+ (for building)
- Node.js 18+ (for building)

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/Grazulex/servermark.git
cd servermark

# Install dependencies
npm install

# Run in development mode
npm run tauri:dev

# Build for production
npm run tauri:build
```

## Project Structure

```
servermark/
├── src/                    # Vue 3 frontend
│   ├── views/              # Page components
│   ├── components/         # Reusable components
│   ├── stores/             # Pinia state management
│   ├── composables/        # Vue 3 composition API hooks
│   ├── types/              # TypeScript types
│   └── styles/             # Global CSS
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri commands
│   │   ├── main.rs
│   │   └── lib.rs
│   └── Cargo.toml
└── package.json
```

## Development

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri:dev

# Type check and lint
npm run check

# Format code
npm run format
```

## Services

ServerMark supports the following services via Docker/Podman:

| Service | Versions | Default Port |
|---------|----------|--------------|
| MySQL | 8.0, 8.4, 5.7 | 3306 |
| PostgreSQL | 16, 15, 14, 13 | 5432 |
| Redis | 7, 6 | 6379 |
| Mailpit | latest | 8025 |
| MinIO | latest | 9000 |
| MongoDB | 7, 6, 5 | 27017 |
| Memcached | latest | 11211 |
| Adminer | latest | 8080 |

## PHP Versions

Supported PHP versions:
- PHP 8.3
- PHP 8.2
- PHP 8.1
- PHP 8.0
- PHP 7.4

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
