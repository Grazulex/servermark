---
id: 4
title: Détection PHP réelle
status: Done
priority: high
assignees:
  - '@claude'
labels:
  - feature
  - php
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-02-01T08:01:57.251Z'
updated_date: '2026-02-01T08:03:56.499Z'
closed_date: '2026-02-01T08:03:56.499Z'
changelog:
  - timestamp: '2026-02-01T08:01:57.251Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-02-01T08:02:07.042Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T08:02:08.115Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-02-01T08:03:51.743Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T08:03:55.426Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T08:03:56.499Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria: []
ai_plan: |-
  ## Plan d'implementation

  ### Objectif
  Détecter les versions PHP réellement installées sur le système.

  ### Approche
  1. Scanner /usr/bin/php* pour trouver les binaires
  2. Exécuter php -v pour chaque binaire trouvé
  3. Détecter la version active (php par défaut)
  4. Retourner la liste au frontend

  ### Backend (Rust)
  - Modifier get_php_versions() pour scanner le système réel
  - Ajouter détection du chemin alternatif (/usr/local/bin, etc.)

  ### Frontend (Vue)
  - Appeler la commande Tauri au mount
  - Afficher les versions détectées
  - Marquer la version active

  ### Fichiers
  - src-tauri/src/commands/php.rs
  - src/stores/php.ts
  - src/views/Php.vue
ai_notes: >
  **2026-02-01T08:03:51.742Z** - **09:10** - PROGRESS: Détection PHP réelle
  implémentée

  - Scan /usr/bin/php* et /usr/local/bin

  - Détection version active via php -v

  - Affichage version complète (ex: 8.3.6)

  - Switch version via update-alternatives

  - Installation via apt/dnf/pacman
ai_review: |-
  ## Self-Review

  ### Complete
  - [x] Scan système pour PHP installés
  - [x] Détection version active
  - [x] Affichage version complète
  - [x] Switch version fonctionnel
  - [x] Install version (apt/dnf/pacman)

  ### Qualite du code
  - Rust: scan multi-chemins, tri par version
  - Vue: appel Tauri au mount, gestion loading/error
---
Détecter les versions PHP installées sur le système Linux. Scanner /usr/bin/php*, vérifier les versions, détecter la version active.
