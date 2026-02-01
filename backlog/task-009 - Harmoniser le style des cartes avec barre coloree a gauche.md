---
id: 9
title: Harmoniser le style des cartes avec barre colorée à gauche
status: Done
priority: medium
assignees:
  - '@claude'
labels:
  - ui
  - design
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-02-01T19:49:15.241Z'
updated_date: '2026-02-01T19:50:33.655Z'
closed_date: '2026-02-01T19:50:33.655Z'
changelog:
  - timestamp: '2026-02-01T19:49:15.241Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-02-01T19:49:36.549Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T19:49:42.299Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-02-01T19:50:21.374Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T19:50:29.181Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T19:50:33.655Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria: []
ai_plan: >-
  ## Plan d'implementation


  ### Objectif

  Harmoniser le design de toutes les cartes de l'application pour utiliser le
  style 'barre colorée à gauche' comme ContainerCard et ServiceCard.


  ### Fichiers concernés

  1. `src/components/SiteCard.vue` (modify) - Ajouter border-left coloré selon
  statut SSL

  2. `src/views/Databases.vue` (modify) - Ajouter border-left aux .database-card

  3. `src/views/Php.vue` (modify) - Changer le style .php-card pour border-left
  au lieu de border complet


  ### Style cible

  ```css

  .card {
    border: 1px solid var(--color-border);
    border-radius: 12px;
  }

  .card.running/.card.active/.card.secured {
    border-left: 3px solid var(--color-success);
  }

  .card.stopped/.card.inactive/.card.unsecured {
    border-left: 3px solid var(--color-text-muted);
  }

  ```


  ### Mapping des statuts par composant

  - **SiteCard**: secured (vert) / non-secured (gris)

  - **DatabaseCard**: MySQL (bleu) / PostgreSQL (violet) - ou simplement présent
  (vert)

  - **PhpCard**: active (vert) / installed (bleu) / non-installed (pas de barre)


  ### Etapes

  1. Modifier SiteCard.vue pour ajouter la barre colorée basée sur le statut SSL

  2. Modifier Databases.vue pour ajouter la barre colorée aux database-card

  3. Modifier Php.vue pour remplacer le style border complet par border-left
ai_notes: >
  **2026-02-01T19:50:21.374Z** - **20:51** - PROGRESS: Modifications effectuées
  sur les 3 fichiers

  - SiteCard.vue: ajout classe .secured et styles border-left (vert si SSL, gris
  sinon)

  - Databases.vue: ajout classe dynamique basée sur db.type et styles
  border-left (bleu MySQL, violet PostgreSQL)

  - Php.vue: changé le style de bordure complète vers border-left (vert si
  active, bleu si installed)
ai_review: >-
  ## Self-Review


  ### Complete

  - [x] SiteCard.vue - Barre colorée ajoutée (vert SSL, gris HTTP)

  - [x] Databases.vue - Barre colorée par type (bleu MySQL, violet PostgreSQL)

  - [x] Php.vue - Style modifié de bordure complète vers border-left


  ### Cohérence du design

  - Toutes les cartes ont maintenant le même pattern: border-left 3px solid

  - Les couleurs sont cohérentes avec le reste de l'UI (var(--color-success),
  var(--color-primary))

  - Les containers et services n'ont pas été modifiés car ils avaient déjà le
  bon style


  ### Tests à effectuer

  - Vérifier visuellement chaque vue (Sites, Databases, PHP)

  - Confirmer que les transitions CSS fonctionnent


  ### Limitations connues

  - Les databases sans type défini n'auront pas de barre colorée

  - Le style :not(.secured) sur SiteCard applique la barre grise même au hover
---
Uniformiser le design de toutes les cartes (Sites, Databases, PHP) pour utiliser le même style que Containers/Services : bordure avec barre colorée sur le côté gauche indiquant le statut
