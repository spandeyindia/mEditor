# mEditor User Guide

Version: 2.2.0.14

Owner contact: Sanjay Pandey <s.pandey.india@gmail.com>

## First Run And License

mEditor is freeware. The first launch displays the mEditor Freeware EULA gate.

Registration is optional. If the user registers, the About dialog displays `Personal License to USERNAME`. If the user continues without registration, About displays `UnRegistered but fully functional copy with no obligation`.

EULA acceptance is stored in the active workspace at:

`./.meditor/license/acceptance.json`

## Workbench Layout

mEditor uses a SQL Developer-style two-panel workbench.

- Left dock: Connections, Browser/File Explorer, Reports.
- Main workspace: same-window editor and tool tabs.
- Bottom panel: Messages and logging output.
- All major panels are resizable with draggable splitters.

The Window mode selector can keep all features in same-window tabs, allow detached windows later, or ask per action.

## File Explorer And Editors

The File Explorer opens workspace files by double-click or context menu.

Available editor workflows:

- Standard editor tab with save, JSON/XML validation, formatting, and security scan.
- Vi Editor tab with modal editing, movement, line delete/yank/paste, search, substitute, undo, redo, and save.
- JSON/XML Formatter with editable input on the left and live formatted output on the right.

## Project Creation And Import

File > New Project creates a project with mEditor metadata, source folders, About Project, and Project Help documents.

File > Import Project scans the workspace and writes an import report under:

`./.meditor/project/import-report.toml`

Supported planning includes major projects and sub-projects. Supported import detection includes common project files from NetBeans, Eclipse, Visual Studio, Maven, Gradle, Cargo, Node.js, and generic source projects.

## SDLC Tasks

Run > Tasks detects the current project type and exposes Build, Clean, Test, Launch, Debug/Check, and Package actions where tooling is installed.

Task output captures:

- Command line
- stdout and stderr
- Compiler warnings and errors
- Suggested fixes with rationale
- A rollback statement
- A debugging handover report

Reports are stored under:

`./.meditor/debug-reports/`

## Language Support

Setup > Programming Language Support checks installed compilers, interpreters, and toolchains for supported languages and stacks, including Assembly, C, C++, Objective-C, Ada, Python, PHP, Perl, Java, Julia, Rust, COBOL, BASIC, Clipper, HTML, CSS, JavaScript, Node.js, React, Prolog, Lisp, R, SQL, PL/SQL, T-SQL, and related variants.

Setup > Verify And Install Dependencies checks the supported executable catalog and marks each dependency as active or missing. For missing tools, it shows the source URL, package-manager command, and package name where automatic installation is supported. mEditor runs an install only after the user selects the dependency and confirms the command.

Supported automatic install managers include Homebrew on macOS, winget on Windows, and apt-get, dnf, or pacman on Linux. If no automatic installer is available for a dependency, use the displayed source URL and install strategy, then rerun verification.

## DBA Workshop

Tools > DBA Workshop supports:

- Connection profile storage
- SQLite worksheet execution
- JDBC driver registration
- JDBC SQL execution through installed Java and a JDBC driver JAR
- JDBC metadata dashboard, object browsing, column, schema, catalog, index, key, procedure, type, and privilege browsing through installed Java and JDBC driver JAR
- Built-in DBA probes for generic connectivity plus common Oracle, PostgreSQL, MySQL, and SQL Server health/history views where the logged-in database user has privileges
- Object browser and DBA dashboard contract
- Patch, backup, and administration dashboard sections

Passwords used for JDBC execution are not written to workspace JSON. Use Settings > Secrets And Credentials for persistent OS-backed secret storage.

## SSH Terminus

Tools > SSH Terminus supports grouped SSH profiles, connectivity tests, one-shot SSH commands, and persistent same-window terminal sessions using the installed local shell or OpenSSH tooling.

Persistent sessions accept input, poll output, and can be stopped from the tab. On Unix-like platforms, mEditor uses the platform `script` utility as a PTY-backed wrapper when available and reports that state in the tab. If `script` is unavailable, it falls back to process IO. A dedicated native PTY/xterm backend remains the path for perfect terminal-control emulation.

Profiles are stored under:

`./.meditor/connections/ssh-profiles.json`

Secrets are not written into those profiles.

## SFTP/SCP Transfer

Tools > SFTP/SCP Transfer supports remote listing and upload/download queue records using OpenSSH/SCP tooling.

Queue records are stored under:

`./.meditor/transfers/queue.json`

## Security Analyzer

Tools > Code Security Analyzer runs open-file scans using seed rules and records CVSS repository setup paths. The SQLite schema is staged by setup and one-click setup.

## AI/ML Assistant

Tools > AI/ML Assistant stores local knowledge sources, builds a local retrieval index from saved knowledge, fits a local TF-IDF retrieval/ranking model with a training report, and can call a local model runtime adapter when installed, such as Ollama.

Knowledge files are stored under:

`./.meditor/ai/knowledge/`

User approval is required before any AI-generated code change. Auto-remediation must remain disabled until the user explicitly enables it.

## Refactor And LSP

Source > Refactor detects installed language servers and debug adapters such as rust-analyzer, clangd, pyright-langserver, typescript-language-server, jdtls, lldb-dap, codelldb, gdb, debugpy, jdb, and related tools.

The guarded rename refactor creates a rollback backup and report under `.meditor/refactor-rollback/` before changing a workspace file.

## Reports And Local Bugs

Report > Reports opens the local-only user bug repository and report browser. This repository is for the user's own programs and is never sent to the mEditor feedback backend unless the user exports it manually.

## Feedback

Feedback And Bugs records mEditor product feedback for later guarded submission to the configured APEX REST endpoint. Users can also contact:

`s.pandey.india@gmail.com`

## Credentials

Settings > Secrets And Credentials uses the operating-system encrypted credential store where available:

- macOS: Keychain through `security`
- Windows: Credential Manager through `cmdkey`
- Linux: Secret Service through `secret-tool`

mEditor stores only service/account/backend pointers in:

`./.meditor/credentials/credential-index.json`

## Updates

Help > Update Channel Manager stores a Git release source and checks remote tags.

Updates are never installed automatically. mEditor must show release notes, target version, rollback plan, and preserve local `.meditor` workspace data before any approved install.

## Packaging

The local packaging script is:

`packaging/build-local-package.sh`

It builds the release binary, stages docs, setup files, and a platform package folder under `dist/`.

The cross-platform package audit script is:

`packaging/verify-cross-platform-package.sh`

It smoke-checks the current host package and records that Windows/Linux production validation requires native runners or VMs.

The repository also includes `.github/workflows/mEditor-ci.yml` for macOS, Linux, and Windows build/test/package validation on GitHub Actions runners.

## Diagnostics

Help > Diagnostics Bundle previews version, platform, workspace, module count, and error capture fields. Diagnostics must be reviewed and redacted before sharing.
