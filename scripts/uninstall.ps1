# uninstall.ps1 — remove the mark CLI from %USERPROFILE%\.mark\bin
#
# Usage (from the mark project root, or anywhere):
#   powershell -ExecutionPolicy Bypass -File scripts\uninstall.ps1
#
# What this script does:
#   1. Removes the mark binary from %USERPROFILE%\.mark\bin
#   2. Removes %USERPROFILE%\.mark\bin from the user PATH (if present)
#   3. Removes the PowerShell completion script and its profile hook (idempotent)
#   4. Asks whether to remove %USERPROFILE%\.mark\rendered
#   5. Removes empty directories (bin, and .mark itself if empty)
#
# No administrator rights required.

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$MarkDir          = Join-Path $env:USERPROFILE '.mark'
$BinDir           = Join-Path $MarkDir 'bin'
$RenderedDir      = Join-Path $MarkDir 'rendered'
$Binary           = Join-Path $BinDir 'mark.exe'
$CompletionsDir   = Join-Path $MarkDir 'completions'
$CompletionFile   = Join-Path $CompletionsDir 'mark.ps1'
$CompletionMarker = '# >>> mark completions >>>'
$CompletionEnd    = '# <<< mark completions <<<'

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

# ── remove PowerShell completion script and profile hook ─────────────────────

if (Test-Path $CompletionFile) {
    Remove-Item -LiteralPath $CompletionFile -Force
    Write-Success "Removed PowerShell completion script: $CompletionFile"
} else {
    Write-Info "Completion script not found at $CompletionFile — skipping."
}

# Remove the sourcing block from $PROFILE, idempotently.
if (Test-Path $PROFILE) {
    $profileContent = Get-Content -Path $PROFILE -Raw -ErrorAction SilentlyContinue
    if ($null -ne $profileContent -and $profileContent -match [regex]::Escape($CompletionMarker)) {
        # Safety: require both markers before rewriting.
        if (-not ($profileContent -match [regex]::Escape($CompletionEnd))) {
            Write-Warn "End marker '$CompletionEnd' missing in $PROFILE — skipping to avoid corruption."
            Write-Warn "Remove the block manually between '$CompletionMarker' and '$CompletionEnd'."
        } else {
            # Remove lines from the marker to the end marker (inclusive).
            $lines = Get-Content -Path $PROFILE
            $out = [System.Collections.Generic.List[string]]::new()
            $inBlock = $false
            foreach ($line in $lines) {
                if ($line.Trim() -eq $CompletionMarker) { $inBlock = $true; continue }
                if ($inBlock) {
                    if ($line.Trim() -eq $CompletionEnd) { $inBlock = $false }
                    continue
                }
                $out.Add($line)
            }
            Set-Content -Path $PROFILE -Value $out -Encoding UTF8
            Write-Success "Removed completion source block from $PROFILE"
        }
    } else {
        Write-Info "No completion block found in $PROFILE — skipping."
    }
}

# Remove the completions directory if empty.
if (Test-Path $CompletionsDir) {
    $compContents = Get-ChildItem -LiteralPath $CompletionsDir -Force
    if ($compContents.Count -eq 0) {
        Remove-Item -LiteralPath $CompletionsDir -Force
        Write-Success "Removed empty directory: $CompletionsDir"
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
