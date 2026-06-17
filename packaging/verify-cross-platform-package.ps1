$ErrorActionPreference = "Stop"
$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $RepoRoot

$Version = (Get-Content VERSION -Raw).Trim()
$Arch = if ($env:PROCESSOR_ARCHITECTURE -eq "ARM64") { "arm64" } else { "x86_64" }
$PackageRoot = Join-Path $RepoRoot "dist\mEditor-$Version-windows-$Arch"

Write-Host "mEditor cross-platform package audit"
Write-Host "Version: $Version"
Write-Host "Current host: windows-$Arch"

if (-not (Test-Path (Join-Path $PackageRoot "bin\mEditor.exe"))) {
    throw "Missing Windows launcher"
}
if (-not (Test-Path (Join-Path $PackageRoot "README-FIRST.txt"))) {
    throw "Missing README-FIRST.txt"
}
if (-not (Test-Path (Join-Path $PackageRoot "docs\USER_GUIDE.md"))) {
    throw "Missing docs\USER_GUIDE.md"
}
if (-not (Test-Path (Join-Path $PackageRoot "setup\setup.cmd"))) {
    throw "Missing setup\setup.cmd"
}
if (-not (Test-Path (Join-Path $PackageRoot "setup\setup.sh"))) {
    throw "Missing setup\setup.sh"
}

Write-Host "Current-host package smoke check: PASS ($PackageRoot)"
