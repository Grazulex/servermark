---
id: 14
title: Setup GitHub CI/CD for tests and Linux releases
status: Done
priority: high
assignees:
  - '@claude'
labels:
  - ci
  - devops
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-02-03T00:50:10.146Z'
updated_date: '2026-02-03T00:51:19.935Z'
closed_date: '2026-02-03T00:51:19.935Z'
changelog:
  - timestamp: '2026-02-03T00:50:10.146Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-02-03T00:50:18.970Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-02-03T00:50:25.272Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-03T00:50:53.156Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-03T00:51:05.366Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-03T00:51:11.125Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-03T00:51:16.553Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-03T00:51:19.935Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria: []
ai_plan: >-
  ## Plan d'implementation


  ### Objectif

  Configurer GitHub Actions pour CI/CD avec tests sur PRs et builds Linux sur
  releases.


  ### Etapes

  1. Creer .github/workflows/ci.yml pour tests sur PRs
     - Lint frontend (ESLint)
     - Type-check (vue-tsc)
     - Cargo check/clippy pour Rust

  2. Creer .github/workflows/release.yml pour builds sur tags
     - Trigger sur push de tags v*
     - Build AppImage et .deb pour Linux
     - Upload des artifacts dans GitHub Releases

  ### Fichiers concernes

  - .github/workflows/ci.yml (create)

  - .github/workflows/release.yml (create)


  ### Approche technique

  - Utiliser tauri-action pour les builds (officiel)

  - Ubuntu 22.04 pour la compatibilite

  - Cache npm et cargo pour performance

  - Matrix build si multi-arch necessaire


  ### Defis potentiels

  - Dependencies systeme pour Tauri (webkit2gtk, etc.)
ai_notes: >
  **2026-02-03T00:50:53.155Z** - **01:51** - PROGRESS: Created
  .github/workflows/ci.yml with frontend lint and Rust check jobs

  **01:51** - PROGRESS: Created .github/workflows/release.yml with tauri-action
  for Linux builds

  **01:51** - DECISION: Using ubuntu-22.04 for better webkit2gtk-4.1
  compatibility

  **01:51** - DECISION: Using tauri-apps/tauri-action@v0 (official Tauri action)
  for release builds

  **2026-02-03T00:51:05.365Z** - **01:52** - RESOLVED: Fixed Rust action name
  (dtolnay/rust-toolchain, not rust-action)
ai_documentation: |-
  ## Documentation

  ### CI Workflow (.github/workflows/ci.yml)

  Declenche sur:
  - Push sur `main`
  - Pull requests vers `main`

  **Jobs:**
  1. **lint-frontend**: ESLint + TypeScript check
  2. **check-rust**: cargo check + clippy + fmt

  ### Release Workflow (.github/workflows/release.yml)

  Declenche sur push de tags `v*` (ex: v0.1.0, v1.0.0).

  **Outputs:**
  - AppImage (portable)
  - .deb (Debian/Ubuntu)

  Les binaires sont automatiquement attaches a la GitHub Release.

  ### Usage

  ```bash
  # Pour creer une release:
  shipmark release

  # Ou manuellement:
  git tag v0.1.0
  git push origin v0.1.0
  ```

  ### Dependencies systeme requises (CI)

  - libwebkit2gtk-4.1-dev
  - libappindicator3-dev
  - librsvg2-dev
  - patchelf
ai_review: |-
  ## Self-Review

  ### Complete
  - [x] CI workflow pour lint frontend (ESLint, vue-tsc)
  - [x] CI workflow pour check Rust (cargo check, clippy, fmt)
  - [x] Release workflow avec tauri-action
  - [x] Upload automatique des binaires Linux

  ### Tests effectues
  - Syntaxe YAML: OK (verification visuelle)
  - Actions utilisees: Verifiees (actions officielles)

  ### Qualite du code
  - Standards respectes: Oui
  - Documentation: Complete

  ### Limitations connues
  - Build Linux uniquement (pas macOS/Windows)
  - Pas de tests unitaires (npm test n'existe pas encore)

  ### Recommandations
  - Ajouter des tests unitaires frontend (Vitest) plus tard
  - Ajouter des tests Rust (cargo test) quand il y en aura
  - Considerer multi-arch (arm64) si besoin
---
Configure GitHub Actions for: 1) Running tests on PRs 2) Building Linux binaries on releases
