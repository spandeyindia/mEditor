# mEditor Actual Status

- **Date:** 2026-06-20
- **Current build:** 2.2.0.20
- **Status:** Pre-pilot macOS validation build, not full production release
- **Current host package:** `dist/mEditor-2.2.0.20-darwin-arm64`
- **Current host package smoke check:** PASS on macOS arm64
- **Repository state before this status commit:** changes staged for commit after verification

## Completed In This Phase

- Full Homebrew LLVM 22.1.7_1 installed on this Mac.
- mEditor now detects full LLVM separately from Apple Clang, including `clang`, `clang++`, `llvm-config`, `llc`, `opt`, and `llvm-as`.
- mEditor treats LLVM/Clang as a backend compiler profile for native-language workflows and writes `.meditor/toolchains/llvm-backend.json` from the LLVM Backend Audit.
- DBA Workshop exposes richer JDBC metadata actions: dashboard, catalogs, schemas, objects, columns, indexes, primary keys, foreign keys, procedures, type information, and table privileges.
- DBA Workshop exposes a workbench audit and DBA probes for generic, Oracle-style patch/RMAN/session, PostgreSQL, MySQL, and SQL Server views.
- SSH Terminus uses PTY-backed launch through the platform `script` broker where available.
- SSH and local terminal tabs now render through vendored `@xterm/xterm` 5.5.0.
- xterm terminal keyboard input is sent raw to the Rust terminal backend.
- The manual terminal command box remains available for paste/send workflows.
- xterm runtime assets and license are vendored under `vendor/xterm` and copied into packages.
- AI/ML Assistant exposes engine health for local knowledge, retrieval index, local retrieval model, dataset export, local runtime adapter, and external trainer configuration.
- Diagnostics exposes a Production Hardening Audit that writes JSON and Markdown evidence under `.meditor/production-hardening`.

## Verified Locally

- `cargo test --offline -p meditor-core -p meditor-gui -p meditor-toolchains -- --test-threads=1`
- `bash packaging/build-local-package.sh`
- `bash packaging/verify-cross-platform-package.sh`
- `git diff --check`
- Metadata scan for forbidden product/email terms
- LLVM smoke compile with `/opt/homebrew/opt/llvm/bin/clang`

## Still Not Full Production Release

- Windows and Linux packages still require native-machine or VM launch testing.
- macOS package is not signed, notarized, or delivered as a final installer yet.
- DBA Workshop is now substantially richer, but it is not yet a full SQL Developer clone for every object action, visual grid edit, explain-plan renderer, session management action, and vendor-specific DBA workflow.
- SSH rendering is xterm-grade, but PTY allocation still depends on the platform broker rather than a dedicated cross-platform Rust PTY crate such as a portable ConPTY/unix-pty abstraction.
- AI/ML self-learning is embedded for local retrieval/index/model artifacts; true model-weight training is still delegated to an explicitly configured external trainer.
- Update installation still needs a real public release source and rollback-tested update flow.
- End-to-end UI hardening is improved, but every workflow still needs manual exploratory testing before a production release.

## Current Readiness Classification

mEditor 2.2.0.20 is suitable for continued development and macOS pre-pilot validation. It should not yet be described as a full production release.
