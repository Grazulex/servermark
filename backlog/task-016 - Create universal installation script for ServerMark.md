---
id: 16
title: Create universal installation script for ServerMark
status: Done
priority: medium
assignees:
  - '@claude'
labels:
  - devops
  - installation
  - linux
subtasks: []
dependencies: []
blocked_by: []
created_date: '2026-02-03T01:23:05.925Z'
updated_date: '2026-02-03T01:25:26.650Z'
closed_date: '2026-02-03T01:25:26.650Z'
changelog:
  - timestamp: '2026-02-03T01:23:05.925Z'
    action: created
    details: Task created
    user: system
  - timestamp: '2026-02-03T01:23:15.894Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-03T01:23:20.897Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-02-03T01:23:21.885Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-02-03T01:23:22.843Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-02-03T01:23:23.771Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-02-03T01:23:24.703Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-02-03T01:23:25.709Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-02-03T01:23:29.980Z'
    action: updated
    details: 'status: To Do → In Progress'
    user: user
  - timestamp: '2026-02-03T01:23:34.223Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-03T01:24:43.507Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-03T01:24:57.488Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-03T01:25:06.218Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-03T01:25:15.630Z'
    action: modified
    details: Task updated
    user: AI
  - timestamp: '2026-02-03T01:25:19.427Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-02-03T01:25:20.157Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-02-03T01:25:20.913Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-02-03T01:25:21.636Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-02-03T01:25:22.381Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-02-03T01:25:23.137Z'
    action: modified
    details: Task updated
    user: user
  - timestamp: '2026-02-03T01:25:26.650Z'
    action: updated
    details: 'status: In Progress → Done'
    user: user
acceptance_criteria:
  - text: Script detects Ubuntu/Debian and installs .deb package
    checked: true
  - text: Script detects Fedora/RHEL and installs .rpm package
    checked: true
  - text: Script falls back to AppImage for unsupported distros
    checked: true
  - text: Script fetches latest version from GitHub Releases
    checked: true
  - text: Script provides clear error messages
    checked: true
  - text: Script verifies successful installation
    checked: true
ai_plan: >-
  ## Implementation Plan


  ### Objective

  Create a universal installation script (install.sh) that users can run with:

  ```bash

  curl -fsSL
  https://raw.githubusercontent.com/Grazulex/servermark/main/install.sh | bash

  ```


  ### Steps

  1. Detect Linux distribution (Ubuntu, Debian, Fedora, Arch, etc.)

  2. Detect system architecture (x86_64, arm64)

  3. Fetch latest release version from GitHub API

  4. Download appropriate package (.deb, .rpm, or .AppImage)

  5. Install the package using the correct package manager

  6. Verify installation

  7. Display success message with next steps


  ### Files to Create

  - `install.sh` - Main installation script (root of project)


  ### Technical Approach

  - Use /etc/os-release for distribution detection

  - Use uname -m for architecture detection

  - Use GitHub API to get latest release:
  `api.github.com/repos/Grazulex/servermark/releases/latest`

  - Support package managers: apt (deb), dnf/yum (rpm), pacman (AppImage
  fallback)

  - Fallback to AppImage for unsupported distros

  - Require root/sudo for package installation

  - Use curl or wget (whichever is available)


  ### Supported Distributions

  - Ubuntu/Debian → .deb via apt

  - Fedora/RHEL/CentOS → .rpm via dnf/yum

  - Arch/Manjaro → AppImage (or AUR later)

  - Other → AppImage fallback


  ### Edge Cases to Consider

  - No curl/wget available

  - No root access

  - Unsupported architecture

  - GitHub API rate limiting

  - Network errors during download

  - Previous installation exists
ai_notes: >
  **2026-02-03T01:23:34.222Z** - **02:24** - PROGRESS: Starting implementation
  of install.sh script

  **2026-02-03T01:24:43.507Z** - **02:25** - PROGRESS: Created install.sh with
  full functionality:

  - Distribution detection (Ubuntu, Debian, Fedora, RHEL, Arch, etc.)

  - Architecture detection (x86_64, aarch64)

  - GitHub API integration for latest version

  - Package installation (.deb, .rpm, .AppImage)

  - Desktop entry creation for AppImage

  - Colored output and error handling

  **2026-02-03T01:24:57.488Z** - **02:26** - PROGRESS: Script syntax validated
  successfully with bash -n
ai_documentation: >-
  ## Installation Script Documentation


  ### Usage


  **With curl:**

  ```bash

  curl -fsSL
  https://raw.githubusercontent.com/Grazulex/servermark/main/install.sh | sudo
  bash

  ```


  **With wget:**

  ```bash

  wget -qO-
  https://raw.githubusercontent.com/Grazulex/servermark/main/install.sh | sudo
  bash

  ```


  ### Supported Distributions


  | Distribution | Package Type |

  |--------------|--------------|

  | Ubuntu, Debian, Linux Mint, Pop\!_OS, Elementary, Zorin | .deb |

  | Fedora, RHEL, CentOS, Rocky, Alma, Nobara | .rpm |

  | openSUSE | .rpm |

  | Arch, Manjaro, EndeavourOS, Garuda | .AppImage |

  | Other | .AppImage (fallback) |


  ### Supported Architectures

  - x86_64 (amd64)

  - aarch64 (arm64)


  ### What the Script Does

  1. Detects your Linux distribution and architecture

  2. Fetches the latest release version from GitHub

  3. Downloads the appropriate package

  4. Installs using your system's package manager

  5. Creates desktop entry (for AppImage installs)

  6. Verifies successful installation


  ### Requirements

  - Root/sudo access

  - curl or wget

  - Internet connection


  ### Troubleshooting


  **Permission denied:**

  Make sure to run with sudo: `sudo bash install.sh`


  **Download failed:**

  Check your internet connection and GitHub accessibility


  **Unsupported architecture:**

  Currently only x86_64 and aarch64 are supported
ai_review: >-
  ## Self-Review


  ### Completed

  - [x] Script detects Ubuntu/Debian and installs .deb package

  - [x] Script detects Fedora/RHEL and installs .rpm package

  - [x] Script falls back to AppImage for unsupported distros

  - [x] Script fetches latest version from GitHub Releases

  - [x] Script provides clear error messages (colored output)

  - [x] Script verifies successful installation

  - [x] Desktop entry created for AppImage installs

  - [x] Cleanup of temporary files on exit


  ### Tests Performed

  - Syntax validation (bash -n): OK


  ### Code Quality

  - Standards: POSIX-compliant bash, follows best practices

  - Error handling: set -e, explicit error messages

  - Documentation: Complete with usage examples


  ### Limitations

  - Download URL pattern assumes Tauri default naming convention

  - May need adjustment once actual releases are created to match exact
  filenames

  - No uninstall script provided yet


  ### Questions for Developer

  - Verify the exact filename pattern in GitHub releases matches the script

  - Consider adding an uninstall option later?


  ### Recommendations

  - Test on actual distributions after first release

  - Add to README.md installation section

  - Consider adding --version and --help flags to the script
---
Create a shell script that detects Linux distribution and installs the appropriate package (.deb, .rpm, or .AppImage) from GitHub Releases
