---
id: 1
title: Implémenter architecture hybride services
status: In Progress
priority: high
assignees:
  - '@claude'
labels:
  - architecture
  - docker
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-02-01T07:10:25.011Z'
updated_date: '2026-02-01T07:10:34.394Z'
changelog:
  - timestamp: '2026-02-01T07:10:25.011Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-02-01T07:10:33.668Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T07:10:34.394Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
acceptance_criteria: []
ai_plan: |-
  ## Plan d'implementation

  ### Objectif
  Implementer une architecture hybride:
  - PHP en local (switch via update-alternatives)
  - Services en containers Docker/Podman

  ### Architecture

  #### Frontend (Vue 3)
  1. Store `docker.ts` - Gestion de l'état Docker/Podman
  2. Vue `Containers.vue` - Interface de gestion des containers
  3. Composant `ContainerCard.vue` - Affichage d'un container

  #### Backend (Rust)
  1. `commands/docker.rs` - Détection et gestion Docker/Podman
  2. `commands/containers.rs` - CRUD containers
  3. Templates de services (MySQL, PostgreSQL, Redis, etc.)

  ### Services supportés
  - MySQL 8.0
  - PostgreSQL 15/16
  - Redis 7
  - Mailpit
  - MinIO
  - Adminer/phpMyAdmin

  ### Etapes
  1. Créer le store Pinia pour Docker
  2. Créer les commandes Rust pour Docker
  3. Créer les templates de services
  4. Créer l'interface containers
  5. Intégrer avec le dashboard
---
PHP local + Services (MySQL, Redis, etc.) en containers Docker/Podman. Détection auto de Docker/Podman, gestion des containers, templates de services.
