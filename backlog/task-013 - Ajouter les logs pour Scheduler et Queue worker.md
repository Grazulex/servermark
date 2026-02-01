---
id: 13
title: Ajouter les logs pour Scheduler et Queue worker
status: Done
priority: high
assignees:
  - '@claude'
labels:
  - feature
  - laravel
  - logs
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-02-01T20:25:40.462Z'
updated_date: '2026-02-01T20:28:57.648Z'
closed_date: '2026-02-01T20:28:57.648Z'
changelog:
  - timestamp: '2026-02-01T20:25:40.462Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-02-01T20:25:50.252Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T20:25:55.794Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-02-01T20:28:41.193Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T20:28:50.606Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T20:28:57.648Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria: []
ai_plan: >-
  ## Plan d'implementation


  ### Objectif

  1. Scheduler écrit les logs dans storage/logs/scheduler.log

  2. Bouton View Logs pour voir les logs du scheduler et queue worker


  ### Modifications


  **Backend (Rust)**

  1. `laravel.rs` - Modifier enable_scheduler pour écrire vers fichier log

  2. Ajouter `get_scheduler_logs(site_path)` - lire storage/logs/scheduler.log

  3. Ajouter `get_queue_logs(site_path)` - lire journalctl --user


  **Frontend (Vue)**  

  4. `SiteCard.vue` - Ajouter bouton View Logs

  5. Créer modale/panneau pour afficher les logs

  6. `Sites.vue` - Gérer l'affichage des logs


  ### Nouveau format cron

  ```

  * * * * * cd /path && php artisan schedule:run >> storage/logs/scheduler.log
  2>&1

  ```


  ### Etapes

  1. Modifier le cron pour écrire vers scheduler.log

  2. Ajouter commandes get_scheduler_logs et get_queue_logs

  3. Ajouter bouton et modale logs dans le frontend
ai_notes: |
  **2026-02-01T20:28:41.192Z** - **21:38** - PROGRESS: Implémentation terminée
  - Scheduler écrit maintenant vers storage/logs/scheduler.log
  - Ajout get_scheduler_logs (tail fichier) et get_queue_logs (journalctl)
  - Ajout clear_scheduler_logs pour vider le fichier
  - Bouton View Logs (icône document) dans SiteCard pour sites Laravel
  - Modale avec onglets Scheduler/Queue et badges ON/OFF
ai_review: |-
  ## Self-Review

  ### Complete
  - [x] Scheduler écrit vers storage/logs/scheduler.log au lieu de /dev/null
  - [x] get_scheduler_logs - lit les N dernières lignes du fichier
  - [x] get_queue_logs - lit journalctl --user pour le service
  - [x] clear_scheduler_logs - vide le fichier scheduler.log
  - [x] Bouton View Logs dans SiteCard (icône document)
  - [x] Modale avec onglets Scheduler/Queue
  - [x] Badges ON/OFF montrant le statut de chaque service
  - [x] Boutons Refresh et Clear Logs

  ### UI
  - Modale responsive avec max-height 80vh
  - Logs affichés en monospace avec scroll
  - Onglets avec indicateur visuel actif/inactif

  ### Tests à effectuer
  - Activer scheduler, attendre 1 minute, voir les logs
  - Activer queue worker, exécuter un job, voir les logs
  - Tester le bouton Clear Logs
  - Tester le bouton Refresh
---
Ajouter des logs fichier pour le scheduler et un bouton View Logs pour voir les logs du scheduler et queue worker
