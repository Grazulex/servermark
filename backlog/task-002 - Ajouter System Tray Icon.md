---
id: 2
title: Ajouter System Tray Icon
status: In Progress
priority: high
assignees:
  - '@claude'
labels:
  - feature
  - ui
  - tauri
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-02-01T07:15:11.559Z'
updated_date: '2026-02-01T07:16:06.229Z'
changelog:
  - timestamp: '2026-02-01T07:15:11.559Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-02-01T07:15:19.077Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T07:15:19.830Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-02-01T07:16:06.229Z'
    action: modified
    details: Task updated
    user: AI
acceptance_criteria: []
ai_plan: |-
  ## Plan d'implementation

  ### Objectif
  Ajouter une icône dans la barre système Linux avec menu contextuel.

  ### Etapes
  1. Ajouter le plugin tauri-plugin-tray au Cargo.toml
  2. Configurer le tray dans lib.rs
  3. Créer le menu contextuel:
     - Start All Services
     - Stop All Services
     - ---
     - Open ServerMark
     - Quit
  4. Gérer les événements du menu
  5. Ajouter l'icône PNG pour le tray
  6. Option: Minimiser en tray au lieu de fermer

  ### Fichiers concernes
  - src-tauri/Cargo.toml (ajouter plugin)
  - src-tauri/src/lib.rs (configurer tray)
  - src-tauri/icons/tray.png (icône 32x32)
  - src-tauri/tauri.conf.json (permissions)

  ### Documentation
  - https://v2.tauri.app/plugin/tray/
ai_notes: |
  **2026-02-01T07:16:06.229Z** - **08:22** - PROGRESS: System Tray implémenté
  - Tray icon avec menu contextuel
  - Actions: Open, Start All, Stop All, Quit
  - Click gauche ouvre la fenêtre
  - Events Tauri émis vers le frontend
  - Composable useTrayEvents pour écouter les events
---
Ajouter une icône dans la barre système Linux avec menu contextuel: Start/Stop All, Open, Quit. Utiliser tauri-plugin-tray.
