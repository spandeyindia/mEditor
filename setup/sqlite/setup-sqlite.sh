#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
SCHEMA_FILE="${SCRIPT_DIR}/init_meditor_sqlite.sql"
WORKING_FOLDER="${mEditor_WORKING_FOLDER:-${APP_ROOT}}"
DO_CHECK=false
DO_INSTALL=false
DO_INIT=false

usage() {
  cat <<'USAGE'
mEditor SQLite setup

Options:
  --check                         Check whether sqlite3 is available.
  --install                       Install SQLite using a supported package manager.
  --init-db                       Create mEditor SQLite data structures.
  --working-folder <path>         Workspace folder where .meditor/security is created.
  --help                          Show this help.

Examples:
  ./setup-sqlite.sh
  ./setup-sqlite.sh --check
  ./setup-sqlite.sh --install
  ./setup-sqlite.sh --init-db --working-folder /path/to/workspace
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --check)
      DO_CHECK=true
      shift
      ;;
    --install)
      DO_INSTALL=true
      shift
      ;;
    --init-db)
      DO_INIT=true
      shift
      ;;
    --working-folder)
      WORKING_FOLDER="${2:?missing path after --working-folder}"
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      usage
      exit 2
      ;;
  esac
done

sqlite_path() {
  command -v sqlite3 || true
}

check_sqlite() {
  local sqlite
  sqlite="$(sqlite_path)"
  if [[ -n "${sqlite}" ]]; then
    echo "sqlite3 found: ${sqlite}"
    "${sqlite}" --version
  else
    echo "sqlite3 was not found on PATH"
    return 1
  fi
}

install_sqlite() {
  if [[ -n "$(sqlite_path)" ]]; then
    echo "sqlite3 is already available. Skipping install."
    return 0
  fi

  case "$(uname -s)" in
    Darwin)
      if command -v brew >/dev/null 2>&1; then
        brew install sqlite
      else
        echo "Homebrew was not found. Install SQLite manually or install Homebrew first." >&2
        return 1
      fi
      ;;
    Linux)
      if command -v apt-get >/dev/null 2>&1; then
        sudo apt-get update
        sudo apt-get install -y sqlite3
      elif command -v dnf >/dev/null 2>&1; then
        sudo dnf install -y sqlite
      elif command -v yum >/dev/null 2>&1; then
        sudo yum install -y sqlite
      elif command -v pacman >/dev/null 2>&1; then
        sudo pacman -S --needed sqlite
      elif command -v zypper >/dev/null 2>&1; then
        sudo zypper install -y sqlite3
      else
        echo "No supported Linux package manager found. Install sqlite3 manually." >&2
        return 1
      fi
      ;;
    *)
      echo "Unsupported OS for this script. Use setup-sqlite.ps1 on Windows or install sqlite3 manually." >&2
      return 1
      ;;
  esac
}

init_db() {
  local sqlite
  sqlite="$(sqlite_path)"
  if [[ -z "${sqlite}" ]]; then
    echo "sqlite3 is required. Run with --install first or install sqlite3 manually." >&2
    return 1
  fi

  local security_dir="${WORKING_FOLDER}/.meditor/security"
  local db_file="${security_dir}/vulnerability-intel.sqlite"
  mkdir -p "${security_dir}"
  "${sqlite}" "${db_file}" < "${SCHEMA_FILE}" >/dev/null
  echo "Initialized mEditor SQLite repository: ${db_file}"
}

if ! ${DO_CHECK} && ! ${DO_INSTALL} && ! ${DO_INIT}; then
  DO_CHECK=true
  DO_INSTALL=true
  DO_INIT=true
fi

if ${DO_CHECK}; then
  check_sqlite || true
fi
if ${DO_INSTALL}; then
  install_sqlite
fi
if ${DO_INIT}; then
  init_db
fi
