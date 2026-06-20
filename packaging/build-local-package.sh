#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

version="$(tr -d '[:space:]' < VERSION)"
platform_os="$(uname -s | tr '[:upper:]' '[:lower:]')"
platform_arch="$(uname -m)"
package_root="dist/mEditor-${version}-${platform_os}-${platform_arch}"
binary_name="meditor-gui"
launcher_name="mEditor"

if [[ "$platform_os" == mingw* || "$platform_os" == msys* || "$platform_os" == cygwin* ]]; then
  binary_name="meditor-gui.exe"
  launcher_name="mEditor.exe"
fi

cargo build --release -p meditor-gui --offline

rm -rf "$package_root"
mkdir -p "$package_root/bin" "$package_root/docs" "$package_root/setup" "$package_root/packaging" "$package_root/vendor"

cp "target/release/$binary_name" "$package_root/bin/$launcher_name"
cp VERSION README.md LICENSE.md "$package_root/"
cp docs/USER_GUIDE.md docs/GLOBAL_METADATA.md docs/mEditor_VER_2_2_DESIGN_FREEZE.md "$package_root/docs/"
cp -R setup/. "$package_root/setup/"
find "$package_root/setup" -name .DS_Store -delete
cp packaging/verify-cross-platform-package.sh packaging/build-local-package.ps1 packaging/verify-cross-platform-package.ps1 "$package_root/packaging/"
cp -R vendor/xterm "$package_root/vendor/"

cat > "$package_root/README-FIRST.txt" <<EOF
mEditor ${version}

Run bin/${launcher_name} to start the native desktop GUI.

First run displays the freeware EULA gate. Registration is optional; the product remains fully functional without registration.

Local workspace data lives under each workspace .meditor folder. Update installs must preserve that data.
EOF

if [[ "$platform_os" == "darwin" ]]; then
  app_root="$package_root/mEditor.app"
  mkdir -p "$app_root/Contents/MacOS" "$app_root/Contents/Resources"
  cp "$package_root/bin/$launcher_name" "$app_root/Contents/MacOS/mEditor"
  cat > "$app_root/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDisplayName</key><string>mEditor</string>
  <key>CFBundleExecutable</key><string>mEditor</string>
  <key>CFBundleIdentifier</key><string>in.sanjpand.meditor</string>
  <key>CFBundleName</key><string>mEditor</string>
  <key>CFBundleShortVersionString</key><string>${version}</string>
  <key>CFBundleVersion</key><string>${version}</string>
  <key>LSMinimumSystemVersion</key><string>12.0</string>
</dict>
</plist>
EOF
fi

printf 'mEditor package staged at %s\n' "$package_root"
