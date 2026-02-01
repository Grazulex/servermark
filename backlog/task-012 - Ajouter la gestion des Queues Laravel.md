---
id: 12
title: Ajouter la gestion des Queues Laravel
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
created_date: '2026-02-01T20:21:15.636Z'
updated_date: '2026-02-01T20:23:41.762Z'
closed_date: '2026-02-01T20:23:41.762Z'
changelog:
  - timestamp: '2026-02-01T20:21:15.636Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-02-01T20:21:29.515Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T20:21:34.963Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-02-01T20:23:23.346Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T20:23:34.722Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T20:23:41.762Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria: []
ai_plan: >-
  ## Plan d'implementation


  ### Objectif

  Permettre de démarrer/arrêter les workers Laravel (`artisan queue:work`) via
  des services systemd utilisateur.


  ### Approche

  Utiliser systemd user services (pas besoin de root) :

  - Service créé dans `~/.config/systemd/user/`

  - Nom: `servermark-queue-{site-slug}.service`

  - Géré via `systemctl --user`


  ### Fichiers concernés


  **Backend (Rust)**

  1. `src-tauri/src/commands/laravel.rs` (modify)
     - `get_queue_status(site_path)` - vérifier si worker actif
     - `start_queue_worker(site_path, php_version)` - créer et démarrer service
     - `stop_queue_worker(site_path)` - arrêter et supprimer service

  2. `src-tauri/src/lib.rs` (modify) - exposer commandes


  **Frontend (Vue)**

  3. `src/types/index.ts` (modify) - ajouter `queue_enabled?: boolean`

  4. `src/stores/sites.ts` (modify) - actions start/stop queue

  5. `src/components/SiteCard.vue` (modify) - bouton toggle queue


  ### Template systemd service

  ```ini

  [Unit]

  Description=Laravel Queue Worker - {site_name}

  After=network.target


  [Service]

  ExecStart=/usr/bin/php{version} {path}/artisan queue:work --sleep=3 --tries=3
  --max-time=3600

  Restart=always

  RestartSec=5


  [Install]

  WantedBy=default.target

  ```


  ### Etapes

  1. Implémenter les commandes Rust avec gestion systemd user

  2. Exposer dans lib.rs

  3. Mettre à jour types + store + SiteCard
ai_notes: >
  **2026-02-01T20:23:23.345Z** - **21:30** - PROGRESS: Implémentation terminée

  - Backend: 3 commandes (get_queue_status, start_queue_worker,
  stop_queue_worker)

  - Utilise systemd user services (~/.config/systemd/user/)

  - Service: servermark-queue-{site-slug}.service

  - Frontend: types, store, bouton toggle (liste icon) dans SiteCard

  - Statut récupéré automatiquement au fetchSites
ai_review: >-
  ## Self-Review


  ### Complete

  - [x] Backend: get_queue_status via systemctl --user is-active

  - [x] Backend: start_queue_worker - crée et démarre service systemd user

  - [x] Backend: stop_queue_worker - arrête et supprime service

  - [x] Frontend: LaravelInfo.queue_enabled ajouté

  - [x] Frontend: Store avec toggle/start/stop queue worker

  - [x] Frontend: SiteCard bouton liste (vert=actif, gris=inactif)

  - [x] Fetch auto du statut au chargement des sites


  ### Service systemd généré

  ```ini

  [Unit]

  Description=Laravel Queue Worker - {site_name}

  [Service]

  ExecStart=/usr/bin/php{version} {path}/artisan queue:work --sleep=3 --tries=3
  --max-time=3600

  Restart=always

  ```


  ### Tests à effectuer

  - Démarrer queue worker sur un site Laravel

  - Vérifier avec `systemctl --user status servermark-queue-*`

  - Arrêter et vérifier suppression du service


  ### Avantages

  - Redémarrage auto si crash (Restart=always)

  - Persiste après redémarrage système (si lingering activé)

  - Pas besoin de root
---
Permettre de démarrer/arrêter les workers Laravel (artisan queue:work) pour chaque site Laravel
