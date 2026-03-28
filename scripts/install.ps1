# install.ps1 — install the mark CLI into %USERPROFILE%\.mark\bin
#
# Usage (from the mark project root):
#   powershell -ExecutionPolicy Bypass -File scripts\install.ps1
#
# Requirements:
#   - Rust/Cargo must be installed (https://rustup.rs/)
#   - No administrator rights required — everything is user-scoped
#
# What this script does:
#   1. Builds mark in release mode
#   2. Creates %USERPROFILE%\.mark\bin and %USERPROFILE%\.mark\rendered
#   3. Copies the binary to %USERPROFILE%\.mark\bin\mark.exe
#   4. Adds %USERPROFILE%\.mark\bin to the user PATH (idempotent)
#   5. Marks %USERPROFILE%\.mark as hidden

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$MarkDir     = Join-Path $env:USERPROFILE '.mark'
$BinDir      = Join-Path $MarkDir 'bin'
$RenderedDir = Join-Path $MarkDir 'rendered'
$Binary      = Join-Path $BinDir 'mark.exe'

function Write-Info    { param($msg) Write-Host "[mark] $msg" }
function Write-Success { param($msg) Write-Host "[mark] OK $msg" -ForegroundColor Green }
function Write-Warn    { param($msg) Write-Warning "[mark] $msg" }
function Exit-Error    { param($msg) Write-Error "[mark] $msg"; exit 1 }

# ── preflight ────────────────────────────────────────────────────────────────

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Exit-Error @"
Cargo not found. Install Rust from https://rustup.rs/ and try again.
After installing, restart this terminal and re-run the script.
"@
}

if (-not (Test-Path 'Cargo.toml')) {
    Exit-Error "Cargo.toml not found. Run this script from the mark project root."
}

# ── build ─────────────────────────────────────────────────────────────────────

Write-Info "Building mark (release)..."
cargo build --release
if ($LASTEXITCODE -ne 0) { Exit-Error "Build failed." }
Write-Success "Build complete."

# ── install dirs ──────────────────────────────────────────────────────────────

foreach ($dir in @($BinDir, $RenderedDir)) {
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
    }
}
Write-Success "Directories ready: $MarkDir"

# ── mark .mark as hidden ──────────────────────────────────────────────────────

try {
    $item = Get-Item -LiteralPath $MarkDir -Force
    $item.Attributes = $item.Attributes -bor [System.IO.FileAttributes]::Hidden
    Write-Info ".mark directory marked as hidden."
} catch {
    Write-Warn "Could not mark .mark as hidden: $_"
}

# ── copy binary ───────────────────────────────────────────────────────────────

Copy-Item -Path 'target\release\mark.exe' -Destination $Binary -Force
Write-Success "Binary installed: $Binary"

# ── PATH setup (idempotent, user-scoped) ──────────────────────────────────────

$currentPath = [System.Environment]::GetEnvironmentVariable('PATH', 'User')
if ($null -eq $currentPath) { $currentPath = '' }

# Normalise: trim trailing backslash for comparison.
$BinDirNorm = $BinDir.TrimEnd('\')
$alreadyPresent = ($currentPath -split ';') | Where-Object {
    $_.TrimEnd('\') -eq $BinDirNorm
}

if ($alreadyPresent) {
    Write-Info "PATH already contains $BinDir — skipping."
    $pathChanged = $false
} else {
    $newPath = ($currentPath.TrimEnd(';') + ';' + $BinDir).TrimStart(';')
    [System.Environment]::SetEnvironmentVariable('PATH', $newPath, 'User')
    Write-Success "Added $BinDir to user PATH."
    $pathChanged = $true
}

# ── done ──────────────────────────────────────────────────────────────────────

Write-Host ""
Write-Success "mark installed successfully!"
Write-Host ""
Write-Host "  Binary : $Binary"
Write-Host "  Renders: $RenderedDir"
Write-Host ""

if ($pathChanged) {
    Write-Host "  NOTE: PATH was updated. Restart this terminal (or open a new one)"
    Write-Host "        before running mark."
} else {
    Write-Host "  mark is ready. Open a new terminal and run: mark --help"
}
Write-Host ""
