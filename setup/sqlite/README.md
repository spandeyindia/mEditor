# mEditor SQLite Setup

This setup folder provides the Ver 2 pre-pilot SQLite setup option for the mEditor CVSS repository and Code Security Analyzer data structures.

## What It Creates

The setup initializes this working-folder layout:

```text
.meditor/
  security/
    vulnerability-intel.sqlite
```

The schema creates these tables:

- `vulnerability`
- `sync_state`
- `rule_pack`
- `scan_run`
- `finding`
- `finding_suppression`

## macOS And Linux

One-click setup from the `setup` folder:

```bash
./setup.sh
```

The default setup checks SQLite, installs it if missing where a supported package manager is available, and creates the mEditor SQLite data structures.

Check for SQLite only:

```bash
./setup-sqlite.sh --check
```

Install SQLite with the detected package manager where supported:

```bash
./setup-sqlite.sh --install
```

Create the mEditor SQLite data structures in the current working folder:

```bash
./setup-sqlite.sh --init-db --working-folder /path/to/workspace
```

## Windows

One-click setup from the `mEditor\setup` folder:

```powershell
.\setup.cmd
```

The default setup checks SQLite, installs it if missing where a supported package manager is available, and creates the mEditor SQLite data structures.

Check for SQLite only:

```powershell
.\setup-sqlite.ps1 -Check
```

Install SQLite with an available package manager where supported:

```powershell
.\setup-sqlite.ps1 -Install
```

Create the mEditor SQLite data structures:

```powershell
.\setup-sqlite.ps1 -InitDb -WorkingFolder C:\path\to\workspace
```

## CLI Alternative

The Rust CLI can initialize the same database when `sqlite3` is on `PATH`:

```bash
meditor init-sqlite /path/to/workspace
```
