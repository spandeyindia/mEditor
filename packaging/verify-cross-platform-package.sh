#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

version="$(tr -d '[:space:]' < VERSION)"
current_os="$(uname -s | tr '[:upper:]' '[:lower:]')"
current_arch="$(uname -m)"
current_package="dist/mEditor-${version}-${current_os}-${current_arch}"

printf 'mEditor cross-platform package audit\n'
printf 'Version: %s\n' "$version"
printf 'Current host: %s-%s\n' "$current_os" "$current_arch"

test -x "$current_package/bin/mEditor"
test -f "$current_package/README-FIRST.txt"
test -f "$current_package/docs/USER_GUIDE.md"
test -f "$current_package/setup/setup.sh"
test -f "$current_package/setup/setup.cmd"

if [[ "$current_os" == "darwin" ]]; then
  test -f "$current_package/mEditor.app/Contents/Info.plist"
fi

printf 'Current-host package smoke check: PASS (%s)\n' "$current_package"
printf 'Windows/Linux production validation still requires native runners or VMs for process launch, installer permissions, and desktop webview checks.\n'
