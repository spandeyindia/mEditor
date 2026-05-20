# mEditor Ver 2.2 Baseline

## Baseline Identity

- **Product name:** mEditor
- **Baseline version:** 2.2.0.0
- **Baseline date:** 2026-05-19
- **Baseline type:** Ver 2.2 implementation baseline and benchmark
- **Design freeze:** Frozen on 2026-05-19 in `docs/mEditor_VER_2_2_DESIGN_FREEZE.md`
- **Owner:** Sanjay Pandey <s.pandey.india@gmail.com>
- **Build note:** This is a baseline and benchmark record. The Build component remains `0`.
- **Baseline tag:** `ver-2.2.0.0`

## Ver 2.2 Capability Map

Ver 2.2 adds these capabilities on top of Ver 2.1:

- `File > Import Project`: Visual Studio solution and project import for `.sln`, `.csproj`, `.vbproj`, `.fsproj`, `.vcxproj`, `.vcproj`, `.sqlproj`, `.dbproj`, `.shproj`, `.esproj`, `.njsproj`, `.wixproj`, `.props`, `.targets`, and `.filters`.
- `Help > Update Channel Manager`: stable, beta, nightly, and offline update package channels with release notes and rollback plan.
- `Help > Diagnostics Bundle`: reviewed, redacted export of logs, environment, configuration, recent errors, plugin list, toolchain paths, workspace trust state, and redacted connection data.
- `Help > Privacy Center`: visible explanation of install, feedback, error, update, and diagnostics data flows.
- `Tools > Extension SDK`: extension skeleton, manifest validation, permission validation, offline bundle packaging, and same-window activation.
- `Tools > Workspace Indexer`: local incremental index of files, symbols, references, call hierarchy, TODOs, dependency graph, tests, and endpoints.
- `Tools > XML Validator`: XML-family validation for active editor tabs, selected text, or chosen files.
- `Tools > JSON Validator`: JSON/JSONC/schema validation for active editor tabs, selected text, or chosen files.

## Visual Studio Import Rule

Visual Studio import must preserve original sources and write mEditor metadata beside the workspace or in a user-selected metadata folder. Import reports must list converted solution/project settings, MSBuild configurations, platforms, properties, imports, references, package references, source roots, generated output folders, unresolved SDKs, missing workloads, package restore state, unsupported settings, and manual follow-up items.

The importer must not require Visual Studio itself for basic metadata import. Build/debug actions use mEditor toolchain detection and configuration for MSBuild, .NET SDK, C++ Build Tools, NuGet, or other required executables.

## Benchmark Commands

Run from the mEditor project root:

```bash
cargo fmt
cargo test
cargo run -p meditor-cli
cargo run -p meditor-cli -- project-importers
cargo run -p meditor-cli -- ver22-pilot
cargo run -p meditor-cli -- validators
```

## Benchmark Results

Recorded on 2026-05-19:

- `cargo fmt`: passed.
- `cargo test`: passed.
- Rust unit tests: 118 passed, 0 failed.
- CLI status: reports `mEditor 2.2.0.0`.
- Current platform reported during benchmark: `MacOS Arm64`.
- Ver 2.2 laptop target check: `true`.
- Default menu items: 53.
- Top-right menu bar items: 1.
- Workbench modules: 57.
- Same-window modules: `true`.
- Project import modes: 3.
- Project import kinds: 4.
- Visual Studio project file kinds: 13.
- Visual Studio import steps: 6.
- Pilot hardening capabilities: 3.
- Pilot hardening workflow steps: 5.
- Update channels: 4.
- Diagnostics bundle items: 8.
- Privacy data flows: 6.
- Extension contribution kinds: 10.
- Extension SDK steps: 5.
- Workspace index kinds: 8.
- Workspace indexer steps: 5.
- Validator tools: 2.
- Validator workflow steps: 5.
- Self-learning AI service contract enabled: `true`.

## Baseline Rules

- Product spelling must remain exactly `mEditor`.
- Visual Studio import must preserve source layout and avoid rewriting original project files unless the user explicitly requests migration.
- Update installation must show release notes and rollback plan first.
- Diagnostics export must be reviewed by the user and redact secrets.
- Privacy Center must show all mEditor data flows in plain language.
- Extension SDK must validate plugin permissions before activation.
- Workspace Indexer must respect Workspace Trust.
- XML Validator and JSON Validator must never apply formatting or quick-fix changes until the user approves.
