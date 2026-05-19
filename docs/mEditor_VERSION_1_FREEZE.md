# mEditor Ver 1 Freeze

## Freeze Identity

- **Product name:** mEditor
- **Expansion:** Multi Editor
- **Frozen version:** 1.5.0.0-frozen
- **Freeze date:** 2026-05-18
- **Versioning scheme:** Major release.Minor release.Bugfix or enhancement.Build
- **Minor release rule:** Each feature imported from another app increments the Minor release component.
- **Build number rule:** Every produced build must increment the Build component.
- **Owner:** Sanjay Pandey <s.pandey.india@gmail.com>
- **Status:** Frozen Ver 1 design baseline.

## Frozen Scope

Ver 1 freezes mEditor as a cross-platform standalone IDE and workbench with these first-class capability groups:

- Core IDE shell, command palette, same-window tab model, optional detached windows, workspace dashboard, and platform adapter layer.
- File Explorer with context-menu `Open in Editor Tab`, open-to-side, open-with, preview, compare, file history, security scan, and large-file routing.
- Embedded full Web Browser opened from `Tools > Web Browser` in the same main window.
- Multi-language editor with LSP, DAP, Tree-Sitter, syntax highlighting, diagnostics, snippets, formatting, refactoring hooks, and large-file mode.
- Setup menu includes `Programming Language Support`, showing every supported language/framework, active tooling detected on the user's machine, missing tooling, install actions, and network/proxy validation for tooling downloads.
- SDLC actions: build, clean, rebuild, test, deploy, launch/run, debug, package, publish, stop, rerun, task chains, and configurable external executables.
- Git and SVN version-control workflows with diff, merge, history, blame/annotate, conflict resolution, and safety checks.
- DBA Workshop with SQL Developer-style connections, worksheets, object browser, result grids, JDBC driver manager, DBA Panel, dashboards, patch history, backup/RMAN history, ER diagrams, schema compare, data compare, DDL generation, migrations, and explain-plan visualization.
- SSH Terminus with grouped SSH profiles, multiple tabbed SSH terminals, group launch/close, reconnect, port forwarding, session history, and safe credential handling.
- Code Security Analyzer with editor right-click scan, source findings, CVSS repository, SQLite vulnerability database, auto-update, dependency correlation, rule packs, remediation, and reports.
- Compiler Error Debugger with staged builds, compiler-output capture, rationale-backed suggestions, user-controlled or explicitly enabled automatic remediation cycles, rollback records for every change, local terminal runs for interpreter languages, embedded localhost web-server debugging for web apps, execution capture, and user handover reports.
- One-click `setup` flow to install SQLite where supported and create the CVSS Repository data structures.
- AI/ML Assistant, AI Knowledge Base, AI Training Studio, and Spec-to-System Workbench for coding help, secure remediation, test generation, multi-format ingestion, controlled retraining/fine-tuning, traceability, and spec-driven architecture/task/test generation.
- Plugin system, Test Explorer, coverage, dependency manager, security scanner, SBOM, offline documentation, keymaps, settings import/export, local terminals, visual diff/merge, large-file/log viewer, framework wizards, project importers, and package/native builder.
- Project Documentation generator for deployment guides, PRDs, TDDs, runbooks, API/program references, process and program flowcharts, ERD/data-structure documentation, and language-agnostic Javadoc-style documentation.
- UML Modeling under Tools with text authoring and live rendering, GUI diagram creation, PlantUML/Mermaid/DOT source generation, and same-window storage of UML text files.
- Local Bug Repository for the user's own programs, separate from mEditor product feedback, with user-entered bugs, debugger-created bug entries, local-only storage, user-driven export/import, and consolidated reports under the Report menu.
- Project Workspace and Project Planner: NetBeans/JDeveloper-style major projects with sub-projects, mEditor project metadata, portfolio/project planning entities, timelines, AI progress updates, and planner-to-editor project creation from user-selected tech stack and functionality.
- Inbuilt professional editor capabilities are part of the workbench: perspectives/workspaces, command palette, refactoring/navigation, visual designers/modelers, profiler/diagnostics, remote/terminal quality, task automation, templates/snippets, and migration/keymap compatibility.
- Freeware EULA display and acceptance during first run, optional registration, About-dialog license display, install-base identification, and dual-mode Feedback And Bugs reporting.
- About dialog includes safe update checking. It will later use the owner-provided public Git source, compare the installed version with the latest available release/build, prompt before install, preserve local data and user setup, and provide rollback.

## Frozen Language And Platform Coverage

Ver 1 freezes broad language support for systems languages, JVM/native-image targets, scripting and data languages, web and JavaScript ecosystems, Prolog/Lisp families, database languages, markup/config files, infrastructure files, and Rust/PHP/JavaScript framework catalogs as described in `docs/mEditor_PRODUCT_PLAN.md`.

Target platforms:

- Windows laptops.
- macOS Intel laptops.
- macOS Apple Silicon laptops, including M4.
- Linux laptops.

## Multi-Platform Packaging Baseline

Ver 1 includes two packaging tracks:

- Interim delivery may use platform-specific launchers on top of a shared generic JAR.
- Final delivery should add native builds and installers for every supported platform.

The interim launcher strategy should preserve one shared application core while platform launchers handle OS detection, runtime discovery or bundled runtime selection, app data folders, environment setup, logging, updates, icons, process startup, and native integration points.

## Imported Feature Minor Release Accounting

The Ver 1 minor release number is `5` because these imported feature groups are part of the frozen baseline:

- DBA Workshop imported from the DevOps desktop workbench.
- SSH Terminus imported from the DevOps desktop workbench.
- Code Security Analyzer imported from DVCLAS.
- CVSS Repository imported from DVCLAS.
- AI/ML Engine imported from DVCLAS.

## Freeze Rules

- New major capabilities after this file is created should be tracked as Ver 1 amendments or Ver 2 candidates.
- Product spelling must remain exactly `mEditor`.
- Each feature imported from another app must increment the Minor release component.
- Every produced build must increment the Build component, for example `1.5.0.0` to `1.5.0.1`.
- Implementation details may evolve as long as the frozen user-visible scope is preserved.
- Platform-specific launcher and native packaging work are part of Ver 1 because they support the frozen cross-platform requirement.
- The implementation baseline and benchmark record is maintained in `docs/mEditor_BASELINE_BENCHMARK.md`.

## Ver 1 Amendment: Freeware License And Install Data

Recorded on 2026-05-19:

- Starting license: `mEditor Freeware EULA`.
- License code: `LicenseRef-mEditor-Freeware-EULA`.
- License version: `1.0`.
- Active license file: `mEditor/LICENSE.md`.
- The Registration dialog and About dialog must display the active license.
- The user must accept the active EULA before mEditor opens.
- EULA acceptance fields are part of the installation record.
- Registration remains optional and does not restrict the full product.
- Network domain, machine name, and IP address are captured for install-base identification only and must not be used in bug reports or other reporting.
- Feedback includes automatic error-capture bug reports and manual bug/enhancement requests.
- The top-right menu bar includes a Feedback icon.
- Unregistered users are asked for a personal email id only to inform them about the bugfix or status update for the submitted issue.
- The feedback UI exposes `s.pandey.india@gmail.com` for direct correspondence.

## Ver 1 Amendment: Language Support Setup And Compiler Debugging

Recorded on 2026-05-19:

- `Setup > Programming Language Support` opens in the same window and lists supported programming languages, frameworks, runtimes, database dialects, markup languages, and infrastructure formats.
- Entries whose required tooling is detected are active; entries with missing tooling show install and configure actions.
- Tooling installation uses open-source tooling sources and must validate direct network access before download.
- If a tooling source is unreachable, mEditor opens a same-window network/proxy dialog where the user can test connectivity, provide a proxy URL, and validate that the target is reachable through that proxy.
- Compiler Error Debugger launches build/debug cycles in an isolated staging area and preserves original sources until a user-approved change is applied.
- The user is the master: user-controlled approval is the default for every change. Automatic debugging/remediation is only enabled by an explicit user setting after the user gains confidence.
- Every applied change must have a rollback record and a restore path.
- Interpreter-based languages run through a local terminal runner in the staging area, with stdout, stderr, and optional screen transcript capture.
- Web-based programs can run through an embedded localhost web server and same-window browser, with server log, request, and browser-console capture for debugging.
- Each suggestion must include rationale and probable impact.
- After each debugging cycle, a learning event is recorded for the embedded AI/ML service contract.
- Each debugging session produces a handover document covering issues, fixes, physical programs, timelines, outputs, learning events, and final status.
- On user request, mEditor generates deployment documents for any implementation stack and target platform. Seed templates include PHP REST deployment on Apache HTTP Server or Nginx, but the feature is generic and can target other servers, runtimes, containers, cloud platforms, or custom deployment environments.
- On user request, mEditor creates process flowcharts, program control-flow charts, data-flow diagrams, deployment flows, and error-handling flows.
- On user request, mEditor creates PRD, TDD, runbook, API reference, program reference, release notes, and user-guide documentation.
- Program documentation must work across supported languages in a language-agnostic Javadoc-style model.
- On user request, mEditor creates ERD documents for any data structure the user wants to document, including database schemas, SQL DDL, ORM models, classes, structs, JSON/XML schemas, spreadsheets, COBOL copybooks, and user-described structures.
- When requested and network access is available, the AI documentation generator may research official documentation and installer pages to derive a working deployment or platform guide.
- `About mEditor` includes `Check For Updates`. The public Git update source is intentionally unconfigured until the owner provides it.
- Update installation must not damage local data, setup, connections, SQLite repositories, generated documentation, or user settings.
- Update flow must prompt the user before installing and keep a rollback path.
- `Tools > UML Modeling` supports text mode with live rendering as the user types and GUI mode with palette/canvas modeling. GUI mode must generate UML text files for user reference, storage, diff, and version control.
- The local bug repository is for user programs only and must never be shipped automatically anywhere. It is separate from APEX feedback, which is only for mEditor product feedback.
- Users can export local bug repositories and send archives by their own channel. Another mEditor user can import the archive and generate consolidated reports under the Report menu.
- During AI/debugging cycles, mEditor should create local bug entries with physical program, diagnostics, AI suggestion summary, applied fix summary, rollback reference, timestamps, and status.
- `File > New Project` creates editor projects and sub-projects.
- `Tools > Project Planner` links to the active editor project and supports portfolio, program, project, deliverable, milestone, epic, story, task, subtask, risk, issue, and dependency planning with timelines.
- Project metadata is mEditor-owned and NetBeans-inspired, with project, module, planning, private-user, and import-report metadata files.
- When a user creates a portfolio or project in Project Planner, the planner can collect tech stack and required functionality and create the corresponding project or portfolio workspace in the editor.
- AI may update planning progress in the background from user activity, but the user must be able to accept, edit, or reject AI progress changes.
- Editor projects can be created in two ways: directly in the editor or from Project Planner.
- Project plans can be created in two ways: manual UI entry or Excel template export/import.
- Project Planner asks the user to choose `Objective -> Milestone -> Task` or Scrum `Epic -> Story -> Task`, then adapts the UI and tracking hierarchy to that model.
- Project Planner must always ask for timezone and adjust timelines accordingly.
- Project Planner tracks planned, actual, and forecast cost and shows a dashboard with Gantt chart, line/bar progress charts, and yellow/red risk indicators.
- Inbuilt editor capability menu placement: `Source > Refactor`, `Source > Reformat`, `Run > Tasks`, `Tools > Profiler`, `Tools > Templates And Snippets`, `Tools > SFTP/SCP Transfer`, `Window > Perspectives`, and `Settings > Keymaps And Imports`.
