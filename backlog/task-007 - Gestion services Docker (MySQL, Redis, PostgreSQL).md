---
id: 7
title: 'Gestion services Docker (MySQL, Redis, PostgreSQL)'
status: Done
priority: high
assignees:
  - '@claude'
labels:
  - feature
  - docker
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-02-01T08:40:13.187Z'
updated_date: '2026-02-01T08:54:44.923Z'
closed_date: '2026-02-01T08:54:44.923Z'
changelog:
  - timestamp: '2026-02-01T08:40:13.187Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-02-01T08:40:45.675Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T08:40:57.968Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-02-01T08:54:44.923Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria: []
ai_plan: |-
  ## Plan d'implementation

  ### Objectif
  Connecter le frontend au backend pour la gestion Docker.

  ### État actuel
  - Backend Rust: COMPLET (docker.rs)
  - Store: Templates OK, invoke() commentés
  - UI: Containers.vue prêt

  ### Étapes
  1. Activer les appels invoke() dans docker.ts
  2. Vérifier/créer ContainerCard.vue
  3. Tester création/démarrage/arrêt containers

  ### Fichiers
  - src/stores/docker.ts (activer invoke)
  - src/components/ContainerCard.vue (vérifier)
---
Implémenter la gestion des containers Docker pour les services de base de données et cache. Créer, démarrer, arrêter, supprimer des containers MySQL, Redis, PostgreSQL via l'UI.
