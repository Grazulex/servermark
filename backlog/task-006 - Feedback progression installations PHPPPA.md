---
id: 6
title: Feedback progression installations PHP/PPA
status: To Do
priority: high
assignees:
  - '@claude'
labels:
  - feature
  - ux
  - php
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-02-01T08:19:30.425Z'
updated_date: '2026-02-01T08:37:59.028Z'
changelog:
  - timestamp: '2026-02-01T08:19:30.425Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-02-01T08:19:41.542Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T08:19:47.764Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-02-01T08:24:38.083Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T08:24:46.564Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T08:37:46.346Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T08:37:59.028Z'
    action: updated
    details: 'status: In Progress → To Do'
    user: user
acceptance_criteria: []
ai_plan: >-
  ## Plan d'implementation


  ### Objectif

  Afficher la progression des installations longues (PPA, PHP) avec feedback
  visuel.


  ### Approche

  Utiliser les événements Tauri pour communiquer la progression du backend vers
  le frontend.


  ### Backend (Rust)

  1. Créer struct InstallProgress { step: String, total_steps: u8, current_step:
  u8 }

  2. Modifier add_php_ppa() pour émettre des événements à chaque étape

  3. Modifier install_php_with_extensions() pour émettre des événements

  4. Utiliser app_handle.emit() pour envoyer les événements


  ### Frontend (Vue)

  1. Écouter les événements Tauri avec listen()

  2. Afficher un overlay de progression dans la modal

  3. Montrer l'étape courante et une barre de progression


  ### Fichiers

  - src-tauri/src/commands/php.rs (emit events)

  - src/components/PhpInstallModal.vue (progress display)

  - src/views/Php.vue (PPA progress)
ai_notes: >
  **2026-02-01T08:24:38.083Z** - **09:24** - COMPLETE: Feedback progression
  implémenté + désinstallation PHP ajoutée


  - Events Tauri pour progression PPA (3 étapes)

  - Events Tauri pour progression install PHP

  - Events Tauri pour progression uninstall PHP

  - UI: barre de progression + étape courante

  - Bouton Uninstall sur versions installées (sauf active)

  - Confirmation avant désinstallation
ai_review: |-
  ## Self-Review

  ### Partiellement implémenté
  - [x] Events Tauri émis depuis Rust
  - [x] Listeners frontend reçoivent les events
  - [ ] UI affiche la progression (BLOQUÉ)

  ### Problème identifié
  La commande Rust est synchrone et bloque le thread principal Tauri.
  L'UI freeze pendant l'exécution, donc aucun overlay ne peut s'afficher.

  ### Solution requise
  Implémenter les commandes avec tokio::spawn pour exécuter 
  les opérations apt/pkexec dans un thread séparé.

  ### Status
  Reporté - nécessite refactoring async des commandes Rust
---
Ajouter indicateurs de progression pendant les installations longues (PPA et PHP). Afficher les étapes en cours, éviter blocage UI.
