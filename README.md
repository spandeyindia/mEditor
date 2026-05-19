# mEditor (Multi Editor)

mEditor is planned as a standalone Rust-based multi-language IDE, code editor, database workbench, SSH terminal manager, and toolchain orchestration desktop suite.

## Identity

- **Product name:** mEditor
- **Expansion:** Multi Editor
- **Owner:** Sanjay Pandey <s.pandey.india@gmail.com>
- **Ver 1 baseline:** `1.5.0.0-frozen`, frozen on 2026-05-18.
- **Ver 2 pre-pilot baseline:** `2.0.0.0-pre-pilot`, frozen on 2026-05-19.
- **Implementation direction:** Standalone Rust workspace with reusable crates for editor shell, language tooling, debugger integration, file explorer, embedded browser, database workbench, SSH terminal management, project importers, framework wizards, packaging, and native-image support.
- **UI direction:** SQL Developer-style desktop workbench with navigator trees, file explorer, tabbed editors, embedded browser tabs, worksheet tabs, result grids, object browser panels, connection profiles, DBA dashboards, task consoles, and a user-controlled same-window tab policy.
- **Target platforms:** Windows laptops, macOS Intel laptops, macOS Apple Silicon laptops including M4, and Linux laptops.
- **Licensing direction:** mEditor starts under the `mEditor Freeware EULA` (`LicenseRef-mEditor-Freeware-EULA`), with optional future open-source relicensing such as GPL v3.0 at owner discretion. Bundled open-source compilers, runtimes, frameworks, and libraries remain under their respective upstream licenses.

## Scope

mEditor should support source editing, build/run/debug tooling, SDLC actions, version control, language servers, syntax trees, compiler discovery, and framework project creation for broad language families:

- Systems languages: Assembly, C, C++, Objective-C, Ada, Fortran, COBOL, BASIC dialects, Clipper/xBase, Rust, Zig, Nim, V, and Go.
- JVM and native-image targets: Java, Kotlin, Scala, Groovy, Clojure, and GraalVM Native Image workflows.
- Scripting and data languages: Python, PHP, Perl, Ruby, Lua, R, Julia, MATLAB/Octave-style workflows, shell, and PowerShell.
- Web and JavaScript ecosystems: HTML, CSS, JavaScript, TypeScript, JSX, TSX, React, Node.js, Deno, Bun, Electron, React Native, Vue, Svelte, Angular templates, Preact, Solid, Qwik, Lit, Astro, Next.js, Remix, Vite, Webpack, Rollup, esbuild, SWC, and Babel.
- Logic and symbolic languages: Prolog, Common Lisp, Scheme, Racket, and related Lisp-family tooling.
- Database languages: SQL, PL/SQL, T-SQL, PL/pgSQL, SQL PL, MySQL/MariaDB SQL, SQLite SQL, BigQuery SQL, Snowflake SQL, and other dialects through a dialect registry.
- Markup, config, and infrastructure files: XML, JSON, YAML, TOML, Markdown, Dockerfile, Terraform HCL, Kubernetes manifests, INI, properties, and Makefiles.

## Database Workbench

The database workbench will be embedded as a Rust module and modeled on SQL Developer-style database connection behavior already present in the sibling DevOps desktop project. The Rust port should preserve the useful ideas: encrypted connection profiles, user-added JDBC drivers, test connection, object browsing, worksheet execution, result grids, dashboard probes, storage/session/high-availability views, forecast-ready metrics, patch and backup history, and approval-gated administration actions.

Inside **DBA Workshop**, any database connection authenticated with `SYSDBA` or an equivalent administrator/monitoring privilege should allow the user to activate a **DBA Panel**. This panel should render a real-time database dashboard from the active connection, including health, sessions, storage, waits/locks, high availability, patch history, RMAN or engine-specific backup history, and SQL Developer-style worksheet execution.

The Tools menu should include **DBA Workshop**, **SSH Terminus**, and **Web Browser** as first-class menu items. SSH Terminus should provide grouped SSH connection profiles and multiple SSH terminals opened in tabs, with group launch, group close, tab search, terminal profiles, and safe credential handling. The Window menu should include **File Explorer**, and a file right-click context menu must let the user open that file directly in an editor tab.

DBA Workshop and SSH Terminus should keep their connection information in encrypted JSON files inside the mEditor working folder. Each tool should also have its own import/export flow for encrypted connection export files.

mEditor must provide standard SDLC actions for projects: build, clean, rebuild, test, deploy, launch/run, debug, package, publish, and custom task execution. If an action needs an external executable, mEditor should auto-detect it first and then allow the user to configure the executable path per language/toolchain when detection fails. Git and SVN should be built-in version-control providers with status, diff, commit, branch/tag, merge/update, revert, history, blame/annotate, and conflict-resolution workflows.

mEditor must also include a plugin system, Test Explorer, code coverage view, dependency manager, security/dependency scanner, ER diagrams, schema/data compare, DDL generator, migration runner, explain-plan visualizer, keymap profiles, project wizards, settings import/export, local terminal manager, task-chain scheduler, workspace dashboard, offline documentation browser, large-file/log viewer, and visual diff/merge tooling.

mEditor must embed the DVCLAS code security analyzer as a separate **Code Security Analyzer** workbench option. It should include a CVSS repository backed by SQLite, keep the vulnerability database auto-updated, and add an editor right-click action to scan the currently open file on demand. It must also embed and enhance the DVCLAS AI/ML engine as an **AI/ML Assistant** chat tab for coding help, security remediation, test suggestions, workspace-aware developer guidance, and multi-format knowledge ingestion with controlled retraining or fine-tuning support.

The AI/ML Assistant should grow into an **AI Training Studio** for programming languages, software design, system design, computer architecture, database design, security engineering, testing, and DevOps knowledge. When a user uploads a specification document or learning source in formats such as PDF, EPUB, MOBI, DOC/DOCX, TXT, XLS/XLSX, CSV, or Markdown, mEditor should extract requirements, produce an architecture plan, generate tasks and tests, propose code changes, and use the trained knowledge base to assist implementation while keeping edits approval-gated.

mEditor should default to opening all functionality in the same main window using tabs. Users should be able to choose the window behavior: same-window tabs only, allow detached windows, or ask each time.

mEditor must be designed and coded as a cross-platform laptop application from the start. Platform-specific behavior for file paths, terminals, shells, keychains, webviews, native menus, installers, code signing, file watchers, keyboard shortcuts, process execution, and bundled runtimes should be isolated behind Rust platform abstractions.

For early multi-platform delivery, mEditor may use platform-specific launchers on top of a generic JAR until the full infrastructure for generating native builds on all supported platforms is ready. The launchers should handle platform detection, JVM/runtime discovery or bundling, environment setup, app data folders, logging, update handoff, and native integration points while preserving one shared application core.

mEditor must provide a one-click setup option in the `setup` folder. The setup should check SQLite, install it where a supported platform package manager is available, and create the `.meditor/security/vulnerability-intel.sqlite` data structures needed by the CVSS Repository and Code Security Analyzer.

mEditor must display the active freeware EULA in the Registration dialog and from an About-dialog license button. The user must accept the EULA before the product opens. Registration remains optional; registered copies show `Personal License to USERNAME`, and unregistered copies show `UnRegistered but fully functional copy with no obligation`. The installation record includes license acceptance plus network domain, machine name, and IP address for install-base identification only. A top-right Feedback icon opens dual-mode feedback: automatic error-capture bug reports and manual bug/enhancement requests. Unregistered users are asked for a personal email id only to receive bugfix or status updates for the submitted issue, and the UI displays `s.pandey.india@gmail.com` for direct correspondence.

mEditor should import NetBeans, Eclipse, and JDeveloper projects by reading their project metadata, classpaths, build files, source roots, dependencies, application server settings, and generated task profiles into an mEditor workspace.

Every mEditor-created project must include embedded About Project and Project Help documentation under `.meditor/project/docs`. These project help surfaces open in the same window from the Help menu, are user-editable, and become the standard model for future projects built with mEditor.

See [mEditor_PRODUCT_PLAN.md](docs/mEditor_PRODUCT_PLAN.md) for the detailed architecture, compiler catalog, Rust framework coverage, PHP framework download manifest, and migration phases.

The frozen Ver 1 baseline is recorded in [mEditor_VERSION_1_FREEZE.md](docs/mEditor_VERSION_1_FREEZE.md). Any new major capability after this point should be tracked as a Ver 1 amendment or a Ver 2 candidate.

The Ver 2 pre-pilot baseline is recorded in [mEditor_VER_2_PRE_PILOT_BASELINE.md](docs/mEditor_VER_2_PRE_PILOT_BASELINE.md). The earlier Ver 1 implementation benchmark remains in [mEditor_BASELINE_BENCHMARK.md](docs/mEditor_BASELINE_BENCHMARK.md).
