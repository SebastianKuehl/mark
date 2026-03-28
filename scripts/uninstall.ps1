# uninstall.ps1 — remove the mark CLI from %USERPROFILE%\.mark\bin
#
# Usage (from the mark project root, or anywhere):
#   powershell -ExecutionPolicy Bypass -File scripts\uninstall.ps1
#
# What this script does:
#   1. Removes the mark binary from %USERPROFILE%\.mark\bin
#   2. Removes %USERPROFILE%\.mark\bin from the user PATH (if present)
#   3. Asks whether to remove %USERPROFILE%\.mark\rendered
#   4. Removes empty directories (bin, and .mark itself if empty)
#
# No administrator rights required.

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$MarkDir     = Join-Path $env:USERPROFILE '.mark'
$BinDir      = Join-Path $MarkDir 'bin'
$RenderedDir = Join-Path $MarkDir 'rendered'
$Binary      = Join-Path $BinDir 'mark.exe'

function Write-Info    { param($msg) Write-Host "[mark] $msg" }
function Write-Success { param($msg) Write-Host "[mark] OK $msg" -ForegroundColor Green }
function Write-Warn    { param($msg) Write-Warning "[mark] $msg" }

# ── remove binary ─────────────────────────────────────────────────────────────

if (Test-Path $Binary) {
    Remove-Item -LiteralPath $Binary -Force
    Write-Success "Removed binary: $Binary"
} else {
    Write-Info "Binary not found at $Binary — nothing to remove."
}

# ── remove PATH entry (user-scoped, idempotent) ───────────────────────────────

$currentPath = [System.Environment]::GetEnvironmentVariable('PATH', 'User')
if ($null -ne $currentPath) {
    $BinDirNorm = $BinDir.TrimEnd('\')
    $parts = ($currentPath -split ';') | Where-Object {
        $_.TrimEnd('\') -ne $BinDirNorm -and $_ -ne ''
    }
    $newPath = $parts -join ';'
    if ($newPath -ne $currentPath) {
        [System.Environment]::SetEnvironmentVariable('PATH', $newPath, 'User')
        Write-Success "Removed $BinDir from user PATH."
    } else {
        Write-Info "$BinDir was not in user PATH — nothing to remove."
    }
}

# ── optionally remove rendered files ─────────────────────────────────────────

Write-Host ""
if (Test-Path $RenderedDir) {
    $answer = Read-Host "[mark] Remove rendered HTML files in $RenderedDir? [y/N]"
    if ($answer -match '^[yY]') {
        Remove-Item -LiteralPath $RenderedDir -Recurse -Force
        Write-Success "Removed $RenderedDir"
    } else {
        Write-Info "Leaving $RenderedDir intact."
    }
}

# ── clean up empty directories ────────────────────────────────────────────────

if (Test-Path $BinDir) {
    $binContents = Get-ChildItem -LiteralPath $BinDir -Force
    if ($binContents.Count -eq 0) {
        Remove-Item -LiteralPath $BinDir -Force
        Write-Success "Removed empty directory: $BinDir"
    }
}

if (Test-Path $MarkDir) {
    $markContents = Get-ChildItem -LiteralPath $MarkDir -Force
    if ($markContents.Count -eq 0) {
        Remove-Item -LiteralPath $MarkDir -Force
        Write-Success "Removed empty directory: $MarkDir"
    }
}

# ── done ──────────────────────────────────────────────────────────────────────

Write-Host ""
Write-Success "mark uninstalled."
Write-Host ""
Write-Host "  Restart this terminal for PATH changes to take effect."
Write-Host ""
