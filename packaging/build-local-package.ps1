param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $RepoRoot

$Version = (Get-Content VERSION -Raw).Trim()
$Arch = if ($env:PROCESSOR_ARCHITECTURE -eq "ARM64") { "arm64" } else { "x86_64" }
$PackageRoot = Join-Path $RepoRoot "dist\mEditor-$Version-windows-$Arch"

if (-not $SkipBuild) {
    cargo build --release -p meditor-gui --offline
}

if (Test-Path $PackageRoot) {
    Remove-Item -Recurse -Force $PackageRoot
}

New-Item -ItemType Directory -Force -Path `
    (Join-Path $PackageRoot "bin"), `
    (Join-Path $PackageRoot "docs"), `
    (Join-Path $PackageRoot "setup"), `
    (Join-Path $PackageRoot "packaging") | Out-Null

Copy-Item "target\release\meditor-gui.exe" (Join-Path $PackageRoot "bin\mEditor.exe")
Copy-Item VERSION, README.md, LICENSE.md $PackageRoot
Copy-Item docs\USER_GUIDE.md, docs\GLOBAL_METADATA.md, docs\mEditor_VER_2_2_DESIGN_FREEZE.md (Join-Path $PackageRoot "docs")
Copy-Item -Recurse setup\* (Join-Path $PackageRoot "setup")
Copy-Item packaging\verify-cross-platform-package.ps1 (Join-Path $PackageRoot "packaging")

@"
mEditor $Version

Run bin\mEditor.exe to start the native desktop GUI.

First run displays the freeware EULA gate. Registration is optional; the product remains fully functional without registration.

Local workspace data lives under each workspace .meditor folder. Update installs must preserve that data.
"@ | Set-Content -Encoding UTF8 (Join-Path $PackageRoot "README-FIRST.txt")

Write-Host "mEditor Windows package staged at $PackageRoot"
