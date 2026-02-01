---
id: 5
title: Modal extensions PHP + détection PPA
status: Done
priority: high
assignees:
  - '@claude'
labels:
  - feature
  - php
  - ui
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-02-01T08:07:24.426Z'
updated_date: '2026-02-01T08:19:00.039Z'
closed_date: '2026-02-01T08:19:00.039Z'
changelog:
  - timestamp: '2026-02-01T08:07:24.426Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-02-01T08:07:32.625Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T08:07:33.572Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-02-01T08:18:47.492Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T08:18:55.848Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T08:19:00.039Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria: []
ai_plan: >-
  ## Plan


  ### Backend (Rust)

  1. Commande `check_php_ppa` - Vérifie si ondrej/php est dans
  /etc/apt/sources.list.d/

  2. Commande `add_php_ppa` - Ajoute le PPA si manquant

  3. Modifier `install_php_version` pour accepter liste d'extensions


  ### Frontend (Vue)

  1. Détecter PPA au mount de la page PHP

  2. Afficher warning si PPA manquant avec bouton "Add PPA"

  3. Modal d'installation avec checkboxes pour extensions


  ### Extensions par défaut

  - fpm, cli, common (obligatoires)

  - mysql, curl, mbstring, xml, zip (recommandées)

  - gd, intl, redis, pgsql, sqlite3 (optionnelles)
ai_notes: >
  **2026-02-01T08:18:47.491Z** - **09:18** - COMPLETE: Modal extensions PHP et
  détection PPA fonctionnels. Tests validés: PPA ajouté, PHP 8.4 et 8.3
  installés, switch entre versions OK. Feedback utilisateur: besoin d'indicateur
  de progression pendant installations longues.
ai_review: |-
  ## Self-Review

  ### Complete
  - [x] Détection PPA ondrej/php
  - [x] Bouton Add PPA avec pkexec
  - [x] Modal sélection extensions (required/recommended/optional)
  - [x] Installation PHP avec extensions custom
  - [x] Switch entre versions PHP

  ### Tests effectués
  - Ajout PPA: OK (3 prompts mot de passe)
  - Install PHP 8.4: OK
  - Install PHP 8.3: OK  
  - Switch 8.4 -> 8.3: OK

  ### Limitations connues
  - Pas de feedback progression pendant install
  - UI bloquée pendant opérations longues
  - Plusieurs prompts mot de passe pour PPA

  ### Recommandations
  - Ajouter streaming logs/progression
  - Exécution async pour éviter blocage UI
---
1. Détecter si le PPA ondrej/php est installé (Ubuntu/Debian) et avertir sinon. 2. Modal pour choisir les extensions PHP à installer.
