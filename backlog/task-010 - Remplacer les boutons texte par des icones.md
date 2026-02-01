---
id: 10
title: Remplacer les boutons texte par des icônes
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
created_date: '2026-02-01T19:52:04.570Z'
updated_date: '2026-02-01T19:54:31.507Z'
closed_date: '2026-02-01T19:54:31.507Z'
changelog:
  - timestamp: '2026-02-01T19:52:04.570Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-02-01T19:52:14.997Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T19:52:19.623Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-02-01T19:54:15.448Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T19:54:26.227Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-01T19:54:31.507Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria: []
ai_plan: >-
  ## Plan d'implementation


  ### Objectif

  Remplacer les boutons avec texte par des boutons icônes (btn-icon) comme dans
  SiteCard.


  ### Fichiers concernés

  1. `src/components/ContainerCard.vue` (modify) - Start, Stop, Restart, Logs,
  Remove

  2. `src/views/Php.vue` (modify) - Install, Set Active, Uninstall

  3. `src/views/Databases.vue` (modify) - Drop


  ### Mapping des icônes (SVG Feather Icons)

  - **Start** (play): triangle pointant à droite

  - **Stop** (square): carré

  - **Restart** (refresh-cw): flèches circulaires

  - **Logs** (file-text): document

  - **Remove/Drop** (trash-2): poubelle

  - **Install** (download): flèche vers le bas

  - **Set Active** (check-circle): coche dans cercle

  - **Uninstall** (x-circle): X dans cercle


  ### Structure cible

  ```html

  <button class="btn btn-icon btn-success" title="Start">
    <svg>...</svg>
  </button>

  ```


  ### Etapes

  1. Modifier ContainerCard.vue - remplacer 5 boutons texte par icônes

  2. Modifier Databases.vue - remplacer bouton Drop par icône

  3. Modifier Php.vue - remplacer boutons par icônes (garder texte pour états
  loading)
ai_notes: >
  **2026-02-01T19:54:15.448Z** - **20:56** - PROGRESS: Toutes les modifications
  effectuées

  - ContainerCard.vue: 5 boutons remplacés par icônes (Start, Stop, Restart,
  Logs, Remove)

  - Databases.vue: Bouton Drop remplacé par icône poubelle

  - Php.vue: Boutons Install, Set Active, Uninstall remplacés par icônes +
  layout horizontal
ai_review: >-
  ## Self-Review


  ### Complete

  - [x] ContainerCard.vue - Icônes: Play (Start), Square (Stop), Refresh
  (Restart), FileText (Logs), Trash (Remove)

  - [x] Databases.vue - Icône: Trash (Drop)

  - [x] Php.vue - Icônes: Download (Install), CheckCircle (Set Active), Trash
  (Uninstall)


  ### Cohérence du design

  - Tous les boutons utilisent la classe btn-icon (36x36px)

  - Tous les boutons ont un attribut title pour l'accessibilité

  - Les couleurs des boutons restent cohérentes (success=vert, warning=orange,
  danger-outline=rouge)

  - Icônes SVG de style Feather Icons (16x16, stroke-width: 2)


  ### Tests à effectuer

  - Vérifier visuellement l'alignement des boutons

  - Tester les tooltips (title) au survol

  - Vérifier que les boutons disabled sont bien grisés


  ### Notes

  - Le label 'Currently Active' dans PHP a été conservé car il n'y a pas
  d'action associée

  - Les états de loading (Installing..., Switching...) sont gérés par l'overlay
  global dans PHP
---
Harmoniser les boutons des sections Containers, PHP et Databases en utilisant des icônes au lieu de texte, comme dans SiteCard
