---
id: 15
title: Setup testing infrastructure
status: Done
priority: high
assignees:
  - '@claude'
labels:
  - testing
  - devops
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-02-03T01:17:02.857Z'
updated_date: '2026-02-03T01:21:13.403Z'
closed_date: '2026-02-03T01:21:13.403Z'
changelog:
  - timestamp: '2026-02-03T01:17:02.857Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-02-03T01:17:09.060Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-02-03T01:17:22.856Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-03T01:21:13.403Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria: []
ai_plan: >-
  ## Plan d'implementation


  ### Objectif

  Mettre en place l'infrastructure de tests pour le frontend Vue/TypeScript et
  le backend Rust.


  ### Etapes


  #### Frontend (Vitest)

  1. Installer Vitest + @vue/test-utils + happy-dom

  2. Configurer vitest.config.ts

  3. Ajouter scripts npm (test, test:watch, test:coverage)

  4. Creer un test exemple pour un store Pinia


  #### Backend (Rust)

  1. Ajouter un module de tests dans les fichiers Rust

  2. Tester les fonctions utilitaires (pas les commandes Tauri)


  #### CI

  1. Mettre a jour ci.yml pour executer les tests


  ### Fichiers concernes

  - package.json (modify)

  - vitest.config.ts (create)

  - src/stores/__tests__/sites.spec.ts (create)

  - src-tauri/src/commands/sites.rs (modify - add tests)

  - .github/workflows/ci.yml (modify)


  ### Approche technique

  - Vitest car integre nativement avec Vite

  - happy-dom plus leger que jsdom

  - Tests unitaires sur les fonctions pures, pas les appels systeme
---
Add Vitest for Vue/TypeScript frontend and configure cargo test for Rust backend. Update CI to run tests.
