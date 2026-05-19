# mEditor Baseline Benchmark

## Baseline Identity

- **Product name:** mEditor
- **Baseline version:** 1.5.0.0-frozen
- **Baseline date:** 2026-05-19
- **Baseline type:** Ver 1 expanded implementation scaffold benchmark
- **Owner:** Sanjay Pandey <s.pandey.india@gmail.com>
- **Build note:** This is a baseline and benchmark record, not a produced release build. The Build component remains `0`.

## Baseline Contents

This benchmark marks the current implementation scaffold as the reference point for future work.

Included Rust workspace:

- `meditor-core`: product identity, four-part version schema, build bump rule, imported-feature minor bump rule.
- `meditor-platform`: platform detection for Windows, macOS Intel, macOS Apple Silicon including M4, and Linux.
- `meditor-shell`: same-window menu model with Setup, Tools, Window, Help, About, Feedback, and update-check entries.
- `meditor-editor`: editor tab routing, same-window buffer model, and open-file buffer behavior.
- `meditor-file-explorer`: File Explorer context actions including `Open in Editor Tab`.
- `meditor-browser`: embedded same-window Web Browser command and policy model.
- `meditor-packaging`: interim generic-JAR plus platform-specific launcher plan and safe About-dialog update-check contract.
- `meditor-setup`: one-click setup orchestration model for SQLite install/check/init plus `Setup > Programming Language Support`.
- `meditor-toolchains`: executable requirement catalog, supported-language/tooling catalog, missing-tool configuration model, install flow, and network/proxy validation surface.
- `meditor-sdlc`: standard SDLC action model and executable-readiness checks.
- `meditor-vcs`: Git/SVN action catalog and safety classification.
- `meditor-cvss-repository`: SQLite repository layout, setup file registry, and schema.
- `meditor-code-security`: seed Code Security Analyzer rule pack and open-file scan model.
- `meditor-compiler-debugger`: staged compiler/interpreter/web debugging cycle, suggestion rationale/impact model, user-master approval policy, rollback records, embedded web server, execution capture, AI learning events, and user handover report model.
- `meditor-db-workbench`: SQL Developer-style connection profile, JDBC driver, worksheet, and DBA dashboard model.
- `meditor-ssh-terminus`: grouped tabbed SSH terminal manager model.
- `meditor-deployment-docs`: deployment document, project documentation, flowchart, Javadoc-style program reference, and ERD/data-structure documentation model.
- `meditor-ai`: knowledge ingestion formats, learning modes, self-learning service contract, compiler-debugging learning events, and coding assistant capability model.
- `meditor-project-importers`: NetBeans, Eclipse, and JDeveloper project metadata detection and import modes.
- `meditor-tests`: Test Explorer action catalog and coverage format model.
- `meditor-dependencies`: dependency manifest catalog and dependency action model.
- `meditor-plugins`: plugin manifest and contribution-point model.
- `meditor-installation-metrics`: freeware registration flow, EULA acceptance, install-base record, dual-mode feedback and bug reporting, APEX table catalog, and user-visible feedback status model.
- APEX REST baseline: configured for `https://oracleapex.com/ords/wksp_myhobbies/meditor/v1` with installation, event, feedback, feedback list, feedback summary, feedback status, OpenAPI catalog, and Swagger UI endpoint metadata.
- `meditor-workbench`: frozen Ver 1 workbench module registry and same-window benchmark model.
- `meditor-cli`: command-line harness for status, scan-file, SQLite schema, SQLite setup help, and SQLite initialization.

Included license and setup baseline:

- `LICENSE.md`
- `setup/setup.sh`
- `setup/setup.command`
- `setup/setup.cmd`
- `setup/apex/meditor_installation_metrics_schema.sql`
- `setup/apex/meditor_ords_rest_endpoints.sql`
- `setup/sqlite/setup-sqlite.sh`
- `setup/sqlite/setup-sqlite.ps1`
- `setup/sqlite/init_meditor_sqlite.sql`

## Benchmark Commands

Run from the mEditor project root unless otherwise noted:

```bash
cargo test
cargo run -p meditor-cli
cargo run -p meditor-cli -- sqlite-setup
cargo run -p meditor-cli -- language-support
cargo run -p meditor-cli -- compiler-debugger
cargo run -p meditor-cli -- doc-generator
cargo run -p meditor-cli -- init-sqlite /private/tmp/meditor-benchmark-2026-05-19-cli
cargo run -p meditor-cli -- scan-file ../samples/python-vulnerable-app/app.py
```

Run from the repository root:

```bash
setup/sqlite/setup-sqlite.sh --check
setup/sqlite/setup-sqlite.sh --init-db --working-folder /private/tmp/meditor-benchmark-2026-05-19-script
sqlite3 /private/tmp/meditor-benchmark-2026-05-19-cli/.meditor/security/vulnerability-intel.sqlite ".tables"
sqlite3 /private/tmp/meditor-benchmark-2026-05-19-script/.meditor/security/vulnerability-intel.sqlite ".tables"
```

## Benchmark Results

Recorded on 2026-05-19:

- `cargo test`: passed.
- Rust unit tests: 85 passed, 0 failed.
- Rust workspace crates: 29.
- CLI status: reports `mEditor 1.5.0.0-frozen`.
- Current platform reported during benchmark: `MacOS Arm64`.
- Ver 1 laptop target check: `true`.
- Default menu items: 29.
- Top-right menu bar items: 1.
- Workbench modules: 37.
- Same-window modules: `true`.
- Editor buffer routes: 4.
- DB worksheet actions: 6.
- DB dashboard sections: 7.
- SSH terminal actions: 7.
- AI knowledge formats: 11.
- AI capabilities: 9.
- Project import modes: 3.
- Project node kinds: 7.
- Planning item kinds: 12.
- Project workspace steps: 7.
- Project planning steps: 15.
- Test actions: 6.
- Coverage formats: 7.
- Dependency manifest kinds: 13.
- Dependency actions: 6.
- Plugin contribution points: 15.
- Registration statuses: 2.
- License acceptance fields: 6.
- Installation event types: 8.
- Bug report fields: 17.
- Feedback form fields: 9.
- Automatic error capture fields: 10.
- Feedback statuses: 8.
- User feedback metrics: 6.
- APEX metric tables: 5.
- APEX REST base URL: `https://oracleapex.com/ords/wksp_myhobbies/meditor/v1`.
- APEX REST endpoint fields: 9.
- OpenAPI catalog configured: `true`.
- First-run notice points: 10.
- VCS actions: 18.
- Generic JAR launchers: 6.
- Setup tasks: 9.
- Update flow steps: 8.
- Update preserved paths: 9.
- Language support catalog entries: 75.
- Language tooling install steps: 9.
- Language tooling network dialog fields: 5.
- SDLC actions: 11.
- Executable requirements: 62.
- Compiler debugger cycle steps: 9.
- Compiler debugger audit sections: 11.
- Deployment doc targets: 11.
- Deployment doc steps: 8.
- Project documentation sections: 13.
- Flowchart generation steps: 5.
- ERD supported source kinds: 10.
- ERD generation steps: 6.
- UML diagram kinds: 11.
- UML text dialects: 3.
- UML workflow steps: 7.
- Local bug repository fields: 13.
- Bug import/export steps: 7.
- Report generation steps: 7.
- Report kinds: 6.
- Self-learning AI service contract enabled: `true`.
- Security seed rules: 9.
- SQLite setup folder: `setup/sqlite`.
- SQLite available locally at benchmark time: `/usr/bin/sqlite3`.
- SQLite version reported: `3.51.0`.
- CLI SQLite initialization: passed.
- Setup script SQLite initialization: passed.
- Created SQLite tables: `finding`, `finding_suppression`, `rest_endpoint_config`, `rule_pack`, `scan_run`, `sync_state`, `vulnerability`.
- Seeded SQLite REST endpoint row: `oracleapex-wksp-myhobbies-meditor-v1`.
- OpenAPI catalog URL: `https://oracleapex.com/ords/wksp_myhobbies/open-api-catalog/meditor/v1/`.
- Swagger UI URL: `https://oracleapex.com/swagger/index.html?url=https://oracleapex.com/ords/wksp_myhobbies/open-api-catalog/meditor/v1/`.
- Python vulnerable sample scan: detected command execution and SQL concatenation findings.

## Baseline Rules

- Future produced builds must increment the Build component.
- Future features imported from another app must increment the Minor release component.
- Future benchmark runs should compare against this file and update a new benchmark record rather than overwriting this baseline.
- Product spelling must remain exactly `mEditor`.
- Active starting license is `mEditor Freeware EULA` with code `LicenseRef-mEditor-Freeware-EULA`.
- EULA acceptance is part of the installation record.
- Network domain, machine name, and IP address are for install-base identification only.
- Feedback supports automatic error-capture bug reports and manual bug/enhancement requests.
- The top-right Feedback icon is part of the Ver 1 workbench surface.
