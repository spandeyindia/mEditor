# mEditor Ver 2.1 Baseline

## Baseline Identity

- **Product name:** mEditor
- **Baseline version:** 2.1.0.0
- **Baseline date:** 2026-05-19
- **Baseline type:** Ver 2.1 implementation baseline and benchmark
- **Owner:** Sanjay Pandey <s.pandey.india@gmail.com>
- **Build note:** This is a baseline and benchmark record. The Build component remains `0`.

## Ver 2.1 Capability Map

Ver 2.1 adds these ten inbuilt same-window capabilities on top of Ver 2 pre-pilot:

- `Settings > Workspace Trust`: unknown projects open Restricted until user trust is granted.
- `File > Local History And Recovery`: file snapshots, project metadata snapshots, crash recovery, session undo, and project-state restore.
- `Settings > Secrets And Credentials`: OS keychain-backed vault for database passwords, SSH keys, API tokens, proxy credentials, Git credentials, and signing keys.
- `Tools > CI/CD Generator`: GitHub Actions, GitLab CI, Jenkins, Azure Pipelines, CircleCI, and local script generation from mEditor project metadata.
- `Tools > API Workbench`: REST, SOAP, GraphQL, gRPC, and WebSocket testing with environments, project-doc links, and generated client hooks.
- `Tools > Database Migration`: Flyway-style and Liquibase-style schema versioning, migration generation, dry run, apply, rollback, drift detect, schema compare, and DDL export.
- `File > Workspace Backup And Restore`: previewed export/import of settings, metadata, encrypted connection stores, snippets, local bugs, reports, planner data, embedded docs, and audit trail.
- `Settings > Plugin Permissions`: deny-by-default permission declarations for plugin filesystem, terminal, network, database, SSH, AI, credential, and workspace metadata access.
- `Settings > Accessibility And Keyboard`: high contrast, screen-reader labels, keyboard navigation, reduced motion, font scaling, focus visibility, command-palette access, and keymap profiles.
- `Report > Audit Trail`: append-only local audit of builds, debug cycles, AI suggestions, applied changes, rollbacks, deploy docs, DBA actions, secret access, plugin decisions, and trust decisions.

## Benchmark Commands

Run from the mEditor project root:

```bash
cargo fmt
cargo test
cargo run -p meditor-cli
cargo run -p meditor-cli -- ver21-safety
cargo run -p meditor-cli -- ver21-delivery
cargo run -p meditor-cli -- ver21-governance
```

## Benchmark Results

Recorded on 2026-05-19:

- `cargo fmt`: passed.
- `cargo test`: passed.
- Rust unit tests: 109 passed, 0 failed.
- CLI status: reports `mEditor 2.1.0.0`.
- Current platform reported during benchmark: `MacOS Arm64`.
- Ver 2.1 laptop target check: `true`.
- Default menu items: 41.
- Top-right menu bar items: 1.
- Workbench modules: 49.
- Same-window modules: `true`.
- Workspace safety capabilities: 5.
- Workspace safety workflow steps: 6.
- Restricted trust capabilities: 8.
- Secret kinds: 6.
- Workspace backup items: 9.
- Audit event kinds: 10.
- Delivery automation capabilities: 3.
- Delivery automation workflow steps: 5.
- CI/CD providers: 6.
- API protocols: 5.
- Database migration actions: 8.
- Governance capabilities: 2.
- Governance workflow steps: 5.
- Plugin permissions: 9.
- Accessibility features: 7.
- Keymap profiles: 7.
- Self-learning AI service contract enabled: `true`.

## Baseline Rules

- Product spelling must remain exactly `mEditor`.
- User remains master for trust, secrets, backup restore, migration apply, plugin permission grants, and any code-changing automation.
- Unknown workspaces must start in Restricted trust.
- Secret values must not be stored as plain JSON.
- Workspace backups must exclude raw secrets.
- Database migration apply must require a backup prompt.
- Plugin permissions must be deny-by-default.
- Audit Trail is local and append-only.
