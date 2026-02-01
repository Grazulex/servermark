---
id: 8
title: Gestion des Sites locaux (.test domains)
status: In Progress
priority: high
assignees:
  - '@claude'
labels:
  - feature
  - sites
  - nginx
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-02-01T08:54:47.385Z'
updated_date: '2026-02-01T09:01:00.818Z'
changelog:
  - timestamp: '2026-02-01T08:54:47.385Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-02-01T08:54:55.286Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-02-01T08:55:42.453Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T09:01:00.818Z'
    action: modified
    details: Task updated
    user: AI
acceptance_criteria: []
ai_plan: |-
  ## Plan d'implémentation Sites

  ### État actuel
  - Frontend: Sites.vue, SiteCard.vue, sites.ts store (non connecté)
  - Types: Site interface complète
  - Backend: Aucune commande Rust pour les sites

  ### Étapes

  1. **Backend Rust - sites.rs**
     - list_sites() - Lire depuis ~/.config/servermark/sites.json
     - add_site(path, name) - Détecter type, créer config Nginx
     - remove_site(id) - Supprimer config Nginx
     - detect_site_type(path) - Laravel/Symfony/WordPress/static

  2. **Configuration Nginx**
     - Générer server blocks dans /etc/nginx/sites-available/
     - Symlink vers sites-enabled
     - Reload Nginx après changements

  3. **/etc/hosts management**
     - Ajouter entrées pour les domaines .test
     - 127.0.0.1 mysite.test

  4. **Frontend - Connecter au backend**
     - Activer invoke() dans sites.ts
     - Modal 'Add Site' avec sélection dossier
     - Refresh liste après actions

  ### Fichiers à créer/modifier
  - src-tauri/src/commands/sites.rs (create)
  - src-tauri/src/commands/mod.rs (update)
  - src-tauri/src/lib.rs (register commands)
  - src/stores/sites.ts (connect to backend)
  - src/views/Sites.vue (add modal)
ai_notes: >
  **2026-02-01T09:01:00.817Z** - **10:05** - Implementation complete: Backend
  Rust sites.rs, Create/Import/Clone modes, Nginx config, /etc/hosts, Frontend
  modal with tabs
---
Implémenter la gestion des sites de développement local. Permettre de lier un dossier à un domaine .test, configurer Nginx/Caddy pour servir les sites, et gérer les versions PHP par site.
