# mEditor Ver 2.2 Design Freeze

## Freeze Identity

- **Product name:** mEditor
- **Frozen design version:** 2.2.0.0
- **First implementation build after freeze:** 2.2.0.1
- **Current implementation build:** 2.2.0.20
- **Freeze date:** 2026-05-19
- **Freeze type:** Ver 2.2 design freeze for the pre-pilot implementation baseline
- **Owner:** Sanjay Pandey <s.pandey.india@gmail.com>
- **Baseline record:** `docs/mEditor_VER_2_2_BASELINE.md`
- **Baseline tag:** `ver-2.2.0.0`

## Freeze Statement

The mEditor Ver 2.2 design is frozen as the pre-pilot design baseline. The user-visible scope, menu model, module map, versioning rules, local data rules, privacy wording, and cross-platform requirements recorded for `2.2.0.0` are now the design contract for this version.

Implementation may continue, but it must preserve the frozen user-facing behavior and documented module boundaries unless a later version explicitly supersedes this design freeze.

## Frozen Design Scope

This freeze includes these design commitments:

- Product spelling remains exactly `mEditor`.
- mEditor remains a standalone Rust-based desktop IDE and workbench.
- The UI remains SQL Developer-style with same-window tabbed behavior as the default and user-controlled detached-window behavior.
- The target platforms remain Windows laptops, macOS Intel laptops, macOS Apple Silicon laptops including M4, and Linux laptops.
- The version scheme remains `Major release.Minor release.Bugfix or enhancement.Build`.
- Every produced build must increment the Build component.
- Each feature imported from another app must increment the Minor release component.
- The mEditor Freeware EULA remains the active license for this baseline.
- Registration remains optional and the product remains fully functional without registration.
- Install-base tracking, bug reporting, feedback, update checks, diagnostics, and privacy disclosures remain governed by the Ver 2.2 Privacy Center and APEX endpoint design.
- DBA Workshop, SSH Terminus, File Explorer, Web Browser, Project Planner, UML Modeling, Report Center, Local Bug Repository, Code Security Analyzer, AI/ML Assistant, AI Training Studio, Workspace Trust, Extension SDK, Workspace Indexer, XML Validator, and JSON Validator remain first-class same-window workbench modules.
- Project import support includes NetBeans, Eclipse, JDeveloper, and Visual Studio solution or MSBuild project files.
- The local bug repository for user projects remains local-only and must not be sent to the mEditor feedback backend.
- User approval remains the default for AI-assisted code changes, debugger fixes, documentation generation, remediation, rollback, and deployment instructions.

## Allowed After Freeze

These changes can continue under Ver 2.2 without changing the frozen design scope:

- Bug fixes.
- Internal implementation of already-frozen features.
- Test coverage and benchmark improvements.
- Documentation clarifications that do not add new product capability.
- Refactoring that preserves public behavior, menus, metadata, storage contracts, and user workflows.
- Platform packaging and launcher work that supports the frozen cross-platform requirement.
- Security fixes that preserve the current privacy and user-consent model.

## Requires A Later Version

These changes require a new version baseline:

- New top-level modules, new menu families, or new user-visible capabilities.
- New background data collection, telemetry, sync, or external service behavior.
- Changes to the freeware EULA, registration rules, privacy rules, or install-base tracking rules.
- Changes to the product name, owner metadata, versioning scheme, or baseline identity.
- Removal of a frozen capability.
- Any change that makes a same-window module detached-only.
- Any change that weakens Workspace Trust, diagnostics redaction, secret handling, rollback, or user approval requirements.

## Verification Baseline

The design freeze inherits the Ver 2.2 benchmark:

- `cargo fmt`: passed.
- `cargo test`: passed.
- Rust unit tests: 118 passed, 0 failed.
- CLI status: reports `mEditor 2.2.0.0`.
- Default menu items: 53.
- Workbench modules: 57.
- Visual Studio project file kinds: 13.
- Validator tools: 2.
- Self-learning AI service contract enabled: `true`.

## Frozen Artifact Set

The Ver 2.2 design freeze is represented by:

- `VERSION`
- `README.md`
- `docs/GLOBAL_METADATA.md`
- `docs/mEditor_PRODUCT_PLAN.md`
- `docs/mEditor_VER_2_2_BASELINE.md`
- `docs/mEditor_VER_2_2_DESIGN_FREEZE.md`
- Rust workspace crates under `crates/`
- Native GUI shell crate at `crates/meditor-gui`
- Setup artifacts under `setup/`
