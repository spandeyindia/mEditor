param(
    [switch]$Check,
    [switch]$Install,
    [switch]$InitDb,
    [string]$WorkingFolder = ""
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$AppRoot = Resolve-Path (Join-Path $ScriptDir "..\..")
$SchemaFile = Join-Path $ScriptDir "init_meditor_sqlite.sql"
if ([string]::IsNullOrWhiteSpace($WorkingFolder)) {
    $WorkingFolder = $AppRoot.Path
}

function Show-Usage {
    Write-Host "mEditor SQLite setup"
    Write-Host ""
    Write-Host "Options:"
    Write-Host "  -Check                  Check whether sqlite3 is available."
    Write-Host "  -Install                Install SQLite using winget or Chocolatey when available."
    Write-Host "  -InitDb                 Create mEditor SQLite data structures."
    Write-Host "  -WorkingFolder <path>   Workspace folder where .meditor\security is created."
}

function Get-Sqlite {
    Get-Command sqlite3 -ErrorAction SilentlyContinue
}

function Check-Sqlite {
    $sqlite = Get-Sqlite
    if ($sqlite) {
        Write-Host "sqlite3 found: $($sqlite.Source)"
        & $sqlite.Source --version
    } else {
        Write-Host "sqlite3 was not found on PATH"
        return $false
    }
    return $true
}

function Install-Sqlite {
    if (Get-Sqlite) {
        Write-Host "sqlite3 is already available. Skipping install."
        return
    }

    if (Get-Command winget -ErrorAction SilentlyContinue) {
        winget install --id SQLite.SQLite --exact
    } elseif (Get-Command choco -ErrorAction SilentlyContinue) {
        choco install sqlite -y
    } else {
        throw "No supported Windows package manager found. Install sqlite3 manually or add it to PATH."
    }
}

function Init-Database {
    $sqlite = Get-Sqlite
    if (-not $sqlite) {
        throw "sqlite3 is required. Run with -Install first or install sqlite3 manually."
    }

    $securityDir = Join-Path $WorkingFolder ".meditor\security"
    $dbFile = Join-Path $securityDir "vulnerability-intel.sqlite"
    New-Item -ItemType Directory -Force -Path $securityDir | Out-Null
    & $sqlite.Source $dbFile ".read $SchemaFile" | Out-Null
    Write-Host "Initialized mEditor SQLite repository: $dbFile"
}

if (-not $Check -and -not $Install -and -not $InitDb) {
    $Check = $true
    $Install = $true
    $InitDb = $true
}

if ($Check) {
    Check-Sqlite | Out-Null
}
if ($Install) {
    Install-Sqlite
}
if ($InitDb) {
    Init-Database
}
