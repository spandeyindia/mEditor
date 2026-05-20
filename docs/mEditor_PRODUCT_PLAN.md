# mEditor Product Plan

## Product Identity

- **Name:** mEditor
- **Expansion:** Multi Editor
- **Owner:** Sanjay Pandey <s.pandey.india@gmail.com>
- **Version:** 2.2.0.15
- **Frozen design version:** 2.2.0.0
- **Freeze date:** 2026-05-19
- **Freeze status:** Ver 2.2 design frozen
- **Primary implementation language:** Rust
- **Target platforms:** Windows laptops, macOS Intel laptops, macOS Apple Silicon laptops including M4, and Linux laptops.
- **Primary experience:** Standalone SQL Developer-style IDE and database workbench with a multi-language editor, file explorer, embedded web browser, SDLC toolbar, Git/SVN version control, connection navigator, worksheets, object browser, result grids, task consoles, grouped tabbed SSH terminals, project importers including Visual Studio, framework wizards, code security analysis, AI/ML coding assistance, AI training, spec-to-system design, XML/JSON validation, and toolchain discovery.
- **Starting product license:** `mEditor Freeware EULA` (`LicenseRef-mEditor-Freeware-EULA`), with optional future open-source relicensing such as GPL v3.0 at owner discretion.

## Ver 1 Freeze

Ver 1 is frozen as `1.5.0.0-frozen` on 2026-05-18. This document is the Ver 1 design baseline for mEditor.

Freeze rules:

- New major capabilities after this point should be tracked as Ver 1 amendments or Ver 2 candidates.
- Clarifications, implementation notes, source references, typo fixes, and platform-specific packaging notes may be added without changing the frozen feature scope.
- The visible product name must remain exactly `mEditor`.
- Version numbers must follow `Major release.Minor release.Bugfix or enhancement.Build`.
- Each feature imported from another app must increment the Minor release component.
- Every produced build must increment the Build component.
- The Ver 1 freeze record is maintained in `docs/mEditor_VERSION_1_FREEZE.md`.
- The Ver 1 implementation baseline and benchmark record is maintained in `docs/mEditor_BASELINE_BENCHMARK.md`.
- The Ver 2 pre-pilot baseline and benchmark record is maintained in `docs/mEditor_VER_2_PRE_PILOT_BASELINE.md`.
- The Ver 2.1 baseline and benchmark record is maintained in `docs/mEditor_VER_2_1_BASELINE.md`.
- The Ver 2.2 baseline and benchmark record is maintained in `docs/mEditor_VER_2_2_BASELINE.md`.
- The Ver 2.2 design freeze record is maintained in `docs/mEditor_VER_2_2_DESIGN_FREEZE.md`.

## Ver 2 Pre-Pilot Freeze

Ver 2 is frozen as `2.0.0.0-pre-pilot` on 2026-05-19. This baseline adds the Project Workspace, Project Planner, embedded project About/Help documentation model, professional IDE capability set, SFTP/SCP transfer manager, UML Modeling, Local Bug Repository, Report Center, and pre-pilot Git workspace readiness.

Every mEditor-created project must include embedded About Project and Project Help documentation under `.meditor/project/docs`, exposed from the Help menu and opened in same-window tabs.

## Ver 2.1 Baseline

Ver 2.1 is tracked as `2.1.0.0` on 2026-05-19. It adds Workspace Trust, Local History and Recovery, Secrets And Credentials, CI/CD Generator, API Workbench, Database Migration, Workspace Backup And Restore, Plugin Permissions, Accessibility And Keyboard, and Audit Trail as inbuilt same-window capabilities.

Safety rules:

- Unknown workspaces open as Restricted until the user trusts them.
- Restricted workspaces disable scripts, auto-builds, terminals, network calls, database/SSH connections, AI code execution, and plugin activation.
- Local History snapshots file saves and project metadata and supports crash recovery and previous-state restore.
- Secret values belong in the OS keychain or equivalent secure store, not in plain JSON.
- Backups exclude raw secrets and use preview plus rollback for restore.
- Plugin permissions are deny-by-default and logged to the local audit trail.
- Accessibility and keyboard compatibility are first-class requirements, not optional polish.

## Ver 2.2 Baseline

Ver 2.2 is tracked as `2.2.0.0` on 2026-05-19. It adds Visual Studio solution/project import, Update Channel Manager, Diagnostics Bundle, Privacy Center, Extension SDK, Workspace Indexer, XML Validator, and JSON Validator.

Ver 2.2 is design-frozen on 2026-05-19. Implementation, tests, packaging, and documentation can continue inside this scope, but new user-visible capability groups, new menu families, new background data collection, or changes to license/privacy/versioning rules require a later version baseline.

The first post-freeze implementation build is `2.2.0.1`, which adds the mandatory native Rust GUI shell. The GUI shell opens the frozen mEditor menu model, file explorer, command palette, feedback surface, about/update surface, workbench dashboard, editor tabs, and module surfaces in the same main window by default.

Build `2.2.0.2` adds Rust-backed GUI actions for immediately verifiable behavior: workspace file open/save, JSON/XML validation, code-security scan, File Explorer refresh, DBA worksheet dry-run, toolchain detection, SQLite setup preview, project import preview, AI assistant request acknowledgement, feedback payload preview, diagnostics preview, and update-check preview.

Build `2.2.0.3` fixes the generated GUI JavaScript that prevented the central dashboard and backend-response panels from rendering, and updates menu behavior so top menus do not stack over each other.

Build `2.2.0.4` moves the visible mEditor version label to the native title bar and changes the main toolbar below the menu to medium icon buttons.

Build `2.2.0.5` adds Rust-backed real-time validation for JSON/XML-family editor tabs and improves top-menu opening speed with custom menu toggles.

Build `2.2.0.6` adds functional New Project scaffold creation, Import Project report writing, and active JSON/XML editor reformatting behind the GUI buttons.

Build `2.2.0.7` adds a same-window Vi-style modal editor under Source, including file/context launch paths, normal/insert/command modes, common Vi movement/edit/search commands, and Rust-backed `:w` save.

Build `2.2.0.8` adds a same-window JSON/XML formatter workbench with a two-panel layout: editable input with normal clipboard paste/copy plus upload on the left, live Rust-backed formatted output on the right, and apply-to-open-editor behavior that remains in memory until the user explicitly saves.

Build `2.2.0.9` completes the first end-to-end project workflow slice by adding runnable scaffolds for new projects, project detection under Run > Tasks, known task execution for build/debug/test/launch/clean/package profiles, stdout/stderr and compiler diagnostic capture, user-controlled suggestions, and saved local debugging handover reports under `.meditor/debug-reports`.

Build `2.2.0.10` completes the remaining pre-pilot readiness gaps as guarded first-pass workflows: one-click SQLite setup, Git/SVN Team actions, DBA SQLite execution with JDBC driver profile registration, SSH profile save/test, SFTP/SCP remote listing, iframe-based same-window browser navigation, AI local knowledge storage, planner item persistence, local bug repository and report listing, Extension SDK skeleton generation, Workspace Indexer output, and feedback outbox storage.

Build `2.2.0.11` adds release-hardening hooks inside the frozen Ver 2.2 scope: first-run EULA gate persistence, refreshed About license display, robust workflow error dialogs, OS-backed credential storage, Update Channel Manager Git tag checks, live JDBC execution through installed Java/JDBC drivers, SSH command-tab execution, SFTP/SCP queue and immediate transfer execution, local AI runtime adapter, LSP server detection, local packaging script, and end-user documentation.

Build `2.2.0.12` adds Setup > Verify And Install Dependencies. It checks the supported executable catalog, identifies missing dependencies, displays source URLs and platform package-manager install commands, and executes the selected install only after user confirmation.

Build `2.2.0.13` adds runtime-completion hardening for the remaining pre-pilot gaps: persistent same-window SSH/local terminal sessions with process input/output, JDBC metadata dashboard/object/column browsing through installed Java and JDBC drivers, local AI knowledge reindexing, guarded rename refactor with rollback backup/report, debug adapter detection, and a cross-platform package audit script.

Build `2.2.0.14` adds production-hardening implementation for the known readiness gaps: terminal sessions expose PTY-backed launch status where the platform `script` wrapper is available, JDBC metadata browsing covers schemas, catalogs, indexes, keys, procedures, type information, and privileges, DBA probes cover common Oracle/PostgreSQL/MySQL/SQL Server health queries, AI/ML Assistant fits a local TF-IDF retrieval/ranking model with a training report, and the repository includes macOS/Linux/Windows CI validation assets.

Build `2.2.0.15` fixes the packaged macOS `.app` launch workflow by resolving a writable startup workspace when the OS starts the app with `/` as current directory, so EULA acceptance and local `.meditor` state are not written to the read-only filesystem root.

Visual Studio import requirements:

- `File > Import Project` must detect `.sln`, `.csproj`, `.vbproj`, `.fsproj`, `.vcxproj`, `.vcproj`, `.sqlproj`, `.dbproj`, `.shproj`, `.esproj`, `.njsproj`, `.wixproj`, `.props`, `.targets`, and `.filters` files.
- Solution entries map to mEditor major projects with sub-projects.
- MSBuild configurations, platforms, properties, imports, references, package references, source roots, and generated output folders are recorded in mEditor metadata.
- C++ `.filters` files map to virtual folders without moving physical files.
- Import reports must show converted settings, unresolved SDKs, missing workloads, package restore requirements, and manual follow-up items.

Pilot-hardening requirements:

- Update Channel Manager supports stable, beta, nightly, and offline packages with release notes and rollback before install.
- Diagnostics Bundle collects logs, environment, config, recent errors, plugin list, toolchain paths, workspace trust state, and redacted connection data after user review.
- Privacy Center shows install, feedback, error, update, and diagnostics data flows.
- Extension SDK supports language, compiler, formatter, debugger, snippet, template, documentation, theme, tool window, and project wizard contributions.
- Workspace Indexer provides symbols, references, call hierarchy, TODOs, dependency graph, files, tests, and endpoints.
- XML Validator and JSON Validator open under Tools and validate active editor tab, selected text, or chosen files with approval-gated fixes.

## Standalone Boundary

mEditor is a standalone desktop IDE. All features should run inside the local desktop application unless the user explicitly opens a database connection, starts an SSH terminal, runs a build, starts a debugger, or invokes a project tool.

Standalone scope:

- Local project workspace management.
- Local editor, file explorer, embedded web browser, LSP, DAP, terminal, search, and framework tooling.
- Local SDLC actions and Git/SVN version-control workflows.
- Local code security analysis, CVSS repository search, remediation guidance, AI/ML coding assistance, knowledge ingestion, and spec-to-system workflows.
- Direct user-initiated database connections from DBA Workshop.
- Direct user-initiated SSH terminal sessions from SSH Terminus.
- Local metadata for imported projects.
- First-run installation record, active freeware EULA acceptance, optional registration identity, and install-base identification using network domain, machine name, and IP address.
- No always-on service requirement for normal IDE use.
- First-class behavior on Windows, macOS Intel, macOS Apple Silicon including M4, and Linux.

## Feasibility

It is feasible to create mEditor, but it should be treated as a staged platform rather than a single-pass editor. The practical approach is to build a Rust desktop shell and integrate existing open-source language tooling instead of writing every compiler, parser, debugger, and framework engine from scratch.

The core feasibility strategy is:

- Use LSP for language intelligence.
- Use DAP for debugging.
- Use Tree-Sitter for fast syntax parsing where available.
- Use a data-driven toolchain registry for compilers, interpreters, package managers, framework CLIs, and documentation links.
- Use a plugin model so rare languages, legacy compilers, and framework templates can be added without rebuilding the editor core.
- Keep database adapters separate from the UI so the same workbench model can support many database engines and SQL dialects.

## Cross-Platform Laptop Support

mEditor must be designed and coded as a cross-platform laptop application from the start.

Primary supported platforms:

- Windows laptops on x64, with ARM64 support where the selected webview, terminal, compiler, and native packaging stack support it.
- macOS laptops on Intel.
- macOS laptops on Apple Silicon, including M1, M2, M3, and M4.
- Linux laptops on x64, with ARM64 support where the selected desktop, webview, terminal, and packaging stack support it.

Cross-platform engineering rules:

- Keep operating-system-specific code behind Rust traits and platform adapters.
- Avoid hardcoded path separators, home directory assumptions, shell names, line endings, font names, modifier keys, application data folders, certificate stores, or terminal behavior.
- Use platform-aware APIs for paths, config directories, file watching, process launching, environment variables, permissions, keychain/credential storage, notifications, clipboard, drag/drop, menus, and file associations.
- Keep keyboard shortcuts configurable, with platform-appropriate defaults for Windows/Linux and macOS.
- Keep native menus, tray behavior, title bars, dialogs, file pickers, and browser/webview differences behind the shell abstraction.
- Terminal support must handle `cmd`, PowerShell, Git Bash, WSL, zsh, bash, fish, and custom shells according to platform availability.
- Browser support must allow a platform webview or bundled browser engine, with feature detection instead of platform guesses.
- Packaging must produce native installers or bundles appropriate to each platform, including Windows installer/MSIX or ZIP, macOS universal or per-architecture app bundles/DMG/PKG, and Linux AppImage/deb/rpm/tar packages where feasible.
- Interim packaging may use platform-specific launchers on top of a generic JAR until the native build infrastructure is ready for every supported platform.
- The generic-JAR strategy should preserve one shared application core while launchers handle OS integration, runtime discovery, app data paths, environment setup, logging, updates, icons, and process startup.
- Platform launchers should include Windows `.exe` or script launchers, macOS `.app` launchers, and Linux shell/AppImage-style launchers as appropriate.
- macOS builds should account for Intel, Apple Silicon, universal binaries where practical, signing, notarization, entitlements, hardened runtime, and keychain access.
- Windows builds should account for code signing, installer elevation, long path handling, Defender-friendly packaging, certificate stores, and Windows Terminal/ConPTY integration.
- Linux builds should account for distro differences, Wayland/X11, desktop portals, system keyrings, AppImage sandbox limits, package dependencies, and user-installed webview libraries.
- Every core feature should have a platform compatibility matrix before implementation is considered complete.
- CI should build and test on Windows, macOS Intel or universal-capable runners, macOS Apple Silicon where available, and Linux.
- Feature code should degrade gracefully when a platform dependency is unavailable and show a clear configuration or install hint.

## Rust Workspace Architecture

Recommended crates:

- `meditor-shell`: desktop shell, menu model, command palette, tab manager, docking layout, preferences, and update hooks.
- `meditor-platform`: platform adapters for paths, config folders, keychains, menus, dialogs, file watching, process launching, terminals, webviews, notifications, clipboard, shortcuts, and OS capability detection.
- `meditor-file-explorer`: workspace and local filesystem trees, file context menus, open-in-editor routing, file operations, filters, bookmarks, and source-control decorations.
- `meditor-browser`: embedded same-window web browser tabs, address bar, navigation history, bookmarks, downloads, developer tools handoff, local preview integration, and browser security policy.
- `meditor-editor`: editor panes, syntax highlighting, folding, selections, snippets, multi-cursor support, and large-file mode.
- `meditor-lsp`: LSP process lifecycle, workspace indexing, diagnostics, completion, hover, code actions, rename, references, and formatting.
- `meditor-dap`: debugger session lifecycle, breakpoints, variables, stack frames, watches, consoles, and launch profiles.
- `meditor-tree-sitter`: grammar registry, parser loading, incremental parse cache, symbol outline, and structural search.
- `meditor-toolchains`: compiler/interpreter discovery, version checks, install hints, build profiles, task runner, and environment isolation.
- `meditor-sdlc`: build, clean, rebuild, test, deploy, launch/run, debug, package, publish, custom tasks, task dependencies, environment profiles, and console output capture.
- `meditor-vcs`: Git and SVN providers, status model, diff views, history, commit, branch/tag, merge/update, revert, blame/annotate, credentials, and conflict resolution.
- `meditor-plugins`: extension manifest, permissions, commands, menus, language contributions, toolchain contributions, database contributions, settings, activation events, and sandboxing.
- `meditor-tests`: Test Explorer tree, test discovery, test execution, rerun failed, debug test, test output, flakiness markers, and framework adapters.
- `meditor-coverage`: coverage import, line/branch coverage gutter markers, reports, filters, trend history, and coverage export.
- `meditor-dependencies`: package/dependency manager for Cargo, Maven, Gradle, npm, pnpm, Yarn, Bun, Composer, pip/uv, NuGet, and generic manifests.
- `meditor-security`: CVE scanning, license scanning, secrets scanning, dependency policy, SBOM generation, and remediation suggestions.
- `meditor-code-security`: embedded DVCLAS code analyzer port, source-rule scanning, taint/correlation hooks, editor context scans, findings lifecycle, and secure rewrite previews.
- `meditor-cvss-repository`: SQLite-backed vulnerability and CVSS repository, CVSS 3.1/4.0 vector validation, NVD/CISA feed sync, encrypted bundle import/export, and repository search.
- `meditor-aiml-engine`: embedded DVCLAS AI/ML engine port, coding assistance models, remediation ranking, vulnerability trend analysis, local context retrieval, PDF-informed learning, controlled retraining/fine-tuning jobs, and privacy controls.
- `meditor-chat-assistant`: same-window AI/ML Assistant chat tab, workspace context prompts, finding explanations, test suggestions, refactoring guidance, and approval-gated edit proposals.
- `meditor-knowledge-base`: multi-format document, ebook, text, and spreadsheet ingestion; text/table/code extraction; chunking; embeddings; citation index; training corpus registry; retraining job queue; model/version provenance; and knowledge export/import.
- `meditor-ai-training-studio`: curriculum builder for programming languages, software design, system design, computer architecture, database design, testing, security engineering, and DevOps; supervised fine-tuning jobs; adapter training; benchmark suites; and model evaluation reports.
- `meditor-spec-workbench`: specification ingestion, requirement extraction, architecture synthesis, module decomposition, backlog generation, traceability matrix, test-plan generation, and approval-gated implementation proposals.
- `meditor-secure-store`: encrypted JSON storage, key derivation, import/export envelopes, schema migration, and secret redaction utilities.
- `meditor-db-workbench`: connection tree, worksheet tabs, explain plan surface, object browser, SQL history, result grid, exports, dashboards, and guarded administration flows.
- `meditor-db-connectors`: SQLite, PostgreSQL, MySQL/MariaDB, SQL Server, DB2, DB/400, ODBC, JDBC bridge, user-added JDBC drivers, DuckDB, ClickHouse, and other database drivers.
- `meditor-db-modeler`: ER diagrams, schema compare, data compare, DDL generation, migration runner, and explain-plan visualization.
- `meditor-ssh-terminus`: grouped SSH profiles, tabbed interactive terminal sessions, known-hosts validation, key/password authentication, jump hosts, port forwarding, command execution, and session logs.
- `meditor-local-terminal`: local shell profiles, project terminals, environment activation, terminal groups, scrollback search, and transcript options.
- `meditor-project-importers`: NetBeans, Eclipse, JDeveloper, and Visual Studio importers with source root, classpath, dependency, builder, run profile, server metadata, solution/project, MSBuild, configuration, platform, reference, package, and import report mapping.
- `meditor-frameworks`: project generators and framework-aware actions for Rust, JavaScript/TypeScript, PHP, Python, Java/JVM, R, Julia, and data apps.
- `meditor-keymaps`: SQL Developer, NetBeans, Eclipse, VS Code, IntelliJ-style, Vim-style, Emacs-style, and custom keymap profiles.
- `meditor-settings`: settings import/export, encrypted profile bundles, layout snapshots, theme export, keymap export, and toolchain export.
- `meditor-task-chains`: saved task chains, scheduled local task sequences, preflight checks, dependent actions, and rollback steps where supported.
- `meditor-dashboard`: workspace dashboard for project health, SDLC status, VCS status, tests, coverage, dependencies, security, database connections, SSH sessions, and recent errors.
- `meditor-offline-docs`: offline documentation browser, indexed language/framework/compiler docs, SQL dialect docs, and local help bundles.
- `meditor-large-file`: large-file mode, log viewer, CSV/TSV viewer, SQL dump viewer, streaming search, tail mode, and memory-safe file handling.
- `meditor-diff-merge`: visual file diff, three-way merge, VCS conflict resolution, folder compare, data compare handoff, and schema compare handoff.
- `meditor-native`: GraalVM Native Image helper profiles, native executable packaging, app signing hooks, and release artifacts.
- `meditor-packaging`: Windows, macOS Intel, macOS Apple Silicon including M4, and Linux packaging, interim generic-JAR launchers, signing, installer metadata, bundle layout, update channels, and release validation.
- `meditor-docs`: embedded help, upstream documentation index, language references, compiler install guides, and offline doc bundles.

## SQL Developer-Style UI Model

The first screen should be the working IDE, not a landing page.

Expected panes and workflows:

- Left navigator with Workspaces, File Explorer, Connections, Toolchains, Frameworks, Files, Symbols, and Tasks.
- Tabbed central area for source editors, embedded browser tabs, SQL worksheets, result grids, explain plans, dashboards, terminals, generated reports, and every opened workbench tool by default.
- Right sidebar for outline, properties, connection details, build variables, diagnostics, and documentation.
- Bottom region for problems, output, debug console, terminal, search results, version control, task logs, and background jobs.
- AI/ML Assistant chat tab for developer Q&A, code explanation, security remediation, test generation, guided refactoring, PDF-enriched project knowledge, and spec-driven system design.
- Main toolbar and project context menu with standard SDLC actions: build, clean, rebuild, test, deploy, launch/run, debug, package, publish, stop, rerun, and open logs.
- Connection workflow with saved profiles, encrypted credentials, test connection, schema/object browser, recent worksheets, query history, and export options.
- DBA workflow with availability, sessions, locks, storage, tablespaces/filegroups, replication/standby evidence, backup/restore readiness, high-availability topology, and forecast-ready time series.
- DBA Panel workflow that can be activated for a connection when the privilege probe confirms `SYSDBA` or an equivalent administrator/monitoring role for that database engine.
- Administration actions should be review-first and approval-gated. Destructive or state-changing actions should generate a plan and evidence package before execution.
- Database driver workflow with a SQL Developer-style driver manager: add JDBC JAR files, detect driver classes, define URL templates, test sample connections, map the driver to a SQL dialect, and enable object browser support where metadata APIs are available.

Tools menu requirements:

- `Tools > DBA Workshop`: opens the embedded database workbench with saved database connections, worksheets, object browser, monitoring probes, dashboard views, and guarded administration actions.
- `Tools > SSH Terminus`: opens grouped SSH connection profiles and multiple SSH terminals in tabs, with open-group, close-group, reconnect, command execution, port forwards, and session history.
- `Tools > Local Terminals`: opens local terminal profiles and project-scoped shell tabs.
- `Tools > Web Browser`: opens the embedded full browser in the same main window, with address bar, back/forward, reload, bookmarks, history, downloads, local project preview, and optional external-browser handoff.
- `Tools > Dependencies`: opens package/dependency inventory, update suggestions, lockfile state, and dependency graph.
- `Tools > Security Scan`: opens CVE, license, secret, and SBOM scanning workflows.
- `Tools > Code Security Analyzer`: opens the embedded DVCLAS code security analyzer with source findings, taint/correlation views, remediation guidance, scan history, and exportable reports.
- `Tools > CVSS Repository`: opens the SQLite-backed vulnerability repository with CVE/CWE/package search, CVSS vector validation, NVD/CISA sync status, encrypted bundle import/export, and manual update actions.
- `Tools > AI/ML Assistant`: opens the same-window coding assistant chat for current file, project, security finding, database worksheet, build error, or selected code context.
- `Tools > AI Knowledge Base`: imports PDFs, EPUB/MOBI ebooks, DOC/DOCX documents, TXT files, XLS/XLSX spreadsheets, CSV files, Markdown, and documentation bundles into the assistant knowledge base, shows extraction quality, manages citations, and controls retraining/fine-tuning jobs.
- `Tools > AI Training Studio`: manages programming-language, software-design, system-design, computer-architecture, database-design, testing, security, and DevOps training corpora, model jobs, benchmark suites, and assistant evaluation reports.
- `Tools > Spec-to-System Workbench`: imports a specification document and produces requirements, architecture, module plan, task backlog, tests, risks, and approval-gated implementation changes.
- `Tools > Database Modeling`: opens ER diagrams, schema compare, data compare, DDL generation, migration runner, and explain-plan visualizer.
- `Tools > Offline Documentation`: opens indexed local documentation bundles.
- `Tools > Large File Viewer`: opens large logs, CSVs, SQL dumps, and generated files in streaming mode.
- `Team > Git`: opens Git status, branches, tags, remotes, history, diff, commit, stash, merge/rebase, pull, push, fetch, and conflict resolution.
- `Team > SVN`: opens SVN checkout, update, commit, diff, revert, log, branch/tag, switch, merge, resolve, cleanup, lock, unlock, and annotate.
- `Team > Diff/Merge`: opens visual diff, three-way merge, conflict resolution, folder compare, and VCS compare tools.
- `Run > Test Explorer`: opens discovered tests, test output, failed tests, debug-test actions, and coverage entry points.
- `Run > Coverage`: opens coverage reports, gutter overlays, and historical coverage.
- `Run > Task Chains`: opens saved build/test/package/deploy chains and custom action sequences.
- `Window > File Explorer`: opens or focuses the File Explorer in the same main window.
- `Window > Workspace Dashboard`: opens project health, VCS, test, coverage, dependency, security, connection, and error summaries.
- `Settings > Keymaps`: configures SQL Developer, NetBeans, Eclipse, VS Code, IntelliJ-style, Vim-style, Emacs-style, and custom shortcuts.
- `Settings > Extensions`: installs, enables, disables, updates, and removes plugins.
- `Settings > Import/Export`: imports and exports settings, layouts, keymaps, themes, toolchains, connections, and extension lists.
- `File > Import Project`: imports NetBeans, Eclipse, JDeveloper, and Visual Studio projects into the mEditor workspace model.
- `Tools > Extension SDK`: creates and validates extension skeletons for languages, compilers, formatters, debuggers, snippets, templates, documentation, themes, tools, and project wizards.
- `Tools > Workspace Indexer`: indexes files, symbols, references, call hierarchy, TODOs, dependency graph, tests, and endpoints.
- `Tools > XML Validator`: validates XML-family documents from open editor tabs, selections, or chosen files.
- `Tools > JSON Validator`: validates JSON and JSON schema documents from open editor tabs, selections, or chosen files.
- `Help > Update Channel Manager`: lets the user choose stable, beta, nightly, or offline update channels with release notes and rollback plan.
- `Help > Diagnostics Bundle`: creates a reviewed and redacted support bundle.
- `Help > Privacy Center`: shows install, feedback, error, update, and diagnostics data flows.

## Window And Tab Policy

mEditor should default to a same-window tabbed model. Opening any file, tool, connection, dashboard, terminal, test view, documentation page, diff, project wizard, settings page, or generated report should create or focus a tab inside the same main window unless the user changes the preference.

User preference:

- `Settings > Appearance > Window Behavior`.
- Default: `Open everything in same window as tabs`.
- Optional: `Allow detached windows`.
- Optional: `Ask each time for detachable tools`.

Behavior rules:

- Same-window tabs are the default for all modules, including File Explorer, Web Browser, DBA Workshop, SSH Terminus, Local Terminals, Test Explorer, Coverage, Dependencies, Security Scan, Code Security Analyzer, CVSS Repository, AI/ML Assistant, AI Knowledge Base, AI Training Studio, Spec-to-System Workbench, Database Modeling, Offline Documentation, Large File Viewer, Diff/Merge, Settings, and Workspace Dashboard.
- Tools should reuse an existing tab when opening the same logical target unless the user chooses `Open in New Tab`.
- Tabs should support pin, close, close others, close tabs to right, duplicate, move to group, split editor area, and restore closed tab.
- Detached windows are allowed only when the user enables detached-window behavior or explicitly chooses a detach action.
- Detached windows should preserve the same command palette, keymaps, theme, secure-store access, and workspace context.
- The workspace layout should be persisted in the mEditor working folder and included in settings export when the user chooses to include layout.
- Plugins must respect the active window behavior and should not spawn external windows unless their permission manifest and the user preference allow it.

## File Explorer And Embedded Web Browser

mEditor must provide both a File Explorer and an embedded full Web Browser as same-window workbench features.

File Explorer requirements:

- Open from `Window > File Explorer` and from the left navigator.
- Show workspace folders, project roots, local folders, hidden-file toggle, ignored-file toggle, bookmarks, recent folders, and source-control decorations.
- Support create file, create folder, rename, duplicate, copy, paste, move, delete with confirmation, refresh, reveal in system file manager, copy path, copy relative path, and open terminal here.
- A file right-click context menu must include `Open in Editor Tab`.
- Double-clicking a recognized text/source/config file should open it in an editor tab by default.
- `Open in Editor Tab` should focus an existing editor tab for that file when already open, or create a new tab in the current editor area when not open.
- Context menu should also support `Open to Side`, `Open With`, `Preview`, `Security > Scan File`, `Run Related Task`, `Compare With`, and `Show File History` where applicable.
- Binary files should open through an appropriate viewer or show an `Open With` choice rather than forcing the source editor.
- Large files should route to Large File Viewer when size or type rules require streaming mode.
- All file operations should respect unsaved editor buffers, VCS state, read-only files, and user confirmation for destructive actions.

Embedded Web Browser requirements:

- Open from `Tools > Web Browser` as a tab inside the same main window.
- Provide address bar, back, forward, reload, stop, home, find in page, zoom, bookmarks, history, downloads, copy URL, open URL in external browser, and clear browsing data.
- Support multiple browser tabs inside the central work area and reuse existing browser tabs when opening the same URL unless the user chooses a new tab.
- Support local project preview for web apps, generated documentation, coverage reports, database reports, API docs, and local development servers.
- Support HTTP, HTTPS, and local files where the security policy allows them.
- Use a platform webview engine or bundled browser engine behind a Rust abstraction so the browser remains part of the standalone desktop app.
- Apply a browser security policy for downloads, mixed content, local file access, certificates, clipboard, popups, permissions, and external protocol handlers.
- Provide a developer-tools handoff when the underlying engine supports it.
- Browser downloads should go through a visible download manager and never overwrite files silently.
- Browser tabs should follow the global same-window/detached-window preference.

## Freeware License, Registration, And Install-Base Record

mEditor starts under the `mEditor Freeware EULA` with internal license code `LicenseRef-mEditor-Freeware-EULA`, version `1.0`. The active EULA is stored in `mEditor/LICENSE.md`.

First-run requirements:

- Display the active freeware EULA in the Registration dialog.
- Require EULA acceptance before the product opens.
- Store license code, license name, license version, accepted flag, accepted timestamp, and accepted-by value in the installation record.
- Registration remains optional and must state that mEditor is freeware with no licensing or financial obligation to the user or the user's organisation.
- If the user registers, capture the confirmed display name and email/id, and show `Personal License to USERNAME` in the About dialog.
- If the user continues without registration, show `UnRegistered but fully functional copy with no obligation` in the About dialog.
- In both registered and unregistered flows, send installation id, product/version, registration status, network domain, machine name, IP address, OS family, and CPU architecture to the backend installation table.
- Network domain, machine name, and IP address are used only to identify the install base and must not be used in bug reports or any other reporting.
- The About dialog must include a License button that displays the active EULA and accepted license metadata.
- A top-right menu-bar Feedback icon must open a same-window feedback panel.
- The feedback panel must support dual mode: automatic error-capture bug reports and manual bug/enhancement requests.
- Automatic error mode should capture app version, OS family, CPU architecture, error message, error kind, stack trace, active module, active file, active command, and recent log excerpt where available.
- Manual feedback mode should let the user choose `Bug Report` or `Enhancement Request` and enter title, details, severity, category, steps, expected behavior, and actual behavior.
- If the user is unregistered, the feedback flow must ask for a personal email id and state that it is used only to inform the user about the bugfix or status update for this issue.
- The feedback panel should show user-visible metrics: feedback given, bugs reported, enhancements requested, open items, fixed items, and latest status.
- The feedback panel must display owner contact email `s.pandey.india@gmail.com` for direct correspondence.
- Bug reports and enhancement requests are sent only when the user submits them.

Backend schema:

- The APEX table proposal is in `setup/apex/meditor_installation_metrics_schema.sql`.
- The implementation note is in `docs/mEditor_INSTALLATION_METRICS_APEX_SCHEMA.md`.
- License option names are in `docs/mEditor_LICENSE_OPTIONS.md`.

## Extension And Plugin System

mEditor must have a plugin system so languages, compilers, database drivers, themes, snippets, keymaps, project templates, tools, and UI contributions can be added without rebuilding the core IDE.

Plugin requirements:

- Plugin manifest in JSON or TOML with id, name, version, publisher, license, activation events, contributions, permissions, and compatibility range.
- Contribution points for languages, grammars, LSP servers, DAP adapters, SDLC actions, project templates, framework wizards, snippets, themes, icons, keymaps, database dialects, JDBC driver definitions, object browser adapters, test adapters, coverage adapters, dependency providers, security scanners, and documentation bundles.
- Commands contributed by plugins should appear in the command palette and may add menu or toolbar actions.
- Plugins should run with least privilege and declare filesystem, process, network, terminal, and credential access needs.
- Plugin install/update/remove should be available from `Settings > Extensions`.
- Workspace-specific enable/disable should be supported.
- mEditor should maintain a local extension cache in the working folder and support offline plugin bundles.

## Test Explorer And Coverage

mEditor must include a unified Test Explorer and code coverage view.

Test Explorer requirements:

- Discover tests from language/framework adapters, including Rust, Java/JVM, JavaScript/TypeScript, PHP, Python, Go, C/C++, R, Julia, and generic command adapters.
- Show tests as a tree grouped by project, module, suite, file, class, function, and parameterized case.
- Actions: run all, run selected, rerun failed, debug selected, stop, filter, search, open source, and show output.
- Preserve test history, duration, status, failure message, stack trace, stdout/stderr, and flaky markers.
- Tests should integrate with SDLC profiles and problem matchers.

Coverage requirements:

- Import coverage from common formats such as LCOV, Cobertura XML, JaCoCo XML, LLVM coverage, grcov, coverage.py XML/JSON, Istanbul/nyc, PHPUnit coverage, and generic line coverage.
- Display line and branch coverage in the editor gutter.
- Provide coverage summary by workspace, project, module, package/folder, file, function, and test run.
- Allow coverage thresholds per project and fail SDLC task chains when thresholds are not met.
- Export coverage reports as HTML, JSON, XML, or text where supported.

## Dependency Manager And Security Scanner

mEditor must include a dependency manager panel and a security scanner.

Dependency manager requirements:

- Detect manifests and lockfiles for Cargo, Maven, Gradle, npm, pnpm, Yarn, Bun, Composer, pip/uv, Poetry, Hatch, NuGet, Go modules, R packages, Julia packages, and generic SBOMs.
- Show direct and transitive dependencies, versions, licenses, update availability, dependency graph, and lockfile drift.
- Provide safe update actions, manifest editor shortcuts, lockfile refresh, dependency search, and dependency impact view.
- Keep package-manager commands configurable through the toolchain executable settings.

Security scanner requirements:

- Scan dependencies for CVEs and advisories.
- Scan source and config files for secrets.
- Scan dependency licenses and compare them with project policy.
- Generate SBOMs where supported.
- Show remediation suggestions, fixed versions, severity, exploit maturity where available, and owner/status fields.
- Integrate findings into Problems, Workspace Dashboard, and SDLC gates.

## Embedded Code Security Analyzer, CVSS Repository, And AI/ML Assistant

mEditor must embed the DVCLAS code security analyzer as a separate workbench module rather than hiding it inside the generic dependency scanner.

Porting source map:

- `core/src/main/java/com/dcsvlas/core/security/CodeSecurityScanner.java` should become the Rust source scanner service.
- `core/src/main/java/com/dcsvlas/core/security/EnterpriseSecurityRulePack.java` should become the seed security rule pack.
- `core/src/main/java/com/dcsvlas/core/util/CvssEngine.java` should become the CVSS 3.1/4.0 scoring and validation engine.
- `core/src/main/java/com/dcsvlas/core/intel/VulnerabilityRepository.java` should become the local SQLite vulnerability repository.
- `core/src/main/java/com/dcsvlas/core/intel/VulnerabilityIntelligenceUpdater.java` should become the auto-update service for public vulnerability feeds and encrypted bundles.
- `core/src/main/java/com/dcsvlas/core/intel/VulnerabilityService.java` should become the orchestrator for scans, repository lookup, remediation, diagnostics, and scheduled updates.
- `core/src/main/java/com/dcsvlas/core/intel/RemediationRecommender.java` and `core/src/main/java/com/dcsvlas/core/intel/PatchAssistant.java` should become remediation and fix-preview providers.
- `core/src/main/java/com/dcsvlas/core/analysis/` should seed taint analysis, dependency correlation, lockfile scanning, CVSS vector repair, semantic parsing, and compile-safe rewrite features.

Code Security Analyzer requirements:

- Open from `Tools > Code Security Analyzer` in the same main window.
- Add an editor context menu entry: `Security > Scan Open File`.
- Add optional editor context actions: `Security > Scan Selection`, `Security > Scan Project`, `Security > Explain Finding`, `Security > Generate Fix Preview`, and `Security > Ask AI/ML Assistant`.
- Scan the open file without requiring a full workspace scan.
- Display findings in the editor gutter, Problems panel, Code Security Analyzer tab, Workspace Dashboard, and exportable reports.
- Support rule-pack updates, project rule overrides, severity policies, false-positive suppression, owner/status fields, and SDLC gate integration.
- Correlate source findings with dependency CVEs, CVSS scores, CWE, remediation text, references, and available fixed versions.
- Keep all state in the mEditor working folder and avoid writing scan output into source trees unless the user exports a report.

CVSS Repository requirements:

- Use SQLite as the local repository engine.
- Keep the repository under `.meditor/security/vulnerability-intel.sqlite`.
- Keep encrypted offline/sync bundles under `.meditor/security/vulnerability-intel-cache.enc`.
- Provide one-click setup from the `setup` folder to check SQLite, install it where supported, and create the required data structures.
- Provide setup scripts for macOS/Linux and Windows plus the raw schema file under `setup/sqlite`.
- Track sync status in SQLite and expose it in `Tools > CVSS Repository`.
- Use SQLite WAL mode, indexed CVE/package/search fields, transactions, and safe schema migrations.
- Seed the schema with `vulnerability`, `sync_state`, `scan_run`, `finding`, `finding_suppression`, and `rule_pack` tables.
- Auto-update from NVD and CISA KEV where network access is allowed.
- Run background auto-update on startup when stale, daily while idle, and manually through `Update Now`.
- Support encrypted repository export/import for offline machines.
- Show last successful sync, last attempted sync, source status, row counts, stale-cache warnings, and update errors.
- Continue scanning with the last local repository when update feeds are unavailable.

AI/ML Assistant requirements:

- Open from `Tools > AI/ML Assistant` as a same-window chat tab.
- Work with the active editor file, selected code, project tree, build output, test failures, database worksheet text, security findings, and CVSS repository matches.
- Explain code, suggest fixes, generate tests, write secure rewrite previews, interpret compiler errors, propose refactors, and explain database SQL issues.
- Use the local CVSS repository and Code Security Analyzer findings as first-class context.
- Never modify source code directly without showing a patch preview and requiring user approval.
- Keep secrets redacted from prompts, logs, indexes, and training corpora unless the user explicitly opts in for a specific artifact.
- Support local/offline model backends where available and external model providers only when the user configures them.
- Keep model settings, provider credentials, prompt templates, and knowledge-base metadata in encrypted or redacted working-folder storage.

Document, ebook, spreadsheet, and text knowledge ingestion:

- `Tools > AI Knowledge Base` should let the user add PDFs, EPUB books, MOBI books, DOC/DOCX documents, TXT files, XLS/XLSX spreadsheets, CSV/TSV files, Markdown, and documentation bundles to the AI/ML Assistant knowledge base.
- The ingestion flow should extract text, tables, code blocks, headings, page numbers, chapter names, sections, worksheet names, cell/range references, formulas where useful, comments, and document metadata.
- The assistant should chunk extracted content, create embeddings or a local searchable index, and preserve citations back to source file, page, chapter, heading, worksheet, row, column, or cell range.
- Newly ingested material should become available to the assistant immediately through retrieval-based context.
- The user should be able to request controlled retraining or fine-tuning from the ingested corpus when the selected AI backend supports training.
- Retraining should run as a background job with progress, logs, cancel, resume, and rollback to the previous model/index version.
- Every training job should record source document hashes, extraction settings, model/base version, training parameters, output model/index version, and evaluation notes.
- The assistant should validate extracted content quality before training and show low-confidence pages, OCR failures, missing fonts, encrypted/protected documents, scanned-image-only pages, broken encodings, unsupported MOBI variants, hidden spreadsheet sheets, macros, external spreadsheet links, and formula extraction warnings.
- PDF and scanned document ingestion should support OCR as an optional plugin-backed path.
- EPUB ingestion should preserve book title, author, table of contents, chapter, section, and anchor references.
- MOBI ingestion should use a native parser where available or a user-configured converter such as Calibre when the MOBI variant cannot be parsed directly.
- DOC/DOCX ingestion should preserve headings, paragraphs, tables, comments, footnotes, tracked-change state where available, and page/section references when layout information is available.
- TXT ingestion should support encoding detection, fixed-width layout hints, code-block detection, and line-number citations.
- XLS/XLSX ingestion should preserve workbook, worksheet, row, column, cell range, formulas, comments, named ranges, hidden-sheet warnings, and table headers.
- The user should be able to remove any source document from the knowledge base and rebuild the index so obsolete material stops influencing assistant answers.
- When full model retraining is not available, mEditor should still support knowledge-base refresh, retrieval indexing, and response grounding with citations.

AI Training Studio:

- The training studio should let the user train or tune the assistant on curated material for programming languages, compilers, interpreters, data structures, algorithms, operating systems, computer architecture, distributed systems, database design, software architecture, design patterns, UI design, testing, secure coding, DevOps, observability, and performance engineering.
- Training sources should include PDFs, EPUB/MOBI books, DOC/DOCX documents, TXT files, XLS/XLSX spreadsheets, CSV/TSV files, Markdown, source repositories, code examples, design documents, API references, architecture decision records, SQL scripts, diagrams, and exported mEditor knowledge bundles.
- Imported material should be assigned to named domains, such as `Rust`, `C`, `Assembly`, `Compiler Design`, `Computer Architecture`, `Distributed Systems`, `Database Internals`, `Secure Coding`, and `Software Design`.
- Each domain should have a curriculum view with source documents, extracted concepts, examples, glossary terms, code snippets, Q&A pairs, exercises, and evaluation tests.
- The assistant should build domain-specific retrieval indexes immediately and optionally create fine-tuning datasets, adapter-training datasets, or evaluation datasets from validated material.
- The user should be able to start, pause, cancel, resume, compare, activate, deactivate, and roll back training jobs.
- mEditor should distinguish three learning modes: retrieval-only knowledge update, lightweight adapter/fine-tune where supported, and full model retraining where a compatible local or configured backend exists.
- The UI should never imply that a PDF upload magically guarantees expertise. It should show corpus coverage, evaluation score, unresolved extraction warnings, and examples of what the assistant learned.
- Training output should be versioned so a project can pin a model/index version and reproduce assistant behavior later.
- All training datasets should preserve source references so generated architecture or code recommendations can cite the document, page, heading, or source file that influenced the answer.

Spec-to-System Workbench:

- The workbench should accept a user-provided specification document, including PDF, EPUB, MOBI, DOC/DOCX, TXT, XLS/XLSX, CSV/TSV, Markdown, or an imported project requirement bundle.
- It should extract goals, actors, user stories, non-functional requirements, constraints, data models, external interfaces, security requirements, deployment assumptions, and acceptance criteria.
- It should produce a traceability matrix from requirement to architecture component, source file, test, build task, and security control.
- It should propose architecture options with tradeoffs, including module boundaries, data storage, APIs, UI flows, integration points, concurrency model, error handling, logging, testing strategy, deployment model, and operational risks.
- It should generate an implementation backlog with phases, tasks, dependency order, effort estimates, verification steps, and affected mEditor modules.
- It should generate test plans, unit-test suggestions, integration-test suggestions, security-test suggestions, migration checks, performance checks, and documentation tasks.
- It should hand proposed implementation changes to the editor as patch previews, never silent modifications.
- It should use the AI Training Studio domains when reasoning about languages, software design, computer architecture, database design, security, and DevOps decisions.
- It should keep a spec revision history and show which assistant recommendations changed when the uploaded specification changes.
- It should support an `Ask Against Specification` chat mode where every answer is grounded in the current specification, project source, and selected training domains.

## Database Modeling And Compare

DBA Workshop must include database modeling and comparison tools.

Database modeling requirements:

- ER diagrams from selected schemas, tables, views, foreign keys, inferred relationships, and saved diagram layouts.
- DDL generator for tables, views, indexes, constraints, procedures/functions, packages/modules where supported, grants, triggers, sequences/identity objects, and schema-level objects.
- Schema compare between two connections, two schemas, a connection and a saved snapshot, or two snapshots.
- Data compare for selected tables/views with primary-key mapping, row differences, missing rows, changed columns, and exportable compare reports.
- Migration runner for SQL migration folders with applied history, pending migrations, checksum validation, dry run, run, rollback where supported, and report output.
- Explain-plan visualizer with plan tree, cost/cardinality/rows, predicates, join order, warnings, and SQL text handoff from worksheets.

## Keymaps, Settings, Wizards, And Dashboard

mEditor must include user-level productivity surfaces.

Keymaps:

- Built-in keymap profiles for SQL Developer-style, NetBeans-style, Eclipse-style, VS Code-style, IntelliJ-style, Vim-style, Emacs-style, and custom mappings.
- Conflict detection, shortcut search, printable/exportable keymap, and per-workspace overrides.

Project templates and wizards:

- Wizards for Rust, React, Node.js, TypeScript, PHP frameworks, Python, Java/JVM, R, Julia, database project, SQL migration project, CLI tool, web app, service/API, and generic project.
- Wizards should validate required tools and offer `Configure Executable` when a generator is missing.
- Templates should be provided by core modules and plugins.

Settings import/export:

- Export IDE preferences, layout, keymaps, themes, toolchains, extension list, DBA Workshop connections, SSH Terminus connections, and framework templates as encrypted or profile-only bundles.
- Import should preview changes and allow merge, replace, or skip.

Workspace Dashboard:

- Show project health, SDLC last run, Git/SVN state, tests, coverage, dependencies, security findings, database connections, SSH sessions, task chains, recent errors, and pending actions.
- Dashboard widgets should deep-link to the relevant tool window.

## Terminals, Task Chains, Offline Docs, Large Files, And Diff/Merge

Local terminal manager:

- Local terminal tabs grouped by project and profile.
- Shell profiles for zsh, bash, fish, PowerShell, cmd, and custom shells.
- Environment activation for toolchains, virtual environments, SDKs, and project scripts.
- Split panes, search, scrollback, transcript, and restart.

Task-chain scheduler:

- Saved chains such as clean -> build -> test -> package -> deploy.
- Support preflight checks, stop-on-failure, continue-on-warning, environment profiles, manual approval steps, and saved run history.
- Task chains should run locally and remain user-triggered inside the standalone IDE.

Offline documentation browser:

- Index offline docs for languages, compilers, frameworks, SQL dialects, database drivers, and mEditor help.
- Search across installed documentation bundles.
- Link diagnostics, toolchain warnings, and framework wizards to local docs where available.

Large-file and log viewer:

- Streaming open for huge logs, CSV/TSV files, SQL dumps, generated output, and trace files.
- Features: tail mode, regex search, bookmarks, column detection, CSV preview, SQL dump outline, encoding detection, and safe read-only mode.

Visual diff and merge:

- Two-way and three-way file diff/merge.
- Folder compare.
- Git/SVN conflict resolution.
- Whitespace, case, line-ending, and encoding options.
- Handoff from schema compare and data compare to specialized database diff views.

## SDLC Actions And Version Control

mEditor must provide standard SDLC actions for every project type. These actions should be visible in the main toolbar, project context menu, command palette, keyboard shortcuts, and task runner.

Core SDLC actions:

- `Build`: compile or assemble the selected project/module.
- `Clean`: remove generated build output using the project toolchain.
- `Rebuild`: clean followed by build.
- `Test`: run unit, integration, framework, or language-specific tests.
- `Deploy`: run the project deployment profile, such as copying artifacts, invoking a framework deploy command, publishing to an app server, or running a custom deployment script.
- `Launch` / `Run`: start the selected app, script, service, website, SQL script, REPL, or executable.
- `Debug`: start a DAP-backed or toolchain-native debug session.
- `Package`: create distributable artifacts.
- `Publish`: push a package, container, static site, binary, or configured artifact to a target.
- `Stop`: terminate the active launch, deploy, test, or debug process.
- `Rerun`: repeat the most recent action with the same profile.

Task model:

- `SdlcAction`: action id, display name, command, working directory, environment, target module, toolchain, profile, before/after hooks, and output parser.
- `SdlcProfile`: Debug, Release, Test, Production, Local, Remote, and user-defined profiles.
- `SdlcConsole`: structured output, warnings/errors, clickable file links, progress, exit code, elapsed time, and saved logs.
- `ProblemMatcher`: maps compiler/test output into the Problems panel.
- `LaunchProfile`: executable/script/URL, arguments, environment, debugger adapter, browser target, terminal mode, and stop behavior.

Toolchain mappings should include:

- Rust: `cargo build`, `cargo clean`, `cargo test`, `cargo run`, `cargo clippy`, `cargo fmt`, package/release profiles.
- Java/JVM: Maven, Gradle, Ant, javac, JUnit, application server deploy profiles, and GraalVM Native Image builds.
- JavaScript/TypeScript: npm, pnpm, Yarn, Bun, Deno, Vite, Next.js, React scripts, test runners, lint, build, preview, and deploy scripts.
- PHP: Composer scripts, PHPUnit/Pest, framework CLIs, static analysis, package/deploy scripts.
- Python: uv, pip, poetry, hatch, pytest, tox/nox, run/debug profiles.
- C/C++/Objective-C: CMake, Meson, Ninja, Make, clang/gcc, tests, package profiles.
- Database: run SQL script, run worksheet, explain plan, export result, deploy schema migration, rollback migration where supported.
- Generic projects: user-defined shell commands with working directory, environment variables, and problem matchers.

External executable configuration:

- mEditor should auto-detect external executables from `PATH`, common installation folders, language version managers, project-local wrappers, and configured SDK locations.
- If auto-detection fails, the SDLC action should show a clear missing-tool state with a `Configure Executable` action.
- Configuration should be available from `Settings > Languages & Toolchains`, the project properties dialog, the command palette, and the failed task output.
- Users must be able to set executable paths per language and per project, such as `cargo`, `rustc`, `java`, `javac`, `mvn`, `gradle`, `ant`, `node`, `npm`, `pnpm`, `yarn`, `bun`, `deno`, `php`, `composer`, `python`, `pytest`, `gcc`, `clang`, `cmake`, `make`, `git`, `svn`, and deployment CLIs.
- Each configured executable should have a validation command, version parser, minimum-version rule, environment variables, working directory defaults, and optional wrapper script.
- Project-specific executable paths should override global settings without modifying unrelated projects.
- mEditor should support multiple named toolchain profiles per language, such as system, project, container, SDK, legacy, and experimental.
- Toolchain configuration should be stored as JSON metadata in the mEditor working folder, with secrets and credentials delegated to the secure store.

Setup menu language support:

- `Setup > Programming Language Support` must open in the same mEditor window.
- The panel lists every supported language, framework, runtime, database dialect, markup format, and infrastructure format from the language support matrix.
- If required tooling is present on the user's machine, the entry is active.
- If required tooling is missing, the entry offers `Install Tooling` and `Configure Executable`.
- `Install Tooling` uses open-source compiler/runtime/tooling sources, validates that the source URL is reachable, downloads the tool, installs it using the platform strategy, registers the executable path, and refreshes editor language services.
- If the tooling source is not reachable, mEditor shows a same-window network/proxy dialog with direct connectivity test, proxy URL entry, proxy connectivity test, and validated-proxy save action.
- The proxy URL must be validated by checking that mEditor can reach the selected tooling source through that proxy before using it.

Compiler debugging:

- mEditor includes a Compiler Error Debugger that can launch configured build actions, capture compiler errors/warnings, parse diagnostics into the Problems panel, and generate suggestions.
- Every suggestion must show the rationale behind the suggestion, probable impact, auto-apply eligibility, risk, and target physical program/file.
- The user is the master. User-controlled approval is the default for every change.
- Automatic debugging and remediation can be enabled only by an explicit configurable user option after the user gains confidence.
- Every applied change must create a rollback record with before/after snapshots and a restore action.
- The debugger must create an isolated staging area under the mEditor working folder, compile and execute compiled programs there, and avoid touching original sources until the user approves a fix.
- Interpreter-based languages must run through a local terminal runner in the staging area and capture stdout, stderr, and optional screen transcript for debugging.
- Web-based programs must be runnable through an embedded localhost web server and same-window browser, with request logs, server logs, browser console output, and page execution evidence captured for debugging.
- In automatic mode, mEditor may apply permitted low-risk fixes, rerun the build/test/debug cycle, and continue until there are zero errors and zero warnings or the configured iteration limit is reached.
- In user-controlled mode, mEditor presents suggestions and lets the user apply, edit, skip, rollback, or rerun each step.
- After each debugging cycle, the AI/ML service records a learning event so future suggestions can adapt from diagnostics, fixes, and outcomes.
- Each debugging session must generate a handover document for the user, including timelines, physical programs, diagnostics, suggestions, rationale, probable impact, fixes applied/skipped, rollback records, execution output, AI learning events, and final zero-error/warning status.

Project documentation generation:

- On user request, mEditor must generate deployment documents for any implementation stack and deployment target the user specifies.
- PHP REST deployment to Apache HTTP Server or Nginx is a seed example only; the same feature must support any platform-tech stack on the implementation side.
- Deployment documents should include runtime installation, server installation, configuration, build/artifact creation, environment variables, smoke tests, troubleshooting, security hardening, and rollback/recovery.
- When the user requests current source-backed deployment guidance and network access is available, the AI may research official documentation and installer pages, then derive a working document from those sources.
- mEditor must generate PRD, TDD, architecture decision records, runbooks, API references, program references, release notes, user guides, and test plans on request.
- mEditor must generate process flowcharts, program control-flow charts, data-flow charts, deployment flows, and error-handling flows on request.
- Generated flowcharts should use Mermaid as an editable first representation and may be rendered to HTML, PDF, DOCX, or images later.
- mEditor must generate language-agnostic Javadoc-style program documentation for all supported languages, extracting symbols, comments, call relationships, endpoints, modules, files, and source links where possible.
- mEditor must generate ERD documents for any user-selected data structure, including database schemas, SQL DDL, ORM models, classes, structs, JSON/XML schemas, spreadsheets, COBOL copybooks, and user-described structures.
- ERD documents should include entities, fields, data types, keys, constraints, relationships, cardinality, optionality, assumptions, Mermaid ER diagrams, and traceability back to physical sources where possible.

Safe update checking:

- `About mEditor` must provide `Check For Updates`.
- The update source will be a public Git URL provided by the owner later.
- mEditor should compare the installed version/build with the latest available release/build and inform the user when an update is available.
- mEditor must prompt before downloading or installing an update.
- Update installation must preserve local data, setup, connections, SQLite repositories, generated documentation, language/toolchain configuration, and user settings.
- The update flow must create a backup/restore point and offer rollback if validation fails or the user rejects the updated state.

UML modeling:

- `Tools > UML Modeling` must open in the same window.
- Text mode must allow PlantUML, Mermaid, and DOT-style UML text authoring and render the diagram live as the user types.
- GUI mode must provide a palette and canvas for creating UML diagrams visually, including classes, interfaces, actors, use cases, lifelines, messages, states, activities, components, packages, relationships, and notes.
- GUI mode must generate a UML text file for user reference, storage, diff, and version control.
- UML models should be storable under the mEditor working folder and embeddable into generated project documentation.

Local user-program bug repository and reports:

- mEditor must include a local bug repository for bugs in the user's own programs.
- This local bug repository is separate from the APEX feedback path. APEX feedback is only for mEditor product feedback and bugs.
- User-program bug records must never be sent anywhere automatically.
- Users can manually enter bugs to work on.
- During compiler/debugging cycles, mEditor should create local bug entries with physical program, diagnostics, reproduction details, AI suggestion summary, applied fix summary, rollback reference, timestamps, and status.
- The repository must support export and import. Export creates a user-controlled archive that the user may email or transfer by any channel they choose.
- A target user can import a bug archive into their mEditor instance and generate a consolidated report from local and imported bugs.
- `Report` must be a main menu. It publishes debugging reports, local bug reports, consolidated bug reports, work summaries, deployment reports, documentation reports, and security reports.

Inbuilt professional editor capabilities:

- These are inbuilt editor capabilities, not optional wishlist items: perspectives/workspaces, command palette, refactoring/navigation, visual designers/modelers, profiler/diagnostics, remote/terminal quality, task automation, templates/snippets, and migration/keymap compatibility.
- `Source > Refactor` and `Source > Reformat` must provide NetBeans-style source operations with preview, formatter profile support, user approval, and rollback where changes are made.
- FileZilla-style `Tools > SFTP/SCP Transfer` must support site profiles, transfer queue, resume, bookmarks, filters, and directory compare for SFTP/SCP workflows.

Project workspace and Project Planner:

- mEditor must create projects in the editor like NetBeans or JDeveloper.
- A workspace can contain one major project with sub-projects inside it.
- `File > New Project` creates editor projects and sub-projects.
- `Tools > Project Planner` opens portfolio/project planning for the active editor project.
- Project metadata should be inspired by NetBeans-style project metadata, with an mEditor metadata folder containing project, module, planning, private-user, and import-report files.
- The metadata design must support editor project needs and portfolio/project management needs.
- Project Planner must support portfolio, program, project, deliverable, milestone, epic, story, task, subtask, risk, issue, and dependency records with owners, timelines, status, and progress.
- Planning items should link to physical source files, local bugs, debug reports, deployment docs, tests, and generated documentation.
- Based on user progress, AI may update planning progress in the background, but those updates must be reviewable by the user.
- When a user creates a portfolio or project in Project Planner, the planner must offer to create the corresponding project in the editor after collecting the tech stack and required functionality from the user.
- There are two ways to create an editor project: directly from the editor's new project wizard, or from Project Planner after portfolio/project planning context is created.
- There are two ways to create a project plan: manually through the Project Planner UI, or by exporting an Excel template, filling it, uploading it, and letting Project Planner consume it.
- Project Planner must ask which planning model to follow: `Objective -> Milestone -> Task` or Scrum `Epic -> Story -> Task`.
- Project Planner must present its UI and tracking hierarchy according to the selected model.
- Manual planning UI must support subtasks, assignees, ETA, timeline, status, and progress tracking.
- Project Planner must always ask the user for timezone before calculating, importing, or displaying project timelines, and must adjust timelines accordingly.
- Project Planner must track project cost, including planned cost, actual cost, forecast cost, and cost health.
- Project Planner must provide a dashboard with Gantt chart, progress line charts, progress bar charts, budget/actual/forecast cost, milestone status, burndown, and yellow/red risk indicators.

Example toolchain metadata:

```json
{
  "schemaVersion": 1,
  "language": "java",
  "profiles": [
    {
      "name": "project-jdk",
      "executables": {
        "java": "/Library/Java/JavaVirtualMachines/jdk-25.jdk/Contents/Home/bin/java",
        "javac": "/Library/Java/JavaVirtualMachines/jdk-25.jdk/Contents/Home/bin/javac",
        "mvn": "/opt/homebrew/bin/mvn"
      },
      "environment": {
        "JAVA_HOME": "/Library/Java/JavaVirtualMachines/jdk-25.jdk/Contents/Home"
      }
    }
  ]
}
```

Git support:

- Repository detection, status, diff, stage/unstage, commit, amend, branch, checkout/switch, tag, stash, merge, rebase, cherry-pick, fetch, pull, push, remote management, blame, history graph, submodules, and conflict resolution.
- File gutter decorations for modified, added, deleted, renamed, conflicted, and ignored files.
- Commit UI with staged/unstaged panes, diff preview, message template, sign-off option, and pre-commit hook output.
- Safe revert and discard workflows that show exactly what will change before touching files.

SVN support:

- Working copy detection, checkout, update, commit, diff, revert, log, annotate, branch/tag copy, switch, merge, resolve, cleanup, lock, unlock, add, delete, move, and status.
- File gutter decorations for modified, added, deleted, replaced, conflicted, locked, and external files.
- Commit UI with changelist support, diff preview, message history, and conflict warnings.
- Repository browser for SVN URLs where credentials are available.

Version-control safety:

- Never discard local changes without explicit confirmation.
- Show uncommitted changes before deploy, launch, or publish when the profile requires a clean working copy.
- Keep credentials in the secure store, not in project metadata.
- Allow projects to disable VCS integration when the workspace is generated, temporary, or read-only.

## Encrypted Connection Storage And Export

mEditor should keep connection information in encrypted JSON format inside the current mEditor working folder.

Working folder layout:

```text
.meditor/
  connections/
    dba-workshop-connections.json.enc
    ssh-terminus-connections.json.enc
    jdbc-driver-registry.json.enc
  exports/
    README.md
```

Storage rules:

- The decrypted payload format is JSON.
- The file stored on disk is encrypted, authenticated, and versioned.
- The encryption envelope should include schema version, created timestamp, updated timestamp, cipher name, KDF name, salt, nonce, payload checksum, and encrypted payload.
- Use a modern authenticated encryption mode such as AES-256-GCM or XChaCha20-Poly1305.
- Derive file keys with Argon2id or use the OS keychain to wrap a randomly generated workspace key.
- Never write plaintext passwords, private key passphrases, database passwords, or SSH secrets to disk.
- Support schema migration so older exported connection files can be imported safely.
- Store DBA Workshop and SSH Terminus connection files separately so each tool can be exported and imported independently.

DBA Workshop import/export:

- `DBA Workshop > Connections > Export Connections` exports database connection profiles, folders, color labels, JDBC driver references or bundled driver definitions, connection URL templates, dialect choices, and optional encrypted secrets.
- `DBA Workshop > Connections > Import Connections` imports an encrypted DBA Workshop export file.
- Import modes: preview, merge, replace, skip duplicate, rename duplicate, and dry run.
- Export options: include secrets with a user-supplied export passphrase, or export profiles without secrets.
- Imported JDBC driver JARs should be copied into the local driver cache only after user confirmation.
- Import should validate driver availability, connection profile schema, encryption envelope, and conflicts before writing to the active connection store.

SSH Terminus import/export:

- `SSH Terminus > Connections > Export Connections` exports SSH groups, SSH profiles, group colors, tags, terminal profile preferences, jump-host settings, port-forward definitions, startup commands, and optional encrypted secrets.
- `SSH Terminus > Connections > Import Connections` imports an encrypted SSH Terminus export file.
- Import modes: preview, merge, replace, skip duplicate, rename duplicate, and dry run.
- Export options: include secrets with a user-supplied export passphrase, or export profiles without secrets.
- Imported key paths should be checked for existence and marked unresolved if the key file is not present on the target machine.
- Import should never overwrite existing groups or profiles without a visible conflict decision.

Export file meaning:

- In this design, "backup" means an encrypted export file created by DBA Workshop or SSH Terminus.
- DBA Workshop exports should be importable by DBA Workshop.
- SSH Terminus exports should be importable by SSH Terminus.
- Cross-import should be rejected with a clear message unless a future migration tool explicitly supports it.

## DBA Workshop DBA Panel

DBA Workshop must include a **DBA Panel** that can be activated per database connection after mEditor verifies the connection has sufficient administrative or monitoring privilege. The DBA Panel belongs to DBA Workshop and should open inside the DBA Workshop UI model, sharing its connection tree, worksheet tabs, object browser, result grids, and driver manager.

Activation rules:

- The panel is hidden or disabled for ordinary application-user connections.
- The panel can be activated when the connection is authenticated with `SYSDBA` or a database-specific equivalent.
- Equivalence must be capability-based, not name-only. mEditor should run a non-destructive privilege probe and record which dashboard sections are available.
- If the user has partial monitoring privileges, the panel should open in partial mode and mark unavailable sections clearly.
- If a dashboard query requires elevated access, mEditor should show the missing privilege and keep the rest of the panel usable.

Privilege examples to map through dialect adapters:

- `SYSDBA` or equivalent administrative database role where supported.
- SQL Server-style `sysadmin`, `serveradmin`, or `VIEW SERVER STATE`/`VIEW DATABASE STATE` combinations.
- PostgreSQL-style superuser, `pg_monitor`, and catalog/statistics access.
- MySQL/MariaDB-style `PROCESS`, `REPLICATION CLIENT`, backup-related privileges, and performance schema access.
- DB2-style SYSADM/SYSCTRL/SYSMAINT/SYSMON or equivalent monitoring authority.

Real-time dashboard behavior:

- Render directly from the active database connection.
- Refresh on a user-configurable cadence, with pause/resume and manual refresh.
- Run dashboard queries as cancellable background tasks so the editor and worksheet stay responsive.
- Show last refresh time, query duration, stale data warnings, and failed section details.
- Cache only lightweight summary data locally unless the user exports a report.
- Never run destructive statements from dashboard refresh paths.

Dashboard sections should include:

- Instance/database identity, version, uptime, role, open/read-only state, and connection status.
- Sessions, active SQL, blockers/waiters, locks, wait classes/events, long-running operations, and top consumers where available.
- Storage, data files, tablespaces/filegroups, temp usage, log usage, growth, and free space.
- Memory, process/thread counts, background jobs, scheduler jobs, invalid objects, alert/error summary, and key configuration parameters.
- High availability and replication status, including primary/standby, cluster/node, failover, lag, and service availability where the engine exposes it.
- Patch history and component/version inventory where the engine exposes it.
- RMAN backup history for database engines that support RMAN metadata, plus engine-specific backup/restore history for other databases.
- Forecast-ready time series for storage, session pressure, waits, backup age, and availability.

Patch and backup history adapters:

- For RMAN-capable connections, show backup sets, backup pieces, backup type, start/end time, elapsed time, status, device type, compression/encryption where available, retention/recovery window indicators, and most recent successful full/incremental/archive-log backup.
- For engines without RMAN, use the database-specific backup catalog, system tables, management views, or plugin queries supplied by the dialect adapter.
- Patch history should show applied patch/component inventory, patch date, status, component name, version/build, and source view/query when available.
- When patch or backup history is not exposed through the connection, the panel should show `Not available through this driver/privilege set` rather than pretending the data is absent.

SQL Developer-style command behavior:

- Every connection should support SQL worksheet tabs with run statement, run script, explain plan where supported, commit, rollback, autocommit toggle, bind variables, SQL history, and cancellable execution.
- A DBA Panel connection should be able to open a worksheet in the same connection context so the user can fire SQL commands like SQL Developer.
- Result grids should support multiple result sets, paging, sorting, filtering, copy/export, execution time, row count, errors, and warnings.
- Dangerous commands from privileged connections should show a confirmation or require an explicit unsafe-command preference.

## Embedded Database Workbench Port

The user identified the DBA workshop material as part of the DevOps desktop workbench. In the local sibling project, the relevant source area is:

- `/Users/sanjpand/Downloads/MyProjects/DevOps Control Panel and Workbench/api/openapi.yaml`
- `/Users/sanjpand/Downloads/MyProjects/DevOps Control Panel and Workbench/backend/src/main/java/com/jdedevops/dcpw/database/`
- `/Users/sanjpand/Downloads/MyProjects/DevOps Control Panel and Workbench/backend/src/main/java/com/jdedevops/dcpw/mssql/`
- `/Users/sanjpand/Downloads/MyProjects/DevOps Control Panel and Workbench/backend/src/main/java/com/jdedevops/dcpw/db2/`
- `/Users/sanjpand/Downloads/MyProjects/DevOps Control Panel and Workbench/backend/src/main/java/com/jdedevops/dcpw/db400/`

Rust conversion target:

- `ConnectionProfile`: connection name, driver type, host, port, service/database, user, TLS options, tunnel options, tags, and metadata.
- `CredentialVault`: encrypted secrets using OS keychain integration where available and file-backed encryption for portable mode.
- `ProbeRunner`: async probe execution with timeouts, cancellation, evidence capture, query hashing, and warning collection.
- `WorksheetSession`: SQL text, bind variables, execution options, autocommit mode, result sets, statement timing, and history.
- `ObjectExplorer`: schemas, databases, tables, views, packages, procedures, indexes, constraints, partitions, grants, synonyms, jobs, and sessions.
- `ResultGrid`: paging, sorting, filtering, copy/export, data type rendering, large result streaming, and safe edit mode.
- `DatabaseDashboard`: status cards, incidents, storage, sessions, high availability, capacity forecasts, and trend charts.
- `DbaPanel`: privilege-gated real-time dashboard, patch history, backup history, RMAN history where supported, and worksheet handoff.
- `PrivilegeProbe`: non-destructive capability detection for administrative and monitoring privileges per database dialect.
- `ApprovalGate`: preview, impact summary, approver note, audit hash, execution guard, and rollback/runbook output.

The port should translate Java resource/repository/probe classes into Rust service traits:

```rust
pub trait DatabaseConnector {
    fn driver_id(&self) -> &'static str;
    async fn test(&self, profile: &ConnectionProfile) -> anyhow::Result<ConnectionTest>;
    async fn probe(&self, profile: &ConnectionProfile, scope: ProbeScope) -> anyhow::Result<ProbeReport>;
    async fn objects(&self, profile: &ConnectionProfile, filter: ObjectFilter) -> anyhow::Result<Vec<DatabaseObject>>;
    async fn execute(&self, session: &WorksheetSession, request: SqlExecutionRequest) -> anyhow::Result<SqlExecutionResult>;
}
```

## JDBC Driver Manager

DBA Workshop must support adding JDBC drivers to expand database support after installation. This should behave like a SQL Developer-style driver manager.

Required UI:

- `Tools > DBA Workshop > Drivers` or `DBA Workshop > Preferences > Database Drivers`.
- Add, remove, enable, disable, duplicate, and reorder JDBC driver definitions.
- Add one or more `.jar` files per driver definition.
- Scan JAR manifests and class contents to suggest driver class names.
- Manual driver class entry when auto-detection is incomplete.
- JDBC URL template editor with named placeholders such as `{host}`, `{port}`, `{database}`, `{service}`, `{schema}`, and `{user}`.
- Connection form generator based on the URL template.
- Test Driver button that loads the class, checks metadata support, and reports driver version.
- Test Connection button that validates URL, credentials, TLS properties, and metadata discovery.
- Dialect selection with a generic SQL fallback.
- Import/export driver definitions as local JSON or TOML without exporting passwords.

Runtime model:

- `JdbcDriverDefinition`: id, name, vendor label, jar paths, driver class, URL templates, default port, dialect id, validation query, connection properties, and enabled state.
- `JdbcDriverRegistry`: local catalog of bundled and user-added JDBC drivers.
- `JdbcClassLoader`: isolated classpath per driver definition so conflicting driver JARs do not contaminate the IDE runtime.
- `JdbcConnectionTemplate`: field definitions, URL rendering, masking rules, TLS options, and advanced properties.
- `JdbcMetadataAdapter`: generic `DatabaseMetaData` browsing for catalogs, schemas, tables, views, columns, indexes, procedures, primary keys, imported/exported keys, and type information.
- `DialectAdapter`: optional SQL formatter rules, limit/offset syntax, explain-plan command, identifier quoting, bind syntax, and object browser query overrides.

Support behavior:

- If a database has a built-in Rust connector, mEditor can use it directly.
- If a database is added through JDBC, mEditor should run the JDBC bridge in an isolated helper process and communicate with the Rust workbench over a local IPC protocol.
- If dialect-specific queries are not available, the object browser should use JDBC metadata APIs and mark the connection as `Generic JDBC`.
- Driver JARs should be copied into a local mEditor driver cache or referenced in place according to user choice.
- Credentials belong to connection profiles, not driver definitions.

SQL Developer-style database UI expectations:

- Connections tree with folders, color labels, filters, reconnect, clone, export, import, and test actions.
- Connection dialog with Basic, Advanced, SSH/Tunnel, TLS, Driver, and Test tabs.
- Worksheet per connection with run statement, run script, explain plan where supported, commit/rollback, SQL history, bind variables, and query timing.
- Object browser with context actions for open, describe, data, DDL, grants, dependencies, indexes, constraints, triggers, procedures/functions, packages/modules where supported, and refresh.
- Result grid with paging, filtering, sorting, copy, export, explain/errors panel, and cancellable execution.
- Driver manager and connection manager should be reachable without leaving the DBA Workshop.

## SSH Terminus

SSH Terminus is a standalone SSH terminal manager inside mEditor. It opens user-requested SSH sessions in terminal tabs and does not require any managed runtime on the target host.

Required UI behavior:

- Connection groups in the left navigator, such as Development, Test, Production, Customer Sites, Lab, and Personal.
- Saved SSH profiles under each group with name, host, port, username, auth method, key path reference, color label, tags, and notes.
- Multiple SSH connections opened as tabbed terminals in the central work area.
- `Open Group` to launch every enabled profile in a group as terminal tabs.
- `Close Group` to close all tabs belonging to a group.
- `Reconnect Tab`, `Reconnect Group`, duplicate tab, rename tab, pin tab, and move tab to another group.
- Optional split terminal panes for side-by-side SSH sessions.
- Search across open tabs by host, group, tab title, or current terminal text.
- Group-level command runner with preview and confirmation before sending a command to more than one session.
- Optional broadcast input mode that is visibly marked and easy to turn off.
- Session history that records profile name, host key fingerprint, start/end time, exit reason, and user note.

Rust model:

- `SshGroup`: group id, name, color, sort order, tags, default terminal profile, and saved profile ids.
- `SshProfile`: profile id, group id, display name, host, port, user, authentication type, key path reference, known-hosts policy, jump host, proxy command, startup command, tags, and notes.
- `SshCredentialPolicy`: session-only password by default, OS keychain references for saved secrets, key passphrase prompts, and no plaintext persistence.
- `SshSession`: connect, reconnect, keepalive, terminal channel, command channel, close, and session events.
- `TerminalTab`: tab id, session id, profile id, group id, title, pin state, split state, scrollback, search state, and transcript setting.
- `GroupCommand`: command text, target profile ids, confirmation state, stdout/stderr stream, per-host exit status, and result summary.
- `PortForward`: local, remote, and dynamic forwarding with visible status and stop controls.
- `SessionHistory`: profile, group, host key fingerprint, connection time, disconnect time, command hashes when command runner is used, and result.

Recommended Rust implementation options:

- `openssh` for Unix-like systems where the local OpenSSH client and user SSH config should be reused.
- `ssh2` for a libssh2-backed cross-platform SSH client.
- `russh` as a pure-Rust SSH option where deeper protocol control is required.

## Project Importers

mEditor should import existing IDE projects without flattening their structure.

NetBeans importer:

- Detect `nbproject/project.xml`, `nbproject/project.properties`, `build.xml`, Maven `pom.xml`, Gradle files, source roots, test roots, libraries, run configurations, and Java platform hints.
- Convert into mEditor workspace tasks for build, clean, run, test, debug, package, and open shell.
- Preserve Ant projects as Ant projects; preserve Maven/Gradle projects as Maven/Gradle projects.

Eclipse importer:

- Detect `.project`, `.classpath`, `.settings/`, `.launch`, Maven/Gradle files, WTP metadata, source folders, output folders, linked resources, builders, facets, and server/runtime hints.
- Convert classpath containers and user libraries into mEditor dependency references with unresolved entries clearly marked.
- Import launch configurations into run/debug profiles where possible.

JDeveloper importer:

- Detect `.jws`, `.jpr`, `.jpx`, Ant/Maven/Gradle files, source roots, libraries, deployment descriptors, application/module grouping, data source references, and server profile hints.
- Convert application and project files into an mEditor workspace with modules, Java/JVM tasks, web deployment descriptors, SQL/database resources, and migration notes for unresolved proprietary settings.

Visual Studio importer:

- Detect `.sln`, `.csproj`, `.vbproj`, `.fsproj`, `.vcxproj`, `.vcproj`, `.sqlproj`, `.dbproj`, `.shproj`, `.esproj`, `.njsproj`, `.wixproj`, `.props`, `.targets`, and `.filters` files.
- Convert solution projects into an mEditor major project with sub-projects.
- Map MSBuild configurations, platforms, properties, imports, references, package references, source roots, generated output folders, build profiles, and debug profiles.
- Preserve C++ `.filters` as virtual folders without moving physical source files.
- Mark unresolved SDKs, missing Visual Studio workloads, missing NuGet restore state, and unsupported project settings in the import report.

Importer rules:

- Never rewrite the original project during import unless the user explicitly chooses an in-place migration.
- Create `.meditor/workspace.toml` and `.meditor/import-report.md` beside the workspace or in a user-selected metadata folder.
- Record every detected source root, dependency, task, warning, unsupported setting, and manual follow-up.
- Offer `Import As-Is`, `Import With Local Metadata`, and `Convert To mEditor Workspace` modes.
- Never require Visual Studio itself to be installed for basic metadata import; if a build/debug action needs MSBuild, .NET SDK, C++ Build Tools, NuGet, or another executable, route it through the standard mEditor executable detection/configuration flow.

## Language Support Matrix

Core language families to register:

- Assembly: NASM, GAS, LLVM integrated assembler, FASM where available.
- C, C++, Objective-C, Objective-C++: GCC, Clang/LLVM, CMake, Meson, Ninja, Make, clangd, ccls.
- Ada: GNAT/GCC Ada, Ada Language Server, gprbuild.
- Fortran: GNU Fortran, LLVM Flang where available, fortls.
- COBOL: GnuCOBOL and GCC COBOL front end where available.
- BASIC: FreeBASIC and QB64 Phoenix Edition.
- Clipper/xBase: Harbour and xHarbour.
- Rust: rustc, cargo, rustfmt, clippy, rust-analyzer, cargo-nextest, cargo-audit, cargo-deny.
- Python: CPython, PyPy, Ruff, Pyright, pylsp, mypy, uv, pip, poetry, hatch, conda.
- PHP: php-src/Zend Engine, OPcache/JIT, Composer, PHPStan, Psalm, PHPCS, Rector, Xdebug, PeachPie, and KPHP as optional compiler paths.
- Perl: perl, perltidy, perlcritic, Perl::LanguageServer.
- Java/JVM: OpenJDK, javac, Maven, Gradle, JDT LS, Kotlin compiler, Scala compiler, Groovy compiler, Clojure CLI, GraalVM Native Image.
- Julia: Julia runtime/compiler, LanguageServer.jl, Pkg.
- R: R, radian, languageserver, lintr, styler, renv.
- Prolog: SWI-Prolog and GNU Prolog.
- Lisp family: SBCL, CLISP, ECL, CCL, Racket, Guile, Chez Scheme, Clojure.
- SQL family: SQL, PL/SQL, T-SQL, PL/pgSQL, SQL PL, SQLite SQL, MySQL/MariaDB SQL, BigQuery SQL, Snowflake SQL, and extensible dialect definitions.
- Web and markup: HTML, CSS, SCSS, Sass, Less, PostCSS, XML, JSON, YAML, TOML, Markdown, MDX.
- JavaScript family: JavaScript, TypeScript, JSX, TSX, Flow, CoffeeScript, Node.js, Deno, Bun, npm, pnpm, Yarn, ESLint, Biome, Prettier, TypeScript server, Babel, SWC, esbuild.
- Frontend frameworks: React, Preact, Vue, Svelte, Angular, Solid, Qwik, Lit, Astro, Next.js, Remix, Vite, Nuxt, SvelteKit, Electron, Tauri frontends, React Native, Expo.
- Infrastructure: shell, PowerShell, Dockerfile, Compose, Terraform HCL, Kubernetes YAML, Helm, Ansible, Nix, GitHub Actions, GitLab CI, Jenkinsfile.

## Open-Source Compiler And Runtime Catalog

The first compiler catalog should include:

| Language family | Open-source compiler/interpreter/runtime | mEditor action |
|---|---|---|
| Assembly | NASM, GNU assembler, LLVM assembler, FASM | Detect, assemble, link, debug |
| C/C++/Objective-C | GCC, Clang/LLVM | Detect, build, format, analyze |
| Ada | GNAT in GCC | Detect, build, run tests |
| Fortran | GNU Fortran, LLVM Flang | Detect, build, run |
| COBOL | GnuCOBOL, GCC COBOL where available | Detect, compile, run |
| BASIC | FreeBASIC, QB64 Phoenix Edition | Detect, compile, run |
| Clipper/xBase | Harbour, xHarbour | Detect, compile, run |
| Rust | rustc and cargo | First-class workspace support |
| Python | CPython, PyPy | Run, test, package, virtual env |
| PHP | php-src/Zend Engine, OPcache/JIT, PeachPie, KPHP | Run, test, compile where compatible |
| Perl | perl | Run, test, lint |
| Java/JVM | OpenJDK, javac, Kotlin, Scala, Groovy, Clojure, GraalVM Native Image | Build, test, package, native-image |
| Julia | Julia | Run, test, package |
| R | R | Run scripts, notebooks, package checks |
| Prolog | SWI-Prolog, GNU Prolog | Run queries, compile where supported |
| Lisp/Scheme | SBCL, CLISP, ECL, CCL, Racket, Guile, Chez Scheme | REPL, compile, run |
| SQL | SQLite, PostgreSQL, MySQL/MariaDB, SQL Server, DB2, DB/400, DuckDB, ClickHouse, ODBC/JDBC bridges | Connect, browse, execute, export |

## Rust Framework Coverage

mEditor should include a Rust framework catalog, not a hardcoded finite list. The seed catalog should cover:

- Desktop/UI: Tauri, Slint, egui/eframe, Iced, Dioxus Desktop, Relm4/GTK, Makepad, GPUI, Floem, Xilem, Freya.
- Web frontend/full-stack: Leptos, Yew, Dioxus, Sycamore, Seed, Perseus.
- Web backend: Axum, Actix Web, Rocket, Poem, Salvo, Warp, Tide, Loco.
- Async/runtime: Tokio, async-std, smol.
- Data/ORM: SQLx, Diesel, SeaORM, rbatis, SurrealDB client, Polars, DataFusion.
- CLI/TUI: clap, argh, pico-args, ratatui, crossterm, inquire.
- Game/visual: Bevy, Fyrox, macroquad, ggez, nannou.
- Embedded/WASM: Embassy, RTIC, wasm-bindgen, wasm-pack, Trunk.
- Testing/quality: nextest, insta, proptest, criterion, cargo-audit, cargo-deny, cargo-vet.

The catalog should store framework name, category, package/crate, template commands, official docs URL, minimum Rust version, example commands, and supported project actions.

## PHP Framework Download Manifest

The framework cache should live under `third_party/php-frameworks/`. Initial open-source repositories to cache:

```bash
git clone --depth 1 https://github.com/laravel/framework.git third_party/php-frameworks/laravel-framework
git clone --depth 1 https://github.com/symfony/symfony.git third_party/php-frameworks/symfony
git clone --depth 1 https://github.com/codeigniter4/CodeIgniter4.git third_party/php-frameworks/codeigniter4
git clone --depth 1 https://github.com/cakephp/cakephp.git third_party/php-frameworks/cakephp
git clone --depth 1 https://github.com/yiisoft/yii2.git third_party/php-frameworks/yii2
git clone --depth 1 https://github.com/slimphp/Slim.git third_party/php-frameworks/slim
git clone --depth 1 https://github.com/laminas/laminas-mvc.git third_party/php-frameworks/laminas-mvc
git clone --depth 1 https://github.com/mezzio/mezzio.git third_party/php-frameworks/mezzio
git clone --depth 1 https://github.com/phalcon/cphalcon.git third_party/php-frameworks/phalcon
git clone --depth 1 https://github.com/spiral/framework.git third_party/php-frameworks/spiral
git clone --depth 1 https://github.com/nette/application.git third_party/php-frameworks/nette-application
```

PHP compiler position:

- The canonical open-source PHP implementation is `php-src`, which compiles PHP source into opcodes for the Zend VM and supports OPcache/JIT in modern PHP.
- PeachPie is an open-source PHP compiler/runtime for .NET and is useful for compatible PHP projects that need .NET packaging.
- KPHP translates supported PHP code to C++ and then compiles it, but it is not a drop-in replacement for every PHP application or framework.
- For mEditor, the PHP workflow should default to standard PHP plus Composer, with optional compatibility checks before enabling PeachPie or KPHP compilation.

## AI Knowledge Source Format Catalog

The AI Knowledge Base should register each supported source format with an adapter, extraction strategy, provenance model, and optional external executable configuration.

Initial adapters:

| Format | Extraction target | Provenance |
|---|---|---|
| PDF | Text, tables, images needing OCR, headings, code blocks | File, page, heading, bounding region where available |
| EPUB | XHTML chapters, table of contents, metadata, code/text blocks | File, book, chapter, section, anchor |
| MOBI | Ebook text and metadata through native parser or configured converter | File, book, section, converted source map where available |
| DOC/DOCX | Paragraphs, headings, tables, comments, footnotes, tracked-change state where available | File, section, heading, paragraph, table |
| TXT | Plain text, fixed-width tables, code blocks, logs | File, line range, detected encoding |
| XLS/XLSX | Worksheets, tables, cells, formulas, comments, named ranges | File, workbook, sheet, row, column, cell/range |
| CSV/TSV | Tables, headers, rows, inferred data types | File, row, column |
| Markdown | Headings, prose, tables, code blocks, links | File, heading, line range |

If an adapter needs an external executable, such as OCR or ebook conversion, mEditor should auto-detect it and then offer `Configure Executable` if it cannot be found.

## Local Toolchain Snapshot

Checked on 2026-05-18 in the current machine:

- PHP found at `/opt/homebrew/bin/php`, version `PHP 8.5.4`.
- Node.js found at `/opt/homebrew/bin/node`, version `v25.6.1`.
- npm found at `/opt/homebrew/bin/npm`, version `11.10.0`.
- Composer was not found on `PATH`.
- SWI-Prolog, GNU Prolog, SBCL, and CLISP were not found on `PATH`.
- OPcache is present in the PHP runtime, but CLI OPcache/JIT is disabled in the current CLI configuration.

## Source References

- GCC: https://www.gnu.org/software/gcc/
- Clang/LLVM: https://clang.llvm.org/docs/CommandGuide/clang.html
- GnuCOBOL: https://gnucobol.sourceforge.io/guides.html
- FreeBASIC: https://www.freebasic.net/
- NASM: https://www.nasm.us/
- Harbour: https://harbour.github.io/about
- xHarbour: https://www.xharbour.org/
- PHP source: https://github.com/php/php-src
- PHP OPcache/JIT configuration: https://www.php.net/manual/en/opcache.configuration.php
- PeachPie: https://www.peachpie.io/
- KPHP: https://vkcom.github.io/kphp/
- GraalVM Native Image: https://www.graalvm.org/latest/reference-manual/native-image/
- openssh crate: https://docs.rs/openssh
- ssh2 crate: https://docs.rs/ssh2
- russh: https://github.com/Eugeny/russh
- SWI-Prolog: https://www.swi-prolog.org/pldoc/doc/_SWI_/index.html
- GNU Prolog: https://www.gprolog.org/
- SBCL: https://sbcl.sourceforge.io/
- Tauri: https://tauri.app/start/
- Dioxus: https://dioxus.dev/
- Axum: https://docs.rs/axum/latest/axum/
- Actix Web: https://actix.rs/docs/
- Rocket: https://rocket.rs/
- Laravel: https://github.com/laravel/framework
- Symfony: https://symfony.com/
- CodeIgniter 4: https://github.com/codeigniter4/CodeIgniter4

## Phased Delivery

1. Build the cross-platform Rust desktop shell, platform adapter layer, command palette, File Explorer, editor tabs, embedded Web Browser, terminal, preferences, and toolchain registry.
2. Add Tree-Sitter parsing, LSP lifecycle, diagnostics, formatting, snippets, and workspace search.
3. Add SDLC actions, compiler/interpreter discovery, executable configuration, local terminals, task chains, and profiles for the core language matrix.
4. Add Git/SVN version control, visual diff/merge, conflict resolution, and repository history views.
5. Port the DevOps desktop database workbench behavior into Rust connectors and SQL Developer-style UI panels.
6. Add DBA Workshop JDBC driver manager, DBA Panel, ER diagrams, schema compare, data compare, DDL generator, migration runner, and explain-plan visualizer.
7. Implement SSH Terminus as a standalone grouped, tabbed SSH terminal manager and expose `Tools > DBA Workshop` plus `Tools > SSH Terminus`.
8. Add NetBeans, Eclipse, JDeveloper, and Visual Studio importers with import reports and mEditor workspace metadata.
9. Add framework wizards and project templates for Rust, JavaScript/TypeScript, PHP, Python, Java/JVM, R, Julia, database projects, CLI tools, and web apps.
10. Add DAP debugging, Test Explorer, coverage reports, package/dependency manager, dependency/security scanner, embedded Code Security Analyzer, CVSS Repository with SQLite auto-update, SBOM, and result exports.
11. Add AI/ML Assistant, PDF knowledge ingestion, retraining/fine-tuning job control, secure context retrieval, citation-backed answers, and approval-gated code edit previews.
12. Add plugin system, extension manager, keymaps, settings import/export, workspace dashboard, large-file/log viewer, and offline documentation browser.
13. Add interim generic-JAR packaging with platform-specific launchers, then native packaging for Windows, macOS Intel, macOS Apple Silicon including M4, and Linux; add GraalVM Native Image profiles, signed release artifacts, and offline plugin/documentation bundle support.
