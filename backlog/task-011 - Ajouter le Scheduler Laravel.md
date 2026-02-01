---
id: 11
title: Ajouter le Scheduler Laravel
status: Done
priority: high
assignees:
  - '@claude'
labels:
  - feature
  - laravel
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-02-01T20:17:00.964Z'
updated_date: '2026-02-01T20:20:35.415Z'
closed_date: '2026-02-01T20:20:35.415Z'
changelog:
  - timestamp: '2026-02-01T20:17:00.964Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-02-01T20:17:25.271Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T20:17:30.333Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-02-01T20:20:18.526Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T20:20:30.941Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T20:20:35.415Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria: []
ai_plan: >-
  ## Plan d'implementation


  ### Objectif

  Permettre d'activer/désactiver le cron Laravel (`artisan schedule:run`) pour
  chaque site Laravel.


  ### Fonctionnement

  Le scheduler Laravel utilise une entrée crontab :

  ```

  * * * * * cd /path/to/project && php artisan schedule:run >> /dev/null 2>&1

  ```


  ### Fichiers concernés


  **Backend (Rust)**

  1. `src-tauri/src/commands/laravel.rs` (modify)
     - `get_scheduler_status(site_path)` - vérifier si cron actif
     - `enable_scheduler(site_path, php_version)` - ajouter entrée cron
     - `disable_scheduler(site_path)` - supprimer entrée cron

  2. `src-tauri/src/lib.rs` (modify)
     - Exposer les nouvelles commandes

  **Frontend (Vue)**

  3. `src/types/index.ts` (modify)
     - Ajouter `scheduler_enabled?: boolean` à LaravelInfo

  4. `src/components/SiteCard.vue` (modify)
     - Ajouter bouton toggle pour le scheduler (sites Laravel uniquement)

  5. `src/stores/sites.ts` (modify)
     - Ajouter actions pour enable/disable scheduler

  ### Etapes

  1. Implémenter les 3 commandes Rust dans laravel.rs

  2. Exposer les commandes dans lib.rs

  3. Mettre à jour les types TypeScript

  4. Ajouter le toggle dans SiteCard

  5. Connecter le frontend au backend
ai_notes: >
  **2026-02-01T20:20:18.526Z** - **21:25** - PROGRESS: Implémentation terminée

  - Backend Rust: 3 commandes ajoutées (get_scheduler_status, enable_scheduler,
  disable_scheduler)

  - Frontend: types mis à jour, store étendu, bouton toggle ajouté dans SiteCard

  - Le statut du scheduler est récupéré automatiquement au chargement des sites

  - Bouton horloge: vert si actif, gris si inactif
ai_review: >-
  ## Self-Review


  ### Complete

  - [x] Backend: get_scheduler_status - vérifie si cron actif via crontab -l

  - [x] Backend: enable_scheduler - ajoute entrée cron avec identifiant
  ServerMark

  - [x] Backend: disable_scheduler - supprime l'entrée cron du site

  - [x] Frontend: LaravelInfo type étendu avec scheduler_enabled

  - [x] Frontend: Store sites.ts avec toggleScheduler, enableScheduler,
  disableScheduler

  - [x] Frontend: SiteCard avec bouton horloge (vert=actif, gris=inactif)

  - [x] Récupération auto du statut au fetchSites


  ### Fonctionnement

  Le cron utilise le format:

  ```

  # ServerMark Scheduler: /path/to/project

  * * * * * cd /path/to/project && /usr/bin/php8.x artisan schedule:run >>
  /dev/null 2>&1

  ```


  ### Tests à effectuer

  - Activer le scheduler sur un site Laravel

  - Vérifier avec `crontab -l` que l'entrée est ajoutée

  - Désactiver et vérifier la suppression

  - Vérifier que le bouton change de couleur


  ### Limitations

  - Utilise le crontab de l'utilisateur courant (pas root)

  - Pas de gestion multi-utilisateurs
---
Permettre d'activer/désactiver le cron Laravel (artisan schedule:run) pour chaque site Laravel
