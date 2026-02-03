---
id: 17
title: Update install script and README with correct CI artifact names
status: Done
priority: medium
assignees:
  - '@claude'
labels:
  - docs
  - ci
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-02-03T01:26:08.301Z'
updated_date: '2026-02-03T01:27:27.520Z'
closed_date: '2026-02-03T01:27:27.520Z'
changelog:
  - timestamp: '2026-02-03T01:26:08.301Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-02-03T01:26:12.890Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-02-03T01:26:57.390Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-03T01:27:23.845Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-03T01:27:27.520Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria: []
ai_notes: >
  **2026-02-03T01:26:57.390Z** - **02:28** - PROGRESS: Verified CI artifact
  names from GitHub API:

  - .deb: ServerMark_0.1.0_amd64.deb

  - .rpm: ServerMark-0.1.0-1.x86_64.rpm

  - .AppImage: ServerMark_0.1.0_amd64.AppImage


  Fixed install.sh to use correct filenames (was server-mark, now ServerMark).

  Updated README.md with Quick Install section.
ai_review: |-
  ## Self-Review

  ### Completed
  - [x] Verified actual CI artifact names from GitHub releases
  - [x] Fixed install.sh URLs (server-mark → ServerMark)
  - [x] Updated README.md with Quick Install section
  - [x] Added Manual Download section with correct filenames
  - [x] Validated script syntax

  ### Files Modified
  - install.sh - Fixed get_download_url() function
  - README.md - Added installation instructions

  ### Tests Performed
  - Syntax validation: OK
  - GitHub API check: Confirmed filenames match

  ### Code Quality
  - URLs now match actual release artifacts
  - README provides multiple installation options

  ### Recommendations
  - Test installation on fresh Ubuntu/Fedora VM after next release
---
Verify CI-generated filenames and update install.sh + README.md
