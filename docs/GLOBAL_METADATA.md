# Global Metadata

- **Product Name:** mEditor
- **Product Expansion:** Multi Editor
- **Short Name:** mEditor
- **Version:** 2.2.0.20
- **Frozen Design Version:** 2.2.0.0
- **Product License:** mEditor Freeware EULA
- **License Code:** LicenseRef-mEditor-Freeware-EULA
- **License File:** mEditor/LICENSE.md
- **Freeze Date:** 2026-05-19
- **Freeze Status:** Ver 2.2 design and implementation baseline frozen.
- **Baseline Benchmark:** docs/mEditor_VER_2_2_BASELINE.md
- **Design Freeze Record:** docs/mEditor_VER_2_2_DESIGN_FREEZE.md
- **Versioning Scheme:** Major release.Minor release.Bugfix or enhancement.Build.
- **Minor Release Rule:** Each feature imported from another app increments the Minor release component.
- **Build Number Rule:** Every produced build must increment the Build component.
- **Organization / Brand:** mEditor
- **Project Type:** Cross-platform standalone Rust-based multi-language IDE, code editor, SQL Developer-style database workbench, SSH terminal manager, and toolchain orchestration desktop suite for Windows, macOS Intel, macOS Apple Silicon including M4, and Linux laptops.
- **Primary Modules:** Editor Shell, Platform Layer, Project Workspace, Project Planner, Embedded Project About and Help, Workspace Trust, Local History and Recovery, Secrets And Credentials, Workspace Backup And Restore, Audit Trail, File Explorer, Embedded Web Browser, LSP/DAP Engine, Tree-Sitter Parser Registry, Toolchain Catalog, SDLC Runner, Version Control, Plugin System, Plugin Permissions, Extension SDK, Workspace Indexer, Test Explorer, Coverage, Dependency Manager, Code Security Analyzer, CVSS Repository, AI/ML Assistant, AI Knowledge Base, AI Training Studio, Spec-to-System Workbench, Security Scanner, DBA Workshop, Database Modeling, Database Migration, API Workbench, CI/CD Generator, XML Validator, JSON Validator, SSH Terminus, SFTP/SCP Transfer, Local Terminal Manager, Project Importers including Visual Studio, Framework Wizards, Task Chains, Workspace Dashboard, Installation Metrics, Feedback And Bugs, License Acceptance, Accessibility And Keyboard, Diff/Merge, Large File Viewer, Offline Documentation, UML Modeling, Local Bug Repository, Report Center, Update Channel Manager, Diagnostics Bundle, Privacy Center, Package/Native Builder, Help and Documentation.
- **Documentation Owner:** Sanjay Pandey <s.pandey.india@gmail.com>
- **Designer:** Sanjay Pandey <s.pandey.india@gmail.com>
- **Developer:** Sanjay Pandey <s.pandey.india@gmail.com>
- **Documentation Assistant:** AI Assistant / Sanjay Pandey <s.pandey.india@gmail.com>
- **Copyright:** Sanjay Pandey 2026
- **External Licensing:** Open-source dependencies remain under their respective upstream licenses.
- **Internal Licensing:** mEditor Freeware EULA by Sanjay Pandey <s.pandey.india@gmail.com>
- **Security Classification:** Ver 2.2 design and implementation baseline.

## Change History

| Serial | Date | Detail | Type of change | Author |
|---:|---|---|---|---|
| 1 | 2026-05-18 | Renamed project identity to mEditor, expanded as Multi Editor, and reset metadata for a Rust-based multi-language IDE and SQL Developer-style database workbench. | Product identity | AI Assistant / Sanjay Pandey |
| 2 | 2026-05-18 | Standardized project identity to Sanjay Pandey <s.pandey.india@gmail.com> and removed legacy corporate email references from the active workspace. | Metadata cleanup | AI Assistant / Sanjay Pandey |
| 3 | 2026-05-18 | Added the design baseline for language support, compiler discovery, JavaScript/PHP/Rust framework catalogs, and the embedded database workbench port. | Design baseline | AI Assistant / Sanjay Pandey |
| 4 | 2026-05-18 | Added SSH Terminus and NetBeans/Eclipse/JDeveloper import capability to the mEditor design baseline. | Design baseline | AI Assistant / Sanjay Pandey |
| 5 | 2026-05-18 | Clarified mEditor as a standalone IDE and defined SSH Terminus as a grouped tabbed SSH terminal manager. | Scope adjustment | AI Assistant / Sanjay Pandey |
| 6 | 2026-05-18 | Added SQL Developer-style JDBC driver manager requirements so users can add database driver JARs and expand database support. | Design baseline | AI Assistant / Sanjay Pandey |
| 7 | 2026-05-18 | Added privilege-gated DBA Panel requirements with real-time database dashboard, SQL worksheet execution, patch history, and backup history. | Design baseline | AI Assistant / Sanjay Pandey |
| 8 | 2026-05-18 | Added encrypted JSON connection storage in the mEditor working folder and separate import/export flows for DBA Workshop and SSH Terminus connections. | Design baseline | AI Assistant / Sanjay Pandey |
| 9 | 2026-05-18 | Added standard SDLC actions and built-in Git/SVN version-control support. | Design baseline | AI Assistant / Sanjay Pandey |
| 10 | 2026-05-18 | Added per-language executable configuration for SDLC actions when required tools cannot be auto-detected. | Design baseline | AI Assistant / Sanjay Pandey |
| 11 | 2026-05-18 | Added plugin system, Test Explorer, coverage, dependency/security scanning, database modeling, keymaps, wizards, settings import/export, task chains, workspace dashboard, offline docs, large-file viewer, and visual diff/merge requirements. | Design baseline | AI Assistant / Sanjay Pandey |
| 12 | 2026-05-18 | Added user-controlled window behavior with same-window tabbed mode as the default and optional detached windows. | Design baseline | AI Assistant / Sanjay Pandey |
| 13 | 2026-05-18 | Added embedded DVCLAS code security analyzer, CVSS repository, SQLite auto-update, editor right-click file scan, and AI/ML Assistant requirements. | Design baseline | AI Assistant / Sanjay Pandey |
| 14 | 2026-05-18 | Added PDF-based AI knowledge ingestion, local knowledge-base updates, and controlled retraining/fine-tuning requirements for the AI/ML Assistant. | Design baseline | AI Assistant / Sanjay Pandey |
| 15 | 2026-05-18 | Added AI Training Studio and Spec-to-System Workbench requirements for training the assistant in programming, system design, computer architecture, and software design from uploaded specifications and learning material. | Design baseline | AI Assistant / Sanjay Pandey |
| 16 | 2026-05-18 | Expanded AI knowledge ingestion to EPUB, MOBI, DOC/DOCX, TXT, XLS/XLSX, CSV, Markdown, and other supported training-source formats. | Design baseline | AI Assistant / Sanjay Pandey |
| 17 | 2026-05-18 | Added embedded same-window Web Browser and File Explorer context-menu open-in-editor behavior. | Design baseline | AI Assistant / Sanjay Pandey |
| 18 | 2026-05-18 | Added cross-platform laptop support requirements for Windows, macOS Intel, macOS Apple Silicon including M4, and Linux. | Design baseline | AI Assistant / Sanjay Pandey |
| 19 | 2026-05-18 | Added interim multi-platform packaging strategy using platform-specific launchers over a shared generic JAR until native build infrastructure is ready. | Design baseline | AI Assistant / Sanjay Pandey |
| 20 | 2026-05-18 | Froze the mEditor Ver 1 design baseline. | Version freeze | AI Assistant / Sanjay Pandey |
| 21 | 2026-05-18 | Updated the versioning scheme to Major release.Minor release.Bugfix or enhancement.Build. | Versioning | AI Assistant / Sanjay Pandey |
| 22 | 2026-05-18 | Added the rule that every produced build must increment the Build component. | Versioning | AI Assistant / Sanjay Pandey |
| 23 | 2026-05-18 | Added the rule that each feature imported from another app increments the Minor release component and adjusted the frozen Ver 1 baseline to 1.5.0.0-frozen. | Versioning | AI Assistant / Sanjay Pandey |
| 24 | 2026-05-18 | Added one-click SQLite setup requirement and setup folder scripts for installing SQLite and creating required data structures. | Implementation baseline | AI Assistant / Sanjay Pandey |
| 25 | 2026-05-18 | Marked the current mEditor scaffold as the Ver 1 implementation baseline and benchmark reference. | Baseline benchmark | AI Assistant / Sanjay Pandey |
| 26 | 2026-05-19 | Expanded the Ver 1 benchmark with setup orchestration, toolchain executable configuration, SDLC action planning, and workbench module registry crates. | Baseline benchmark | AI Assistant / Sanjay Pandey |
| 27 | 2026-05-19 | Completed the expanded Ver 1 pending-area scaffold across editor buffers, VCS, DBA Workbench, SSH Terminus, AI knowledge ingestion, project importers, tests, coverage, dependency management, plugins, and benchmark reporting. | Implementation baseline | AI Assistant / Sanjay Pandey |
| 28 | 2026-05-19 | Added the mEditor Freeware EULA, optional registration flow, About-dialog license text, EULA acceptance gate, and install-base identity scope. | License baseline | AI Assistant / Sanjay Pandey |
| 29 | 2026-05-19 | Added APEX installation data and bug report schema proposal with license acceptance fields and install-base-only machine/domain/IP capture. | Implementation baseline | AI Assistant / Sanjay Pandey |
| 30 | 2026-05-19 | Added dual-mode Feedback And Bugs interface model, automatic error capture, manual bug/enhancement requests, unregistered-user issue email prompt, owner contact email, feedback status metrics, and APEX feedback status tables. | Implementation baseline | AI Assistant / Sanjay Pandey |
| 31 | 2026-05-19 | Added ORDS REST endpoint deployment SQL/PLSQL for installation, event, feedback, feedback summary, and feedback status APIs, plus local SQLite REST endpoint configuration storage. | Implementation baseline | AI Assistant / Sanjay Pandey |
| 32 | 2026-05-19 | Wired the created APEX ORDS endpoint URLs, OpenAPI catalog URL, and Swagger UI URL into the mEditor Rust endpoint profile and local SQLite setup seed. | Implementation baseline | AI Assistant / Sanjay Pandey |
| 33 | 2026-05-19 | Added Setup menu programming-language support with active/missing tooling state, open-source tooling install flow, and network/proxy validation. | Implementation baseline | AI Assistant / Sanjay Pandey |
| 34 | 2026-05-19 | Added Compiler Error Debugger contract with staging area, interpreter terminal capture, embedded web server debugging, user-master approval defaults, rollback records, AI learning events, and debugging handover reports. | Implementation baseline | AI Assistant / Sanjay Pandey |
| 35 | 2026-05-19 | Added documentation generator contract for deployment guides, PRD/TDD/project docs, process and program flowcharts, language-agnostic Javadoc-style program docs, and ERD documentation for arbitrary data structures. | Implementation baseline | AI Assistant / Sanjay Pandey |
| 36 | 2026-05-19 | Added About-dialog update-check contract with future public Git source, user prompt before install, local data/setup preservation, and rollback requirement. | Implementation baseline | AI Assistant / Sanjay Pandey |
| 37 | 2026-05-19 | Added UML Modeling under Tools with text live preview, GUI modeling, and generated UML text storage. | Implementation baseline | AI Assistant / Sanjay Pandey |
| 38 | 2026-05-19 | Added local-only user-program bug repository, debugger-created bug entries, user-controlled export/import, and consolidated Report menu output. | Implementation baseline | AI Assistant / Sanjay Pandey |
| 39 | 2026-05-19 | Added project workspace and Tools > Project Planner model with major project/sub-project metadata, portfolio planning items, AI progress updates, and planner-to-editor project creation. | Implementation baseline | AI Assistant / Sanjay Pandey |
| 40 | 2026-05-19 | Added inbuilt professional editor capability group with perspectives, command palette, refactor/reformat, visual modelers, profiler, terminal quality, task automation, snippets/templates, SFTP/SCP transfer, and keymap/import compatibility. | Implementation baseline | AI Assistant / Sanjay Pandey |
| 41 | 2026-05-19 | Added the embedded About Project and Project Help documentation model required for every mEditor-created project. | Implementation baseline | AI Assistant / Sanjay Pandey |
| 42 | 2026-05-19 | Froze mEditor Ver 2 as the pre-pilot baseline at 2.0.0.0-pre-pilot. | Version freeze | AI Assistant / Sanjay Pandey |
| 43 | 2026-05-19 | Made the mEditor workspace Git-compatible with repository metadata, ignore rules, text/binary attributes, and baseline tagging. | Repository baseline | AI Assistant / Sanjay Pandey |
| 44 | 2026-05-19 | Added Ver 2.1 Workspace Trust, Local History and Recovery, Secrets And Credentials, Workspace Backup And Restore, and Audit Trail. | Implementation baseline | AI Assistant / Sanjay Pandey |
| 45 | 2026-05-19 | Added Ver 2.1 CI/CD Generator, API Workbench, Database Migration, Plugin Permissions, Accessibility And Keyboard, and keymap compatibility. | Implementation baseline | AI Assistant / Sanjay Pandey |
| 46 | 2026-05-19 | Marked mEditor Ver 2.1 baseline as 2.1.0.0. | Versioning | AI Assistant / Sanjay Pandey |
| 47 | 2026-05-19 | Added Visual Studio solution and MSBuild project import support under File > Import Project. | Implementation baseline | AI Assistant / Sanjay Pandey |
| 48 | 2026-05-19 | Added Ver 2.2 Update Channel Manager, Diagnostics Bundle, Privacy Center, Extension SDK, and Workspace Indexer. | Implementation baseline | AI Assistant / Sanjay Pandey |
| 49 | 2026-05-19 | Added XML Validator and JSON Validator as separate Tools menu options. | Implementation baseline | AI Assistant / Sanjay Pandey |
| 50 | 2026-05-19 | Marked mEditor Ver 2.2 baseline as 2.2.0.0. | Versioning | AI Assistant / Sanjay Pandey |
| 51 | 2026-05-19 | Froze the mEditor Ver 2.2 design contract for the pre-pilot implementation baseline. | Version freeze | AI Assistant / Sanjay Pandey |
| 52 | 2026-05-19 | Added the mandatory native Rust GUI shell using same-window tabs, menu bar, file explorer, command palette, feedback, about, workbench dashboard, and module surfaces; bumped the build to 2.2.0.1. | Implementation build | AI Assistant / Sanjay Pandey |
| 53 | 2026-05-19 | Added Rust-backed GUI actions for file open/save, JSON/XML validation, code-security scan, explorer refresh, DBA worksheet dry-run, toolchain detection, SQLite setup preview, project import preview, AI assistant request acknowledgement, feedback preview, diagnostics preview, and update-check preview; bumped the build to 2.2.0.2. | Implementation build | AI Assistant / Sanjay Pandey |
| 54 | 2026-05-19 | Fixed GUI script escaping so the default dashboard and backend-response panels render, and made top menus mutually exclusive; bumped the build to 2.2.0.3. | Bugfix build | AI Assistant / Sanjay Pandey |
| 55 | 2026-05-19 | Moved the visible build label from the web menubar to the native title bar and changed the main toolbar to medium icon buttons; bumped the build to 2.2.0.4. | GUI refinement build | AI Assistant / Sanjay Pandey |
| 56 | 2026-05-19 | Added Rust-backed real-time JSON/XML validation in editor tabs and changed top menus to faster custom toggles; bumped the build to 2.2.0.5. | GUI validation build | AI Assistant / Sanjay Pandey |
| 57 | 2026-05-19 | Added working New Project metadata/docs scaffold, Import Project report writing, and Source > Reformat for active JSON/XML editor tabs; bumped the build to 2.2.0.6. | Functionality build | AI Assistant / Sanjay Pandey |
| 58 | 2026-05-19 | Added Source > Vi Editor, file context Vi opening, modal normal/insert/command editing, common Vi commands, and Rust-backed :w save; bumped the build to 2.2.0.7. | Editor functionality build | AI Assistant / Sanjay Pandey |
| 59 | 2026-05-20 | Added same-window JSON/XML formatter workbench with two-panel editable/upload input and live Rust-formatted output; bumped the build to 2.2.0.8. | Formatter functionality build | AI Assistant / Sanjay Pandey |
| 60 | 2026-05-20 | Added end-to-end project task workflow with runnable scaffolds, project detection, task execution, compiler output capture, suggestions, and local debug reports; bumped the build to 2.2.0.9. | SDLC workflow build | AI Assistant / Sanjay Pandey |
| 61 | 2026-05-20 | Completed remaining pre-pilot gaps as guarded first-pass workflows for setup, VCS, DBA SQLite/JDBC profiles, SSH/SFTP, browser, AI knowledge, planner, reports, extension skeletons, workspace indexing, and feedback outbox; bumped the build to 2.2.0.10. | Pre-pilot completion build | AI Assistant / Sanjay Pandey |
| 62 | 2026-05-21 | Added release-hardening hooks for first-run EULA persistence, OS-backed credential storage, JDBC execution, SSH command tabs, SFTP/SCP queue and transfers, local AI runtime adapter, LSP detection, update-source checks, packaging, and end-user help; bumped the build to 2.2.0.11. | Release hardening build | AI Assistant / Sanjay Pandey |
| 63 | 2026-05-21 | Added Setup > Verify And Install Dependencies with executable catalog verification, missing dependency status, source URLs, platform package-manager install commands, and user-confirmed install execution; bumped the build to 2.2.0.12. | Dependency installer build | AI Assistant / Sanjay Pandey |
| 64 | 2026-05-21 | Added persistent terminal sessions, JDBC metadata dashboard/object browsing, local AI knowledge reindexing, guarded rename refactor rollback, debug adapter detection, and cross-platform package audit tooling; bumped the build to 2.2.0.13. | Runtime completion hardening build | AI Assistant / Sanjay Pandey |
| 65 | 2026-05-21 | Added PTY-aware terminal launch status, richer JDBC metadata and DBA probes, local AI retrieval model fitting with a training report, and macOS/Linux/Windows CI package validation assets; bumped the build to 2.2.0.14. | Production hardening build | AI Assistant / Sanjay Pandey |
| 66 | 2026-05-21 | Fixed packaged macOS app startup workspace detection so first-run EULA/license state uses a writable workspace instead of the read-only filesystem root; bumped the build to 2.2.0.15. | Bugfix build | AI Assistant / Sanjay Pandey |
| 67 | 2026-05-21 | Replaced remaining static frozen menu behavior with concrete Rust-backed workflows and local artifacts for every menu item; bumped the build to 2.2.0.16. | Menu workflow build | AI Assistant / Sanjay Pandey |
| 68 | 2026-05-22 | Added the production readiness gate, JSON/Markdown readiness evidence, AI fine-tune dataset export, external trainer handoff, and Windows package/CI verification path; bumped the build to 2.2.0.17. | Production readiness build | AI Assistant / Sanjay Pandey |
| 69 | 2026-05-22 | Fixed desktop screen-width behavior with a laptop-friendly default window, smaller minimum width, responsive workbench panes, scrollable menu/toolbar chrome, and bounded left-panel resizing; bumped the build to 2.2.0.18. | Screen-width bugfix build | AI Assistant / Sanjay Pandey |
| 70 | 2026-06-20 | Installed full Homebrew LLVM 22.1.7 and added production-hardening bridges for DBA Workshop, SSH terminal capability audit, UI validation evidence, LLVM backend compiler detection/install support, and embedded AI/ML engine health; bumped the build to 2.2.0.19. | Production hardening bridge build | AI Assistant / Sanjay Pandey |
| 71 | 2026-06-20 | Downloaded and vendored @xterm/xterm 5.5.0, replaced the plain SSH/local terminal output pane with the embedded xterm.js renderer, added raw keyboard input transport to the Rust terminal backend, and packaged xterm runtime/license assets; bumped the build to 2.2.0.20. | xterm renderer build | AI Assistant / Sanjay Pandey |
