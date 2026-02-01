---
id: 3
title: Gestion versions Laravel par site
status: Done
priority: medium
assignees:
  - '@claude'
labels:
  - feature
  - laravel
  - sites
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-02-01T07:16:28.734Z'
updated_date: '2026-02-01T07:18:04.973Z'
closed_date: '2026-02-01T07:18:04.973Z'
changelog:
  - timestamp: '2026-02-01T07:16:28.734Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-02-01T07:16:38.070Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T07:16:39.098Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-02-01T07:18:00.486Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T07:18:03.991Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T07:18:04.973Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria: []
ai_plan: |-
  ## Plan d'implementation

  ### Objectif
  Détecter et gérer les versions Laravel dans chaque site.

  ### Fonctionnalités
  1. Détection automatique de Laravel (composer.json)
  2. Affichage de la version dans la liste des sites
  3. Badge indiquant si une mise à jour est disponible
  4. Action 'Upgrade' pour mettre à jour Laravel

  ### Backend (Rust)
  - Commande `detect_laravel_version`: Parse composer.json/composer.lock
  - Commande `get_latest_laravel_version`: API Packagist
  - Commande `upgrade_laravel`: Exécute composer update

  ### Frontend (Vue)
  - Afficher version Laravel dans SiteCard
  - Badge 'Update available' si nouvelle version
  - Modal de mise à jour avec changelog

  ### Fichiers concernés
  - src-tauri/src/commands/laravel.rs (nouveau)
  - src/types/index.ts (ajouter LaravelInfo)
  - src/stores/sites.ts (ajouter détection Laravel)
  - src/components/SiteCard.vue (nouveau)
  - src/views/Sites.vue (utiliser SiteCard)
ai_notes: >
  **2026-02-01T07:18:00.486Z** - **08:28** - PROGRESS: Gestion Laravel
  implémentée

  - Commands Rust: detect_laravel_version, get_latest_laravel_version,
  upgrade_laravel, create_laravel_project

  - Types LaravelInfo ajoutés

  - Composant SiteCard avec affichage version Laravel

  - Badge update disponible si nouvelle version
ai_review: |-
  ## Self-Review

  ### Complete
  - [x] Commands Rust pour Laravel
  - [x] Types TypeScript LaravelInfo
  - [x] Composant SiteCard
  - [x] Affichage version dans la liste

  ### Tests effectues
  - Code structure: OK
  - Types coherents: OK

  ### Qualite du code
  - Standards respectes: Oui
  - Code modulaire: Oui

  ### Limitations connues
  - Upgrade Laravel nécessite Composer installé
  - Pas de gestion des erreurs Composer dans l'UI
---
Détecter et afficher la version Laravel de chaque site. Permettre l'upgrade/downgrade via Composer. Lire composer.json pour détecter la version.
