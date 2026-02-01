---
id: 1
title: Implémenter architecture hybride services
status: Done
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
updated_date: '2026-02-01T07:15:05.096Z'
closed_date: '2026-02-01T07:15:05.096Z'
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
  - timestamp: '2026-02-01T07:13:45.720Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T07:15:04.399Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T07:15:05.096Z'
    action: updated
    details: 'status: In Progress → Done'
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
ai_notes: >
  **2026-02-01T07:13:45.719Z** - **08:20** - PROGRESS: Architecture hybride
  implémentée

  - Store docker.ts avec templates de services

  - Commands Rust pour Docker/Podman

  - Vue Containers avec modal de création

  - Composant ContainerCard

  - Support Docker et Podman
ai_review: |-
  ## Self-Review

  ### Complete
  - [x] Store Pinia docker.ts avec templates de services
  - [x] Types TypeScript pour Docker/containers
  - [x] Commands Rust pour Docker/Podman
  - [x] Vue Containers avec modal création
  - [x] Composant ContainerCard

  ### Tests effectues
  - Code compile: OK
  - Structure coherente: OK

  ### Qualite du code
  - Standards respectes: Oui
  - TypeScript strict: Oui

  ### Limitations connues
  - Pas encore testé avec Docker réel (mock data)
  - Pas de gestion des logs containers dans l'UI
---
PHP local + Services (MySQL, Redis, etc.) en containers Docker/Podman. Détection auto de Docker/Podman, gestion des containers, templates de services.
